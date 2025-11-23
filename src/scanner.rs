use crate::http::send_request;
use crate::model::CheckResult;
use chrono::Utc;
use colored::*;
use indicatif::ProgressBar;
use std::error::Error;
use std::time::Duration;

// Detection thresholds
const TIMING_MULTIPLIER: u128 = 3; // Flag if response is 3x slower than baseline
const MIN_DELAY_MS: u128 = 1000;   // Minimum delay to consider (1 second)

/// Parameters for running vulnerability checks
pub struct CheckParams<'a> {
    pub pb: &'a ProgressBar,
    pub check_name: &'a str,
    pub host: &'a str,
    pub port: u16,
    pub path: &'a str,
    pub attack_requests: Vec<String>,
    pub timeout: u64,
    pub verbose: bool,
    pub use_tls: bool,
}

/// Runs a set of attack requests for a given check type.
pub async fn run_checks_for_type(params: CheckParams<'_>) -> Result<CheckResult, Box<dyn Error>> {
    if !params.verbose {
        params.pb.set_message(format!("Checking for {}...", params.check_name));
    } else {
        println!("\n{}", format!("[!] Checking for {} vulnerability", params.check_name).bold());
    }
    
    let (normal_response, normal_duration) = send_request(params.host, params.port, &format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        params.path, params.host
    ), params.timeout, params.verbose, params.use_tls).await?;
    let normal_status = normal_response.lines().next().unwrap_or("").to_string();

    let mut vulnerable = false;
    let mut result_payload_index = None;
    let mut result_attack_status = None;
    let mut last_attack_duration = None;
    
    // Threshold for detecting timing-based smuggling
    let timing_threshold = normal_duration.as_millis() * TIMING_MULTIPLIER;
    
    for (i, attack_request) in params.attack_requests.iter().enumerate() {
        match send_request(params.host, params.port, attack_request, params.timeout, params.verbose, params.use_tls).await {
            Ok((attack_response, attack_duration)) => {
                last_attack_duration = Some(attack_duration);
                let attack_status_line = attack_response.lines().next().unwrap_or("");
                let attack_millis = attack_duration.as_millis();
                
                // Extract HTTP status code from status line (e.g., "HTTP/1.1 504 Gateway Timeout")
                // Validate proper HTTP response format before parsing
                let status_code = {
                    let parts: Vec<&str> = attack_status_line.split_whitespace().collect();
                    if parts.len() >= 2 
                        && (parts[0].starts_with("HTTP/1.") || parts[0].starts_with("HTTP/2")) {
                        parts[1].parse::<u16>().ok()
                    } else {
                        None
                    }
                };
                
                // Check for smuggling indicators:
                // 1. Timeout status codes (408 Request Timeout, 504 Gateway Timeout)
                // 2. Significantly delayed response (3x+ slower than baseline AND exceeds minimum threshold)
                let is_timeout_error = matches!(status_code, Some(408) | Some(504));
                let is_delayed = attack_millis > timing_threshold && attack_millis > MIN_DELAY_MS;
                
                if is_timeout_error || is_delayed {
                    vulnerable = true;
                    result_payload_index = Some(i);
                    result_attack_status = Some(attack_status_line.to_string());
                    
                    let result_text = format!("[!] {} Result:", params.check_name);
                    let vulnerable_text = "[!!!] VULNERABLE".red().bold();
                    let reason = if is_timeout_error {
                        "Timeout status code detected (408/504)"
                    } else {
                        "Excessive delay detected (possible desync)"
                    };
                    
                    if params.verbose {
                        println!("\n{}", result_text.bold());
                        println!("  {}", vulnerable_text);
                        println!("  {} Reason: {}", "[+] ".green(), reason.yellow());
                        println!("  {} Payload index: {}", "[+] ".green(), i);
                        println!("  {} Normal response: {} (took {:.2?})", "[+] ".green(), normal_status, normal_duration);
                        println!("  {} Attack response: {} (took {:.2?})", "[+] ".green(), attack_status_line, attack_duration);
                    } else {
                        params.pb.println(format!("\n{}", result_text.bold()));
                        params.pb.println(format!("  {}", vulnerable_text));
                        params.pb.println(format!("  {} Reason: {}", "[+] ".green(), reason.yellow()));
                        params.pb.println(format!("  {} Payload index: {}", "[+] ".green(), i));
                        params.pb.println(format!("  {} Normal response: {} (took {:.2?})", "[+] ".green(), normal_status, normal_duration));
                        params.pb.println(format!("  {} Attack response: {} (took {:.2?})", "[+] ".green(), attack_status_line, attack_duration));
                    }
                    break;
                }
            }
            Err(e) => {
                // Check if error is a timeout error by examining the error chain
                // Priority: tokio timeout errors, then IO timeout errors, then string fallback
                let is_timeout = e.downcast_ref::<tokio::time::error::Elapsed>().is_some() || {
                    // Check the error chain for IO timeout errors
                    let mut source = e.source();
                    let mut found_io_timeout = false;
                    while let Some(err) = source {
                        if let Some(io_err) = err.downcast_ref::<std::io::Error>() 
                            && io_err.kind() == std::io::ErrorKind::TimedOut {
                            found_io_timeout = true;
                            break;
                        }
                        source = err.source();
                    }
                    
                    found_io_timeout || {
                        // Last resort: string matching for other timeout errors
                        let error_str = e.to_string().to_lowercase();
                        error_str.contains("timed out") || error_str.contains("timeout")
                    }
                };
                
                if is_timeout {
                    vulnerable = true;
                    result_payload_index = Some(i);
                    result_attack_status = Some("Connection Timeout".to_string());
                    last_attack_duration = Some(Duration::from_secs(params.timeout));
                    
                    let result_text = format!("[!] {} Result:", params.check_name);
                    let vulnerable_text = "[!!!] VULNERABLE".red().bold();
                    if params.verbose {
                        println!("\n{}", result_text.bold());
                        println!("  {}", vulnerable_text);
                        println!("  {} Reason: {}", "[+] ".green(), "Connection timeout (desync hang detected)".yellow());
                        println!("  {} Payload index: {}", "[+] ".green(), i);
                        println!("  {} Normal response: {} (took {:.2?})", "[+] ".green(), normal_status, normal_duration);
                        println!("  {} Attack request timed out after {:.2?}", "[+] ".green(), Duration::from_secs(params.timeout));
                    } else {
                        params.pb.println(format!("\n{}", result_text.bold()));
                        params.pb.println(format!("  {}", vulnerable_text));
                        params.pb.println(format!("  {} Reason: {}", "[+] ".green(), "Connection timeout (desync hang detected)".yellow()));
                        params.pb.println(format!("  {} Payload index: {}", "[+] ".green(), i));
                        params.pb.println(format!("  {} Normal response: {} (took {:.2?})", "[+] ".green(), normal_status, normal_duration));
                        params.pb.println(format!("  {} Attack request timed out after {:.2?}", "[+] ".green(), Duration::from_secs(params.timeout)));
                    }
                    break;
                } else {
                    let error_text = format!("\n{} Error during {} attack request (payload {}): {}", "[!] ".yellow(), params.check_name, i, e);
                    if params.verbose { println!("{}", error_text); } else { params.pb.println(error_text); }
                }
            }
        }
    }

    if !vulnerable {
        let result_text = format!("[!] {} Result:", params.check_name);
        let not_vulnerable_text = "[+] Not Vulnerable".green();
        if params.verbose {
            println!("\n{}", result_text.bold());
            println!("  {}", not_vulnerable_text);
        } else {
            params.pb.println(format!("\n{}", result_text.bold()));
            params.pb.println(format!("  {}", not_vulnerable_text));
        }
    }

    Ok(CheckResult {
        check_type: params.check_name.to_string(),
        vulnerable,
        payload_index: result_payload_index,
        normal_status,
        attack_status: result_attack_status,
        normal_duration_ms: normal_duration.as_millis() as u64,
        attack_duration_ms: last_attack_duration.map(|d| d.as_millis() as u64),
        timestamp: Utc::now().to_rfc3339(),
    })
}
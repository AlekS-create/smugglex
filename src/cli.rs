use clap::Parser;

/// HTTP Request Smuggling tester
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Target URL
    #[arg(required = true)]
    pub url: String,

    /// Custom method for the attack request
    #[arg(short, long, default_value = "POST")]
    pub method: String,

    /// Socket timeout in seconds
    #[arg(short, long, default_value_t = 10)]
    pub timeout: u64,

    /// Verbose mode
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    /// Output file for results (JSON format)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Custom headers (format: "Header: Value")
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,

    /// Specify which checks to run (comma-separated: cl-te,te-cl,te-te)
    #[arg(short = 'c', long = "checks")]
    pub checks: Option<String>,
}

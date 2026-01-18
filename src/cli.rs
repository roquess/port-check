//! Command Line Interface using clap
//! pc [OPTIONS] <PORT>
use clap::Parser;
use loki_weave::OutputFormat;

/// Command line arguments structure
#[derive(Parser, Debug)]
#[command(
    name = "pc",
    version = "0.1.0",
    author = "Steve Roques <steve.roques@gmail.com>",
    about = "Check what's using a port",
    long_about = "Cross-platform tool to inspect which process is listening on a given TCP port.\nSupports multiple output formats: human (default), json, yaml, toml, xml, toon."
)]
pub struct Cli {
    /// TCP port numbers to check (1-65535)
    #[arg(value_parser = validate_port, required = true)]
    pub ports: Vec<u16>,

    /// Output format
    #[arg(
        short = 'f',
        long,
        value_name = "FORMAT",
        default_value = "human",
        help = "Output format: human, json, yaml, toml, xml, toon"
    )]
    pub format: String,

    /// Show extra process information (TTY, start time, uptime)
    #[arg(short = 'x', long)]
    pub extra: bool,
}

/// Validate port numbers (1-65535)
fn validate_port(s: &str) -> Result<u16, String> {
    let port: u16 = s
        .parse()
        .map_err(|_| format!("Port must be a number between 1-65535"))?;
    if port == 0 {
        Err("Port must be between 1-65535".to_string())
    } else {
        Ok(port)
    }
}

impl Cli {
    /// Parse command line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Get the output format, returns None for "human" format
    pub fn get_output_format(&self) -> Option<OutputFormat> {
        match self.format.as_str() {
            "human" => None,
            other => OutputFormat::from_str(other),
        }
    }

    /// Check if using human format
    pub fn is_human_format(&self) -> bool {
        self.format == "human"
    }

    /// Check if using structured format (non-human)
    pub fn is_structured_format(&self) -> bool {
        !self.is_human_format()
    }
}

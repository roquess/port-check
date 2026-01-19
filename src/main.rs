/*!
Port Check CLI - Cross-platform port usage inspector
Supports human, JSON, YAML, TOML, XML, Toon output formats
*/
mod cli;
mod format;
mod platform;
mod types;

use cli::Cli;
use colored::Colorize;

/// Main entry point
fn main() {
    let cli = Cli::parse_args();

    for port in &cli.ports {
        match platform::check_port(*port) {
            Ok(full_infos) => {
                print_output(*port, &full_infos, &cli);
            }
            Err(e) => {
                handle_error(*port, cli.is_structured_format(), e);
            }
        }
    }
}

/// Print output based on CLI format
fn print_output(port: u16, infos: &[types::ProcessInfo], cli: &Cli) {
    match cli.get_output_format() {
        Some(output_format) => {
            // Format structuré (json, yaml, toml, xml, toon)
            format::print_loki(port, infos, output_format);
        }
        None => {
            // Format humain (défaut)
            format::print_human(port, infos);
        }
    }
}

/// Handle errors consistently across output formats
fn handle_error(port: u16, structured: bool, error: String) {
    if structured {
        // Structured error for scripts/parsing
        let error_json = format!(
            r#"{{"port": {}, "status": "error", "message": "{}"}}"#,
            port,
            error.replace("\"", "\\\"")
        );
        println!("{}", error_json);
    } else {
        // Human-readable error with colors
        eprintln!("{} {}", "Error:".red().bold(), error);
        std::process::exit(1);
    }
}

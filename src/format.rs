use crate::types::ProcessInfo;
use colored::*;
use loki_weave::{OutputFormat, format_data, to_value};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PortStatus {
    port: u16,
    status: String,
    process: String,
    pid: u32,
    user: String,
    command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uptime_seconds: Option<u64>,
}

impl From<&ProcessInfo> for PortStatus {
    fn from(info: &ProcessInfo) -> Self {
        let start_ts = info
            .start_time
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs());
        let uptime = info.uptime.map(|d| d.as_secs());

        Self {
            port: info.port,
            status: "in_use".to_string(),
            process: info.process_name.clone(),
            pid: info.pid,
            user: info.user.clone().unwrap_or_default(),
            command: info.command.clone().unwrap_or_default(),
            tty: info.tty.clone(),
            start_time: start_ts,
            uptime_seconds: uptime,
        }
    }
}

fn prepare_data(port: u16, infos: &[ProcessInfo]) -> Value {
    if infos.is_empty() {
        let status = PortStatus {
            port,
            status: "free".to_string(),
            process: "".to_string(),
            pid: 0,
            user: "".to_string(),
            command: "".to_string(),
            tty: None,
            start_time: None,
            uptime_seconds: None,
        };
        return to_value(&status).unwrap();
    }

    if infos.len() == 1 {
        let status = PortStatus::from(&infos[0]);
        return to_value(&status).unwrap();
    }

    let statuses: Vec<PortStatus> = infos.iter().map(|i| i.into()).collect();
    to_value(&statuses).unwrap()
}

/// Format avec loki_formatter (json, yaml, toml, xml, toon)
pub fn print_loki(port: u16, infos: &[ProcessInfo], format: OutputFormat) {
    let data = prepare_data(port, infos);
    match format_data(&data, format) {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("Error formatting output: {}", e),
    }
}

/// Format humain avec couleurs
pub fn print_human(port: u16, infos: &[ProcessInfo]) {
    if infos.is_empty() {
        println!(
            "{} Port {} is {}",
            "✓".green().bold(),
            port.to_string().cyan(),
            "free".green()
        );
        return;
    }

    for info in infos {
        print_process_block(info);
    }
}

// === Helpers pour format humain ===

fn format_duration(d: std::time::Duration) -> String {
    let secs = d.as_secs();
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    if h > 0 {
        format!("{}:{:02}:{:02}", h, m, s)
    } else {
        format!("{:02}:{:02}", m, s)
    }
}

fn format_since(start: &std::time::SystemTime) -> String {
    std::time::SystemTime::now()
        .duration_since(*start)
        .map(format_duration)
        .unwrap_or_else(|_| "unknown".to_string())
}

fn print_process_block(info: &ProcessInfo) {
    println!(
        "{} Port {} is {}",
        "●".red().bold(),
        info.port.to_string().cyan(),
        "in use".red()
    );
    println!();

    println!("  {}  {}", "Process:".bold(), info.process_name.yellow());
    println!("  {}      {}", "PID:".bold(), info.pid);
    println!(
        "  {}     {}",
        "User:".bold(),
        info.user.clone().unwrap_or_default()
    );

    if let Some(tty) = &info.tty {
        println!("  {}      {}", "TTY:".bold(), tty);
    }
    if let Some(start) = &info.start_time {
        println!("  {}  {}", "Since:".bold(), format_since(start));
    }
    if let Some(up) = &info.uptime {
        println!("  {} {}", "Uptime:".bold(), format_duration(*up));
    }

    println!();
    println!(
        "  {}  {}",
        "Command:".bold(),
        info.command.clone().unwrap_or_default().dimmed()
    );
    println!();
}

use crate::types::FullPortInfo;
use colored::*;
use loki_weave::{format_data, to_value, OutputFormat};
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

impl From<&FullPortInfo> for PortStatus {
    fn from(info: &FullPortInfo) -> Self {
        let start_ts = info
            .extra
            .start_time
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs());
        let uptime = info.extra.uptime.map(|d| d.as_secs());

        Self {
            port: info.base.port,
            status: "in_use".to_string(),
            process: info.base.process_name.clone(),
            pid: info.base.pid,
            user: info.base.user.clone(),
            command: info.base.command.clone(),
            tty: info.extra.tty.clone(),
            start_time: start_ts,
            uptime_seconds: uptime,
        }
    }
}

fn prepare_data(port: u16, infos: &[FullPortInfo]) -> Value {
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
pub fn print_loki(port: u16, infos: &[FullPortInfo], format: OutputFormat) {
    let data = prepare_data(port, infos);
    match format_data(&data, format) {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("Error formatting output: {}", e),
    }
}

/// Format humain avec couleurs
pub fn print_human(port: u16, infos: &[FullPortInfo]) {
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

fn print_process_block(info: &FullPortInfo) {
    println!(
        "{} Port {} is {}",
        "●".red().bold(),
        info.base.port.to_string().cyan(),
        "in use".red()
    );
    println!();

    println!(
        "  {}  {}",
        "Process:".bold(),
        info.base.process_name.yellow()
    );
    println!("  {}      {}", "PID:".bold(), info.base.pid);
    println!("  {}     {}", "User:".bold(), info.base.user);

    if let Some(tty) = &info.extra.tty {
        println!("  {}      {}", "TTY:".bold(), tty);
    }
    if let Some(start) = &info.extra.start_time {
        println!("  {}  {}", "Since:".bold(), format_since(start));
    }
    if let Some(up) = &info.extra.uptime {
        println!("  {} {}", "Uptime:".bold(), format_duration(*up));
    }

    println!();
    println!("  {}  {}", "Command:".bold(), info.base.command.dimmed());
    println!();
}

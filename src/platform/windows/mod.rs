mod process;

use crate::types::ProcessInfo;
use std::process::Command;

pub struct Windows;

impl Windows {
    pub fn check_port(port: u16) -> Result<Vec<ProcessInfo>, String> {
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", &format!("netstat -ano | findstr :{}", port)]);

        let output = cmd.output().map_err(|e| format!("Failed to execute command: {}", e))?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let processes: Vec<ProcessInfo> = stdout
            .lines()
            .filter_map(Self::parse_port_line)
            .collect();

        Ok(processes)
    }

    fn parse_port_line(row: &str) -> Option<ProcessInfo> {
        let parts: Vec<&str> = row.split_whitespace().collect();
        if parts.len() < 5 || parts[0] != "TCP" || parts[3] != "LISTENING" {
            return None;
        }

        let local = parts[1];
        let pid: u32 = parts[4].parse().ok()?;
        let port: u16 = local.rsplit(':').next()?.parse().ok()?;

        let (process_name, command, user) = process::process_info_from_pid(pid);

        Some(ProcessInfo {
            port,
            pid,
            process_name: process_name.unwrap_or_else(|| "unknown".into()),
            user,
            command,
            tty: None,
            start_time: None,
            uptime: None,
        })
    }
}

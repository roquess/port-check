use super::UnixProvider;
use crate::types::ProcessInfo;
use std::process::Command;

pub struct Linux;

impl UnixProvider for Linux {
    fn port_command(port: u16) -> Command {
        let mut cmd = Command::new("ss");
        cmd.args(["-ltnp", &format!("sport = :{}", port)]);
        cmd
    }

    fn parse_port_line(line: &str) -> Option<ProcessInfo> {
        // ss example:
        // LISTEN 0 128 127.0.0.1:8080 0.0.0.0:* users:("nginx",pid=1234,fd=6)
        if !line.contains("pid=") {
            return None;
        }

        let port = line
            .split_whitespace()
            .find(|s| s.contains(':'))
            .and_then(|addr| addr.split(':').last())
            .and_then(|p| p.parse::<u16>().ok())?;

        let pid = line
            .split("pid=")
            .nth(1)
            .and_then(|s| s.split(',').next())
            .and_then(|p| p.parse::<u32>().ok())?;

        let process_name = line.split('"').nth(1).map(|s| s.to_string())?;

        Some(ProcessInfo {
            port,
            pid,
            process_name,
            user: None,
            command: None,
            tty: None,
            start_time: None,
            uptime: None,
        })
    }
}

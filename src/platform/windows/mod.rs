mod process;

use crate::platform::backend::{PlatformProvider, PortSource};
use crate::types::PortInfo;
use std::process::Command;

pub struct Windows;

impl PlatformProvider for Windows {
    fn port_source(port: u16) -> Result<PortSource, String> {
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", &format!("netstat -ano | findstr :{}", port)]);
        Ok(PortSource::Command(cmd))
    }

    fn parse_port(row: &str) -> Option<PortInfo> {
        let parts: Vec<&str> = row.split_whitespace().collect();
        if parts.len() < 5 || parts[0] != "TCP" || parts[3] != "LISTENING" {
            return None;
        }

        let local = parts[1];
        let pid: u32 = parts[4].parse().ok()?;
        let port: u16 = local.rsplit(':').next()?.parse().ok()?;

        let (process_name, command, user) = process::process_info_from_pid(pid);

        Some(PortInfo {
            port,
            process_name: process_name.unwrap_or_else(|| "unknown".into()),
            pid,
            user: user.unwrap_or_default(),
            command: command.unwrap_or_default(),
        })
    }
}


use super::UnixProvider;
use crate::types::ProcessInfo;
use std::process::Command;
use std::fs;

pub struct Linux;

impl UnixProvider for Linux {
    fn port_command(port: u16) -> Command {
        let mut cmd = Command::new("sudo");
        cmd.args(["ss", "-ltnp", &format!("sport = :{}", port)]);
        cmd
    }

    fn parse_port_line(line: &str) -> Option<ProcessInfo> {
        // ss output format (with sudo):
        // LISTEN   0   4096   127.0.0.1:11434   0.0.0.0:*   users:(("ollama",pid=1115,fd=3))
        
        // Skip header line
        if line.starts_with("State") || line.starts_with("Netid") {
            return None;
        }

        if !line.contains("users:") {
            return None;
        }

        // Split by whitespace to get columns
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 {
            return None;
        }

        // Local Address:Port is typically the 4th column (index 3)
        let local_addr = parts.get(3)?;
        let port = local_addr
            .rsplit_once(':')
            .and_then(|(_, p)| p.parse::<u16>().ok())?;

        // Find the users:(...) part
        let users_part = line.split("users:").nth(1)?;
        
        // Extract process name - between ((" and ",pid
        let process_name = users_part
            .split("((\"")
            .nth(1)
            .and_then(|s| s.split("\",pid").next())
            .map(|s| s.to_string())?;

        // Extract PID - after pid= and before ,fd or ))
        let pid = users_part
            .split("pid=")
            .nth(1)
            .and_then(|s| s.split(',').next())
            .and_then(|p| p.parse::<u32>().ok())?;

        // Get additional info from /proc
        let user = get_process_user(pid);
        let command = get_process_command(pid);

        Some(ProcessInfo {
            port,
            pid,
            process_name,
            user,
            command,
            tty: None,
            start_time: None,
            uptime: None,
        })
    }
}

fn get_process_user(pid: u32) -> Option<String> {
    // Read /proc/[pid]/status to get UID, then look up username
    let status_path = format!("/proc/{}/status", pid);
    let status = fs::read_to_string(status_path).ok()?;
    
    let uid = status
        .lines()
        .find(|line| line.starts_with("Uid:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|uid_str| uid_str.parse::<u32>().ok())?;
    
    // Try to get username from /etc/passwd
    if let Ok(passwd) = fs::read_to_string("/etc/passwd") {
        for line in passwd.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                if let Ok(line_uid) = parts[2].parse::<u32>() {
                    if line_uid == uid {
                        return Some(parts[0].to_string());
                    }
                }
            }
        }
    }
    
    // Fallback to UID as string
    Some(uid.to_string())
}

fn get_process_command(pid: u32) -> Option<String> {
    // Read /proc/[pid]/cmdline
    let cmdline_path = format!("/proc/{}/cmdline", pid);
    let cmdline = fs::read_to_string(cmdline_path).ok()?;
    
    // cmdline uses null bytes as separators
    let cmd = cmdline
        .split('\0')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    
    if cmd.is_empty() {
        None
    } else {
        Some(cmd)
    }
}

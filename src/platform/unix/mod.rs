use crate::types::ProcessInfo;
use std::process::Command;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(any(
    target_os = "macos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd"
))]
mod bsd;

trait UnixProvider {
    fn port_command(port: u16) -> Command;
    fn parse_port_line(line: &str) -> Option<ProcessInfo>;
}

pub fn check_port(port: u16) -> Result<Vec<ProcessInfo>, String> {
    #[cfg(target_os = "linux")]
    type Provider = linux::Linux;

    #[cfg(any(
        target_os = "macos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    type Provider = bsd::Bsd;

    let output = Provider::port_command(port)
        .output()
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if let Some(info) = Provider::parse_port_line(line) {
            results.push(info);
        }
    }

    Ok(results
        .into_iter()
        .filter(|i| i.port == port)
        .collect())
}

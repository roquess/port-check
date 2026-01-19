use crate::types::ProcessInfo;
use std::process::Command;

pub enum PortSource {
    Command(Command),
}

pub trait PlatformProvider {
    fn port_source(port: u16) -> Result<PortSource, String>;
    fn parse_port(row: &str) -> Option<ProcessInfo>;
}

pub fn run<P: PlatformProvider>(port: u16) -> Result<Vec<ProcessInfo>, String> {
    let source = P::port_source(port)?;
    let rows: Vec<String> = match source {
        PortSource::Command(mut cmd) => {
            let out = cmd.output().map_err(|e| e.to_string())?;
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .map(|s| s.to_string())
                .collect()
        }
    };

    let mut results = Vec::new();

    for row in rows {
        if let Some(port_info) = P::parse_port(&row) {
            results.push(port_info);
        }
    }

    Ok(results)
}

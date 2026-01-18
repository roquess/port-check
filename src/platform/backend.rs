use crate::types::{FullPortInfo, PortInfo, ProcessExtra};
use std::process::Command;

pub enum PortSource {
    Command(Command),
}

pub trait PlatformProvider {
    fn port_source(port: u16) -> Result<PortSource, String>;
    fn parse_port(row: &str) -> Option<PortInfo>;
}

pub fn run<P: PlatformProvider>(port: u16) -> Result<Vec<FullPortInfo>, String> {
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
        if let Some(base) = P::parse_port(&row) {
            results.push(FullPortInfo {
                base,
                extra: ProcessExtra::default(),
            });
        }
    }

    Ok(results)
}

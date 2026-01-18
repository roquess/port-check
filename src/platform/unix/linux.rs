use super::UnixProvider;
use crate::types::{BasePortInfo, ProcessExtra};
use std::process::Command;

pub struct Linux;

impl UnixProvider for Linux {
    fn port_command(port: u16) -> Command {
        let mut cmd = Command::new("ss");
        cmd.args(["-ltnp", &format!("sport = :{}", port)]);
        cmd
    }

    fn parse_port_line(_line: &str) -> Option<BasePortInfo> {
        None
    }

    fn enrich_process(_pid: u32) -> ProcessExtra {
        ProcessExtra::default()
    }
}

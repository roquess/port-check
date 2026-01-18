use super::UnixProvider;
use crate::types::{BasePortInfo, ProcessExtra};
use std::process::Command;

pub struct Bsd;

impl UnixProvider for Bsd {
    fn port_command(port: u16) -> Command {
        let mut cmd = Command::new("lsof");
        cmd.args(["-nP", "-iTCP", &format!(":{}", port), "-sTCP:LISTEN"]);
        cmd
    }

    fn parse_port_line(_line: &str) -> Option<BasePortInfo> {
        None
    }

    fn enrich_process(_pid: u32) -> ProcessExtra {
        ProcessExtra::default()
    }
}

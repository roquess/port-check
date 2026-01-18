use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
pub struct PortInfo {
    pub port: u16,
    pub process_name: String,
    pub pid: u32,
    pub user: String,
    pub command: String,
}

#[derive(Debug, Clone)]
pub struct ProcessExtra {
    pub tty: Option<String>,
    pub start_time: Option<SystemTime>,
    pub uptime: Option<Duration>,
}

impl Default for ProcessExtra {
    fn default() -> Self {
        Self {
            tty: None,
            start_time: None,
            uptime: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FullPortInfo {
    pub base: PortInfo,
    pub extra: ProcessExtra,
}

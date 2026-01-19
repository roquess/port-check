use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub port: u16,
    pub pid: u32,
    pub process_name: String,
    pub user: Option<String>,
    pub command: Option<String>,
    pub tty: Option<String>,
    pub start_time: Option<SystemTime>,
    pub uptime: Option<Duration>,
}

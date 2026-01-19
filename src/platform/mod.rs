use crate::types::ProcessInfo;

mod backend;

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

use backend::run;

pub fn check_port(port: u16) -> Result<Vec<ProcessInfo>, String> {
    #[cfg(windows)]
    {
        return run::<windows::Windows>(port);
    }

    #[cfg(unix)]
    {
        return unix::check_port(port);
    }
}

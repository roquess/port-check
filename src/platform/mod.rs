use crate::types::ProcessInfo;

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

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

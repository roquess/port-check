use std::process::Command;

/// Returns (process_name, command_line, user)
pub fn process_info_from_pid(pid: u32) -> (Option<String>, Option<String>, Option<String>) {
    let name = get_process_name(pid);
    let command = get_command_line(pid);
    let user = get_process_user(pid);

    (name, command, user)
}

fn get_process_name(pid: u32) -> Option<String> {
    ps_utf8(&format!("(Get-Process -Id {}).ProcessName", pid)).map(|s| format!("{}.exe", s.trim()))
}

fn get_command_line(pid: u32) -> Option<String> {
    ps_utf8(&format!(
        "(Get-CimInstance Win32_Process -Filter \"ProcessId={}\").CommandLine",
        pid
    ))
}

fn get_process_user(pid: u32) -> Option<String> {
    // SOLUTION 1: tasklist /V (verbose) - TOUTES les colonnes visibles
    let output = Command::new("tasklist")
        .args(&["/FI", &format!("PID eq {}", pid), "/V", "/NH"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Colonne 12 = User Name dans /V
    let parts: Vec<&str> = stdout.split_whitespace().collect();

    if parts.len() >= 12 {
        let user_candidate = parts[11].trim(); // Index 11 = colonne 12
        if !user_candidate.is_empty()
            && user_candidate != "N/A"
            && !user_candidate.contains("Services")
        {
            return Some(user_candidate.to_string());
        }
    }

    Some(get_current_user())
}

fn get_current_user() -> String {
    std::env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string())
}

fn ps_utf8(cmd: &str) -> Option<String> {
    let full_cmd = format!(
        "[Console]::OutputEncoding=[Text.UTF8Encoding]::new(); {}",
        cmd
    );

    let out = Command::new("powershell")
        .args(["-NoProfile", "-Command", &full_cmd])
        .output()
        .ok()?;

    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() { None } else { Some(s) }
}

use chrono::Local;
use std::process::Command;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Ok,
    Warn,
    Error,
    Start,
    End,
}

#[derive(Debug, Clone)]
pub struct LogMessage {
    pub time: String,
    pub level: LogLevel,
    pub message: String,
}

impl LogMessage {
    pub fn new(level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            time: Local::now().format("%H:%M:%S").to_string(),
            level,
            message: message.into(),
        }
    }
}

pub fn is_admin() -> bool {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("net")
            .arg("session")
            .output();
        if let Ok(out) = output {
            return out.status.success();
        }
        false
    }
    #[cfg(not(target_os = "windows"))]
    {
        true
    }
}

pub fn run_native_cmd(program: &str, args: &[&str]) -> (bool, String) {
    let output = Command::new(program)
        .args(args)
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            let full_out = if stderr.is_empty() { stdout } else { format!("{}\n{}", stdout, stderr) };
            (out.status.success(), full_out.trim().to_string())
        }
        Err(e) => (false, e.to_string()),
    }
}

pub fn run_powershell_cmd(cmd_str: &str) -> (bool, String) {
    #[cfg(target_os = "windows")]
    let program = "powershell";
    #[cfg(not(target_os = "windows"))]
    let program = "pwsh";

    run_native_cmd(program, &["-NoProfile", "-Command", cmd_str])
}

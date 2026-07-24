use chrono::Local;
use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

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
        let output = Command::new("net").arg("session").output();
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
    run_native_cmd_timeout(program, args, 60)
}

pub fn run_native_cmd_timeout(program: &str, args: &[&str], timeout_secs: u64) -> (bool, String) {
    let mut child = match Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => return (false, format!("进程启动失败: {}", e)),
    };

    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut stdout_bytes = Vec::new();
                let mut stderr_bytes = Vec::new();
                if let Some(mut out) = child.stdout.take() {
                    let _ = out.read_to_end(&mut stdout_bytes);
                }
                if let Some(mut err) = child.stderr.take() {
                    let _ = err.read_to_end(&mut stderr_bytes);
                }
                let stdout = String::from_utf8_lossy(&stdout_bytes).to_string();
                let stderr = String::from_utf8_lossy(&stderr_bytes).to_string();
                let full_out = if stderr.is_empty() {
                    stdout
                } else {
                    format!("{}\n{}", stdout, stderr)
                };
                return (status.success(), full_out.trim().to_string());
            }
            Ok(None) => {
                if start.elapsed() > Duration::from_secs(timeout_secs) {
                    let _ = child.kill();
                    return (false, format!("执行超时 (已超时 {} 秒)", timeout_secs));
                }
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => return (false, e.to_string()),
        }
    }
}

pub fn run_powershell_cmd(cmd_str: &str) -> (bool, String) {
    run_powershell_cmd_timeout(cmd_str, 60)
}

pub fn run_powershell_cmd_timeout(cmd_str: &str, timeout_secs: u64) -> (bool, String) {
    #[cfg(target_os = "windows")]
    let program = "powershell";
    #[cfg(not(target_os = "windows"))]
    let program = "pwsh";

    let utf8_cmd = format!(
        "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $OutputEncoding = [System.Text.Encoding]::UTF8; {}",
        cmd_str
    );

    run_native_cmd_timeout(program, &["-NoProfile", "-NonInteractive", "-Command", &utf8_cmd], timeout_secs)
}

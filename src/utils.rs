use chrono::Local;
use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(not(target_os = "windows"))]
fn command_with_tool_paths(program: &str) -> Command {
    Command::new(program)
}

#[cfg(target_os = "windows")]
fn command_with_tool_paths(program: &str) -> Command {
    let mut command = Command::new(program);
    let mut paths: Vec<_> = std::env::var_os("PATH").map(|value| std::env::split_paths(&value).collect()).unwrap_or_default();

    let mut add = |base: Option<std::ffi::OsString>, suffix: &[&str]| {
        if let Some(base) = base {
            let path = suffix.iter().fold(std::path::PathBuf::from(base), |path, part| path.join(part));
            if !paths.contains(&path) {
                paths.push(path);
            }
        }
    };

    add(std::env::var_os("LOCALAPPDATA"), &["Microsoft", "WindowsApps"]);
    add(std::env::var_os("USERPROFILE"), &["scoop", "shims"]);
    add(std::env::var_os("USERPROFILE"), &[".cargo", "bin"]);
    add(std::env::var_os("USERPROFILE"), &[".local", "bin"]);
    add(std::env::var_os("APPDATA"), &["npm"]);

    if let Ok(path) = std::env::join_paths(paths) {
        command.env("PATH", path);
    }

    command
}

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
        Self { time: Local::now().format("%H:%M:%S").to_string(), level, message: message.into() }
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
    let mut child = match command_with_tool_paths(program).args(args).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn() {
        Ok(c) => c,
        Err(e) => return (false, format!("进程启动失败: {}", e)),
    };

    let stdout = child.stdout.take().map(|mut pipe| {
        thread::spawn(move || {
            let mut bytes = Vec::new();
            let _ = pipe.read_to_end(&mut bytes);
            bytes
        })
    });
    let stderr = child.stderr.take().map(|mut pipe| {
        thread::spawn(move || {
            let mut bytes = Vec::new();
            let _ = pipe.read_to_end(&mut bytes);
            bytes
        })
    });

    let start = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Ok(status),
            Ok(None) => {
                if start.elapsed() > Duration::from_secs(timeout_secs) {
                    let _ = child.kill();
                    let _ = child.wait();
                    break Err(format!("执行超时 (已超时 {} 秒)", timeout_secs));
                }
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                break Err(e.to_string());
            }
        }
    };

    let stdout = stdout.and_then(|reader| reader.join().ok()).unwrap_or_default();
    let stderr = stderr.and_then(|reader| reader.join().ok()).unwrap_or_default();
    let output = [stdout, stderr].concat();
    let output = String::from_utf8_lossy(&output).trim().to_string();

    match status {
        Ok(status) => (status.success(), output),
        Err(error) if output.is_empty() => (false, error),
        Err(error) => (false, format!("{}\n{}", error, output)),
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

    let utf8_cmd = format!("[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $OutputEncoding = [System.Text.Encoding]::UTF8; {}", cmd_str);

    run_native_cmd_timeout(program, &["-NoProfile", "-NonInteractive", "-Command", &utf8_cmd], timeout_secs)
}

#[cfg(test)]
mod tests {
    use super::run_native_cmd_timeout;

    #[test]
    fn drains_large_stdout_and_stderr_without_timing_out() {
        let executable = std::env::current_exe().unwrap();
        let executable = executable.to_str().unwrap();
        let args = ["--exact", "utils::tests::emit_large_output", "--ignored", "--nocapture"];

        let (ok, output) = run_native_cmd_timeout(executable, &args, 10);

        assert!(ok, "child failed: {output}");
        assert!(output.contains("large-output-complete"));
    }

    #[test]
    #[ignore]
    fn emit_large_output() {
        for _ in 0..5_000 {
            println!("stdout-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
            eprintln!("stderr-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        }
        println!("large-output-complete");
    }
}

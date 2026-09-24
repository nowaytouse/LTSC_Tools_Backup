#[cfg(target_os = "windows")]
#[allow(dead_code)]
#[path = "../config.rs"]
mod config;
#[cfg(target_os = "windows")]
#[allow(dead_code)]
#[path = "../inventory.rs"]
mod inventory;
#[cfg(target_os = "windows")]
#[allow(dead_code)]
#[path = "../utils.rs"]
mod utils;

#[cfg(target_os = "windows")]
use config::SetupProfile;
#[cfg(target_os = "windows")]
use utils::{run_native_cmd_timeout, CancellationToken};

#[cfg(target_os = "windows")]
fn main() -> anyhow::Result<()> {
    let profile = SetupProfile::load_default();
    profile.validate()?;

    let mut failures = Vec::new();
    for app in profile
        .packages
        .winget_core
        .iter()
        .chain(&profile.packages.winget_dev)
    {
        let result = run_native_cmd_timeout(
            "winget.exe",
            &[
                "search",
                "--id",
                &app.id,
                "-e",
                "--source",
                "winget",
                "--accept-source-agreements",
                "--disable-interactivity",
            ],
            120,
            &CancellationToken::default(),
        );
        if !(result.succeeded()
            && result
                .output
                .to_ascii_lowercase()
                .contains(&app.id.to_ascii_lowercase()))
        {
            failures.push(format!("WinGet {}: {}", app.id, result.diagnostic()));
        }
    }

    let checks = build_http_checks(&profile);
    failures.extend(run_http_checks(checks, 8));
    if failures.is_empty() {
        println!(
            "Profile 在线审计通过：{} WinGet，{} Scoop，{} Cargo，{} NPM，{} Pip/UV",
            profile.packages.winget_core.len() + profile.packages.winget_dev.len(),
            profile.packages.scoop_tools.len(),
            profile.packages.cargo_packages.len(),
            profile.packages.npm_globals.len(),
            profile.packages.pip_packages.len() + profile.packages.uv_tools.len()
        );
        return Ok(());
    }

    for failure in &failures {
        eprintln!("{failure}");
    }
    anyhow::bail!("Profile 在线审计失败：{} 项", failures.len())
}

#[cfg(target_os = "windows")]
#[derive(Debug)]
struct HttpCheck {
    label: String,
    urls: Vec<String>,
}

#[cfg(target_os = "windows")]
fn build_http_checks(profile: &SetupProfile) -> Vec<HttpCheck> {
    let mut checks = Vec::new();
    for package in &profile.packages.scoop_tools {
        checks.push(HttpCheck {
            label: format!("Scoop {package}"),
            urls: ["Main", "Extras", "Versions"]
                .into_iter()
                .map(|bucket| {
                    format!(
                        "https://raw.githubusercontent.com/ScoopInstaller/{bucket}/master/bucket/{package}.json"
                    )
                })
                .collect(),
        });
    }
    for package in &profile.packages.cargo_packages {
        checks.push(HttpCheck {
            label: format!("Cargo {package}"),
            urls: vec![format!("https://crates.io/api/v1/crates/{package}")],
        });
    }
    for package in &profile.packages.npm_globals {
        checks.push(HttpCheck {
            label: format!("NPM {package}"),
            urls: vec![format!(
                "https://registry.npmjs.org/{}",
                package.replace('/', "%2F")
            )],
        });
    }
    for package in profile
        .packages
        .pip_packages
        .iter()
        .chain(&profile.packages.uv_tools)
    {
        checks.push(HttpCheck {
            label: format!("PyPI {package}"),
            urls: vec![format!("https://pypi.org/pypi/{package}/json")],
        });
    }
    checks
}

#[cfg(target_os = "windows")]
fn run_http_checks(checks: Vec<HttpCheck>, workers: usize) -> Vec<String> {
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex};

    let queue = Arc::new(Mutex::new(VecDeque::from(checks)));
    let failures = Arc::new(Mutex::new(Vec::new()));
    let mut threads = Vec::new();
    for _ in 0..workers {
        let queue = Arc::clone(&queue);
        let failures = Arc::clone(&failures);
        threads.push(std::thread::spawn(move || loop {
            let Some(check) = queue.lock().expect("audit queue poisoned").pop_front() else {
                break;
            };
            let mut diagnostics = Vec::new();
            let found = check.urls.iter().any(|url| {
                let result = run_native_cmd_timeout(
                    "curl.exe",
                    &[
                        "--fail",
                        "--silent",
                        "--show-error",
                        "--location",
                        "--retry",
                        "2",
                        "--retry-all-errors",
                        "--retry-delay",
                        "1",
                        "--user-agent",
                        "LTSCWorkspace/2.2 (+https://github.com/nowaytouse/LTSC_Tools_Backup)",
                        "--max-time",
                        "20",
                        "--output",
                        "NUL",
                        url,
                    ],
                    45,
                    &CancellationToken::default(),
                );
                if result.succeeded() {
                    true
                } else {
                    diagnostics.push(format!("{url}: {}", result.diagnostic()));
                    false
                }
            });
            if !found {
                failures
                    .lock()
                    .expect("audit failures poisoned")
                    .push(format!("{}: {}", check.label, diagnostics.join(" | ")));
            }
        }));
    }
    for thread in threads {
        let _ = thread.join();
    }
    Arc::try_unwrap(failures)
        .expect("audit failure list still shared")
        .into_inner()
        .expect("audit failures poisoned")
}

#[cfg(not(target_os = "windows"))]
fn main() -> anyhow::Result<()> {
    anyhow::bail!("profile_audit 需要 Windows/WinGet，完整在线审计由 Windows CI 执行")
}

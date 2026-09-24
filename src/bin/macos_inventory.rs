#[cfg(target_os = "macos")]
#[allow(dead_code)]
#[path = "../config.rs"]
mod config;
#[cfg(target_os = "macos")]
#[allow(dead_code)]
#[path = "../inventory.rs"]
mod inventory;
#[cfg(target_os = "macos")]
#[allow(dead_code)]
#[path = "../utils.rs"]
mod utils;

#[cfg(target_os = "macos")]
use config::SetupProfile;
#[cfg(target_os = "macos")]
use inventory::MacosInventory;
#[cfg(target_os = "macos")]
use std::path::PathBuf;
#[cfg(target_os = "macos")]
use utils::{run_native_cmd_timeout, CancellationToken};

#[cfg(target_os = "macos")]
fn main() -> anyhow::Result<()> {
    let argument = std::env::args_os().nth(1);
    let sync = argument.as_deref() == Some(std::ffi::OsStr::new("--sync"));
    if sync {
        let root = git(&["rev-parse", "--show-toplevel"])?;
        std::env::set_current_dir(root.trim())?;
    }
    let output = if sync {
        PathBuf::from("src/assets/macos_inventory.json")
    } else {
        argument
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("src/assets/macos_inventory.json"))
    };
    if sync {
        let status = git(&["status", "--porcelain=v1", "--untracked-files=all"])?;
        if !status.trim().is_empty() {
            anyhow::bail!("工作区有未提交改动；请先处理这些改动，再运行 --sync");
        }
        git(&["pull", "--ff-only"])?;
    }
    let mut inventory = MacosInventory::capture()?;
    let mut changed = true;
    if sync {
        let previous = MacosInventory::load_file(&output)?;
        if inventory
            .warnings
            .iter()
            .any(|warning| warning.starts_with("VS Code extensions 未采集"))
        {
            inventory.vscode_extensions = previous.vscode_extensions.clone();
        }
        if inventory
            .warnings
            .iter()
            .any(|warning| warning.starts_with("Cursor extensions 未采集"))
        {
            inventory.cursor_extensions = previous.cursor_extensions.clone();
        }
        changed = !same_except_capture_time(&inventory, &previous);
    }
    inventory.validate()?;
    let report = SetupProfile::load_default().parity_report(&inventory)?;
    if sync {
        let failed_capture = inventory
            .warnings
            .iter()
            .filter(|warning| {
                (warning.contains("未采集") || warning.contains("解析失败"))
                    && !warning.starts_with("VS Code extensions 未采集")
                    && !warning.starts_with("Cursor extensions 未采集")
            })
            .collect::<Vec<_>>();
        if !failed_capture.is_empty() {
            anyhow::bail!(
                "采集不完整，未发布清单：{}",
                failed_capture
                    .iter()
                    .map(|warning| warning.as_str())
                    .collect::<Vec<_>>()
                    .join("；")
            );
        }
        if !report.unmapped.is_empty() {
            anyhow::bail!(
                "新工具尚未建立 Windows 映射，未发布清单：{}",
                report.unmapped.join("；")
            );
        }
    }
    if changed {
        inventory.save_atomic(&output)?;
        println!(
            "已更新 {}：{} 个工具",
            output.display(),
            inventory.item_count()
        );
    } else {
        println!("清单无变化：{}", output.display());
    }
    println!(
        "Windows 对等：{} 自动、{} 内置/替代、{} Mac 专属、{} 待手动、{} 未映射",
        report.automatic,
        report.compatible,
        report.mac_only,
        report.manual.len(),
        report.unmapped.len()
    );
    if !report.unmapped.is_empty() {
        eprintln!("未映射：{}", report.unmapped.join("；"));
    }
    if sync {
        let status = git(&[
            "status",
            "--porcelain=v1",
            "--",
            "src/assets/macos_inventory.json",
        ])?;
        if !status.trim().is_empty() {
            git(&[
                "commit",
                "--only",
                "-m",
                "chore: refresh Mac tool inventory",
                "--",
                "src/assets/macos_inventory.json",
            ])?;
        }
        git(&["push"])?;
        let status = git(&["status", "--porcelain=v1", "--untracked-files=all"])?;
        if !status.trim().is_empty() {
            anyhow::bail!("清单已处理，但工作区仍有改动：{status}");
        }
        let ahead_behind = git(&["rev-list", "--left-right", "--count", "HEAD...@{upstream}"])?;
        if ahead_behind.split_whitespace().collect::<Vec<_>>() != ["0", "0"] {
            anyhow::bail!("远端尚未与本机同步：{ahead_behind}");
        }
        println!("Mac 清单已与远端同步；工作区干净");
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn same_except_capture_time(current: &MacosInventory, previous: &MacosInventory) -> bool {
    let mut comparable = current.clone();
    comparable.captured_at = previous.captured_at.clone();
    &comparable == previous
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::{same_except_capture_time, MacosInventory};

    #[test]
    fn unchanged_inventory_does_not_create_timestamp_only_commit() {
        let previous = MacosInventory::load_embedded().unwrap();
        let mut current = previous.clone();
        current.captured_at = "later".into();
        assert!(same_except_capture_time(&current, &previous));
        current.homebrew_formulae.push("new-tool".into());
        assert!(!same_except_capture_time(&current, &previous));
    }
}

#[cfg(target_os = "macos")]
fn git(args: &[&str]) -> anyhow::Result<String> {
    let result = run_native_cmd_timeout("git", args, 120, &CancellationToken::default());
    if !result.succeeded() {
        anyhow::bail!("git {} 失败：{}", args.join(" "), result.diagnostic());
    }
    Ok(result.output)
}

#[cfg(not(target_os = "macos"))]
fn main() -> anyhow::Result<()> {
    anyhow::bail!("macos_inventory 只能在源 Mac 上运行")
}

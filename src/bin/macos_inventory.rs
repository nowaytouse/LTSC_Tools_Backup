#[cfg(target_os = "macos")]
#[allow(dead_code)]
#[path = "../inventory.rs"]
mod inventory;
#[cfg(target_os = "macos")]
#[allow(dead_code)]
#[path = "../utils.rs"]
mod utils;

#[cfg(target_os = "macos")]
use inventory::MacosInventory;
#[cfg(target_os = "macos")]
use std::path::PathBuf;

#[cfg(target_os = "macos")]
fn main() -> anyhow::Result<()> {
    let output = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("src/assets/macos_inventory.json"));
    let inventory = MacosInventory::capture()?;
    inventory.save_atomic(&output)?;
    println!(
        "已写入 {}：{} 个工具，{} 条采集警告",
        output.display(),
        inventory.item_count(),
        inventory.warnings.len()
    );
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn main() -> anyhow::Result<()> {
    anyhow::bail!("macos_inventory 只能在源 Mac 上运行")
}

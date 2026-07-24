# Windows LTSC Setup Notes

## Summary

The setup flow is consolidated into a single, fully idempotent execution script: `Scripts\00_QuickSetup.ps1`.

## Why The Script Exists

Standard Windows LTSC installations omit key modern capabilities out of the box:

- Microsoft Store and AppInstaller / Winget infrastructure
- Modern developer CLI utilities and AI agent toolchains
- PowerShell 7 and modern shell customizers
- Pre-configured package managers (Winget, Scoop, Cargo, NPM, UV)
- System tweaks such as NTFS Long Paths and Developer Mode

## Embedded Strategy

The setup script executes the following sequence:

1. **Network Optimization**: Hardens TLS 1.2/1.3, flushes DNS, and tunes TCP stack based on the specified `-NetworkMode`.
2. **Package Manager Bootstrap**: Checks and provisions NuGet, PowerShellGet, Microsoft Store Appx manifests, Winget dependencies, Scoop, and Chocolatey.
3. **Store & AppX Repair**: Re-registers AppX manifests for Store apps and Start menu visibility.
4. **Built-In App Restoration**: Restores Calculator, Photos, Paint, Snipping Tool, and Windows Terminal.
5. **Optional Features Audit**: Checks and enables Windows Sandbox and WSL.
6. **Core Desktop Applications**: Installs baseline utilities (7-Zip, VLC, Chrome, Notepad++, ShareX, IrfanView).
7. **Developer Environment (macOS Feature Parity)**:
   - Installs IDEs (VS Code, Cursor) and desktop tools (Podman, Krita, PureRef, Cryptomator, Bitwarden, LocalSend).
   - Installs Scoop CLI developer tools (git, gh, ripgrep, fd, fzf, bat, eza, starship, fastfetch, sccache, etc.).
   - Installs Rust toolchain & Cargo tools (`rtk`, `kondo`, `krokiet`, `rust-script`, etc.).
   - Installs NPM global AI & code tools (`@anthropic-ai/claude-code`, `opencode-ai`, `pyright`, `context-mode`, etc.).
   - Installs Python libraries and UV global CLIs (`kimi-cli`, `ruff`).
8. **PowerShell 7 Installation**: Upgrades shell runtime to PowerShell 7 (`pwsh`).
9. **System Tweaks**: Enables NTFS Long Paths (`LongPathsEnabled = 1`), Developer Mode, and unhides file extensions.
10. **Audit Summary**: Emits a comprehensive component health report to console and log.

## Operational Parameters

- `-SkipDevTools`: Skips developer apps, Scoop, Cargo, NPM, and Pip batches.
- `-SkipOptionalFeatures`: Skips Sandbox and WSL enablement.
- `-SkipSystemTweaks`: Skips system registry modifications.
- `-NetworkMode`: Basic, Optimized, or Extreme TCP/DNS tuning.

Last Updated: 2026-07-24

# Windows LTSC One-Click Setup Backup

This repository is now centered on a single standalone script that can bootstrap a fresh LTSC machine from package managers to developer tooling.

## Quick Start

Run one script in an elevated PowerShell window:

```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force
.\Scripts\00_QuickSetup.ps1
```

That single script now covers:

- Network repair and TLS hardening
- Microsoft Store and Winget bootstrap
- Scoop and Chocolatey setup
- LTSC UWP app restoration
- Core desktop app installation
- Developer CLI and desktop tool installation
- Rust, Cargo, npm, pip, and `uv` tooling
- PowerShell 7 installation
- Common LTSC registry tweaks

## Optional Modes

If you want the same script to do less, use these switches:

```powershell
.\Scripts\00_QuickSetup.ps1 -SkipDevTools
.\Scripts\00_QuickSetup.ps1 -SkipOptionalFeatures
.\Scripts\00_QuickSetup.ps1 -SkipSystemTweaks
```

## Repository Structure

```text
LTSC_Tools_Backup/
├── Scripts/
│   ├── 00_QuickSetup.ps1        # Main standalone one-click setup script
│   ├── 01_bootstrap_ltsc.ps1    # Legacy standalone bootstrap flow
│   ├── 03_install_windows.ps1   # Legacy standalone dev environment flow
│   └── (other legacy helpers)
├── Docs/
├── Logs/
└── START_HERE.md
```

## Current Flow

The old manual sequence:

1. `00_QuickSetup.ps1`
2. `01_bootstrap_ltsc.ps1`
3. `03_install_windows.ps1`

has been merged into one actual runnable file:

1. `00_QuickSetup.ps1`

The older scripts are kept only as references. `00_QuickSetup.ps1` no longer depends on them.

## Notes

- Run the script as Administrator.
- A reboot after completion is recommended.
- Execution details are written to `Logs\setup_YYYYMMDD_HHMMSS.log`.

## Compatibility

- Windows 10 IoT Enterprise LTSC 2021
- Windows 11 IoT Enterprise LTSC 2024
- Other Windows Enterprise/LTSC variants with similar component baselines

Last Updated: 2026-03-28

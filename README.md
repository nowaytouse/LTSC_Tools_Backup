# Windows LTSC One-Click Setup

This repository is intentionally reduced to one primary setup script for rebuilding a Windows LTSC workstation with one entrypoint.

## Quick Start

Run this in an elevated PowerShell window:

```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force
.\Scripts\00_QuickSetup.ps1
```

## What The Script Handles

- network repair, TLS hardening, DNS tuning, and TCP optimization
- Microsoft Store bootstrap and Start menu repair
- Winget dependency installation and App Installer setup
- Scoop and Chocolatey bootstrap
- LTSC UWP app restore plus practical desktop alternatives
- core desktop applications
- developer CLI stack and desktop tools
- Rust, Cargo, npm, pip, and `uv` tooling
- PowerShell 7 installation
- common LTSC registry tweaks
- final component audit summary

## Optional Parameters

```powershell
.\Scripts\00_QuickSetup.ps1 -SkipDevTools
.\Scripts\00_QuickSetup.ps1 -SkipOptionalFeatures
.\Scripts\00_QuickSetup.ps1 -SkipSystemTweaks
.\Scripts\00_QuickSetup.ps1 -NetworkMode Basic
.\Scripts\00_QuickSetup.ps1 -NetworkMode Extreme
```

## Repository Layout

```text
LTSC_Tools_Backup/
├── Scripts/
│   └── 00_QuickSetup.ps1
├── Docs/
│   ├── final_setup_report.md
│   ├── setup_notes.md
│   └── software_list.md
├── Logs/
└── START_HERE.md
```

## Notes

- Run as Administrator.
- Reboot after completion.
- Review the generated log under `Logs\`.
- The repository no longer depends on helper setup scripts.

## Compatibility

- Windows 10 IoT Enterprise LTSC 2021
- Windows 11 IoT Enterprise LTSC 2024
- similar Windows Enterprise or LTSC environments

Last Updated: 2026-03-29

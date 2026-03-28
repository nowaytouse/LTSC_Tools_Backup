# Windows LTSC Backup - Start Here

## One Command

Use only this one script in an elevated PowerShell window:

```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force
D:\LTSC_Tools_Backup\Scripts\00_QuickSetup.ps1
```

## What It Does

`00_QuickSetup.ps1` now combines the old multi-step process into one standalone file and one run:

- fixes network and TLS issues
- installs Store, Winget, Scoop, and Chocolatey
- restores common LTSC app packages
- installs core applications
- installs developer toolchains and package ecosystems
- installs PowerShell 7
- applies common LTSC tweaks
- runs a final component audit summary

## Optional Flags

```powershell
D:\LTSC_Tools_Backup\Scripts\00_QuickSetup.ps1 -SkipDevTools
D:\LTSC_Tools_Backup\Scripts\00_QuickSetup.ps1 -SkipOptionalFeatures
D:\LTSC_Tools_Backup\Scripts\00_QuickSetup.ps1 -SkipSystemTweaks
```

`Scripts\` now contains only `00_QuickSetup.ps1`.

## After The Run

- Check `Logs\` for the latest setup log.
- Reboot Windows once.
- Verify with:

```powershell
winget --version
pwsh --version
git --version
node --version
python --version
```

Last Updated: 2026-03-28

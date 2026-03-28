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

## Optional Flags

```powershell
D:\LTSC_Tools_Backup\Scripts\00_QuickSetup.ps1 -SkipDevTools
D:\LTSC_Tools_Backup\Scripts\00_QuickSetup.ps1 -SkipOptionalFeatures
D:\LTSC_Tools_Backup\Scripts\00_QuickSetup.ps1 -SkipSystemTweaks
```

## Legacy Scripts

These remain in `Scripts\` for reference only:

- `01_bootstrap_ltsc.ps1`
- `03_install_windows.ps1`

They are no longer required, and `00_QuickSetup.ps1` does not call them.

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

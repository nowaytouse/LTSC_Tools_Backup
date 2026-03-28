# Windows LTSC Start Here

## One Command

Run only this script in an elevated PowerShell window:

```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force
D:\LTSC_Tools_Backup\Scripts\00_QuickSetup.ps1
```

## Recommended Variants

```powershell
D:\LTSC_Tools_Backup\Scripts\00_QuickSetup.ps1 -NetworkMode Basic
D:\LTSC_Tools_Backup\Scripts\00_QuickSetup.ps1 -NetworkMode Optimized
D:\LTSC_Tools_Backup\Scripts\00_QuickSetup.ps1 -NetworkMode Extreme
D:\LTSC_Tools_Backup\Scripts\00_QuickSetup.ps1 -SkipDevTools
```

## Included In One Run

- network repair and tuning
- Store, Winget, Scoop, and Chocolatey bootstrap
- Store registration repair
- LTSC app restoration and desktop alternatives
- common apps and developer tools
- PowerShell 7
- LTSC system tweaks
- final audit summary

## After The Run

- check the newest log in `Logs\`
- reboot once
- verify with:

```powershell
winget --version
pwsh --version
git --version
node --version
python --version
```

Last Updated: 2026-03-29

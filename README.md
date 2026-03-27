# Windows 11 LTSC Enterprise - Complete Configuration Backup

This repository contains a full set of scripts and documentation for configuring and optimizing Windows 11 IoT Enterprise LTSC.

## 📁 Repository Structure

```
D:\LTSC_Tools_Backup\
├── Scripts\           # Installation and optimization scripts
├── Docs\              # Documentation and configuration notes
├── Config\            # Configuration backup files (winget exports, etc.)
└── Logs\              # Installation logs
```

---

## 📋 Compatible Systems

- Windows 10 IoT Enterprise LTSC 2021 (Build 19044)
- Windows 11 IoT Enterprise LTSC 2024 (Build 26100/26200)
- Other Windows LTSC/Enterprise editions

---

## 🚀 Quick Start

### Recommended: All-in-One Setup
Run the main installer to handle everything from network repair to app installation.
```powershell
# Run as Administrator
.\Scripts\00_QuickSetup.ps1
```

### Advanced: Selective Optimization
For specific network tuning (Basic, Optimized, or Extreme):
```powershell
.\Scripts\Optimize-Network.ps1 -Mode Extreme
```

---

## 📦 Script Descriptions

### Main Workflow Scripts

| Script Name | Purpose | Recommended Order |
|-------------|---------|-------------------|
| `00_QuickSetup.ps1` | **Master Setup**: Network, Store, Apps, and System Tweaks. | 1 |
| `01_bootstrap_ltsc.ps1` | Package Manager Setup (NuGet, Winget, Scoop). | 2 |
| `03_install_windows.ps1` | Full Development Environment Setup. | 3 |

### Specialty Tools

| Script Name | Purpose |
|-------------|---------|
| `Optimize-Network.ps1` | Advanced TCP and Adapter performance tuning. |
| `check_ltsc_components.ps1` | Audit system for missing LTSC components. |
| `fix_store_menu.ps1` | Repair Microsoft Store visibility in Start Menu. |
| `install_uwp_cdn.ps1` | Install UWP apps directly from Microsoft CDN. |

---

## 🔧 Installation Methodology

### Option 1: Sequential Method (Recommended)
1. Run `00_QuickSetup.ps1` for baseline configuration.
2. Run `01_bootstrap_ltsc.ps1` to ensure all package managers are ready.
3. Use `03_install_windows.ps1` for development tools.

### Option 2: Modular Approach
- Every script in the `Scripts/` folder is designed to be stand-alone.
- You can run any script individually based on your needs.

---

## ⚠️ Troubleshooting & FAQ

### Issue 1: Winget Download Failures
**Cause:** Unstable connection to GitHub.  
**Solution:** The scripts automatically attempt to use BITS for more reliable transfers.
```powershell
# Manual BITS download example
Start-BitsTransfer -Source <URL> -Destination <File>
```

### Issue 2: Store Missing After Install
**Cause:** Start Menu cache issues.  
**Solution:** Re-register the Store app package.
```powershell
Get-AppxPackage -Name Microsoft.WindowsStore | 
  Add-AppxPackage -Register "$($_.InstallLocation)\appxmanifest.xml"
```

### Issue 3: Networking Timeouts
**Cause:** Improper TLS or DNS configuration.  
**Solution:** Run `Optimize-Network.ps1` (or the network step in QuickSetup).

---

## 🔐 System Optimization Reference

### Essential Registry Tweaks
```powershell
# Enable Long Paths (260+ characters)
Set-ItemProperty "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" "LongPathsEnabled" 1

# Show File Extensions
Set-ItemProperty "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced" "HideFileExt" 0

# Show Hidden Files
Set-ItemProperty "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced" "Hidden" 1
```

### Optional Windows Features
```powershell
# Enable Windows Sandbox
dism /online /enable-feature /featurename:Containers-DisposableClientVM /All

# Enable WSL 2
dism /online /enable-feature /featurename:Microsoft-Windows-Subsystem-Linux /All
```

---

## 📅 Backup Metadata

- **Backup Date:** 2026-03-25
- **OS Version:** Windows 11 IoT Enterprise LTSC 2024
- **Build Number:** 26200
- **Primary Location:** D:\LTSC_Tools_Backup

---

*Last Updated: 2026-03-28*

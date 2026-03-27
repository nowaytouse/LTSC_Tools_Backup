# Recommended LTSC Software Library

## Essential Tools (Must-Install)

```powershell
# Archiver
winget install 7zip.7zip

# Browsers
winget install Google.Chrome
winget install Mozilla.Firefox

# Media Playback
winget install VideoLAN.VLC

# Text Editing
winget install Notepad++.Notepad++
```

## Development Suite

```powershell
# Version Control
winget install Git.Git

# Code Editor
winget install Microsoft.VisualStudioCode

# PowerShell & Terminal
winget install Microsoft.PowerShell
winget install Microsoft.WindowsTerminal

# Runtimes & SDKs
winget install Microsoft.DotNet.SDK.8
winget install Python.Python.3.12
winget install NodeJS.NodeJS.LTS
winget install Oracle.JavaRuntimeEnvironment
```

## System Utilities

```powershell
# Microsoft Utility Suite
winget install Microsoft.PowerToys

# Disk & Storage Analysis
winget install CrystalDewWorld.CrystalDiskInfo
winget install JAMSoftware.TreeSize.Free

# System Auditing
winget install Belarc.Advisor

# Clean Uninstaller
winget install GeekSoftware.GeeekUninstaller
```

## Productivity & Efficiency

```powershell
# Capture & Screenshot
winget install ShareX.ShareX

# Image Viewer
winget install IrfanSkiljan.IrfanView

# Instant Search
winget install Everything.Alpha

# File/App Launcher
winget install Flow-Launcher.Flow-Launcher

# Knowledge Base / Notes
winget install Obsidian.Obsidian
```

## Office & Documentation

```powershell
# PDF Viewer
winget install SumatraPDF.SumatraPDF

# Mind Mapping
winget install FreePlane.FreePlane
```

## Network & Connectivity

```powershell
# Download Manager
winget install Motrix.Motrix

# Remote Desktop
winget install RustDesk.RustDesk
winget install RealVNC.VNCViewer

# Traffic Analysis
winget install WiresharkFoundation.Wireshark
```

## Security

```powershell
# Password Management
winget install KeePassXCTeam.KeePassXC

# Antivirus (Optional - Windows Defender is usually sufficient)
# winget install Malwarebytes.Malwarebytes
```

## Gaming

```powershell
# Platforms
winget install Valve.Steam
winget install Epic.EpicGamesLauncher

# Frameworks
winget install Microsoft.DirectX
```

---

## Batch Installation Script

```powershell
# One-click installation of a standardized baseline
$apps = @(
    "7zip.7zip",
    "Google.Chrome",
    "VideoLAN.VLC",
    "Notepad++.Notepad++",
    "Git.Git",
    "Microsoft.VisualStudioCode",
    "Microsoft.PowerShell",
    "Microsoft.DotNet.SDK.8",
    "Python.Python.3.12",
    "NodeJS.NodeJS.LTS",
    "Microsoft.PowerToys",
    "ShareX.ShareX",
    "IrfanSkiljan.IrfanView",
    "SumatraPDF.SumatraPDF",
    "Obsidian.Obsidian"
)

foreach ($app in $apps) {
    Write-Host "Installing $app..." -ForegroundColor Cyan
    winget install --id $app --silent --accept-package-agreements --accept-source-agreements
}
```

---

*Last Updated: 2026-03-28*

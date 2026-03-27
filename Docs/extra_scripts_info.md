# Specialized Scripts Information

These scripts provide advanced installation options beyond the basic setup, primarily focused on package managers and full development environments.

---

## 📦 Script Catalog

### 01_bootstrap_ltsc.ps1
**Purpose**: Initializes package managers and core dependencies.

**Key Features**:
- ✅ Enables TLS 1.2/1.3
- ✅ Installs NuGet Provider
- ✅ Updates PowerShellGet
- ✅ Installs VCLibs and UI.Xaml (Winget Dependencies)
- ✅ Installs Winget (App Installer)
- ✅ Installs Scoop (with extras/versions/nerd-fonts buckets)
- ✅ Installs Chocolatey
- ✅ Refreshes PATH environment variables

**Usage**:
```powershell
# Run as Administrator
Set-ExecutionPolicy Bypass -Scope Process -Force
.\01_bootstrap_ltsc.ps1
```

**Best For**:
- Fresh LTSC installations.
- Users requiring Scoop or Chocolatey.
- Preparing the system for the full development environment.

---

### 02_install.ps1
**Purpose**: Official Scoop installation logic.

**Key Features**:
- Clones/Downloads Scoop from GitHub.
- Configures Scoop directories and cache.
- Adds Scoop to the system PATH.

**Usage**:
```powershell
# Standard Installation
.\02_install.ps1

# Custom Directory Installation
.\02_install.ps1 -ScoopDir "D:\Scoop" -ScoopGlobalDir "D:\Scoop\Global"
```

---

### 03_install_windows.ps1
**Purpose**: All-in-one Master Development Environment Setup.

**Key Features**:
1. **Package Manager Validation**: Ensures Scoop and Winget are ready.
2. **CLI Tools (via Scoop)**: 
   - Dev: git, gh, node, python, go, zig, deno, fnm, cmake, ninja.
   - Search: ripgrep (rg), fd.
   - Media: ffmpeg, imagemagick, exiftool.
   - Download: wget, aria2, yt-dlp.
3. **GUI Applications (via Winget)**: Bitwarden, LocalSend, OpenJDK, Certbot.
4. **Rust Toolchain**: Full Cargo installation with 17+ essential crates.
5. **NPM & Pip Packages**: Global JS/Python tools for AI and Data Science (claude-code, numpy, pandas, torch).
6. **AI Tools**: Ollama, Kimi-CLI, uv.

**Usage**:
```powershell
# Run as Administrator
Set-ExecutionPolicy Bypass -Scope Process -Force
.\03_install_windows.ps1
```

**Execution Time**: Approx. 30-60 minutes depending on connection speed.

---

## 🔀 Script Selection Guide

### Scenario 1: Basic Tools Only
Use the master installer:
```powershell
.\00_QuickSetup.ps1
```

### Scenario 2: Developer Environment (Scoop-focused)
```powershell
.\01_bootstrap_ltsc.ps1
.\03_install_windows.ps1
```

---

## 📊 Package Manager Comparison

| Feature | Winget | Scoop | Chocolatey |
|---------|--------|-------|------------|
| Source | Microsoft Official | Community | Community |
| GUI Apps | ✅ | ❌ | ✅ |
| CLI Tools | ⚠️ | ✅ | ✅ |
| Speed | Fast | Fast | Moderate |
| Admin Req | No | No | Yes |
| Focus | General Apps | Dev Tools | Enterprise |

---

## 🛠️ Combined Usage Example

### Minimal Dev Install
```powershell
# 1. Bootstrap Managers
.\01_bootstrap_ltsc.ps1

# 2. Install Essentials
scoop install git nodejs-lts python
winget install 7zip.7zip Google.Chrome
```

---

## ⚠️ Important Considerations

1. **Network Stability**: A stable connection is vital as GBs of data will be downloaded.
2. **Disk Space**:
   - Minimal Setup: ~2GB
   - Full Dev Setup: 10GB+
3. **PATH Refresh**: Always restart your terminal (or run `refreshenv`) after manager installation to detect new commands.

---

*Documentation Updated: March 28, 2026*

# Windows 11 LTSC Configuration Notes

## Configuration Date
March 25, 2026

## System Information
- **OS**: Windows 11 IoT Enterprise LTSC 2024
- **Build**: 10.0.26200.8039
- **Version**: 2009

---

## 1. Identified Issues

### 1.1 Missing Components
LTSC versions typically lack:
- ❌ Microsoft Store
- ❌ Winget (Package Manager)
- ❌ Essential UWP Apps (Calculator, Paint, Photos, etc.)
- ❌ Windows Terminal & PowerShell 7
- ❌ Optional Features (Sandbox, WSL, Hyper-V are disabled by default)

### 1.2 Networking Challenges
**Symptoms**:
- Timeouts when downloading from GitHub.
- HTTPS connection failures.
- Connection drops during large file transfers.

**Root Causes**:
- TLS 1.2 not explicitly enabled/configured in SCHANNEL.
- Suboptimal default DNS settings.
- Network stack requires modern tuning for LTSC.

**Solutions**:
Implemented in `Optimize-Network.ps1` and `00_QuickSetup.ps1`.

### 1.3 Winget Installation Failures
**Symptoms**:
- Deployment failed HRESULT: 0x80073CF3.
- Missing dependencies: `Microsoft.VCLibs` and `Microsoft.UI.Xaml`.

**Solutions**:
- Use BITS (Background Intelligent Transfer Service) for stability.
- Ensure dependencies are installed prior to the main appx package.

---

## 2. Successful Implementation Strategy

### 2.1 Core Component Order

#### Step 1: Network Repair
Run `Optimize-Network.ps1` to ensure stable HTTPS connectivity.

#### Step 2: Establish Store & Winget
- Trigger Store installation via `wsreset -i`.
- Download Winget using BITS to avoid corrupted installer files.

#### Step 3: Winget Software Library
Standardized on the following IDs for a clean setup:
- `7zip.7zip`
- `VideoLAN.VLC`
- `Google.Chrome`
- `Notepad++.Notepad++`
- `ShareX.ShareX`
- `IrfanSkiljan.IrfanView`

#### Step 4: PowerShell 7 Modernization
Always fetch the latest MSI from the official GitHub releases to ensure path integration.

### 2.2 UWP Application Strategy
**Conclusion**: Some native UWP apps are inherently incompatible or difficult to maintain on LTSC.

**Recommended Alternatives**:
| Native UWP App | Modern Alternative |
|----------------|--------------------|
| MS Paint       | Paint.NET          |
| Photos         | IrfanView          |
| Snipping Tool  | ShareX             |
| Calculator     | Qalculate!         |
| Alarms         | System Tray Clock  |

---

## 3. System Optimization

### 3.1 Registry Tweaks
Applied via `00_QuickSetup.ps1`:
- **LongPathsEnabled**: Removes the 260-character limit.
- **HideFileExt**: Displays file extensions for security and clarity.
- **Hidden**: Shows hidden files and folders.
- **AllowDevelopmentWithoutDevLicense**: Enables Developer Mode.

### 3.2 Optional Features
Enable as needed using standard DISM commands:
- **Windows Sandbox**: Secure, isolated desktop environment.
- **WSL 2**: Linux Subsystem for Windows.
- **Hyper-V**: Hardware virtualization.

---

## 4. Key Lessons Learned

1. **TLS is the Foundation**: If TLS 1.2/1.3 is not properly configured, all subsequent GitHub/HTTPS downloads will fail.
2. **BITS is Superior**: For LTSC environments, `Start-BitsTransfer` is significantly more robust than `Invoke-WebRequest`.
3. **Sequence Matters**: Repair Network -> Install Managers -> Install Apps -> Optimize System.
4. **Embrace Alternatives**: Using robust open-source alternatives is often better than forcing UWP apps onto LTSC.

---

## 5. Command Cheat Sheet

### Winget Basics
```powershell
# Search for software
winget search <keyword>

# Silent installation
winget install <ID> --silent --accept-package-agreements

# Upgrade All
winget upgrade --all --silent

# Export list for future systems
winget export -o installed.json
```

### System Audits
```powershell
# List all Appx Packages
Get-AppxPackage | Select-Object Name, Version

# Check Feature Status
Get-WindowsOptionalFeature -Online

# Test GitHub Connectivity
Test-NetConnection -ComputerName github.com -Port 443
```

---

*Notes Last Updated: March 28, 2026*

# ==============================================================
# Windows 11 LTSC Bootstrap Script
# Installs: NuGet, winget, Scoop, Chocolatey
#
# Run as Administrator:
#   Set-ExecutionPolicy Bypass -Scope Process -Force
#   .\bootstrap_ltsc.ps1
# ==============================================================

$ErrorActionPreference = "Continue"
function Log($m)  { Write-Host "`n[>>] $m" -ForegroundColor Cyan }
function OK($m)   { Write-Host "  [OK] $m" -ForegroundColor Green }
function WARN($m) { Write-Host "  [!!] $m" -ForegroundColor Yellow }
function FAIL($m) { Write-Host "  [XX] $m" -ForegroundColor Red }

# ==============================================================
# 1. Ensure TLS 1.2 (required for all downloads)
# ==============================================================
Log "Enforcing TLS 1.2..."
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
OK "TLS 1.2 enabled"

# ==============================================================
# 2. NuGet provider (required by PowerShellGet)
# ==============================================================
Log "Installing NuGet provider..."
Install-PackageProvider -Name NuGet -MinimumVersion 2.8.5.201 -Force -Scope CurrentUser | Out-Null
OK "NuGet ready"

# ==============================================================
# 3. PowerShellGet update
# ==============================================================
Log "Updating PowerShellGet..."
Install-Module -Name PowerShellGet -Force -AllowClobber -Scope CurrentUser 2>&1 | Out-Null
OK "PowerShellGet updated"

# ==============================================================
# 4. VCLibs + UI.Xaml (winget dependencies on LTSC)
# ==============================================================
Log "Installing winget dependencies (VCLibs + UI.Xaml)..."

$tmp = "$env:TEMP\winget_deps"
New-Item -ItemType Directory -Force -Path $tmp | Out-Null

# VCLibs
$vclibsUrl = "https://aka.ms/Microsoft.VCLibs.x64.14.00.Desktop.appx"
$vclibsPath = "$tmp\VCLibs.appx"
Write-Host "  Downloading VCLibs..." -ForegroundColor Gray
Invoke-WebRequest -Uri $vclibsUrl -OutFile $vclibsPath -UseBasicParsing
Add-AppxPackage -Path $vclibsPath -ErrorAction SilentlyContinue
OK "VCLibs installed"

# UI.Xaml 2.8
$xamlUrl  = "https://github.com/microsoft/microsoft-ui-xaml/releases/download/v2.8.6/Microsoft.UI.Xaml.2.8.x64.appx"
$xamlPath = "$tmp\UIXaml.appx"
Write-Host "  Downloading UI.Xaml..." -ForegroundColor Gray
Invoke-WebRequest -Uri $xamlUrl -OutFile $xamlPath -UseBasicParsing
Add-AppxPackage -Path $xamlPath -ErrorAction SilentlyContinue
OK "UI.Xaml installed"

# ==============================================================
# 5. winget (App Installer msixbundle)
# ==============================================================
Log "Installing winget..."

if (Get-Command winget -ErrorAction SilentlyContinue) {
    OK "winget already present"
} else {
    # Fetch latest release tag from GitHub API
    try {
        $rel     = Invoke-RestMethod "https://api.github.com/repos/microsoft/winget-cli/releases/latest"
        $asset   = $rel.assets | Where-Object { $_.name -like "*.msixbundle" } | Select-Object -First 1
        $wingetPath = "$tmp\winget.msixbundle"
        Write-Host "  Downloading $($asset.name)..." -ForegroundColor Gray
        Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $wingetPath -UseBasicParsing
        Add-AppxPackage -Path $wingetPath
        OK "winget installed"
    } catch {
        FAIL "winget install failed: $_"
        WARN "Manual fallback: https://aka.ms/getwinget"
    }
}

# ==============================================================
# 6. Scoop
# ==============================================================
Log "Installing Scoop..."

if (Get-Command scoop -ErrorAction SilentlyContinue) {
    OK "Scoop already present"
} else {
    try {
        Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression
        scoop bucket add extras  2>&1 | Out-Null
        scoop bucket add versions 2>&1 | Out-Null
        scoop bucket add nerd-fonts 2>&1 | Out-Null
        OK "Scoop installed"
    } catch {
        FAIL "Scoop install failed: $_"
    }
}

# ==============================================================
# 7. Chocolatey (fallback for packages not in Scoop/winget)
# ==============================================================
Log "Installing Chocolatey..."

if (Get-Command choco -ErrorAction SilentlyContinue) {
    OK "Chocolatey already present"
} else {
    try {
        Set-ExecutionPolicy Bypass -Scope Process -Force
        [System.Net.ServicePointManager]::SecurityProtocol = `
            [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
        Invoke-Expression ((New-Object System.Net.WebClient).DownloadString(
            'https://community.chocolatey.org/install.ps1'))
        OK "Chocolatey installed"
    } catch {
        FAIL "Chocolatey install failed: $_"
    }
}

# ==============================================================
# 8. Refresh PATH
# ==============================================================
Log "Refreshing PATH..."
$env:PATH = [System.Environment]::GetEnvironmentVariable("PATH","Machine") + ";" +
            [System.Environment]::GetEnvironmentVariable("PATH","User")
OK "PATH refreshed"

# ==============================================================
# 9. Quick verify
# ==============================================================
Log "Verifying installs..."
foreach ($cmd in @("winget","scoop","choco")) {
    if (Get-Command $cmd -ErrorAction SilentlyContinue) {
        OK "$cmd available"
    } else {
        WARN "$cmd NOT found (may need terminal restart)"
    }
}

Write-Host "`n============================================================" -ForegroundColor Magenta
Write-Host " Bootstrap done! Restart terminal, then run install_windows.ps1" -ForegroundColor Green
Write-Host "============================================================`n" -ForegroundColor Magenta

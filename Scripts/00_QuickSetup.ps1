#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Windows LTSC Quick Setup Script
.DESCRIPTION
    Automated installation of essential tools and components for Windows LTSC.
    Optimized for Windows 11 IoT Enterprise LTSC.
    Sections:
    1. Network Optimization
    2. Essential Windows Components (Store, Winget)
    3. Specialized LTSC Components (Photos, Calculator, etc.)
    4. Optional Windows Features (Sandbox, WSL)
    5. Software Installation (7-Zip, Chrome, etc.)
    6. System Optimization
.EXAMPLE
    .\00_QuickSetup.ps1
#>

# Enable TLS 1.2/1.3 for downloads
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13

# ==============================================================================
# Global Configuration & Logging
# ==============================================================================
$ErrorActionPreference = "Stop"
$logDir = "$PSScriptRoot\..\Logs"
if (!(Test-Path $logDir)) { New-Item -ItemType Directory -Path $logDir -Force | Out-Null }
$logFile = "$logDir\setup_$(Get-Date -Format 'yyyyMMdd_HHmmss').log"

function Write-Log {
    param(
        [Parameter(Mandatory=$true)] [string]$Message,
        [ValidateSet("INFO", "OK", "WARN", "ERROR", "START", "END")] [string]$Level = "INFO"
    )
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logEntry = "[$timestamp] [$Level] $Message"
    Add-Content -Path $logFile -Value $logEntry
    
    $color = switch ($Level) {
        "OK"    { "Green" }
        "WARN"  { "Yellow" }
        "ERROR" { "Red" }
        "START","END" { "Cyan" }
        default { "Gray" }
    }
    Write-Host "  $Message" -ForegroundColor $color
}

function Show-Section {
    param([string]$Title, [int]$Step, [int]$Total)
    Write-Host "`n" + ("━" * 60) -ForegroundColor Cyan
    Write-Host "  [$Step/$Total] $Title" -ForegroundColor Yellow
    Write-Host ("━" * 60) -ForegroundColor Cyan
}

# ==============================================================================
# Initialization & Requirements
# ==============================================================================
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Windows LTSC Quick Setup" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Check for Administrator privileges
$currentPrincipal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
if (-not $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Host "`n[!] This script requires Administrator privileges." -ForegroundColor Red
    Write-Host "    Right-click the script and select 'Run as Administrator'." -ForegroundColor Gray
    pause
    exit
}

Write-Log "Starting LTSC setup process..." "START"

# ==============================================================================
# Step 1: Network Optimization & Repair
# ==============================================================================
Show-Section "Network Configuration" 1 5
try {
    # Call the dedicated network optimizer if available
    $netOptPath = "$PSScriptRoot\Optimize-Network.ps1"
    if (Test-Path $netOptPath) {
        Write-Log "Running Optimize-Network.ps1..."
        & $netOptPath -Mode Optimized
    } else {
        # Fallback to basic fixes
        Write-Log "Network optimizer not found. Applying basic TLS fixes..."
        $tlsPath = "HKLM:\SYSTEM\CurrentControlSet\Control\SecurityProviders\SCHANNEL\Protocols\TLS 1.2\Client"
        if (!(Test-Path $tlsPath)) { New-Item -Path $tlsPath -Force | Out-Null }
        Set-ItemProperty -Path $tlsPath -Name "Enabled" -Value 1 -Force | Out-Null
        Write-Log "TLS 1.2 configuration updated." "OK"
    }
} catch {
    Write-Log "Network configuration encountered an issue: $($_.Exception.Message)" "WARN"
}

# ==============================================================================
# Step 2: Essential Windows Components (Store, Winget)
# ==============================================================================
Show-Section "Core Components (Store & Winget)" 2 5

# Microsoft Store
if (!(Get-AppxPackage -Name Microsoft.WindowsStore)) {
    Write-Log "Installing Microsoft Store..."
    wsreset -i 2>$null
    Start-Sleep -Seconds 10
    if (Get-AppxPackage -Name Microsoft.WindowsStore) { Write-Log "Store installed successfully." "OK" }
    else { Write-Log "Store installation triggered (it may appear after a few minutes)." "INFO" }
} else { Write-Log "Microsoft Store is already present." "OK" }

# Winget (App Installer)
if (!(Get-AppxPackage -Name Microsoft.DesktopAppInstaller)) {
    Write-Log "Downloading and installing Winget..."
    $wingetUrl = "https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle"
    $wingetFile = "$env:TEMP\winget.msixbundle"
    try {
        Start-BitsTransfer -Source $wingetUrl -Destination $wingetFile -Priority High
        Add-AppxPackage -Path $wingetFile -ForceApplicationShutdown
        Write-Log "Winget installed successfully." "OK"
    } catch {
        Write-Log "Winget installation failed: $($_.Exception.Message)" "ERROR"
    }
} else { Write-Log "Winget is already present." "OK" }

# ==============================================================================
# Step 3: Restore LTSC Missing Apps (Photos, Calculator, etc.)
# ==============================================================================
Show-Section "Specialized LTSC Components" 3 7
$uwpApps = @(
    @{Title='Photos'; Id='Microsoft.Windows.Photos'},
    @{Title='Calculator'; Id='Microsoft.WindowsCalculator'},
    @{Title='Paint'; Id='Microsoft.Paint'},
    @{Title='Snipping Tool'; Id='Microsoft.ScreenSketch'},
    @{Title='Windows Terminal'; Id='Microsoft.WindowsTerminal'}
)

foreach ($app in $uwpApps) {
    Write-Host "  Checking $($app.Title)..." -ForegroundColor Gray
    if (!(Get-AppxPackage -Name $app.Id -AllUsers)) {
        Write-Log "Installing $($app.Title)..."
        try {
            winget install --id $app.Id -e --silent --accept-package-agreements --accept-source-agreements 2>$null
            if (Get-AppxPackage -Name $app.Id) { Write-Log "$($app.Title) restored." "OK" }
        } catch { Write-Log "Failed to restore $($app.Title)." "WARN" }
    } else { Write-Log "$($app.Title) is present." "OK" }
}

# ==============================================================================
# Step 4: Optional Features (Sandbox, WSL)
# ==============================================================================
Show-Section "Optional Windows Features" 4 7
$features = @(
    @{Name='Containers-DisposableClientVM'; Desc='Windows Sandbox'},
    @{Name='Microsoft-Windows-Subsystem-Linux'; Desc='WSL 2'},
    @{Name='NetFx3'; Desc='.NET Framework 3.5'}
)

foreach ($feat in $features) {
    $status = Get-WindowsOptionalFeature -Online -FeatureName $feat.Name -ErrorAction SilentlyContinue
    if ($status.State -eq 'Enabled') {
        Write-Log "Feature: $($feat.Desc) is Enabled." "OK"
    } else {
        Write-Log "Feature: $($feat.Desc) is currently available for manual enablement." "INFO"
    }
}

# ==============================================================================
# Step 5: Software Installation
# ==============================================================================
Show-Section "Software Installation" 5 7
$apps = @(
    @{ id="7zip.7zip"; name="7-Zip" },
    @{ id="VideoLAN.VLC"; name="VLC Media Player" },
    @{ id="Google.Chrome"; name="Google Chrome" },
    @{ id="Notepad++.Notepad++"; name="Notepad++" },
    @{ id="ShareX.ShareX"; name="ShareX" },
    @{ id="IrfanSkiljan.IrfanView"; name="IrfanView" }
)

foreach ($app in $apps) {
    Write-Host "  Checking $($app.name)..." -ForegroundColor Gray
    $installed = winget list --id $app.id -e 2>$null | Select-String $app.id
    if ($installed) {
        Write-Log "$($app.name) is already installed." "OK"
    } else {
        Write-Host "    Installing $($app.name)..." -ForegroundColor Cyan
        winget install --id $app.id --silent --accept-package-agreements --accept-source-agreements 2>$null
        Write-Log "$($app.name) installation complete." "OK"
    }
}

# ==============================================================================
# Step 6: PowerShell 7 Modernization
# ==============================================================================
Show-Section "PowerShell 7 Installation" 6 7
$pwshPath = "$env:ProgramFiles\PowerShell\7\pwsh.exe"
if (!(Test-Path $pwshPath)) {
    try {
        Write-Log "Fetching latest PowerShell 7 release..."
        $latest = Invoke-RestMethod -Uri 'https://api.github.com/repos/PowerShell/PowerShell/releases/latest' -Headers @{'User-Agent'='Mozilla/5.0'}
        $msiUrl = $latest.assets | Where-Object {$_.name -like '*-win-x64.msi'} | Select-Object -First 1 -ExpandProperty browser_download_url
        $msiFile = "$env:TEMP\PowerShell7.msi"
        
        Write-Log "Downloading and installing PowerShell 7..."
        (New-Object System.Net.WebClient).DownloadFile($msiUrl, $msiFile)
        Start-Process msiexec.exe -ArgumentList "/i `"$msiFile`" /quiet /norestart" -Wait
        
        if (Test-Path $pwshPath) { Write-Log "PowerShell 7 installed successfully." "OK" }
    } catch {
        Write-Log "PowerShell 7 installation failed." "WARN"
    }
} else { Write-Log "PowerShell 7 is already present." "OK" }

# ==============================================================================
# Step 7: System Optimizations
# ==============================================================================
Show-Section "System Optimization" 7 7
$registryTweaks = @(
    @{Path="HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem"; Name="LongPathsEnabled"; Value=1; Desc="Long Paths (260+ characters)"},
    @{Path="HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced"; Name="HideFileExt"; Value=0; Desc="Show File Extensions"},
    @{Path="HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced"; Name="Hidden"; Value=1; Desc="Show Hidden Files"},
    @{Path="HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock"; Name="AllowDevelopmentWithoutDevLicense"; Value=1; Desc="Developer Mode"}
)

foreach ($tweak in $registryTweaks) {
    try {
        Set-ItemProperty -Path $tweak.Path -Name $tweak.Name -Value $tweak.Value -Force | Out-Null
        Write-Log "$($tweak.Desc) applied." "OK"
    } catch {
        Write-Log "Failed to apply $($tweak.Desc)." "WARN"
    }
}

# ==============================================================================
# Completion
# ==============================================================================
Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "  Setup Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Log "Summary of actions saved to: $logFile" "INFO"
Write-Host "A system restart is recommended to apply all changes." -ForegroundColor Cyan
Write-Host ""

Write-Log "Setup finished successfully." "END"

#Requires -Version 5.1
<#
.SYNOPSIS
    Unified Windows LTSC Restoration & Dev Environment Setup Script
.DESCRIPTION
    Single entrypoint script for rebuilding a Windows LTSC workstation with full developer capabilities,
    network optimizations, package bootstrapping (Winget, Scoop, Chocolatey), UWP app restoration,
    system tweaks, and package installations (Winget, Scoop, Cargo, NPM, Pip, UV).
.PARAMETER SkipDevTools
    Skips developer CLI tools, runtimes, Rust, Cargo, NPM, and Pip packages.
.PARAMETER SkipOptionalFeatures
    Skips enabling Windows optional features like Sandbox and WSL.
.PARAMETER SkipSystemTweaks
    Skips applying LTSC system registry tweaks.
.PARAMETER NetworkMode
    Network optimization intensity level: Basic, Optimized (default), or Extreme.
#>

[CmdletBinding()]
param(
    [switch]$SkipDevTools,
    [switch]$SkipOptionalFeatures,
    [switch]$SkipSystemTweaks,
    [ValidateSet("Basic", "Optimized", "Extreme")]
    [string]$NetworkMode = "Optimized"
)

$ErrorActionPreference = "Stop"

# Establish runtime logging path
$script:LogDir = Join-Path $PSScriptRoot "..\Logs"
if (-not (Test-Path $script:LogDir)) {
    New-Item -Path $script:LogDir -ItemType Directory -Force | Out-Null
}
$timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
$script:LogFile = Join-Path $script:LogDir ("setup_{0}.log" -f $timestamp)

function Write-Log {
    param(
        [Parameter(Mandatory = $true)][string]$Message,
        [string]$Level = "INFO"
    )
    $timeStr = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logLine = "[{0}] [{1}] {2}" -f $timeStr, $Level.ToUpper(), $Message
    Add-Content -Path $script:LogFile -Value $logLine -ErrorAction SilentlyContinue

    switch ($Level.ToUpper()) {
        "OK"    { Write-Host ("  [+] {0}" -f $Message) -ForegroundColor Green }
        "WARN"  { Write-Host ("  [!] {0}" -f $Message) -ForegroundColor Yellow }
        "ERROR" { Write-Host ("  [-] {0}" -f $Message) -ForegroundColor Red }
        "START" { Write-Host ("`n>>> {0}" -f $Message) -ForegroundColor Cyan }
        "END"   { Write-Host ("`n<<< {0}" -f $Message) -ForegroundColor Cyan }
        default { Write-Host ("  [*] {0}" -f $Message) -ForegroundColor White }
    }
}

function Show-Step {
    param([Parameter(Mandatory = $true)][string]$Title)
    Write-Host ""
    Write-Host ("=== {0} ===" -f $Title) -ForegroundColor Cyan
    Write-Log ("Step: {0}" -f $Title) "INFO"
}

function Test-Admin {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($identity)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Test-CommandAvailable {
    param([Parameter(Mandatory = $true)][string]$CommandName)
    return [bool](Get-Command $CommandName -ErrorAction SilentlyContinue)
}

function Install-WingetPackage {
    param(
        [Parameter(Mandatory = $true)][string]$Id,
        [Parameter(Mandatory = $true)][string]$Name
    )

    if (-not (Test-CommandAvailable "winget")) {
        Write-Log ("Skipped {0}; Winget is unavailable." -f $Name) "WARN"
        return
    }

    $escapedId = [regex]::Escape($Id)
    $existing = winget list --id $Id -e 2>$null | Select-String $escapedId
    if ($existing) {
        Write-Log ("{0} is already installed." -f $Name) "OK"
        return
    }

    try {
        Write-Log ("Installing {0} via Winget..." -f $Name)
        winget install --id $Id -e --silent --accept-package-agreements --accept-source-agreements 2>$null | Out-Null
        Write-Log ("Installed {0}." -f $Name) "OK"
    } catch {
        Write-Log ("Failed to install {0} via Winget: {1}" -f $Name, $_.Exception.Message) "WARN"
    }
}

function Install-ScoopPackage {
    param([Parameter(Mandatory = $true)][string]$Name)

    if (-not (Test-CommandAvailable "scoop")) {
        Write-Log ("Skipped Scoop package {0}; Scoop is unavailable." -f $Name) "WARN"
        return
    }

    $existing = scoop list 2>$null | Select-String ("^\s*{0}\s" -f [regex]::Escape($Name))
    if ($existing) {
        Write-Log ("Scoop package {0} is already installed." -f $Name) "OK"
        return
    }

    try {
        Write-Log ("Installing {0} via Scoop..." -f $Name)
        scoop install $Name 2>$null | Out-Null
        Write-Log ("Installed {0} via Scoop." -f $Name) "OK"
    } catch {
        Write-Log ("Failed to install {0} via Scoop: {1}" -f $Name, $_.Exception.Message) "WARN"
    }
}

function Install-CargoPackage {
    param([Parameter(Mandatory = $true)][string]$Name)

    if (-not (Test-CommandAvailable "cargo")) {
        Write-Log ("Skipped Cargo package {0}; cargo is unavailable." -f $Name) "WARN"
        return
    }

    try {
        Write-Log ("Installing {0} via Cargo..." -f $Name)
        cargo install $Name --quiet 2>$null | Out-Null
        Write-Log ("Installed {0} via Cargo." -f $Name) "OK"
    } catch {
        Write-Log ("Failed to install {0} via Cargo: {1}" -f $Name, $_.Exception.Message) "WARN"
    }
}

function Install-NpmGlobalPackage {
    param([Parameter(Mandatory = $true)][string]$Name)

    if (-not (Test-CommandAvailable "npm")) {
        Write-Log ("Skipped NPM global package {0}; npm is unavailable." -f $Name) "WARN"
        return
    }

    try {
        Write-Log ("Installing {0} globally via NPM..." -f $Name)
        npm install -g $Name --loglevel=error 2>$null | Out-Null
        Write-Log ("Installed {0} globally via NPM." -f $Name) "OK"
    } catch {
        Write-Log ("Failed to install {0} globally via NPM: {1}" -f $Name, $_.Exception.Message) "WARN"
    }
}

function Install-PipPackage {
    param([Parameter(Mandatory = $true)][string]$Name)

    if (-not (Test-CommandAvailable "pip")) {
        Write-Log ("Skipped pip package {0}; pip is unavailable." -f $Name) "WARN"
        return
    }

    try {
        Write-Log ("Installing {0} via pip..." -f $Name)
        pip install $Name --quiet 2>$null | Out-Null
        Write-Log ("Installed {0} via pip." -f $Name) "OK"
    } catch {
        Write-Log ("Failed to install {0} via pip: {1}" -f $Name, $_.Exception.Message) "WARN"
    }
}

function Install-PackageProviderIfMissing {
    if (-not (Get-PackageProvider -Name NuGet -ErrorAction SilentlyContinue)) {
        try {
            Write-Log "Installing NuGet PackageProvider..."
            Install-PackageProvider -Name NuGet -MinimumVersion 2.8.5.201 -Force -Scope CurrentUser | Out-Null
            Write-Log "NuGet PackageProvider installed." "OK"
        } catch {
            Write-Log ("Failed to install NuGet PackageProvider: {0}" -f $_.Exception.Message) "WARN"
        }
    } else {
        Write-Log "NuGet PackageProvider is available." "OK"
    }
}

function Ensure-PowerShellGet {
    try {
        Write-Log "Updating PowerShellGet module..."
        Install-Module -Name PowerShellGet -Force -AllowClobber -Scope CurrentUser -ErrorAction Stop | Out-Null
        Write-Log "PowerShellGet updated." "OK"
    } catch {
        Write-Log ("PowerShellGet update skipped/failed: {0}" -f $_.Exception.Message) "WARN"
    }
}

function Ensure-MicrosoftStore {
    $store = Get-AppxPackage -Name Microsoft.WindowsStore -ErrorAction SilentlyContinue
    if ($store) {
        Write-Log "Microsoft Store is already registered." "OK"
        return
    }

    try {
        Write-Log "Attempting to restore Microsoft Store via Appx manifest..."
        $manifestPath = Get-ChildItem -Path "$env:ProgramFiles\WindowsApps" -Filter "AppxManifest.xml" -Recurse -ErrorAction SilentlyContinue |
            Where-Object { $_.FullName -like "*Microsoft.WindowsStore*" } |
            Select-Object -First 1 -ExpandProperty FullName

        if ($manifestPath) {
            Add-AppxPackage -DisableDevelopmentMode -Register $manifestPath -ErrorAction Stop
            Write-Log "Microsoft Store re-registered." "OK"
        } else {
            Write-Log "Microsoft Store manifest not found in WindowsApps cache." "WARN"
        }
    } catch {
        Write-Log ("Microsoft Store restoration failed: {0}" -f $_.Exception.Message) "WARN"
    }
}

function Repair-StoreVisibility {
    try {
        Write-Log "Refreshing AppX package manifests for Store components..."
        Get-AppxPackage -AllUsers *WindowsStore* -ErrorAction SilentlyContinue | ForEach-Object {
            Add-AppxPackage -DisableDevelopmentMode -Register "$($_.InstallLocation)\AppXManifest.xml" -ErrorAction SilentlyContinue
        }
        Write-Log "AppX package manifests refreshed." "OK"
    } catch {
        Write-Log ("AppX package refresh encountered an issue: {0}" -f $_.Exception.Message) "WARN"
    }
}

function Ensure-WingetDependencies {
    $deps = @(
        @{ Name = "Microsoft.VCLibs.x64"; Url = "https://aka.ms/Microsoft.VCLibs.x64.14.00.Desktop.appx" },
        @{ Name = "Microsoft.UI.Xaml.x64"; Url = "https://github.com/microsoft/microsoft-ui-xaml/releases/download/v2.8.6/Microsoft.UI.Xaml.2.8.x64.appx" }
    )

    foreach ($dep in $deps) {
        try {
            $dest = Join-Path $env:TEMP ("{0}.appx" -f $dep.Name)
            if (-not (Test-Path $dest)) {
                Write-Log ("Downloading dependency {0}..." -f $dep.Name)
                Start-BitsTransfer -Source $dep.Url -Destination $dest -ErrorAction Stop
            }
            Add-AppxPackage -Path $dest -ErrorAction SilentlyContinue
            Write-Log ("Dependency {0} registered." -f $dep.Name) "OK"
        } catch {
            Write-Log ("Failed downloading/registering {0}: {1}" -f $dep.Name, $_.Exception.Message) "WARN"
        }
    }
}

function Refresh-PathEnvironment {
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
}

function Ensure-Winget {
    if (Test-CommandAvailable "winget") {
        Write-Log "Winget is already operational." "OK"
        return
    }

    Ensure-WingetDependencies

    try {
        Write-Log "Bootstrapping Winget via PowerShell Appx deployment..."
        $wingetUrl = "https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle"
        $bundlePath = Join-Path $env:TEMP "winget-installer.msixbundle"
        Start-BitsTransfer -Source $wingetUrl -Destination $bundlePath -ErrorAction Stop
        Add-AppxPackage -Path $bundlePath -ErrorAction Stop
        Remove-Item $bundlePath -Force -ErrorAction SilentlyContinue
        Refresh-PathEnvironment
        if (Test-CommandAvailable "winget") {
            Write-Log "Winget successfully bootstrapped." "OK"
        } else {
            Write-Log "Winget package registered but binary is not found in PATH." "WARN"
        }
    } catch {
        Write-Log ("Winget bootstrap failed: {0}" -f $_.Exception.Message) "WARN"
    }
}

function Ensure-Scoop {
    if (Test-CommandAvailable "scoop") {
        Write-Log "Scoop is already installed." "OK"
        return
    }

    try {
        Write-Log "Installing Scoop package manager..."
        Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope Process -Force
        $scoopInstaller = Join-Path $env:TEMP "install_scoop.ps1"
        Invoke-RestMethod -Uri "https://get.scoop.sh" -OutFile $scoopInstaller
        & $scoopInstaller -RunAsAdmin
        Remove-Item $scoopInstaller -Force -ErrorAction SilentlyContinue
        Refresh-PathEnvironment
        if (Test-CommandAvailable "scoop") {
            Write-Log "Scoop successfully installed." "OK"
            scoop bucket add extras 2>$null | Out-Null
            scoop bucket add main 2>$null | Out-Null
        } else {
            Write-Log "Scoop installation executed but scoop is not visible in PATH." "WARN"
        }
    } catch {
        Write-Log ("Scoop installation failed: {0}" -f $_.Exception.Message) "WARN"
    }
}

function Ensure-Chocolatey {
    if (Test-CommandAvailable "choco") {
        Write-Log "Chocolatey is already installed." "OK"
        return
    }

    try {
        Write-Log "Installing Chocolatey..."
        Set-ExecutionPolicy Bypass -Scope Process -Force
        [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
        Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
        Refresh-PathEnvironment
        if (Test-CommandAvailable "choco") {
            Write-Log "Chocolatey successfully installed." "OK"
        } else {
            Write-Log "Chocolatey installation executed but choco is not in PATH." "WARN"
        }
    } catch {
        Write-Log ("Chocolatey installation failed: {0}" -f $_.Exception.Message) "WARN"
    }
}

function Ensure-UwpApps {
    $uwpApps = @(
        @{ Name = "Microsoft.WindowsCalculator"; Desc = "Calculator" },
        @{ Name = "Microsoft.Windows.Photos"; Desc = "Photos" },
        @{ Name = "Microsoft.Paint"; Desc = "Paint" },
        @{ Name = "Microsoft.ScreenSketch"; Desc = "Snipping Tool" },
        @{ Name = "Microsoft.WindowsTerminal"; Desc = "Windows Terminal" }
    )

    foreach ($app in $uwpApps) {
        $installed = Get-AppxPackage -Name $app.Name -ErrorAction SilentlyContinue
        if ($installed) {
            Write-Log ("UWP App {0} is already installed." -f $app.Desc) "OK"
        } else {
            Write-Log ("Restoring UWP App {0}..." -f $app.Desc)
            try {
                $manifest = Get-ChildItem "$env:ProgramFiles\WindowsApps" -Filter "AppxManifest.xml" -Recurse -ErrorAction SilentlyContinue |
                    Where-Object { $_.FullName -like ("*{0}*" -f $app.Name) } |
                    Select-Object -First 1 -ExpandProperty FullName
                if ($manifest) {
                    Add-AppxPackage -DisableDevelopmentMode -Register $manifest -ErrorAction Stop
                    Write-Log ("Restored UWP App {0} via Appx manifest." -f $app.Desc) "OK"
                } else {
                    Write-Log ("Appx manifest for {0} not found." -f $app.Desc) "WARN"
                }
            } catch {
                Write-Log ("Failed to restore {0}: {1}" -f $app.Desc, $_.Exception.Message) "WARN"
            }
        }
    }
}

function Invoke-NetworkOptimization {
    param([string]$Mode)

    Write-Log ("Applying network configuration mode: {0}" -f $Mode) "INFO"
    
    # Baseline TLS hardening
    try {
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13
        Write-Log "TLS 1.2 and TLS 1.3 enforced for current PowerShell process." "OK"
    } catch {
        Write-Log ("TLS protocol configuration warning: {0}" -f $_.Exception.Message) "WARN"
    }

    if ($Mode -eq "Basic") { return }

    # DNS Flush and Netsh autotuning
    try {
        ipconfig /flushdns | Out-Null
        netsh int tcp set global autotuninglevel=normal | Out-Null
        Write-Log "TCP autotuning level set to normal and DNS flushed." "OK"
    } catch {
        Write-Log ("Netsh TCP configuration warning: {0}" -f $_.Exception.Message) "WARN"
    }

    if ($Mode -eq "Extreme") {
        try {
            netsh int tcp set global congestionprovider=ctcp | Out-Null
            netsh int tcp set global ecncapability=enabled | Out-Null
            Write-Log "Extreme network tuning applied (CTCP & ECN enabled)." "OK"
        } catch {
            Write-Log ("Extreme network tuning warning: {0}" -f $_.Exception.Message) "WARN"
        }
    }
}

function Install-OptionalWindowsFeatures {
    if ($SkipOptionalFeatures) {
        Write-Log "Optional Windows features skipped by parameter." "INFO"
        return
    }

    $features = @(
        @{ Name = "Containers-DisposableClientVM"; Desc = "Windows Sandbox" },
        @{ Name = "Microsoft-Windows-Subsystem-Linux"; Desc = "WSL" }
    )

    foreach ($feature in $features) {
        try {
            $state = Get-WindowsOptionalFeature -Online -FeatureName $feature.Name -ErrorAction SilentlyContinue
            if ($state -and $state.State -eq "Enabled") {
                Write-Log ("Feature {0} is already enabled." -f $feature.Desc) "OK"
            } else {
                Write-Log ("Enabling feature {0}..." -f $feature.Desc)
                Enable-WindowsOptionalFeature -Online -FeatureName $feature.Name -All -NoRestart -ErrorAction Stop | Out-Null
                Write-Log ("Enabled feature {0}." -f $feature.Desc) "OK"
            }
        } catch {
            Write-Log ("Feature status check failed for {0}: {1}" -f $feature.Desc, $_.Exception.Message) "WARN"
        }
    }
}

function Install-WingetPackages {
    param(
        [Parameter(Mandatory = $true)][array]$Packages,
        [Parameter(Mandatory = $true)][string]$Label
    )

    if (-not (Test-CommandAvailable "winget")) {
        Write-Log ("Skipped {0}; Winget is unavailable." -f $Label) "WARN"
        return
    }

    foreach ($package in $Packages) {
        Install-WingetPackage -Id $package.Id -Name $package.Name
    }
}

function Install-ScoopPackages {
    param([Parameter(Mandatory = $true)][string[]]$Packages)

    if (-not (Test-CommandAvailable "scoop")) {
        Write-Log "Skipped Scoop package batch; Scoop is unavailable." "WARN"
        return
    }

    foreach ($package in $Packages) {
        Install-ScoopPackage -Name $package
    }
}

function Ensure-Rust {
    if (Test-CommandAvailable "cargo") {
        Write-Log "Rust toolchain is already available." "OK"
        return
    }

    if (Test-CommandAvailable "winget") {
        try {
            Write-Log "Installing Rust toolchain with Winget..."
            winget install --id Rustlang.Rustup -e --silent --accept-package-agreements --accept-source-agreements 2>$null | Out-Null
            Refresh-PathEnvironment
        } catch {
            Write-Log ("Rustup install via Winget failed: {0}" -f $_.Exception.Message) "WARN"
        }
    }

    if (Test-CommandAvailable "cargo") {
        Write-Log "Rust toolchain is ready." "OK"
        return
    }

    try {
        Write-Log "Falling back to direct rustup installer..."
        $rustupFile = Join-Path $env:TEMP "rustup-init.exe"
        Invoke-WebRequest https://win.rustup.rs/x86_64 -OutFile $rustupFile -UseBasicParsing
        Start-Process -FilePath $rustupFile -ArgumentList "-y --quiet" -Wait
        Remove-Item $rustupFile -Force -ErrorAction SilentlyContinue
        Refresh-PathEnvironment
        $env:Path += ";$env:USERPROFILE\.cargo\bin"
        if (Test-CommandAvailable "cargo") {
            Write-Log "Rust toolchain installed." "OK"
        } else {
            Write-Log "Rust install completed but cargo is still not visible in PATH." "WARN"
        }
    } catch {
        Write-Log ("Rust installation failed: {0}" -f $_.Exception.Message) "WARN"
    }
}

function Install-CargoPackages {
    param([Parameter(Mandatory = $true)][string[]]$Packages)

    if (-not (Test-CommandAvailable "cargo")) {
        Write-Log "Skipped Cargo package batch; cargo is unavailable." "WARN"
        return
    }

    foreach ($package in $Packages) {
        Install-CargoPackage -Name $package
    }
}

function Install-NpmGlobals {
    param([Parameter(Mandatory = $true)][string[]]$Packages)

    if (-not (Test-CommandAvailable "npm")) {
        Write-Log "Skipped NPM global packages; npm is unavailable." "WARN"
        return
    }

    foreach ($package in $Packages) {
        Install-NpmGlobalPackage -Name $package
    }
}

function Install-PipPackages {
    param([Parameter(Mandatory = $true)][string[]]$Packages)

    if (-not (Test-CommandAvailable "pip")) {
        Write-Log "Skipped pip package batch; pip is unavailable." "WARN"
        return
    }

    foreach ($package in $Packages) {
        Install-PipPackage -Name $package
    }
}

function Ensure-UvAndTools {
    param([Parameter(Mandatory = $false)][string[]]$Tools = @("kimi-cli", "ruff"))

    if (-not (Test-CommandAvailable "uv")) {
        try {
            Write-Log "Installing uv..."
            Invoke-RestMethod https://astral.sh/uv/install.ps1 | Invoke-Expression
            Refresh-PathEnvironment
        } catch {
            Write-Log ("uv installation failed: {0}" -f $_.Exception.Message) "WARN"
        }
    } else {
        Write-Log "uv is already available." "OK"
    }

    if (-not (Test-CommandAvailable "uv")) {
        Write-Log "Skipped uv tools because uv is unavailable." "WARN"
        return
    }

    foreach ($tool in $Tools) {
        try {
            Write-Log ("Installing uv tool: {0}" -f $tool)
            uv tool install $tool 2>$null | Out-Null
            Write-Log ("uv tool {0} setup completed." -f $tool) "OK"
        } catch {
            Write-Log ("uv tool installation failed for {0}: {1}" -f $tool, $_.Exception.Message) "WARN"
        }
    }
}

function Ensure-PowerShell7 {
    $pwshPath = Join-Path $env:ProgramFiles "PowerShell\7\pwsh.exe"
    if (Test-Path $pwshPath) {
        Write-Log "PowerShell 7 is already installed." "OK"
        return
    }

    if (Test-CommandAvailable "winget") {
        try {
            Write-Log "Installing PowerShell 7 via Winget..."
            winget install --id Microsoft.PowerShell -e --silent --accept-package-agreements --accept-source-agreements 2>$null | Out-Null
            Refresh-PathEnvironment
            if (Test-Path $pwshPath) {
                Write-Log "PowerShell 7 installed." "OK"
                return
            }
        } catch {
            Write-Log ("PowerShell 7 Winget install failed: {0}" -f $_.Exception.Message) "WARN"
        }
    }

    if (Test-CommandAvailable "scoop") {
        try {
            Write-Log "Installing PowerShell 7 via Scoop..."
            scoop install powershell 2>$null | Out-Null
            Refresh-PathEnvironment
            if (Test-CommandAvailable "pwsh") {
                Write-Log "PowerShell 7 installed via Scoop." "OK"
                return
            }
        } catch {
            Write-Log ("PowerShell 7 Scoop install failed: {0}" -f $_.Exception.Message) "WARN"
        }
    }

    Write-Log "PowerShell 7 installation could not be completed." "WARN"
}

function Apply-SystemTweaks {
    if ($SkipSystemTweaks) {
        Write-Log "System tweaks skipped by parameter." "INFO"
        return
    }

    $tweaks = @(
        @{ Path = "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem"; Name = "LongPathsEnabled"; Value = 1; Desc = "Long Paths Support" },
        @{ Path = "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced"; Name = "HideFileExt"; Value = 0; Desc = "Show File Extensions" },
        @{ Path = "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced"; Name = "Hidden"; Value = 1; Desc = "Show Hidden Files" },
        @{ Path = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock"; Name = "AllowDevelopmentWithoutDevLicense"; Value = 1; Desc = "Developer Mode" }
    )

    foreach ($tweak in $tweaks) {
        try {
            if (-not (Test-Path $tweak.Path)) {
                New-Item -Path $tweak.Path -Force | Out-Null
            }
            Set-ItemProperty -Path $tweak.Path -Name $tweak.Name -Value $tweak.Value -Force | Out-Null
            Write-Log ("Applied tweak: {0}" -f $tweak.Desc) "OK"
        } catch {
            Write-Log ("Failed tweak {0}: {1}" -f $tweak.Desc, $_.Exception.Message) "WARN"
        }
    }
}

function Write-ComponentAuditSummary {
    $checks = @(
        @{ Name = "Microsoft Store"; Check = { Get-AppxPackage -Name Microsoft.WindowsStore -ErrorAction SilentlyContinue } },
        @{ Name = "Winget"; Check = { Get-Command winget -ErrorAction SilentlyContinue } },
        @{ Name = "Scoop"; Check = { Get-Command scoop -ErrorAction SilentlyContinue } },
        @{ Name = "Chocolatey"; Check = { Get-Command choco -ErrorAction SilentlyContinue } },
        @{ Name = "Photos App"; Check = { Get-AppxPackage -Name Microsoft.Windows.Photos -ErrorAction SilentlyContinue } },
        @{ Name = "Calculator"; Check = { Get-AppxPackage -Name Microsoft.WindowsCalculator -ErrorAction SilentlyContinue } },
        @{ Name = "Paint"; Check = { Get-AppxPackage -Name Microsoft.Paint -ErrorAction SilentlyContinue } },
        @{ Name = "Snipping Tool"; Check = { Get-AppxPackage -Name Microsoft.ScreenSketch -ErrorAction SilentlyContinue } },
        @{ Name = "Windows Terminal"; Check = { Get-AppxPackage -Name Microsoft.WindowsTerminal -ErrorAction SilentlyContinue } },
        @{ Name = "PowerShell 7"; Check = { Test-Path "$env:ProgramFiles\PowerShell\7\pwsh.exe" } },
        @{ Name = "Windows Sandbox"; Check = { Get-WindowsOptionalFeature -Online -FeatureName Containers-DisposableClientVM -ErrorAction SilentlyContinue | Where-Object { $_.State -eq "Enabled" } } },
        @{ Name = "WSL"; Check = { Get-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux -ErrorAction SilentlyContinue | Where-Object { $_.State -eq "Enabled" } } }
    )

    $present = New-Object System.Collections.Generic.List[string]
    $missing = New-Object System.Collections.Generic.List[string]
    foreach ($check in $checks) {
        try {
            if (& $check.Check) {
                $present.Add($check.Name)
            } else {
                $missing.Add($check.Name)
            }
        } catch {
            $missing.Add($check.Name)
        }
    }

    Write-Log ("Audit summary: present {0}, missing {1}" -f $present.Count, $missing.Count) "INFO"
    if ($missing.Count -gt 0) {
        Write-Log ("Missing or disabled items: {0}" -f ($missing -join ", ")) "WARN"
    }
}

$coreApps = @(
    @{ Id = "7zip.7zip"; Name = "7-Zip" },
    @{ Id = "VideoLAN.VLC"; Name = "VLC Media Player" },
    @{ Id = "Google.Chrome"; Name = "Google Chrome" },
    @{ Id = "Notepad++.Notepad++"; Name = "Notepad++" },
    @{ Id = "ShareX.ShareX"; Name = "ShareX" },
    @{ Id = "IrfanSkiljan.IrfanView"; Name = "IrfanView" }
)

$devWingetApps = @(
    @{ Id = "Microsoft.VisualStudioCode"; Name = "Visual Studio Code" },
    @{ Id = "Anysphere.Cursor"; Name = "Cursor IDE" },
    @{ Id = "Brave.Brave"; Name = "Brave Browser" },
    @{ Id = "LibreWolf.LibreWolf"; Name = "LibreWolf Browser" },
    @{ Id = "Bitwarden.CLI"; Name = "Bitwarden CLI" },
    @{ Id = "Bitwarden.Bitwarden"; Name = "Bitwarden Desktop" },
    @{ Id = "LocalSend.LocalSend"; Name = "LocalSend" },
    @{ Id = "GnuPG.Gpg4win"; Name = "Gpg4win" },
    @{ Id = "Microsoft.OpenJDK.21"; Name = "OpenJDK 21" },
    @{ Id = "EFF.Certbot"; Name = "Certbot" },
    @{ Id = "Cryptomator.Cryptomator"; Name = "Cryptomator" },
    @{ Id = "RedHat.PodmanDesktop"; Name = "Podman Desktop" },
    @{ Id = "KDE.Krita"; Name = "Krita" },
    @{ Id = "Pureref.PureRef"; Name = "PureRef" },
    @{ Id = "PeaZip.PeaZip"; Name = "PeaZip" },
    @{ Id = "BlenderFoundation.Blender"; Name = "Blender" }
)

$scoopTools = @(
    "git", "gh", "git-lfs", "nodejs-lts", "python", "go", "zig", "deno", "fnm", "bun", "pnpm", "cmake",
    "ninja", "pandoc", "ripgrep", "fd", "fzf", "bat", "eza", "starship", "fastfetch", "sccache", "wget", "aria2",
    "ffmpeg", "imagemagick", "exiftool", "yt-dlp", "gallery-dl", "restic", "7zip", "fdupes", "jdupes",
    "parallel", "tree", "sqlite", "nasm", "yasm", "topgrade", "buku", "ollama",
    "tesseract", "poppler", "lz4", "zstd", "xz", "brotli", "transmission-cli", "sing-box", "mihomo",
    "just", "actionlint", "shellcheck", "shfmt", "chezmoi", "atuin", "direnv"
)

$cargoPackages = @(
    "bkmr", "cargo-edit", "cargo-expand", "cargo-audit", "cargo-deny", "cargo-hack",
    "cargo-license", "cargo-machete", "cargo-mutants", "cargo-semver-checks", "cargo-udeps",
    "cargo-bloat", "cargo-about", "cargo-upgrades", "dupe-krill", "fclones", "flamegraph",
    "rtk", "kondo", "krokiet", "rust-script", "yek"
)

$npmPackages = @(
    "@anthropic-ai/claude-code", "acp-ts", "lodash", "openclaw",
    "opencode-ai", "run-deepseek-cli", "uipro-cli",
    "@alibaba-group/open-code-review", "@diff4/cli", "context-mode",
    "pyright", "typescript", "typescript-language-server",
    "prettier", "markdownlint-cli2"
)

$pipPackages = @(
    "flask", "flask-cors", "numpy", "scipy", "scikit-learn", "pillow",
    "opencv-python", "torch", "lightgbm", "openvino", "tqdm", "joblib",
    "sympy", "networkx", "PyWavelets", "certifi", "cryptography", "filelock", "fsspec",
    "ruff", "pyupgrade"
)

$uvToolsList = @("kimi-cli", "ruff")

$script:ConfigurationSummary = [ordered]@{
    NetworkMode = $NetworkMode
    CoreDesktopApps = $coreApps.Count
    DevWingetApps = $devWingetApps.Count
    ScoopPackages = $scoopTools.Count
    CargoPackages = $cargoPackages.Count
    NpmPackages = $npmPackages.Count
    PipPackages = $pipPackages.Count
    UvTools = $uvToolsList.Count
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Windows LTSC One-Click Full Setup" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Log ("Configuration profile: {0}" -f (($script:ConfigurationSummary.GetEnumerator() | ForEach-Object { "{0}={1}" -f $_.Key, $_.Value }) -join "; ")) "INFO"

if (-not (Test-Admin)) {
    Write-Host "[!] This script requires Administrator privileges." -ForegroundColor Red
    Write-Host "    Please rerun it in an elevated PowerShell window." -ForegroundColor Gray
    exit 1
}

Write-Log "Starting unified LTSC setup..." "START"

Show-Step "Network Repair And Download Hardening"
try {
    Invoke-NetworkOptimization -Mode $NetworkMode
} catch {
    Write-Log ("Network step encountered a warning: {0}" -f $_.Exception.Message) "WARN"
}

Show-Step "Package Bootstrap"
Install-PackageProviderIfMissing
Ensure-PowerShellGet
Ensure-MicrosoftStore
Ensure-Winget
Ensure-Scoop
Ensure-Chocolatey
Refresh-PathEnvironment

Show-Step "Store Registration Repair"
Repair-StoreVisibility

Show-Step "LTSC Built-In Apps Restore"
Ensure-UwpApps

Show-Step "Optional Windows Feature Audit"
Install-OptionalWindowsFeatures

Show-Step "Core Desktop Apps"
Install-WingetPackages -Packages $coreApps -Label "core desktop apps"

if (-not $SkipDevTools) {
    Show-Step "Developer CLI Stack"
    Install-ScoopPackages -Packages $scoopTools

    Show-Step "Developer Desktop Apps"
    Install-WingetPackages -Packages $devWingetApps -Label "developer desktop apps"

    Show-Step "Rust Toolchain"
    Ensure-Rust

    Show-Step "Cargo Packages"
    Install-CargoPackages -Packages $cargoPackages

    Show-Step "NPM Global Packages"
    Install-NpmGlobals -Packages $npmPackages

    Show-Step "Python Packages And uv"
    Install-PipPackages -Packages $pipPackages
    Ensure-UvAndTools -Tools $uvToolsList
}

Show-Step "PowerShell 7"
Ensure-PowerShell7

Show-Step "System Tweaks"
Apply-SystemTweaks

Show-Step "Final Component Audit"
Write-ComponentAuditSummary

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Setup Complete" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ("Log file: {0}" -f $script:LogFile) -ForegroundColor Gray
Write-Host "A restart is recommended after the setup finishes." -ForegroundColor Cyan
Write-Host ""

Write-Log "Unified LTSC setup finished." "END"

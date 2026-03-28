#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Windows LTSC one-click full setup script.
.DESCRIPTION
    Single-entry LTSC rebuild script covering network repair, Store/Winget
    bootstrap, package managers, core apps, developer toolchains,
    PowerShell 7, and system tweaks.
.EXAMPLE
    powershell -ExecutionPolicy Bypass -File .\Scripts\00_QuickSetup.ps1
.EXAMPLE
    powershell -ExecutionPolicy Bypass -File .\Scripts\00_QuickSetup.ps1 -SkipDevTools
#>

param(
    [switch]$SkipDevTools,
    [switch]$SkipOptionalFeatures,
    [switch]$SkipSystemTweaks,
    [ValidateSet("Basic", "Optimized", "Extreme")]
    [string]$NetworkMode = "Optimized"
)

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13
$ErrorActionPreference = "Stop"

$script:RepoRoot = Split-Path -Parent $PSScriptRoot
$script:LogDir = Join-Path $script:RepoRoot "Logs"
if (-not (Test-Path $script:LogDir)) {
    New-Item -ItemType Directory -Path $script:LogDir -Force | Out-Null
}
$script:LogFile = Join-Path $script:LogDir ("setup_{0}.log" -f (Get-Date -Format "yyyyMMdd_HHmmss"))
$script:StepIndex = 0
$script:StepTotal = if ($SkipDevTools) { 9 } else { 15 }

function Write-Log {
    param(
        [Parameter(Mandatory = $true)][string]$Message,
        [ValidateSet("INFO", "OK", "WARN", "ERROR", "START", "END")][string]$Level = "INFO"
    )

    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $entry = "[{0}] [{1}] {2}" -f $timestamp, $Level, $Message
    Add-Content -Path $script:LogFile -Value $entry

    $color = switch ($Level) {
        "OK" { "Green" }
        "WARN" { "Yellow" }
        "ERROR" { "Red" }
        "START" { "Cyan" }
        "END" { "Cyan" }
        default { "Gray" }
    }

    Write-Host ("  {0}" -f $Message) -ForegroundColor $color
}

function Show-Step {
    param([Parameter(Mandatory = $true)][string]$Title)

    $script:StepIndex++
    Write-Host ""
    Write-Host ("━" * 68) -ForegroundColor Cyan
    Write-Host ("  [{0}/{1}] {2}" -f $script:StepIndex, $script:StepTotal, $Title) -ForegroundColor Yellow
    Write-Host ("━" * 68) -ForegroundColor Cyan
}

function Test-Admin {
    $principal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Test-CommandAvailable {
    param([Parameter(Mandatory = $true)][string]$Name)
    return [bool](Get-Command $Name -ErrorAction SilentlyContinue)
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

    $existing = winget list --id $Id -e 2>$null | Select-String $Id
    if ($existing) {
        Write-Log ("{0} is already installed." -f $Name) "OK"
        return
    }

    try {
        Write-Log ("Installing {0}..." -f $Name)
        winget install --id $Id -e --silent --accept-package-agreements --accept-source-agreements 2>$null | Out-Null
        Write-Log ("Installed {0}." -f $Name) "OK"
    } catch {
        Write-Log ("Failed to install {0}: {1}" -f $Name, $_.Exception.Message) "WARN"
    }
}

function Install-ScoopPackage {
    param([Parameter(Mandatory = $true)][string]$Name)

    if (-not (Test-CommandAvailable "scoop")) {
        Write-Log ("Skipped {0}; Scoop is unavailable." -f $Name) "WARN"
        return
    }

    try {
        $scoopStatus = scoop list 2>$null | Select-String ("^" + [regex]::Escape($Name) + "\s")
        if ($scoopStatus) {
            Write-Log ("Scoop package already installed: {0}" -f $Name) "OK"
            return
        }

        Write-Log ("Installing Scoop package: {0}" -f $Name)
        scoop install $Name 2>$null | Out-Null
        Write-Log ("Installed Scoop package: {0}" -f $Name) "OK"
    } catch {
        Write-Log ("Failed Scoop install for {0}: {1}" -f $Name, $_.Exception.Message) "WARN"
    }
}

function Install-CargoPackage {
    param([Parameter(Mandatory = $true)][string]$Name)

    if (-not (Test-CommandAvailable "cargo")) {
        Write-Log ("Skipped {0}; cargo is unavailable." -f $Name) "WARN"
        return
    }

    try {
        Write-Log ("Installing cargo package: {0}" -f $Name)
        cargo install $Name 2>$null | Out-Null
        Write-Log ("Cargo package processed: {0}" -f $Name) "OK"
    } catch {
        Write-Log ("Failed cargo install for {0}: {1}" -f $Name, $_.Exception.Message) "WARN"
    }
}

function Install-NpmGlobalPackage {
    param([Parameter(Mandatory = $true)][string]$Name)

    if (-not (Test-CommandAvailable "npm")) {
        Write-Log ("Skipped {0}; npm is unavailable." -f $Name) "WARN"
        return
    }

    try {
        Write-Log ("Installing NPM global package: {0}" -f $Name)
        npm install -g $Name 2>$null | Out-Null
        Write-Log ("Installed NPM global package: {0}" -f $Name) "OK"
    } catch {
        Write-Log ("Failed npm install for {0}: {1}" -f $Name, $_.Exception.Message) "WARN"
    }
}

function Install-PipPackage {
    param([Parameter(Mandatory = $true)][string]$Name)

    if (-not (Test-CommandAvailable "pip")) {
        Write-Log ("Skipped {0}; pip is unavailable." -f $Name) "WARN"
        return
    }

    try {
        Write-Log ("Installing pip package: {0}" -f $Name)
        pip install $Name --quiet 2>$null | Out-Null
        Write-Log ("Installed pip package: {0}" -f $Name) "OK"
    } catch {
        Write-Log ("Failed pip install for {0}: {1}" -f $Name, $_.Exception.Message) "WARN"
    }
}

function Install-PackageProviderIfMissing {
    try {
        $nugetProvider = Get-PackageProvider -Name NuGet -ErrorAction SilentlyContinue
        if (-not $nugetProvider) {
            Write-Log "Installing NuGet provider..."
            Install-PackageProvider -Name NuGet -MinimumVersion 2.8.5.201 -Force -Scope CurrentUser | Out-Null
            Write-Log "NuGet provider ready." "OK"
        } else {
            Write-Log "NuGet provider already available." "OK"
        }
    } catch {
        Write-Log ("NuGet provider setup failed: {0}" -f $_.Exception.Message) "WARN"
    }
}

function Ensure-PowerShellGet {
    try {
        Write-Log "Updating PowerShellGet..."
        Install-Module -Name PowerShellGet -Force -AllowClobber -Scope CurrentUser -ErrorAction Stop | Out-Null
        Write-Log "PowerShellGet updated." "OK"
    } catch {
        Write-Log ("PowerShellGet update skipped: {0}" -f $_.Exception.Message) "WARN"
    }
}

function Ensure-MicrosoftStore {
    if (Get-AppxPackage -Name Microsoft.WindowsStore -ErrorAction SilentlyContinue) {
        Write-Log "Microsoft Store is already present." "OK"
        return
    }

    Write-Log "Triggering Microsoft Store installation..."
    try {
        wsreset -i 2>$null
        Start-Sleep -Seconds 10
        if (Get-AppxPackage -Name Microsoft.WindowsStore -ErrorAction SilentlyContinue) {
            Write-Log "Microsoft Store installed successfully." "OK"
        } else {
            Write-Log "Store installation was triggered; it may appear after a short delay." "INFO"
        }
    } catch {
        Write-Log ("Microsoft Store installation failed: {0}" -f $_.Exception.Message) "WARN"
    }
}

function Repair-StoreVisibility {
    $store = Get-AppxPackage -Name Microsoft.WindowsStore -ErrorAction SilentlyContinue
    if (-not $store) {
        Write-Log "Skipped Store visibility repair because Store is not installed." "WARN"
        return
    }

    try {
        $manifestPath = Join-Path $store.InstallLocation "AppxManifest.xml"
        if (Test-Path $manifestPath) {
            Add-AppxPackage -Register $manifestPath -DisableDevelopmentMode -ForceApplicationShutdown -ErrorAction Stop
            Write-Log "Re-registered Microsoft Store package." "OK"
        }
    } catch {
        Write-Log ("Store re-registration skipped: {0}" -f $_.Exception.Message) "WARN"
    }

    try {
        if (Get-Process ShellExperienceHost -ErrorAction SilentlyContinue) {
            Stop-Process -Name ShellExperienceHost -Force -ErrorAction SilentlyContinue
            Write-Log "Restarted ShellExperienceHost to refresh Start menu visibility." "OK"
        }
    } catch {
        Write-Log ("Shell refresh skipped: {0}" -f $_.Exception.Message) "WARN"
    }
}

function Ensure-WingetDependencies {
    $tempDir = Join-Path $env:TEMP "winget_deps"
    if (-not (Test-Path $tempDir)) {
        New-Item -ItemType Directory -Path $tempDir -Force | Out-Null
    }

    $deps = @(
        @{
            Name = "Microsoft.VCLibs"
            Url = "https://aka.ms/Microsoft.VCLibs.x64.14.00.Desktop.appx"
            File = "Microsoft.VCLibs.x64.appx"
        },
        @{
            Name = "Microsoft.UI.Xaml"
            Url = "https://github.com/microsoft/microsoft-ui-xaml/releases/download/v2.8.6/Microsoft.UI.Xaml.2.8.x64.appx"
            File = "Microsoft.UI.Xaml.2.8.x64.appx"
        }
    )

    foreach ($dep in $deps) {
        $existingDep = Get-AppxPackage -AllUsers -ErrorAction SilentlyContinue | Where-Object { $_.Name -like "$($dep.Name)*" }
        if ($existingDep) {
            Write-Log ("Dependency already present: {0}" -f $dep.Name) "OK"
            continue
        }

        $targetFile = Join-Path $tempDir $dep.File
        try {
            Write-Log ("Downloading dependency: {0}" -f $dep.Name)
            Start-BitsTransfer -Source $dep.Url -Destination $targetFile -Priority High -ErrorAction Stop
            Add-AppxPackage -Path $targetFile -ErrorAction Stop
            Write-Log ("Installed dependency: {0}" -f $dep.Name) "OK"
        } catch {
            Write-Log ("Dependency install failed for {0}: {1}" -f $dep.Name, $_.Exception.Message) "WARN"
        }
    }
}

function Refresh-PathEnvironment {
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" +
                [System.Environment]::GetEnvironmentVariable("Path", "User")
}

function Ensure-Winget {
    if (Test-CommandAvailable "winget") {
        Write-Log "Winget is already available." "OK"
        return
    }

    Ensure-WingetDependencies

    $tempDir = Join-Path $env:TEMP "WingetSetup"
    if (-not (Test-Path $tempDir)) {
        New-Item -ItemType Directory -Path $tempDir -Force | Out-Null
    }
    $bundleFile = Join-Path $tempDir "winget.msixbundle"
    $sources = @(
        "https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle",
        "https://mirror.ghproxy.com/https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle",
        "https://ghproxy.net/https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle",
        "https://github.moeyy.xyz/https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle"
    )

    $downloaded = $false
    foreach ($source in $sources) {
        try {
            Write-Log ("Downloading Winget bundle from {0}" -f $source)
            $wc = New-Object System.Net.WebClient
            $wc.Headers.Add("User-Agent", "Mozilla/5.0")
            $wc.DownloadFile($source, $bundleFile)
            if ((Test-Path $bundleFile) -and ((Get-Item $bundleFile).Length -gt 50MB)) {
                $downloaded = $true
                break
            }
        } catch {
            Write-Log ("Winget source failed: {0}" -f $_.Exception.Message) "WARN"
        }
    }

    if ($downloaded) {
        try {
            Add-AppxPackage -Path $bundleFile -ForceApplicationShutdown -ErrorAction Stop
            Write-Log "Winget package installed." "OK"
        } catch {
            Write-Log ("Winget package install failed: {0}" -f $_.Exception.Message) "WARN"
        }
    } else {
        Write-Log "All Winget download mirrors failed." "WARN"
    }

    Refresh-PathEnvironment
    if (Test-CommandAvailable "winget") {
        Write-Log "Winget is ready." "OK"
    } else {
        Write-Log "Winget is still unavailable; later package installs may be skipped." "WARN"
    }
}

function Ensure-Scoop {
    if (Test-CommandAvailable "scoop") {
        Write-Log "Scoop is already available." "OK"
        return
    }

    try {
        Write-Log "Installing Scoop..."
        $installerFile = Join-Path $env:TEMP "install_scoop.ps1"
        Invoke-WebRequest -Uri "https://get.scoop.sh" -OutFile $installerFile -UseBasicParsing
        & $installerFile -RunAsAdmin
        Remove-Item $installerFile -Force -ErrorAction SilentlyContinue
        Refresh-PathEnvironment
        if (Test-CommandAvailable "scoop") {
            foreach ($bucket in @("extras", "versions", "nerd-fonts")) {
                scoop bucket add $bucket 2>$null | Out-Null
            }
            Write-Log "Scoop installed." "OK"
        } else {
            Write-Log "Scoop installation did not expose the command immediately." "WARN"
        }
    } catch {
        Write-Log ("Scoop installation failed: {0}" -f $_.Exception.Message) "WARN"
    }
}

function Ensure-Chocolatey {
    if (Test-CommandAvailable "choco") {
        Write-Log "Chocolatey is already available." "OK"
        return
    }

    try {
        Write-Log "Installing Chocolatey..."
        Set-ExecutionPolicy Bypass -Scope Process -Force
        [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
        Invoke-Expression ((New-Object System.Net.WebClient).DownloadString("https://community.chocolatey.org/install.ps1"))
        Refresh-PathEnvironment
        if (Test-CommandAvailable "choco") {
            Write-Log "Chocolatey installed." "OK"
        } else {
            Write-Log "Chocolatey installation finished but command is not yet visible." "WARN"
        }
    } catch {
        Write-Log ("Chocolatey installation failed: {0}" -f $_.Exception.Message) "WARN"
    }
}

function Ensure-UwpApps {
    if (-not (Test-CommandAvailable "winget")) {
        Write-Log "Skipped UWP restore because Winget is unavailable." "WARN"
        return
    }

    $uwpApps = @(
        @{ Title = "Photos"; Id = "Microsoft.Windows.Photos" },
        @{ Title = "Calculator"; Id = "Microsoft.WindowsCalculator" },
        @{ Title = "Paint"; Id = "Microsoft.Paint" },
        @{ Title = "Snipping Tool"; Id = "Microsoft.ScreenSketch" },
        @{ Title = "Alarms & Clock"; Id = "Microsoft.WindowsAlarms" },
        @{ Title = "Windows Camera"; Id = "Microsoft.WindowsCamera" },
        @{ Title = "Xbox Game Bar"; Id = "Microsoft.XboxGamingOverlay" },
        @{ Title = "Windows Terminal"; Id = "Microsoft.WindowsTerminal" }
    )

    $alternatives = @(
        @{ Title = "ShareX"; Id = "ShareX.ShareX" },
        @{ Title = "IrfanView"; Id = "IrfanSkiljan.IrfanView" },
        @{ Title = "Paint.NET"; Id = "dotPDN.Paint.NET" },
        @{ Title = "Q-Dir"; Id = "QDir.QDir" }
    )

    foreach ($app in $uwpApps) {
        if (Get-AppxPackage -Name $app.Id -AllUsers -ErrorAction SilentlyContinue) {
            Write-Log ("{0} is already present." -f $app.Title) "OK"
            continue
        }

        try {
            Write-Log ("Installing {0}..." -f $app.Title)
            winget install --id $app.Id -e --silent --accept-package-agreements --accept-source-agreements 2>$null | Out-Null
            Write-Log ("Completed install request for {0}." -f $app.Title) "OK"
        } catch {
            Write-Log ("Failed to restore {0}: {1}" -f $app.Title, $_.Exception.Message) "WARN"
        }
    }

    foreach ($app in $alternatives) {
        $existing = winget list --id $app.Id -e 2>$null | Select-String $app.Id
        if ($existing) {
            Write-Log ("Alternative already installed: {0}" -f $app.Title) "OK"
            continue
        }

        try {
            Write-Log ("Installing alternative app: {0}" -f $app.Title)
            winget install --id $app.Id -e --silent --accept-package-agreements --accept-source-agreements 2>$null | Out-Null
            Write-Log ("Installed alternative app: {0}" -f $app.Title) "OK"
        } catch {
            Write-Log ("Failed alternative app install for {0}: {1}" -f $app.Title, $_.Exception.Message) "WARN"
        }
    }
}

function Invoke-NetworkOptimization {
    param(
        [ValidateSet("Basic", "Optimized", "Extreme")]
        [string]$Mode = "Optimized"
    )

    foreach ($proto in @("TLS 1.2", "TLS 1.3")) {
        foreach ($side in @("Client", "Server")) {
            $path = "HKLM:\SYSTEM\CurrentControlSet\Control\SecurityProviders\SCHANNEL\Protocols\$proto\$side"
            if (-not (Test-Path $path)) {
                New-Item -Path $path -Force | Out-Null
            }
            New-ItemProperty -Path $path -Name "Enabled" -Value 1 -PropertyType DWORD -Force | Out-Null
            New-ItemProperty -Path $path -Name "DisabledByDefault" -Value 0 -PropertyType DWORD -Force | Out-Null
        }
    }
    Write-Log "Enabled TLS 1.2/1.3 client and server profiles." "OK"

    try {
        $activeInterface = Get-NetConnectionProfile | Where-Object { $_.IPv4Connectivity -eq "Internet" } | Select-Object -First 1
        if ($activeInterface) {
            Set-DnsClientServerAddress -InterfaceAlias $activeInterface.InterfaceAlias -ServerAddresses ("1.1.1.1", "8.8.8.8", "223.5.5.5")
            Write-Log ("Configured DNS for {0}." -f $activeInterface.InterfaceAlias) "OK"
        }
    } catch {
        Write-Log ("DNS optimization skipped: {0}" -f $_.Exception.Message) "WARN"
    }

    try {
        netsh winsock reset | Out-Null
        netsh int ip reset | Out-Null
        netsh winhttp reset proxy | Out-Null
        ipconfig /flushdns | Out-Null
        Write-Log "Reset Winsock, IP stack, proxy, and DNS cache." "OK"
    } catch {
        Write-Log ("Network reset encountered a warning: {0}" -f $_.Exception.Message) "WARN"
    }

    if ($Mode -in @("Optimized", "Extreme")) {
        try {
            if ($Mode -eq "Extreme") {
                netsh int tcp set global autotuninglevel=experimental | Out-Null
                netsh int tcp set global ecncapability=enabled | Out-Null
                netsh int tcp set global initialrto=300 | Out-Null
                netsh int tcp set global maxsynretransmissions=3 | Out-Null
                netsh int tcp set global pacingprofile=always | Out-Null
            } else {
                netsh int tcp set global autotuninglevel=normal | Out-Null
                netsh int tcp set global ecncapability=enabled | Out-Null
            }

            netsh int tcp set global timestamps=disabled | Out-Null
            netsh int tcp set global rss=enabled | Out-Null
            netsh int tcp set global rsc=enabled | Out-Null
            netsh int tcp set global nonsackrttresiliency=enabled | Out-Null
            Set-NetTCPSetting -SettingAlias Internet -CongestionProvider CTCP -ErrorAction SilentlyContinue

            $adapters = Get-NetAdapter -ErrorAction SilentlyContinue | Where-Object { $_.Status -eq "Up" }
            foreach ($nic in $adapters) {
                Disable-NetAdapterPowerManagement -Name $nic.Name -ErrorAction SilentlyContinue
            }
            Write-Log ("Applied {0} network tuning." -f $Mode) "OK"
        } catch {
            Write-Log ("Advanced network tuning skipped: {0}" -f $_.Exception.Message) "WARN"
        }
    }
}

function Install-OptionalWindowsFeatures {
    if ($SkipOptionalFeatures) {
        Write-Log "Optional feature checks skipped by parameter." "INFO"
        return
    }

    $features = @(
        @{ Name = "Containers-DisposableClientVM"; Desc = "Windows Sandbox" },
        @{ Name = "Microsoft-Windows-Subsystem-Linux"; Desc = "WSL 2" },
        @{ Name = "NetFx3"; Desc = ".NET Framework 3.5" }
    )

    foreach ($feature in $features) {
        try {
            $status = Get-WindowsOptionalFeature -Online -FeatureName $feature.Name -ErrorAction Stop
            if ($status.State -eq "Enabled") {
                Write-Log ("Feature enabled: {0}" -f $feature.Desc) "OK"
            } else {
                Write-Log ("Feature available for manual enablement: {0}" -f $feature.Desc) "INFO"
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

    try {
        Write-Log "Installing uv tool: kimi-cli"
        uv tool install kimi-cli 2>$null | Out-Null
        Write-Log "uv tool setup completed." "OK"
    } catch {
        Write-Log ("uv tool installation failed: {0}" -f $_.Exception.Message) "WARN"
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
            Write-Log "Installing PowerShell 7 with Winget..."
            winget install --id Microsoft.PowerShell -e --silent --accept-package-agreements --accept-source-agreements 2>$null | Out-Null
            if (Test-Path $pwshPath) {
                Write-Log "PowerShell 7 installed." "OK"
                return
            }
        } catch {
            Write-Log ("PowerShell 7 Winget install failed: {0}" -f $_.Exception.Message) "WARN"
        }
    }

    try {
        Write-Log "Falling back to GitHub release lookup for PowerShell 7..."
        $release = Invoke-RestMethod -Uri "https://api.github.com/repos/PowerShell/PowerShell/releases/latest" -Headers @{ "User-Agent" = "Mozilla/5.0" }
        $msiUrl = $release.assets | Where-Object { $_.name -like "*-win-x64.msi" } | Select-Object -First 1 -ExpandProperty browser_download_url
        $msiFile = Join-Path $env:TEMP "PowerShell7.msi"
        Invoke-WebRequest -Uri $msiUrl -OutFile $msiFile -UseBasicParsing
        Start-Process msiexec.exe -ArgumentList "/i `"$msiFile`" /quiet /norestart" -Wait
        if (Test-Path $pwshPath) {
            Write-Log "PowerShell 7 installed successfully." "OK"
        } else {
            Write-Log "PowerShell 7 install finished but executable is not yet visible." "WARN"
        }
    } catch {
        Write-Log ("PowerShell 7 installation failed: {0}" -f $_.Exception.Message) "WARN"
    }
}

function Apply-SystemTweaks {
    if ($SkipSystemTweaks) {
        Write-Log "System tweaks skipped by parameter." "INFO"
        return
    }

    $tweaks = @(
        @{ Path = "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem"; Name = "LongPathsEnabled"; Value = 1; Desc = "Long Paths" },
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
    @{ Id = "Bitwarden.CLI"; Name = "Bitwarden CLI" },
    @{ Id = "LocalSend.LocalSend"; Name = "LocalSend" },
    @{ Id = "GnuPG.Gpg4win"; Name = "Gpg4win" },
    @{ Id = "Microsoft.OpenJDK.21"; Name = "OpenJDK 21" },
    @{ Id = "EFF.Certbot"; Name = "Certbot" }
)

$scoopTools = @(
    "git", "gh", "nodejs-lts", "python", "go", "zig", "deno", "fnm", "cmake",
    "ninja", "pandoc", "ripgrep", "wget", "aria2", "ffmpeg", "imagemagick",
    "exiftool", "yt-dlp", "gallery-dl", "restic", "7zip", "fdupes", "jdupes",
    "parallel", "tree", "sqlite", "nasm", "yasm", "topgrade", "buku", "ollama",
    "tesseract", "poppler", "lz4", "zstd", "xz", "brotli", "transmission-cli"
)

$cargoPackages = @(
    "bkmr", "cargo-edit", "cargo-expand", "cargo-audit", "cargo-deny", "cargo-hack",
    "cargo-license", "cargo-machete", "cargo-mutants", "cargo-semver-checks", "cargo-udeps",
    "cargo-bloat", "cargo-about", "cargo-upgrades", "dupe-krill", "fclones", "flamegraph"
)

$npmPackages = @(
    "@anthropic-ai/claude-code", "acp-ts", "lodash", "openclaw",
    "opencode-ai", "run-deepseek-cli", "uipro-cli"
)

$pipPackages = @(
    "flask", "flask-cors", "numpy", "scipy", "scikit-learn", "pillow",
    "opencv-python", "torch", "lightgbm", "openvino", "tqdm", "joblib",
    "sympy", "networkx", "PyWavelets", "certifi", "cryptography", "filelock", "fsspec"
)

$script:ConfigurationSummary = [ordered]@{
    NetworkMode = $NetworkMode
    CoreDesktopApps = $coreApps.Count
    DevWingetApps = $devWingetApps.Count
    ScoopPackages = $scoopTools.Count
    CargoPackages = $cargoPackages.Count
    NpmPackages = $npmPackages.Count
    PipPackages = $pipPackages.Count
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
    Ensure-UvAndTools
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

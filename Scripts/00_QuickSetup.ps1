#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Windows LTSC one-click full setup script.
.DESCRIPTION
    Consolidates the old multi-step flow into a single entrypoint:
    network repair, Store/Winget bootstrap, package managers, core apps,
    developer toolchains, PowerShell 7, and system tweaks.
.EXAMPLE
    powershell -ExecutionPolicy Bypass -File .\Scripts\00_QuickSetup.ps1
.EXAMPLE
    powershell -ExecutionPolicy Bypass -File .\Scripts\00_QuickSetup.ps1 -SkipDevTools
#>

param(
    [switch]$SkipDevTools,
    [switch]$SkipOptionalFeatures,
    [switch]$SkipSystemTweaks
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
$script:StepTotal = if ($SkipDevTools) { 7 } else { 13 }

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

function Invoke-ScriptIfPresent {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [string[]]$ArgumentList = @()
    )

    if (-not (Test-Path $Path)) {
        Write-Log ("Skipped missing helper script: {0}" -f (Split-Path -Leaf $Path)) "WARN"
        return $false
    }

    & $Path @ArgumentList
    return $true
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

    $installerPath = Join-Path $PSScriptRoot "Install-Winget.ps1"
    if (Test-Path $installerPath) {
        Write-Log "Running Winget installer helper..."
        try {
            & $installerPath
        } catch {
            Write-Log ("Winget helper reported an error: {0}" -f $_.Exception.Message) "WARN"
        }
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
        $scoopInstaller = Join-Path $PSScriptRoot "02_install.ps1"
        if (Test-Path $scoopInstaller) {
            & $scoopInstaller -RunAsAdmin
        } else {
            Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression
        }
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
    $uwpScript = Join-Path $PSScriptRoot "Install-UWP-Apps.ps1"
    if (Invoke-ScriptIfPresent -Path $uwpScript) {
        Write-Log "UWP restore helper finished." "OK"
        return
    }

    if (-not (Test-CommandAvailable "winget")) {
        Write-Log "Skipped UWP restore because Winget is unavailable." "WARN"
        return
    }

    $uwpApps = @(
        @{ Title = "Photos"; Id = "Microsoft.Windows.Photos" },
        @{ Title = "Calculator"; Id = "Microsoft.WindowsCalculator" },
        @{ Title = "Paint"; Id = "Microsoft.Paint" },
        @{ Title = "Snipping Tool"; Id = "Microsoft.ScreenSketch" },
        @{ Title = "Windows Terminal"; Id = "Microsoft.WindowsTerminal" }
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
        $existing = winget list --id $package.Id -e 2>$null | Select-String $package.Id
        if ($existing) {
            Write-Log ("{0} is already installed." -f $package.Name) "OK"
            continue
        }

        try {
            Write-Log ("Installing {0}..." -f $package.Name)
            winget install --id $package.Id -e --silent --accept-package-agreements --accept-source-agreements 2>$null | Out-Null
            Write-Log ("Installed {0}." -f $package.Name) "OK"
        } catch {
            Write-Log ("Failed to install {0}: {1}" -f $package.Name, $_.Exception.Message) "WARN"
        }
    }
}

function Install-ScoopPackages {
    param([Parameter(Mandatory = $true)][string[]]$Packages)

    if (-not (Test-CommandAvailable "scoop")) {
        Write-Log "Skipped Scoop package batch; Scoop is unavailable." "WARN"
        return
    }

    foreach ($package in $Packages) {
        try {
            $scoopStatus = scoop list 2>$null | Select-String ("^" + [regex]::Escape($package) + "\s")
            if ($scoopStatus) {
                Write-Log ("Scoop package already installed: {0}" -f $package) "OK"
                continue
            }

            Write-Log ("Installing Scoop package: {0}" -f $package)
            scoop install $package 2>$null | Out-Null
            Write-Log ("Installed Scoop package: {0}" -f $package) "OK"
        } catch {
            Write-Log ("Failed Scoop install for {0}: {1}" -f $package, $_.Exception.Message) "WARN"
        }
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
        try {
            Write-Log ("Installing cargo package: {0}" -f $package)
            cargo install $package 2>$null | Out-Null
            Write-Log ("Cargo package processed: {0}" -f $package) "OK"
        } catch {
            Write-Log ("Failed cargo install for {0}: {1}" -f $package, $_.Exception.Message) "WARN"
        }
    }
}

function Install-NpmGlobals {
    param([Parameter(Mandatory = $true)][string[]]$Packages)

    if (-not (Test-CommandAvailable "npm")) {
        Write-Log "Skipped NPM global packages; npm is unavailable." "WARN"
        return
    }

    foreach ($package in $Packages) {
        try {
            Write-Log ("Installing NPM global package: {0}" -f $package)
            npm install -g $package 2>$null | Out-Null
            Write-Log ("Installed NPM global package: {0}" -f $package) "OK"
        } catch {
            Write-Log ("Failed npm install for {0}: {1}" -f $package, $_.Exception.Message) "WARN"
        }
    }
}

function Install-PipPackages {
    param([Parameter(Mandatory = $true)][string[]]$Packages)

    if (-not (Test-CommandAvailable "pip")) {
        Write-Log "Skipped pip package batch; pip is unavailable." "WARN"
        return
    }

    foreach ($package in $Packages) {
        try {
            Write-Log ("Installing pip package: {0}" -f $package)
            pip install $package --quiet 2>$null | Out-Null
            Write-Log ("Installed pip package: {0}" -f $package) "OK"
        } catch {
            Write-Log ("Failed pip install for {0}: {1}" -f $package, $_.Exception.Message) "WARN"
        }
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

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Windows LTSC One-Click Full Setup" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

if (-not (Test-Admin)) {
    Write-Host "[!] This script requires Administrator privileges." -ForegroundColor Red
    Write-Host "    Please rerun it in an elevated PowerShell window." -ForegroundColor Gray
    exit 1
}

Write-Log "Starting unified LTSC setup..." "START"

Show-Step "Network Repair And Download Hardening"
try {
    $networkScript = Join-Path $PSScriptRoot "Optimize-Network.ps1"
    if (Test-Path $networkScript) {
        Write-Log "Running network optimizer..."
        & $networkScript -Mode Optimized
        Write-Log "Network optimization completed." "OK"
    } else {
        $tlsPath = "HKLM:\SYSTEM\CurrentControlSet\Control\SecurityProviders\SCHANNEL\Protocols\TLS 1.2\Client"
        if (-not (Test-Path $tlsPath)) {
            New-Item -Path $tlsPath -Force | Out-Null
        }
        Set-ItemProperty -Path $tlsPath -Name "Enabled" -Value 1 -Force | Out-Null
        Write-Log "Applied basic TLS 1.2 fallback." "OK"
    }
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

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Setup Complete" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ("Log file: {0}" -f $script:LogFile) -ForegroundColor Gray
Write-Host "A restart is recommended after the setup finishes." -ForegroundColor Cyan
Write-Host ""

Write-Log "Unified LTSC setup finished." "END"

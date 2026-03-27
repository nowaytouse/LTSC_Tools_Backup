#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Robust Winget Installer for Windows LTSC
.DESCRIPTION
    Attempts to download and install Microsoft.DesktopAppInstaller (Winget) from multiple 
    sources with automatic retries and fallback mirrors.
.EXAMPLE
    .\Install-Winget.ps1
#>

# Enable TLS 1.2/1.3
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Installing Winget (App Installer)" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

$tempDir = "$env:TEMP\WingetSetup"
if (!(Test-Path $tempDir)) { New-Item -ItemType Directory -Path $tempDir -Force | Out-Null }
$file = "$tempDir\winget.msixbundle"

# Mirror and Direct Sources
$sources = @(
    "https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle",
    "https://mirror.ghproxy.com/https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle",
    "https://ghproxy.net/https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle",
    "https://github.moeyy.xyz/https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle"
)

$downloaded = $false
$maxRetries = 3

foreach ($source in $sources) {
    $retryCount = 0
    while ($retryCount -lt $maxRetries -and !$downloaded) {
        $retryCount++
        Write-Host "`n Trying source: $source (Attempt $retryCount/$maxRetries)" -ForegroundColor Yellow
        
        try {
            $ProgressPreference = 'SilentlyContinue'
            # Using WebClient for better progress/header control in some environments
            $wc = New-Object System.Net.WebClient
            $wc.Headers.Add("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            
            $wc.DownloadFile($source, $file)
            
            if (Test-Path $file) {
                $size = (Get-Item $file).Length / 1MB
                if ($size -gt 50) {
                    Write-Host "  Download successful: $($size.ToString('F2')) MB" -ForegroundColor Green
                    $downloaded = $true
                    break
                } else {
                    Write-Host "  File too small ($($size.ToString('F2')) MB), likely a broken link. Retrying..." -ForegroundColor Red
                    Remove-Item $file -Force
                }
            }
        } catch {
            Write-Host "  Failed: $($_.Exception.Message)" -ForegroundColor Red
            Start-Sleep -Seconds 2
        }
    }
    if ($downloaded) { break }
}

if (!$downloaded) {
    Write-Host "`n[ERROR] All sources failed. Please check your internet connection or use a proxy." -ForegroundColor Red
    Write-Host "Alternatively, you can manually install 'App Installer' from the Microsoft Store." -ForegroundColor Gray
    Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
    exit 1
}

Write-Host "`n Installing Winget..." -ForegroundColor Cyan
try {
    Add-AppxPackage -Path $file -ForceApplicationShutdown -ErrorAction Stop
    Write-Host "  ✓ Winget installed successfully!" -ForegroundColor Green
    
    # Refresh PATH and Verify
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
    Write-Host "`n Verification:" -ForegroundColor Yellow
    & winget --version
} catch {
    Write-Host "`n[ERROR] Installation failed: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "Note: You might need to install 'Windows App Runtime' dependencies first." -ForegroundColor Gray
}

# Cleanup
Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
Write-Host "`n Done!" -ForegroundColor Green

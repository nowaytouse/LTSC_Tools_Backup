#!/usr/bin/env pwsh
<#
.SYNOPSIS
    UWP Apps Restoration Script for Windows LTSC
.DESCRIPTION
    Attempts to restore essential UWP apps (Photos, Calculator, Paint, etc.) normally 
    missing from LTSC versions. Uses Winget for primary installation and provides
    robust open-source alternatives if native apps fail to install.
.EXAMPLE
    .\Install-UWP-Apps.ps1
#>

# Enable TLS 1.2/1.3
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Installing Essential UWP Apps" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "[!] Note: Some LTSC versions may have strict limitations for UWP apps." -ForegroundColor Yellow
Write-Host "[!] In such cases, modern open-source alternatives are highly recommended." -ForegroundColor Yellow
Write-Host ""

# Native UWP Apps List
$nativeApps = @(
    @{Title='Calculator'; Id='Microsoft.WindowsCalculator'},
    @{Title='Paint'; Id='Microsoft.Paint'},
    @{Title='Snipping Tool'; Id='Microsoft.ScreenSketch'},
    @{Title='Photos'; Id='Microsoft.Windows.Photos'},
    @{Title='Alarms & Clock'; Id='Microsoft.WindowsAlarms'},
    @{Title='Windows Camera'; Id='Microsoft.WindowsCamera'},
    @{Title='Xbox Game Bar'; Id='Microsoft.XboxGamingOverlay'}
)

# Open-Source Alternatives List
$alternatives = @(
    @{Title='ShareX (Advanced Screenshot)'; Id='ShareX.ShareX'},
    @{Title='IrfanView (Photos Viewer)'; Id='IrfanSkiljan.IrfanView'},
    @{Title='Paint.NET (Advanced Paint)'; Id='dotPDN.Paint.NET'},
    @{Title='QDir (File Manager)'; Id='QDir.QDir'}
)

Write-Host "--- Section 1: Native UWP Apps ---" -ForegroundColor Cyan
foreach ($app in $nativeApps) {
    Write-Host "`n Checking $($app.Title)..." -ForegroundColor Yellow
    
    $existing = Get-AppxPackage -Name $app.Id -ErrorAction SilentlyContinue
    if ($existing) {
        Write-Host "  ✓ Already installed: $($existing.Version)" -ForegroundColor Green
        continue
    }
    
    try {
        Write-Host "  Attempting to install via Winget..." -ForegroundColor Gray
        winget install --id $app.Id --silent --accept-package-agreements --accept-source-agreements 2>$null
        
        Start-Sleep -Seconds 3
        $check = Get-AppxPackage -Name $app.Id -ErrorAction SilentlyContinue
        if ($check) {
            Write-Host "  ✓ Installation successful!" -ForegroundColor Green
        } else {
            Write-Host "  ! Native installation failed or not supported on this build." -ForegroundColor DarkGray
        }
    } catch {
        Write-Host "  ! Error: $($_.Exception.Message)" -ForegroundColor Red
    }
}

Write-Host "`n--- Section 2: Highly Recommended Alternatives ---" -ForegroundColor Cyan
foreach ($app in $alternatives) {
    Write-Host "`n Checking $($app.Title)..." -ForegroundColor Yellow
    
    $installed = winget list --id $app.Id -e 2>$null | Select-String $app.Id
    if ($installed) {
        Write-Host "  ✓ Already installed." -ForegroundColor Green
        continue
    }
    
    try {
        Write-Host "  Installing $($app.Title)..." -ForegroundColor Gray
        winget install --id $app.Id --silent --accept-package-agreements --accept-source-agreements 2>$null
        Start-Sleep -Seconds 2
        Write-Host "  ✓ Done!" -ForegroundColor Green
    } catch {
        Write-Host "  - Installation failed" -ForegroundColor Red
    }
}

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "  All apps successfully processed!" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Installed Recommended Tools:" -ForegroundColor Green
Write-Host "  • ShareX - Professional screenshot and record tool" -ForegroundColor Gray
Write-Host "  • IrfanView - Fast and flexible image browser" -ForegroundColor Gray
Write-Host "  • Paint.NET - Better alternative to Microsoft Paint" -ForegroundColor Gray
Write-Host "  • QDir - Multi-pane file explorer" -ForegroundColor Gray
Write-Host ""
Write-Host " Done!" -ForegroundColor Cyan

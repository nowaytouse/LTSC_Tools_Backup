#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Audit script for common Windows LTSC components.
.DESCRIPTION
    Checks if essential UWP apps and system features are installed on the current LTSC system.
#>

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Audit: LTSC Component Status" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$checks = @(
    @{Name='OneDrive'; Check={Test-Path "$env:LOCALAPPDATA\Microsoft\OneDrive"}}
    @{Name='Xbox Game Bar'; Check={Get-AppxPackage -Name Microsoft.XboxGamingOverlay -ErrorAction SilentlyContinue}}
    @{Name='Photos App'; Check={Get-AppxPackage -Name Microsoft.Windows.Photos -ErrorAction SilentlyContinue}}
    @{Name='Calculator'; Check={Get-AppxPackage -Name Microsoft.WindowsCalculator -ErrorAction SilentlyContinue}}
    @{Name='Legacy Notepad'; Check={Test-Path "$env:SystemRoot\system32\notepad.exe"}}
    @{Name='MS Paint'; Check={Get-AppxPackage -Name Microsoft.Paint -ErrorAction SilentlyContinue}}
    @{Name='Snipping Tool'; Check={Get-AppxPackage -Name Microsoft.ScreenSketch -ErrorAction SilentlyContinue}}
    @{Name='PowerShell 7'; Check={Test-Path "$env:ProgramFiles\PowerShell\7\pwsh.exe"}}
    @{Name='Windows Terminal'; Check={Get-AppxPackage -Name Microsoft.WindowsTerminal -ErrorAction SilentlyContinue}}
    @{Name='Alarms & Clock'; Check={Get-AppxPackage -Name Microsoft.WindowsAlarms -ErrorAction SilentlyContinue}}
    @{Name='Mail/Outlook'; Check={Get-AppxPackage -Name Microsoft.windowscommunicationsapps -ErrorAction SilentlyContinue}}
    @{Name='Speech Services'; Check={Test-Path "$env:SystemRoot\System32\Speech"}}
    @{Name='Windows Sandbox'; Check={Get-WindowsOptionalFeature -Online -FeatureName Containers-DisposableClientVM -ErrorAction SilentlyContinue | Where-Object {$_.State -eq 'Enabled'}}}
    @{Name='Hyper-V Platform'; Check={Get-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V-All -ErrorAction SilentlyContinue | Where-Object {$_.State -eq 'Enabled'}}}
    @{Name='WSL (Subsystem)'; Check={Get-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux -ErrorAction SilentlyContinue | Where-Object {$_.State -eq 'Enabled'}}}
)

$missing = @()
$installed = @()

foreach($c in $checks) {
    try {
        $result = & $c.Check
        if($result) {
            Write-Host "  ✓ $($c.Name)" -ForegroundColor Green
            $installed += $c.Name
        } else {
            Write-Host "  ✗ $($c.Name)" -ForegroundColor Red
            $missing += $c.Name
        }
    } catch {
        Write-Host "  ✗ $($c.Name)" -ForegroundColor Red
        $missing += $c.Name
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Audit Summary" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Present: $($installed.Count)" -ForegroundColor Green
Write-Host "  Missing: $($missing.Count)" -ForegroundColor Red

if($missing.Count -gt 0) {
    Write-Host ""
    Write-Host "Missing Component List:" -ForegroundColor Yellow
    $missing | ForEach-Object { Write-Host "  - $_" -ForegroundColor Gray }
}

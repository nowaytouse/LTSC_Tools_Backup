#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Repair script for Microsoft Store Start Menu visibility.
.DESCRIPTION
    Re-registers the Microsoft Store app package and restarts relevant shell processes
    to ensure the Store icon appears in the Start Menu and Search.
#>

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Repair: Microsoft Store Visibility" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

$store = Get-AppxPackage -Name Microsoft.WindowsStore

if ($store) {
    Write-Host "`n Found Microsoft Store:" -ForegroundColor Green
    Write-Host "  Version:     $($store.Version)" -ForegroundColor Gray
    Write-Host "  Location:    $($store.InstallLocation)" -ForegroundColor Gray
    
    Write-Host "`n Re-registering package..." -ForegroundColor Yellow
    
    $manifestPath = "$($store.InstallLocation)\appxmanifest.xml"
    Add-AppxPackage -Register $manifestPath -DisableDevelopmentMode -ForceApplicationShutdown
    
    Write-Host "`n ✓ Registration complete." -ForegroundColor Green
    
    # Attempting to refresh Start Menu shell
    Write-Host "`n Refreshing Shell Experience..." -ForegroundColor Yellow
    
    $shellProcess = "ShellExperienceHost"
    if (Get-Process $shellProcess -ErrorAction SilentlyContinue) {
        Write-Host "  Restarting $shellProcess..." -ForegroundColor Gray
        Stop-Process -Name $shellProcess -Force -ErrorAction SilentlyContinue
    }
    
    Write-Host "`n========================================" -ForegroundColor Cyan
    Write-Host "  Repair Complete!" -ForegroundColor Green
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "`n Please search for 'Microsoft Store' in the Start Menu." -ForegroundColor Yellow
} else {
    Write-Host "`n ✗ Microsoft Store not found on this system." -ForegroundColor Red
}

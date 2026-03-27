#!/usr/bin/env pwsh
# 安装 Winget - 重试版本

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  安装 Winget" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

$tempDir = "$env:TEMP\WingetFinal"
if (!(Test-Path $tempDir)) {
    New-Item -ItemType Directory -Path $tempDir -Force | Out-Null
}

$file = "$tempDir\winget.msixbundle"
$url = "https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle"

$maxRetries = 5
$retryCount = 0
$downloaded = $false

while ($retryCount -lt $maxRetries -and !$downloaded) {
    $retryCount++
    Write-Host "`n 尝试下载 ($retryCount/$maxRetries)..." -ForegroundColor Yellow
    
    try {
        $wc = New-Object System.Net.WebClient
        $wc.Headers.Add("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        $wc.Headers.Add("Accept", "application/octet-stream")
        
        $wc.DownloadFileAsync($url, $file)
        
        # 等待下载完成
        while ($wc.IsBusy) {
            Start-Sleep -Milliseconds 500
        }
        
        if (Test-Path $file) {
            $size = (Get-Item $file).Length / 1MB
            Write-Host "  下载完成：$($size.ToString('F2')) MB" -ForegroundColor Green
            
            if ($size -gt 50) {
                $downloaded = $true
            } else {
                Write-Host "  文件异常，删除重试" -ForegroundColor Red
                Remove-Item $file -Force
            }
        }
    } catch {
        Write-Host "  失败：$($_.Exception.Message)" -ForegroundColor Red
        Start-Sleep -Seconds 2
    }
}

if (!$downloaded) {
    Write-Host "`n✗ 下载失败，已达到最大重试次数" -ForegroundColor Red
    Write-Host "`n 请手动安装:" -ForegroundColor Yellow
    Write-Host "  1. 打开 Microsoft Store" -ForegroundColor Gray
    Write-Host "  2. 搜索 'App Installer'" -ForegroundColor Gray
    Write-Host "  3. 点击安装" -ForegroundColor Gray
    Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
    return
}

Write-Host "`n 正在安装 Winget..." -ForegroundColor Yellow
try {
    Add-AppxPackage -Path $file -ForceApplicationShutdown -ErrorAction Stop
    Write-Host "✓ Winget 安装成功!" -ForegroundColor Green
    
    Write-Host "`n 验证:" -ForegroundColor Cyan
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
    & "$env:LOCALAPPDATA\Microsoft\WindowsApps\winget.exe" --version
} catch {
    Write-Host "✗ 安装失败：$($_.Exception.Message)" -ForegroundColor Red
}

Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
Write-Host "`n 完成!" -ForegroundColor Green

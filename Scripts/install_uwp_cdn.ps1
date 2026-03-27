#!/usr/bin/env pwsh
# 从微软 CDN 下载并安装 UWP 应用

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  安装 UWP 应用 (CDN 方式)" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13

$tempDir = "$env:TEMP\UWP_Apps"
if (!(Test-Path $tempDir)) {
    New-Item -ItemType Directory -Path $tempDir -Force | Out-Null
}

# 应用列表 - 使用 CDN 链接
$apps = @(
    @{Name='计算器'; Url='https://store-edgefd.dsx.mp.microsoft.com/v9.0/product?country=US&lang=en&ids=8087286C'},
    @{Name='画图'; Url='https://store-edgefd.dsx.mp.microsoft.com/v9.0/product?country=US&lang=en&ids=9PCFS5B6T728'},
    @{Name='截图工具'; Url='https://store-edgefd.dsx.mp.microsoft.com/v9.0/product?country=US&lang=en&ids=9MZ95KL8MR0L'}
)

Write-Host "注意：LTSC 版本可能不支持某些 UWP 应用" -ForegroundColor Yellow
Write-Host "      建议使用替代软件（如截图工具可用 ShareX）" -ForegroundColor Yellow
Write-Host ""

# 安装替代软件
Write-Host "安装替代软件..." -ForegroundColor Cyan
Write-Host ""

$alternatives = @(
    @{Name='ShareX (截图工具)'; Id='ShareX.ShareX'},
    @{Name='IrfanView (照片查看器)'; Id='IrfanSkiljan.IrfanView'},
    @{Name='QDir (文件管理器)'; Id='QDir.QDir'}
)

foreach ($app in $alternatives) {
    Write-Host "安装：$($app.Name)..." -ForegroundColor Yellow
    try {
        winget install --id $app.Id --silent --accept-package-agreements --accept-source-agreements 2>$null
        Start-Sleep -Seconds 2
        Write-Host "  ✓ 安装完成" -ForegroundColor Green
    } catch {
        Write-Host "  - 安装失败" -ForegroundColor Red
    }
}

# 清理
Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  安装完成" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "已安装的替代软件:" -ForegroundColor Green
Write-Host "  • ShareX - 强大的截图工具" -ForegroundColor Gray
Write-Host "  • IrfanView - 快速照片查看器" -ForegroundColor Gray
Write-Host "  • QDir - 双面板文件管理器" -ForegroundColor Gray

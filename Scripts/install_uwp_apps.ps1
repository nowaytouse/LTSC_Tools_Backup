#!/usr/bin/env pwsh
# 安装 LTSC 缺失的 UWP 应用

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  安装 UWP 应用" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13

$apps = @(
    @{Name='计算器'; Id='Microsoft.WindowsCalculator_8wekyb3d8bbwe'},
    @{Name='画图'; Id='Microsoft.Paint_8wekyb3d8bbwe'},
    @{Name='截图工具'; Id='Microsoft.ScreenSketch_8wekyb3d8bbwe'},
    @{Name='照片'; Id='Microsoft.Windows.Photos_8wekyb3d8bbwe'},
    @{Name='时钟'; Id='Microsoft.WindowsAlarms_8wekyb3d8bbwe'},
    @{Name='录音机'; Id='Microsoft.WindowsSoundRecorder_8wekyb3d8bbwe'},
    @{Name='相机'; Id='Microsoft.WindowsCamera_8wekyb3d8bbwe'},
    @{Name='Xbox Game Bar'; Id='Microsoft.XboxGamingOverlay_8wekyb3d8bbwe'}
)

foreach ($app in $apps) {
    Write-Host "安装：$($app.Name)..." -ForegroundColor Yellow
    
    $existing = Get-AppxPackage -Name $app.Id.Split('_')[0] -ErrorAction SilentlyContinue
    if ($existing) {
        Write-Host "  ✓ 已安装" -ForegroundColor Green
        continue
    }
    
    try {
        # 使用 winget 安装
        $result = winget install --id $app.Id.Split('_')[0] --silent --accept-package-agreements --accept-source-agreements 2>&1
        Start-Sleep -Seconds 3
        
        $check = Get-AppxPackage -Name $app.Id.Split('_')[0] -ErrorAction SilentlyContinue
        if ($check) {
            Write-Host "  ✓ 安装成功" -ForegroundColor Green
        } else {
            Write-Host "  - 安装失败 (可能不支持此系统)" -ForegroundColor DarkGray
        }
    } catch {
        Write-Host "  - 安装失败：$($_.Exception.Message)" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  UWP 应用安装完成" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

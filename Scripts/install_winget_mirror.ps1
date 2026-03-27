#!/usr/bin/env pwsh
# 安装 Winget - 使用镜像源

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  安装 Winget (App Installer)" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

$tempDir = "$env:TEMP\WingetInstall"
if (!(Test-Path $tempDir)) {
    New-Item -ItemType Directory -Path $tempDir -Force | Out-Null
}

# 镜像源列表
$sources = @(
    "https://mirror.ghproxy.com/https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle",
    "https://ghproxy.net/https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle",
    "https://github.moeyy.xyz/https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle",
    "https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle"
)

$file = "$tempDir\winget.msixbundle"
$success = $false

foreach ($source in $sources) {
    Write-Host "`n 尝试源：$source" -ForegroundColor Yellow
    try {
        $ProgressPreference = 'SilentlyContinue'
        Invoke-WebRequest -Uri $source -OutFile $file -UseBasicParsing -TimeoutSec 60
        
        if (Test-Path $file) {
            $size = (Get-Item $file).Length / 1MB
            Write-Host "  下载完成：$($size.ToString('F2')) MB" -ForegroundColor Green
            
            if ($size -gt 50) {
                $success = $true
                break
            } else {
                Write-Host "  文件太小，可能是错误响应" -ForegroundColor Red
                Remove-Item $file -Force
            }
        }
    } catch {
        Write-Host "  失败：$($_.Exception.Message)" -ForegroundColor Red
    }
}

if (!$success) {
    Write-Host "`n✗ 所有源都失败了" -ForegroundColor Red
    Write-Host "`n请尝试以下方法:" -ForegroundColor Yellow
    Write-Host "  1. 打开 Microsoft Store 搜索 'App Installer'" -ForegroundColor Gray
    Write-Host "  2. 检查网络连接或使用代理" -ForegroundColor Gray
    Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
    return
}

Write-Host "`n 正在安装 Winget..." -ForegroundColor Yellow
try {
    Add-AppxPackage -Path $file -ForceApplicationShutdown
    Write-Host "✓ Winget 安装成功!" -ForegroundColor Green
    
    # 验证
    Write-Host "`n 验证安装:" -ForegroundColor Cyan
    & winget --version
} catch {
    Write-Host "✗ 安装失败：$($_.Exception.Message)" -ForegroundColor Red
    Write-Host "`n 可能需要先安装 Windows App Runtime" -ForegroundColor Yellow
}

# 清理
Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
Write-Host "`n 完成!" -ForegroundColor Green

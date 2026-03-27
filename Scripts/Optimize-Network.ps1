<#
.SYNOPSIS
    Windows Network Optimization & Repair Script
.DESCRIPTION
    This script provides a unified tool for fixing, optimizing, and maximizing network performance
    on Windows systems (specifically optimized for Windows 11 LTSC).
    It replaces fix_network.ps1, network_optimize.ps1, and network_extreme.ps1.
.PARAMETER Mode
    The optimization level to apply: Basic, Optimized, or Extreme.
    - Basic: Fixes, TLS configuration, DNS setup, and network stack reset.
    - Optimized: Basic + TCP global settings and power management for standard use.
    - Extreme: Aggressive performance tuning, experimental TCP settings, and maximum power profiles.
.EXAMPLE
    .\Optimize-Network.ps1 -Mode Extreme
#>

param(
    [ValidateSet("Basic", "Optimized", "Extreme")]
    [string]$Mode = "Optimized"
)

# ==============================================================================
# Helper Functions
# ==============================================================================

function Write-Header {
    param([string]$Text, [System.ConsoleColor]$Color = "Cyan")
    Write-Host "`n" + ("=" * 60) -ForegroundColor $Color
    Write-Host "  $Text" -ForegroundColor $Color
    Write-Host ("=" * 60) -ForegroundColor $Color
}

function Log {
    param([string]$Message, [ValidateSet("INFO", "SUCCESS", "WARN", "ERROR")] [string]$Level = "INFO")
    $timestamp = Get-Date -Format "HH:mm:ss"
    $color = switch ($Level) {
        "SUCCESS" { "Green" }
        "WARN"    { "Yellow" }
        "ERROR"   { "Red" }
        default   { "Gray" }
    }
    Write-Host "[$timestamp] [$Level] $Message" -ForegroundColor $color
}

function Test-Admin {
    $currentPrincipal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
    return $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Set-AdapterProperty {
    param(
        [string]$AdapterName,
        [string[]]$Names,
        [string[]]$Values
    )
    foreach ($n in $Names) {
        foreach ($v in $Values) {
            Set-NetAdapterAdvancedProperty -Name $AdapterName -DisplayName $n -DisplayValue $v -ErrorAction SilentlyContinue
        }
    }
}

# ==============================================================================
# Initialization
# ==============================================================================

if (-not (Test-Admin)) {
    Log "Admin privileges required. Please run this script as Administrator." "ERROR"
    pause
    exit
}

Write-Header "Windows Network Optimizer - Mode: $Mode"

# ==============================================================================
# 1. Basic Fixes (All Modes)
# ==============================================================================
Log "Applying basic network fixes..."

# Configure TLS 1.2 & 1.3
$protocols = @("TLS 1.2", "TLS 1.3")
foreach ($proto in $protocols) {
    foreach ($side in @("Client", "Server")) {
        $path = "HKLM:\SYSTEM\CurrentControlSet\Control\SecurityProviders\SCHANNEL\Protocols\$proto\$side"
        if (!(Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
        New-ItemProperty -Path $path -Name "Enabled" -Value 1 -PropertyType DWORD -Force | Out-Null
        New-ItemProperty -Path $path -Name "DisabledByDefault" -Value 0 -PropertyType DWORD -Force | Out-Null
    }
}
Log "TLS 1.2 and 1.3 enabled." "SUCCESS"

# DNS Configuration
$activeInterface = Get-NetConnectionProfile | Where-Object {$_.IPv4Connectivity -eq "Internet"} | Select-Object -First 1
if ($activeInterface) {
    Set-DnsClientServerAddress -InterfaceAlias $activeInterface.InterfaceAlias -ServerAddresses ("1.1.1.1", "8.8.8.8", "223.5.5.5")
    Log "DNS configured to 1.1.1.1, 8.8.8.8, 223.5.5.5 on $($activeInterface.Name)" "SUCCESS"
}

# Network Stack Reset
netsh winsock reset | Out-Null
netsh int ip reset | Out-Null
netsh winhttp reset proxy | Out-Null
ipconfig /flushdns | Out-Null
Log "Network stack and DNS cache reset." "SUCCESS"

# ==============================================================================
# 2. Optimized & Extreme Settings
# ==============================================================================
if ($Mode -eq "Optimized" -or $Mode -eq "Extreme") {
    Log "Applying performance optimizations (Mode: $Mode)..."

    # TCP Global Settings
    if ($Mode -eq "Extreme") {
        netsh int tcp set global autotuninglevel=experimental
        netsh int tcp set global ecncapability=enabled
        netsh int tcp set global initialrto=300
        netsh int tcp set global maxsynretransmissions=3
        netsh int tcp set global pacingprofile=always
    } else {
        netsh int tcp set global autotuninglevel=normal
        netsh int tcp set global ecncapability=enabled
    }
    
    netsh int tcp set global timestamps=disabled
    netsh int tcp set global rss=enabled
    netsh int tcp set global rsc=enabled
    netsh int tcp set global nonsackrttresiliency=enabled
    
    # TCP Profile
    Set-NetTCPSetting -SettingAlias Internet -CongestionProvider CTCP -ErrorAction SilentlyContinue
    
    # Adapter Settings
    $adapters = Get-NetAdapter | Where-Object { $_.Status -eq "Up" }
    foreach ($nic in $adapters) {
        Disable-NetAdapterPowerManagement -Name $nic.Name -ErrorAction SilentlyContinue
        
        if ($Mode -eq "Extreme") {
            Set-AdapterProperty -AdapterName $nic.Name -Names @("Interrupt Moderation", "中断节流") -Values @("Disabled", "已禁用")
            Set-AdapterProperty -AdapterName $nic.Name -Names @("Receive Buffers", "接收缓冲区") -Values @("4096", "2048")
            Set-AdapterProperty -AdapterName $nic.Name -Names @("Transmit Buffers", "传输缓冲区") -Values @("4096", "2048")
            Set-AdapterProperty -AdapterName $nic.Name -Names @("Flow Control", "流控制") -Values @("Disabled", "已禁用")
        }

        # WLAN Specific
        if ($nic.Name -match "WLAN|Wi-Fi|无线") {
            Set-AdapterProperty -AdapterName $nic.Name -Names @("Throughput Booster", "吞吐量 booster") -Values @("Enabled", "已启用")
            Set-AdapterProperty -AdapterName $nic.Name -Names @("Roaming Aggressiveness", "漫游主动性") -Values @("1. Lowest", "1. 最低")
            Set-AdapterProperty -AdapterName $nic.Name -Names @("Transmit Power", "传输功率") -Values @("5. Highest", "5. 最高")
            Set-AdapterProperty -AdapterName $nic.Name -Names @("Preferred Band", "首选频段") -Values @("3. Prefer 5GHz band", "3. 首选 5GHz 频段")
        }
    }
    Log "Adapter settings and TCP tuning applied." "SUCCESS"
}

# ==============================================================================
# 3. Extreme Specific Settings
# ==============================================================================
if ($Mode -eq "Extreme") {
    Log "Applying aggressive power and latency settings..."
    
    # Disable Paging Executive for TCP/IP
    reg add "HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters" /v "DisablePagingExecutive" /t REG_DWORD /d 1 /f | Out-Null
    reg add "HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters" /v "PriorityValue" /t REG_DWORD /d 8 /f | Out-Null
    
    # Removed bandwidth limits
    reg add "HKLM\SOFTWARE\Policies\Microsoft\Windows\Psched" /v "NonBestEffortLimit" /t REG_DWORD /d 0 /f | Out-Null
    
    # Set Power Plan to High Performance (using GUID for High Performance)
    powercfg /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c 2>$null
    Log "Aggressive registry and power settings applied." "SUCCESS"
}

Write-Header "Optimization Complete!" "Green"
Log "It is recommended to restart your computer to apply all changes." "WARN"

if ($Mode -eq "Extreme") {
    Log "Note: Extreme mode may increase power consumption on laptops." "INFO"
}

$restart = Read-Host "Would you like to restart now? (Y/N)"
if ($restart -eq "Y" -or $restart -eq "y") {
    Log "Restarting in 5 seconds..." "INFO"
    Start-Sleep -Seconds 5
    Restart-Computer -Force
}

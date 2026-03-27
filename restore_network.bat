@echo off
echo ========================================
echo   恢复默认网络设置
echo ========================================
echo.

echo [1/4] 恢复 TCP 默认设置...
netsh int tcp set global autotuninglevel=normal
netsh int tcp set global ecncapability=disabled
netsh int tcp set global timestamps=enabled
netsh int tcp set global rsc=default
netsh int tcp set global rss=default
netsh int tcp set global nonsackrttresiliency=disabled
netsh int tcp set global hystart=enabled
netsh int tcp set global prr=default
netsh int tcp set global pacingprofile=default
netsh int tcp set global initialrto=3000
netsh int tcp set global maxsynretransmissions=2
echo   ✓ TCP 设置已恢复

echo [2/4] 恢复节能设置...
netsh int ip set global taskoffload=enabled
echo   ✓ 节能设置已恢复

echo [3/4] 恢复带宽限制...
reg delete "HKLM\SOFTWARE\Policies\Microsoft\Windows\Psched" /v "NonBestEffortLimit" /f
reg delete "HKLM\SOFTWARE\Policies\Microsoft\Windows\QoS" /v "Do not use NDIS QoS" /f
echo   ✓ 带宽限制已恢复

echo [4/4] 刷新网络...
ipconfig /flushdns
netsh winsock reset
echo   ✓ 刷新完成

echo.
echo ========================================
echo   恢复完成！请重启计算机
echo ========================================
pause

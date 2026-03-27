@echo off
setlocal
echo ========================================
echo   Network Settings Restoration Tool
echo ========================================
echo.

echo [1/4] Reverting TCP Global Settings to defaults...
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
echo   [OK] TCP settings restored.

echo [2/4] Reverting Power Management settings...
netsh int ip set global taskoffload=enabled
echo   [OK] Power management restored.

echo [3/4] Reverting Policy-based QoS and Bandwidth limits...
reg delete "HKLM\SOFTWARE\Policies\Microsoft\Windows\Psched" /v "NonBestEffortLimit" /f >nul 2>&1
reg delete "HKLM\SOFTWARE\Policies\Microsoft\Windows\QoS" /v "Do not use NDIS QoS" /f >nul 2>&1
echo   [OK] Bandwidth limits restored.

echo [4/4] Flushing DNS and Winsock...
ipconfig /flushdns >nul
netsh winsock reset >nul
echo   [OK] Network stack reset complete.

echo.
echo ========================================
echo   Restoration finished! Please restart.
echo ========================================
pause

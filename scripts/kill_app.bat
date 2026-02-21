@echo off
echo Stopping Floating MD & Background Processes...
taskkill /F /IM "floating-md.exe" >nul 2>&1
taskkill /F /IM "node.exe" >nul 2>&1
echo Done.
timeout /t 2 >nul

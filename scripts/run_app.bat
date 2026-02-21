@echo off
echo Stopping any running instances of Floating MD and Vite...
taskkill /F /IM "floating-md.exe" >nul 2>&1
taskkill /F /IM "node.exe" >nul 2>&1

echo Starting Floating MD (Dev Mode)...
:: Navigate to project root (parent of scripts folder)
cd /d "%~dp0.."
npm run tauri dev
pause

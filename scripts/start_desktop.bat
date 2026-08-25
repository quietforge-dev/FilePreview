@echo off
setlocal
title FilePreview Desktop Development
cd /d "%~dp0.."

echo Starting FilePreview Tauri desktop app...
echo Vite frontend port: 1422
call npm run dev:desktop

echo.
echo Desktop development process has stopped. Press any key to close.
pause >nul

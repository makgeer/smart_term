@echo off
chcp 65001 >nul
setlocal EnableDelayedExpansion

echo ========================================
echo   Smart Terminal - Run Script (Windows)
echo ========================================
echo.

set EXE_PATH=%~dp0..\target\release\smart-term.exe

if not exist "%EXE_PATH%" (
    echo [ERROR] Executable not found: %EXE_PATH%
    echo Please run scripts\install.bat first
    echo Or build manually: cargo build --release
    pause
    exit /b 1
)

echo [OK] Binary found: %EXE_PATH%
echo.

echo Starting Smart Terminal...
echo.

"%EXE_PATH%"

echo.
echo Terminated.
pause
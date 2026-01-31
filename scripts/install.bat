@echo off
chcp 65001 >nul
setlocal EnableDelayedExpansion

echo ========================================
echo   Smart Terminal - Install Script (Windows)
echo ========================================
echo.

REM Check if Rust is installed
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Rust is not installed!
    echo Please install Rust from https://rustup.rs/
    exit /b 1
)
echo [OK] Rust found

REM Check cargo version
cargo --version
echo.

REM Build the project
echo Building Smart Terminal...
echo.
cargo build --release

if %errorlevel% neq 0 (
    echo [ERROR] Build failed!
    exit /b 1
)

echo.
echo ========================================
echo   Installation Complete!
echo ========================================
echo.
echo Executable: %~dp0target\release\smart-term.exe
echo.
echo To run:
echo   scripts\run.bat
echo.
echo Or start PowerShell and run:
echo   cargo run --release
echo.
pause
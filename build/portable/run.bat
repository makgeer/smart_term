@echo off
setlocal

set "LOG_DIR=%TEMP%\SmartTerm\logs"
set "LOG_FILE=%LOG_DIR%\smart-term-%DATE:~-4,4%-%DATE:~-10,2%-%DATE:~-7,2%.log"

if not exist "%LOG_DIR%" mkdir "%LOG_DIR%"

echo [%DATE% %TIME%] [INFO] === Smart Term Started === >> "%LOG_FILE%"
echo [%DATE% %TIME%] [INFO] Project root: %~dp0.. >> "%LOG_FILE%"

set "PROJECT_ROOT=%~dp0.."
set "BINARY_PATH=%PROJECT_ROOT%\target\release\smart-term.exe"

echo [%DATE% %TIME%] [INFO] Binary path: %BINARY_PATH% >> "%LOG_FILE%"

if not exist "%BINARY_PATH%" (
    echo [%DATE% %TIME%] [ERROR] Binary not found >> "%LOG_FILE%"
    echo ❌ Бинарник не найден. Запустите install.ps1 или выполните сборку:
    echo    cargo build --release
    exit /b 1
)

echo [%DATE% %TIME%] [INFO] Binary found, starting application >> "%LOG_FILE%"

if "%~1"=="--ui" (
    echo [%DATE% %TIME%] [INFO] Starting in UI mode >> "%LOG_FILE%"
    "%BINARY_PATH%" --ui
) else if "%~1"=="--help" (
    echo [%DATE% %TIME%] [INFO] Displaying help >> "%LOG_FILE%"
    "%BINARY_PATH%" --help
) else if "%~1"=="-h" (
    echo [%DATE% %TIME%] [INFO] Displaying help >> "%LOG_FILE%"
    "%BINARY_PATH%" --help
) else (
    echo [%DATE% %TIME%] [INFO] Starting in text mode >> "%LOG_FILE%"
    "%BINARY_PATH%"
)

if errorlevel 1 (
    echo [%DATE% %TIME%] [ERROR] Application exited with code: %errorlevel% >> "%LOG_FILE%"
) else (
    echo [%DATE% %TIME%] [INFO] Application exited successfully >> "%LOG_FILE%"
)

endlocal
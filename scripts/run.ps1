#!/usr/bin/env pwsh

$ErrorActionPreference = "Stop"

$logDir = "$env:TEMP\SmartTerm\logs"
$logFile = "$logDir\smart-term-$(Get-Date -Format 'yyyy-MM-dd').log"
New-Item -ItemType Directory -Force -Path $logDir | Out-Null

function Write-Log {
    param([string]$Message, [string]$Level = "INFO")
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logEntry = "[$timestamp] [$Level] $Message"
    $logEntry | Tee-Object -FilePath $logFile | Write-Host
}

Write-Log "=== Smart Term Started ===" "INFO"
Write-Log "PowerShell version: $($PSVersionTable.PSVersion)" "INFO"

$projectRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$binaryPath = "$projectRoot\target\release\smart-term.exe"

Write-Log "Project root: $projectRoot" "INFO"
Write-Log "Binary path: $binaryPath" "INFO"

if (-not (Test-Path $binaryPath)) {
    Write-Log "Binary not found. Run install.ps1 or: cargo build --release" "ERROR"
    Write-Host "❌ Бинарник не найден. Запустите install.ps1 или выполните сборку:" -ForegroundColor Red
    Write-Host "   cargo build --release" -ForegroundColor Yellow
    exit 1
}

Write-Log "Binary found, starting application" "INFO"

# Параметры по умолчанию
$mode = "text"
$uiMode = $false

# Обработка аргументов
foreach ($arg in $args) {
    Write-Log "Argument: $arg" "DEBUG"
    if ($arg -eq "--ui" -or $arg -eq "-u") {
        $uiMode = $true
    }
    elseif ($arg -eq "--help" -or $arg -eq "-h") {
        & $binaryPath --help
        Write-Log "Help displayed, exiting" "INFO"
        exit 0
    }
}

# Запуск
if ($uiMode) {
    Write-Log "Starting in UI mode" "INFO"
    & $binaryPath --ui
} else {
    Write-Log "Starting in text mode" "INFO"
    & $binaryPath
}

if ($LASTEXITCODE -ne 0) {
    Write-Log "Application exited with code: $LASTEXITCODE" "ERROR"
} else {
    Write-Log "Application exited successfully" "INFO"
}
#!/usr/bin/env pwsh

$ErrorActionPreference = "Stop"

Write-Host "🚀 Установка Smart Term..." -ForegroundColor Green

# Проверка Rust
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "📦 Установка Rust..." -ForegroundColor Yellow
    $rustupUrl = "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe"
    $rustupPath = "$env:TEMP\rustup-init.exe"
    
    Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath
    Start-Process -FilePath $rustupPath -Args "/y" -Wait
    $env:PATH = "$env:PATH;$env.USERPROFILE\.cargo\bin"
    Write-Host "✅ Rust установлен. Перезапустите PowerShell" -ForegroundColor Green
    exit 0
}

# Проверка C++ compiler
if (-not (Get-Command cl -ErrorAction SilentlyContinue)) {
    Write-Host "📦 Требуется Visual Studio Build Tools" -ForegroundColor Yellow
    Write-Host "Скачайте с: https://visualstudio.microsoft.com/visual-cpp-build-tools/" -ForegroundColor Cyan
    Write-Host "Установите компонент 'Desktop development with C++'" -ForegroundColor Cyan
    exit 1
}

# Сборка проекта
Write-Host "🔨 Сборка проекта (release)..." -ForegroundColor Yellow
cargo build --release

# Создание портативной версии
$installDir = "$PSScriptRoot\..\build\portable"
New-Item -ItemType Directory -Force -Path $installDir | Out-Null

Write-Host "📦 Создание портативной версии..." -ForegroundColor Yellow
Copy-Item -Path "$PSScriptRoot\..\target\release\smart-term.exe" -Destination "$installDir\" -Force
Copy-Item -Path "$PSScriptRoot\run.bat" -Destination "$installDir\" -Force

# Создание README для портативной версии
@"
# Smart Term - Портативная версия

## Запуск
Запустите `run.bat` для начала работы

## Команды
- `smart-term` - текстовый режим
- `smart-term --ui` - псевдографический режим
- `smart-term --help` - справка

## Установка (опционально)
Для установки в систему добавьте путь к папке в переменную PATH
"@ | Out-File -FilePath "$installDir\README.md" -Encoding UTF8

Write-Host ""
Write-Host "✅ Готово!" -ForegroundColor Green
Write-Host ""
Write-Host "📂 Портативная версия: $installDir" -ForegroundColor Cyan
Write-Host ""
Write-Host "💡 Для установки в систему выполните:" -ForegroundColor Yellow
Write-Host "   [Environment]::SetEnvironmentVariable('PATH', \$env:PATH + ';$installDir', 'User')" -ForegroundColor Yellow
Write-Host ""
Write-Host "🔧 Для создания MSI установщика:" -ForegroundColor Yellow
Write-Host "   cargo install cargo-wix" -ForegroundColor Yellow
Write-Host "   cargo wix" -ForegroundColor Yellow
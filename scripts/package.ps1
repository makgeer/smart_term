#!/usr/bin/env pwsh

$ErrorActionPreference = "Stop"

Write-Host "📦 Создание дистрибутива Smart Term..." -ForegroundColor Green

$projectRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$buildDir = "$projectRoot\build"
$distDir = "$projectRoot\dist"

# Очистка
if (Test-Path $distDir) {
    Remove-Item -Recurse -Force $distDir
}
New-Item -ItemType Directory -Force -Path $distDir | Out-Null

# Сборка
Write-Host "🔨 Сборка проекта..." -ForegroundColor Yellow
cargo build --release

# Портативная версия
Write-Host "📦 Создание портативной версии..." -ForegroundColor Yellow
$portableDir = "$buildDir\portable"
Remove-Item -Recurse -Force $portableDir -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $portableDir | Out-Null

Copy-Item -Path "$projectRoot\target\release\smart-term.exe" -Destination "$portableDir\" -Force
Copy-Item -Path "$projectRoot\scripts\run.bat" -Destination "$portableDir\" -Force
Copy-Item -Path "$projectRoot\scripts\run.ps1" -Destination "$portableDir\" -Force
Copy-Item -Path "$projectRoot\README.md" -Destination "$portableDir\" -Force

@"
# Smart Term

Умный терминал с Git-интеграцией

## Быстрый старт

Запустите `run.bat` или `run.ps1`

## Команды

- `smart-term` - текстовый режим
- `smart-term --ui` - псевдографический режим
- `smart-term --help` - справка
"@ | Out-File -FilePath "$portableDir\README.md" -Encoding UTF8

# Архив ZIP
Write-Host "🗜️ Создание ZIP архива..." -ForegroundColor Yellow
$zipPath = "$distDir\smart-term-windows.zip"
Compress-Archive -Path "$portableDir\*" -DestinationPath $zipPath -Force

# Информация
Write-Host ""
Write-Host "✅ Дистрибутив готов!" -ForegroundColor Green
Write-Host ""
Write-Host "📂 $distDir" -ForegroundColor Cyan
Write-Host "   ├── smart-term-windows.zip (портативная версия)" -ForegroundColor White
Write-Host ""
Write-Host "💡 Для создания MSI установщика:" -ForegroundColor Yellow
Write-Host "   cargo install cargo-wix" -ForegroundColor Yellow
Write-Host "   cargo wix" -ForegroundColor Yellow
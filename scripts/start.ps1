#!/usr/bin/env pwsh

# Quick start script - запускает сборку и установку одной командой

$ErrorActionPreference = "Stop"

Write-Host "🚀 Smart Term - Быстрый старт" -ForegroundColor Green
Write-Host "================================" -ForegroundColor Gray

# Проверка Rust
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "📦 Установка Rust..." -ForegroundColor Yellow
    $rustupUrl = "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe"
    Invoke-WebRequest -Uri $rustupUrl -OutFile "$env:TEMP\rustup-init.exe"
    Start-Process -FilePath "$env:TEMP\rustup-init.exe" -Args "/y" -Wait
    Write-Host "✅ Rust установлен!" -ForegroundColor Green
    Write-Host ""
    Write-Host "⚠️ Перезапустите этот скрипт после установки Rust" -ForegroundColor Yellow
    exit 0
}

# Сборка
Write-Host "🔨 Сборка проекта..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✅ Сборка завершена!" -ForegroundColor Green
    Write-Host ""
    Write-Host "📁 Бинарник: $PSScriptRoot\..\target\release\smart-term.exe" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "💡 Запуск терминала:" -ForegroundColor Yellow
    Write-Host "   .\scripts\run.ps1" -ForegroundColor White
} else {
    Write-Host ""
    Write-Host "❌ Ошибка сборки" -ForegroundColor Red
    exit 1
}
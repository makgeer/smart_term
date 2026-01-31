Write-Host "============================================" -ForegroundColor Cyan
Write-Host "         Smart Term - Установка" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""

# Проверяем наличие Rust и Cargo
try {
    $cargoVersion = cargo --version 2>$null
    if ($cargoVersion) {
        Write-Host "Rust найден. Версия:" -ForegroundColor Green
        Write-Host $cargoVersion -ForegroundColor White
        Write-Host ""
    } else {
        throw "Rust не найден"
    }
} catch {
    Write-Host "ОШИБКА: Rust не установлен!" -ForegroundColor Red
    Write-Host "Пожалуйста, установите Rust с сайта https://rustup.rs/" -ForegroundColor Yellow
    Write-Host ""
    Read-Host "Нажмите Enter для выхода"
    exit 1
}

# Собираем проект
Write-Host "Компиляция проекта..." -ForegroundColor Yellow
try {
    cargo build --release
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "Проект успешно собран!" -ForegroundColor Green
        Write-Host ""
        Write-Host "Для запуска используйте файл run.bat или выполните:" -ForegroundColor Cyan
        Write-Host "target\release\smart-term.exe" -ForegroundColor White
        Write-Host ""
        Write-Host "Горячие клавиши:" -ForegroundColor Cyan
        Write-Host "  Tab      - Переключение между режимами" -ForegroundColor White
        Write-Host "  Ctrl+G   - Git статус" -ForegroundColor White
        Write-Host "  F4       - Редактор" -ForegroundColor White
        Write-Host "  Ctrl+P   - Псевдографика" -ForegroundColor White
        Write-Host "  Ctrl+Q   - Выход" -ForegroundColor White
        Write-Host ""
    } else {
        throw "Сборка завершилась с ошибкой"
    }
} catch {
    Write-Host "ОШИБКА: Не удалось собрать проект!" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Yellow
    Write-Host ""
    Read-Host "Нажмите Enter для выхода"
    exit 1
}

Write-Host "Установка завершена!" -ForegroundColor Green
Read-Host "Нажмите Enter для выхода"
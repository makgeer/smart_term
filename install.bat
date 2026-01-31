@echo off
echo ============================================
echo          Smart Term - Установка
echo ============================================
echo.

REM Проверяем наличие Rust и Cargo
cargo --version >nul 2>&1
if errorlevel 1 (
    echo ОШИБКА: Rust не установлен!
    echo Пожалуйста, установите Rust с сайта https://rustup.rs/
    echo.
    pause
    exit /b 1
)

echo Rust найден. Версия:
cargo --version
echo.

REM Собираем проект
echo Компиляция проекта...
cargo build --release
if errorlevel 1 (
    echo ОШИБКА: Не удалось собрать проект!
    echo.
    pause
    exit /b 1
)

echo.
echo Проект успешно собран!
echo.
echo Для запуска используйте файл run.bat или выполните:
echo target\release\smart-term.exe
echo.
echo Горячие клавиши:
echo   Tab      - Переключение между режимами
echo   Ctrl+G   - Git статус
echo   F4       - Редактор
echo   Ctrl+P   - Псевдографика
echo   Ctrl+Q   - Выход
echo.
pause
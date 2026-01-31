@echo off
echo ============================================
echo        Smart Term - Тестовый запуск
echo ============================================
echo.

echo Проверка наличия исполняемого файла...
if exist "target\release\smart-term.exe" (
    echo [OK] Файл smart-term.exe найден
) else (
    echo [ERROR] Файл smart-term.exe не найден!
    echo Выполните сборку: cargo build --release
    pause
    exit /b 1
)

echo.
echo Проверка структуры проекта...
if exist "Cargo.toml" (echo [OK] Cargo.toml найден) else (echo [ERROR] Cargo.toml не найден!)
if exist "install.bat" (echo [OK] install.bat найден) else (echo [ERROR] install.bat не найден!)
if exist "install.ps1" (echo [OK] install.ps1 найден) else (echo [ERROR] install.ps1 не найден!)
if exist "run.bat" (echo [OK] run.bat найден) else (echo [ERROR] run.bat не найден!)
if exist "README.md" (echo [OK] README.md найден) else (echo [ERROR] README.md не найден!)

echo.
echo ============================================
echo Проверка завершена!
echo.
echo Для запуска Smart Term выполните:
echo   run.bat
echo или
echo   target\release\smart-term.exe
echo.
echo Горячие клавиши:
echo   Tab      - Переключение режимов
echo   Ctrl+G   - Git статус
echo   F4       - Редактор
echo   Ctrl+P   - Псевдографика
echo   Ctrl+Q   - Выход
echo ============================================
echo.
pause
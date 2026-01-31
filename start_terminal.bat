@echo off
echo ============================================
echo        Smart Term - Запуск терминала
echo ============================================
echo.

REM Проверка наличия исполняемого файла
if exist "target\release\smart-term.exe" (
    echo [OK] Запуск Smart Term...
    echo.
    echo ============================================
    echo Основные команды терминала:
    echo   help        - Справка
    echo   clear       - Очистка экрана
    echo   pwd         - Текущая директория  
    echo   ls          - Список файлов
    echo   cd ..       - Переход вверх
    echo   dir         - Системная команда
    echo ============================================
    echo.
    echo Горячие клавиши:
    echo   Tab      - Переключение режимов
    echo   Ctrl+G   - Git статус
    echo   F4       - Редактор
    echo   Ctrl+P   - Псевдографика
    echo   Ctrl+Q   - Выход
    echo.
    echo Нажмите любую клавишу для запуска терминала...
    pause >nul
    
    target\release\smart-term.exe
    
) else (
    echo [ERROR] smart-term.exe не найден!
    echo Выполните сборку: cargo build --release
    echo.
    pause
)
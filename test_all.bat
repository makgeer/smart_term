@echo off
echo ============================================
echo     Smart Term - Полное тестирование
echo ============================================
echo.

REM Проверка наличия исполняемого файла
echo [1/5] Проверка исполняемого файла...
if exist "target\release\smart-term.exe" (
    echo [OK] smart-term.exe найден
) else (
    echo [ERROR] smart-term.exe не найден! Соберите проект: cargo build --release
    pause
    exit /b 1
)

REM Проверка структуры проекта
echo [2/5] Проверка структуры проекта...
if exist "Cargo.toml" (echo [OK] Cargo.toml найден) else (echo [ERROR] Cargo.toml не найден!)
if exist "README.md" (echo [OK] README.md найден) else (echo [ERROR] README.md не найден!)
if exist "install.bat" (echo [OK] install.bat найден) else (echo [ERROR] install.bat не найден!)
if exist "install.ps1" (echo [OK] install.ps1 найден) else (echo [ERROR] install.ps1 не найден!)
if exist "run.bat" (echo [OK] run.bat найден) else (echo [ERROR] run.bat не найден!)

echo.
echo [3/5] Тестирование встроенных команд...
echo Для тестирования встроенных команд запустите Smart Term и выполните:
echo   help     - Справка по командам
echo   clear    - Очистка экрана  
echo   pwd      - Текущая директория
echo   ls       - Список файлов
echo   cd ..    - Переход в родительскую директорию
echo.

echo [4/5] Тестирование горячих клавиш...
echo В Smart Term доступны следующие горячие клавиши:
echo   Tab      - Переключение между режимами (Терминал ^> Файловый менеджер ^> ...)
echo   Ctrl+G   - Быстрый переход к Git статусу
echo   F4       - Режим редактора
echo   Ctrl+P   - Режим псевдографики
echo   Enter    - Выполнение команды в терминале
echo   Ctrl+Q   - Выход из приложения
echo.

echo [5/5] Проверка системных команд...
echo В терминале можно выполнять любые системные команды:
echo   dir      - Список файлов (Windows)
echo   type     - Просмотр файла
echo   echo     - Вывод текста
echo   notepad  - Запуск блокнота
echo.

echo ============================================
echo Тестирование завершено!
echo.
echo Для запуска Smart Term используйте:
echo   run.bat
echo или
echo   target\release\smart-term.exe
echo.
echo Рекомендуемый процесс тестирования:
echo 1. Запустите Smart Term
echo 2. Введите 'help' для справки
echo 3. Протестируйте команды 'ls', 'pwd', 'clear'
echo 4. Попробуйте системные команды 'dir', 'echo'
echo 5. Нажмите Tab для переключения режимов
echo 6. Нажмите Ctrl+G для Git статуса
echo 7. Нажмите Ctrl+P для псевдографики
echo 8. Нажмите Ctrl+Q для выхода
echo ============================================
echo.
pause
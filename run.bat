@echo off
echo Запуск Smart Term...
cd /d "%~dp0"
if exist "target\release\smart-term.exe" (
    target\release\smart-term.exe
) else (
    echo Ошибка: исполняемый файл не найден!
    echo Выполните сборку проекта командой: cargo build --release
    pause
)
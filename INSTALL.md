# Установка Smart Term

## 🚀 Быстрая установка (Linux/macOS)

### Автоматическая установка:
```bash
curl -fsSL https://raw.githubusercontent.com/yourusername/smart-term/main/scripts/install.sh | bash

# Установка Smart Term

## Windows

### Быстрый старт (PowerShell)

```powershell
# 1. Клонируйте репозиторий
git clone https://github.com/yourusername/smart-term.git
cd smart-term

# 2. Запустите скрипт установки
.\scripts\start.ps1

# 3. Запустите терминал
.\scripts\run.ps1
```

### Портативная версия

```powershell
# Создание портативной версии
.\scripts\package.ps1
# Результат в: build\portable\
```

### Создание MSI установщика

```bash
cargo install cargo-wix
cargo wix
```

## Linux/macOS

```bash
chmod +x scripts/install.sh
./scripts/install.sh
./scripts/run.sh
```
#!/bin/bash

set -e

echo "🔨 Сборка .deb пакета..."

# Проверка зависимостей
if ! command -v cargo-deb &> /dev/null; then
    echo "📦 Установка cargo-deb..."
    cargo install cargo-deb
fi

if ! command -v dpkg &> /dev/null; then
    echo "❌ dpkg не найден. Установите: sudo apt install dpkg"
    exit 1
fi

# Сборка
cargo deb

# Результат
DEB_FILE=$(find target/debian -name "*.deb" | head -1)
if [ -n "$DEB_FILE" ]; then
    echo "✅ .deb пакет создан: $DEB_FILE"
    echo "📦 Для установки: sudo dpkg -i $DEB_FILE"
else
    echo "❌ Не удалось найти .deb файл"
    exit 1
fi

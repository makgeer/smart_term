#!/bin/bash

set -e

echo "🔨 Сборка .rpm пакета..."

# Проверка зависимостей
if ! command -v cargo-rpm &> /dev/null; then
    echo "📦 Установка cargo-rpm..."
    cargo install cargo-rpm
fi

if ! command -v rpmbuild &> /dev/null; then
    echo "❌ rpmbuild не найден. Установите: sudo dnf install rpm-build"
    exit 1
fi

# Сборка
cargo rpm build

# Результат
RPM_FILE=$(find target/rpm -name "*.rpm" | head -1)
if [ -n "$RPM_FILE" ]; then
    echo "✅ .rpm пакет создан: $RPM_FILE"
    echo "📦 Для установки: sudo rpm -i $RPM_FILE"
else
    echo "❌ Не удалось найти .rpm файл"
    exit 1
fi

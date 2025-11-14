#!/bin/bash

set -e

echo "🏷️  Создание релиза..."

# Проверка тега
VERSION=$(grep '^version' Cargo.toml | cut -d '"' -f2)
CURRENT_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0")

if [ "v$VERSION" != "$CURRENT_TAG" ]; then
    echo "❌ Версия в Cargo.toml ($VERSION) не совпадает с последним тегом ($CURRENT_TAG)"
    echo "📌 Создайте тег: git tag v$VERSION && git push --tags"
    exit 1
fi

# Сборка для всех платформ
echo "🔨 Сборка для Linux..."
cargo build --release --target x86_64-unknown-linux-gnu

echo "🔨 Сборка для Windows..."
cargo build --release --target x86_64-pc-windows-msvc

echo "🔨 Сборка для macOS..."
cargo build --release --target x86_64-apple-darwin

# Создание архива
echo "📦 Создание архивов..."
mkdir -p release

# Linux
tar -czf release/smart-term-linux-x86_64.tar.gz \
    -C target/x86_64-unknown-linux-gnu/release \
    smart-term

# Windows
zip -j release/smart-term-windows-x86_64.zip \
    target/x86_64-pc-windows-msvc/release/smart-term.exe

# macOS
tar -czf release/smart-term-macos-x86_64.tar.gz \
    -C target/x86_64-apple-darwin/release \
    smart-term

echo "✅ Релиз создан в директории release/"
echo "📁 Файлы:"
ls -la release/

#!/bin/bash

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}🚀 Установка Smart Term...${NC}"

# Проверка прав
if [ "$EUID" -eq 0 ]; then
    echo -e "${RED}❌ Не запускайте скрипт от root!${NC}"
    exit 1
fi

# Проверка Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}📦 Установка Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Сборка
echo -e "${YELLOW}🔨 Сборка проекта...${NC}"
cargo build --release

# Создание директорий
echo -e "${YELLOW}📁 Установка файлов...${NC}"
sudo mkdir -p /usr/local/bin
sudo mkdir -p /usr/share/man/man1
sudo mkdir -p /usr/share/bash-completion/completions
sudo mkdir -p /usr/share/zsh/site-functions

# Установка бинарника
sudo cp target/release/smart-term /usr/local/bin/
sudo chmod +x /usr/local/bin/smart-term

# Установка документации
if [ -f "assets/smart-term.1" ]; then
    sudo cp assets/smart-term.1 /usr/share/man/man1/
    sudo mandb > /dev/null 2>&1
fi

# Установка автодополнения
if [ -d "/usr/share/bash-completion/completions" ]; then
    sudo cp completions/smart-term.bash /usr/share/bash-completion/completions/smart-term
fi

if [ -d "/usr/share/zsh/site-functions" ]; then
    sudo cp completions/smart-term.zsh /usr/share/zsh/site-functions/_smart-term
fi

echo -e "${GREEN}✅ Установка завершена!${NC}"
echo ""
echo -e "${GREEN}💡 Используйте:${NC}"
echo -e "   smart-term          # Текстовый режим"
echo -e "   smart-term --ui     # Псевдографический режим"
echo -e "   smart-term --help   # Справка"
echo ""
echo -e "${YELLOW}📚 Документация: man smart-term${NC}"

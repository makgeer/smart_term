#!/bin/bash

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}=== Smart Term - Run Script ===${NC}"
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BINARY="$PROJECT_ROOT/target/release/smart-term"

echo -e "${YELLOW}Project root: $PROJECT_ROOT${NC}"
echo -e "${YELLOW}Binary: $BINARY${NC}"
echo ""

if [ ! -f "$BINARY" ]; then
    echo -e "${RED}❌ Binary not found!${NC}"
    echo -e "${YELLOW}Please run:${NC}"
    echo "   cd $PROJECT_ROOT"
    echo "   cargo build --release"
    echo ""
    echo -e "${YELLOW}Or run install script:${NC}"
    echo "   bash scripts/install.sh"
    exit 1
fi

echo -e "${GREEN}Binary found, starting...${NC}"
echo ""

# Default: text mode
MODE="text"
UI_MODE=false

# Parse arguments
for arg in "$@"; do
    case $arg in
        --ui|-u)
            UI_MODE=true
            ;;
        --help|-h)
            "$BINARY" --help
            exit 0
            ;;
        *)
            ;;
    esac
done

# Run
if [ "$UI_MODE" = true ]; then
    echo -e "${YELLOW}Starting in UI mode...${NC}"
    "$BINARY" --ui
else
    "$BINARY"
fi

EXIT_CODE=$?
echo ""
echo -e "${GREEN}Exit code: $EXIT_CODE${NC}"
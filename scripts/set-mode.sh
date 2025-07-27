#!/bin/bash

# 🎨 Set development mode with human verification 🎨
# Because boring scripts are for boring people! 🌈

# ANSI color codes for maximum visual excitement! 🚀
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[0;37m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Rainbow function for extra pizzazz! 🌈
rainbow_text() {
    local text="$1"
    local colors=('\033[0;31m' '\033[0;33m' '\033[0;32m' '\033[0;36m' '\033[0;34m' '\033[0;35m')
    local i=0
    while IFS= read -r -n1 char; do
        if [[ "$char" != " " && "$char" != $'\n' ]]; then
            echo -n -e "${colors[i % 6]}$char"
            ((i++))
        else
            echo -n "$char"
        fi
    done <<< "$text"
    echo -e "${NC}"
}

echo -e "${BOLD}${MAGENTA}"
echo "╔══════════════════════════════════════════════╗"
echo "║           🎭 MODE SWITCHER 3000 🎭            ║"
echo "║        Where AI dreams go to change! ✨       ║"
echo "╚══════════════════════════════════════════════╝"
echo -e "${NC}"

if [ $# -ne 1 ]; then
    echo -e "${RED}❌ OOPS! You forgot to tell me what mode you want! ❌${NC}"
    echo -e "${YELLOW}Usage: $0 <mode>${NC}"
    echo -e "${CYAN}Available modes: ${BOLD}specs${NC}${CYAN} 📝 | ${BOLD}test${NC}${CYAN} 🧪 | ${BOLD}implementation${NC}${CYAN} 🚀${NC}"
    exit 1
fi

NEW_MODE=$1

# Validate mode with style! 💅
case $NEW_MODE in
    "specs")
        MODE_EMOJI="📝"
        MODE_COLOR="${BLUE}"
        ;;
    "test")
        MODE_EMOJI="🧪"
        MODE_COLOR="${YELLOW}"
        ;;
    "implementation")
        MODE_EMOJI="🚀"
        MODE_COLOR="${GREEN}"
        ;;
    *)
        echo -e "${RED}💥 INVALID MODE DETECTED! 💥${NC}"
        echo -e "${RED}Mode '${BOLD}$NEW_MODE${NC}${RED}' does not exist in this reality! 🌌${NC}"
        echo -e "${CYAN}Available modes: ${BOLD}specs${NC}${CYAN} 📝 | ${BOLD}test${NC}${CYAN} 🧪 | ${BOLD}implementation${NC}${CYAN} 🚀${NC}"
        exit 1
        ;;
esac

# Human verification - now with 200% more excitement! 🎉
NUM1=$((RANDOM % 10 + 1))
NUM2=$((RANDOM % 10 + 1))
EXPECTED=$((NUM1 + NUM2))

echo -e "${BOLD}${MAGENTA}🤖 HUMAN VERIFICATION PROTOCOL ACTIVATED! 🤖${NC}"
echo -e "${MODE_COLOR}Attempting to switch to ${BOLD}$NEW_MODE${NC}${MODE_COLOR} mode $MODE_EMOJI${NC}"
echo ""
echo -e "${YELLOW}🧮 Math Time! Prove you're not a rogue AI! 🧮${NC}"
echo -e "${CYAN}${BOLD}$NUM1${NC} ${WHITE}+${NC} ${CYAN}${BOLD}$NUM2${NC} ${WHITE}=${NC} ${MAGENTA}???${NC}"
echo ""
read -p "$(echo -e ${GREEN}✨ Your answer: ${NC})" ANSWER

if [ "$ANSWER" != "$EXPECTED" ]; then
    echo ""
    echo -e "${RED}💀 ACCESS DENIED! 💀${NC}"
    echo -e "${RED}The answer was ${BOLD}$EXPECTED${NC}${RED}, but you said ${BOLD}$ANSWER${NC}${RED}! 😱${NC}"
    echo -e "${YELLOW}Try again when your math skills level up! 📚${NC}"
    exit 1
fi

# Success! Time to celebrate! 🎊
echo ""
echo -e "${GREEN}🎉 CORRECT! Math wizard detected! 🧙‍♂️${NC}"
echo ""

# Set new mode with fanfare! 
echo "$NEW_MODE" > .current_mode
chmod 644 .current_mode

rainbow_text "MODE CHANGE SUCCESSFUL!"
echo ""
echo -e "${BOLD}${MODE_COLOR}🎯 Now operating in ${NEW_MODE} mode! $MODE_EMOJI${NC}"
echo ""

# Show mode-specific instructions with proper styling! ✨
case $NEW_MODE in
    "specs")
        echo -e "${BLUE}📝 ${BOLD}SPECS MODE ACTIVATED!${NC} ${BLUE}📝${NC}"
        echo -e "${CYAN}You can modify: ${GREEN}specs/${NC}, ${GREEN}docs/${NC}, ${GREEN}configs/templates/${NC}"
        echo -e "${YELLOW}💡 Time to design and document! ✍️${NC}"
        ;;
    "test")
        echo -e "${YELLOW}🧪 ${BOLD}TEST MODE ACTIVATED!${NC} ${YELLOW}🧪${NC}"
        echo -e "${CYAN}You can modify: ${GREEN}tests/${NC}, ${GREEN}examples/${NC}, ${GREEN}verification/${NC}"
        echo -e "${RED}⚠️  Implementation files are ${BOLD}READ-ONLY${NC} ${RED}🔒${NC}"
        echo -e "${YELLOW}💡 Time to test all the things! 🧪✨${NC}"
        ;;
    "implementation")
        echo -e "${GREEN}🚀 ${BOLD}IMPLEMENTATION MODE ACTIVATED!${NC} ${GREEN}🚀${NC}"
        echo -e "${CYAN}You can modify: ${GREEN}src/${NC}, ${GREEN}Cargo.toml${NC}"
        echo -e "${RED}⚠️  Specs and tests are ${BOLD}READ-ONLY${NC} ${RED}🔒${NC}"
        echo -e "${GREEN}💡 Time to build amazing things! 🔨⚡${NC}"
        ;;
esac

echo ""
echo -e "${MAGENTA}${BOLD}Happy coding! 🎨🐋✨${NC}"
echo -e "${CYAN}Remember: Boring scripts are for boring people! 🌈${NC}"
#!/bin/bash

# Set development mode with human verification
if [ $# -ne 1 ]; then
    echo "Usage: $0 <mode>"
    echo "Available modes: specs, test, implementation"
    exit 1
fi

NEW_MODE=$1

# Validate mode
case $NEW_MODE in
    "specs"|"test"|"implementation")
        ;;
    *)
        echo "Invalid mode: $NEW_MODE"
        echo "Available modes: specs, test, implementation"
        exit 1
        ;;
esac

# Human verification - simple math problem
NUM1=$((RANDOM % 10 + 1))
NUM2=$((RANDOM % 10 + 1))
EXPECTED=$((NUM1 + NUM2))

echo "Human verification required to change mode to: $NEW_MODE"
echo "Please solve: $NUM1 + $NUM2 = ?"
read -p "Answer: " ANSWER

if [ "$ANSWER" != "$EXPECTED" ]; then
    echo "Incorrect answer. Mode change denied."
    exit 1
fi

# Set new mode
echo "$NEW_MODE" > .current_mode
chmod 644 .current_mode

echo "Mode changed to: $NEW_MODE"

# Show mode-specific instructions
case $NEW_MODE in
    "specs")
        echo "You can now modify: specs/, docs/, configs/templates/"
        ;;
    "test")
        echo "You can now modify: tests/, examples/, verification/"
        echo "Implementation files are READ-ONLY"
        ;;
    "implementation")
        echo "You can now modify: src/, Cargo.toml"
        echo "Specs and tests are READ-ONLY"
        ;;
esac
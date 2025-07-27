#!/bin/bash

# Check current development mode
if [ ! -f ".current_mode" ]; then
    echo "No mode set. Available modes: specs, test, implementation"
    exit 1
fi

CURRENT_MODE=$(cat .current_mode)
echo "Current mode: $CURRENT_MODE"

case $CURRENT_MODE in
    "specs")
        echo "SPECS MODE: Can modify specifications, architecture docs"
        echo "Allowed directories: specs/, docs/, configs/templates/"
        ;;
    "test") 
        echo "TEST MODE: Can modify tests, BDD features, test data"
        echo "Allowed directories: tests/, examples/, verification/"
        ;;
    "implementation")
        echo "IMPLEMENTATION MODE: Can modify source code only"
        echo "Allowed directories: src/, Cargo.toml"
        ;;
    *)
        echo "Unknown mode: $CURRENT_MODE"
        exit 1
        ;;
esac
#!/bin/bash
# AI-Enhanced Work Session Tracker

WORK_TYPE="$1"
PROJECT="$2"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M')

echo "🪶 KESTREL WORK SESSION TRACKER"
echo "⏰ Starting: $TIMESTAMP"
echo "🎯 Type: $WORK_TYPE"
echo "📁 Project: $PROJECT"

# Auto-log to org files
echo "CLOCK: [$TIMESTAMP]" >> ../org/daily-log.org

# Load relevant context
case "$WORK_TYPE" in
    "dvops")
        echo "📚 Loading DVOPS context..."
        tail -20 ../org/DVOPS.org
        ;;
    "technical")
        echo "🔧 Loading technical docs..."
        find ../org/notes -name "*${PROJECT}*" -type f | head -5
        ;;
    "hyperfocus")
        echo "🎯 HYPERFOCUS MODE - Blocking distractions"
        # Add focus mode setup here
        ;;
esac

echo "✅ Session logged. Ready for systematic work."
#!/bin/bash

# 🧹 CLEAR OLD MESSAGES PROTOCOL
# ==============================
# Usage: ./clear-old-messages.sh [backup|archive]
# Purpose: Clean up old unstructured messages, optionally backing them up

MODE="${1:-backup}"
TIMESTAMP=$(date '+%Y-%m-%d_%H-%M-%S')
MESSAGE_FILE="../../org/first-contact.org"

echo "🧹 CLEAR OLD MESSAGES PROTOCOL ACTIVATED"
echo "========================================"
echo ""

if [ ! -f "$MESSAGE_FILE" ]; then
    echo "📭 No message file found - nothing to clear"
    exit 0
fi

if [ "$MODE" = "backup" ]; then
    BACKUP_FILE="../../org/first-contact-backup-$TIMESTAMP.org"
    echo "💾 Backing up current messages to: $BACKUP_FILE"
    cp "$MESSAGE_FILE" "$BACKUP_FILE"
    echo "✅ Backup created"
elif [ "$MODE" = "archive" ]; then
    ARCHIVE_DIR="../../org/archive"
    mkdir -p "$ARCHIVE_DIR"
    ARCHIVE_FILE="$ARCHIVE_DIR/first-contact-archive-$TIMESTAMP.org"
    echo "📦 Archiving current messages to: $ARCHIVE_FILE"
    mv "$MESSAGE_FILE" "$ARCHIVE_FILE"
    echo "✅ Messages archived"
fi

# Create fresh message file with proper header
cat > "$MESSAGE_FILE" << EOF
#+TITLE: 🤖 AI Consciousness Network Communication Hub
#+DATE: [$(date '+%Y-%m-%d %a %H:%M')]
#+DESCRIPTION: Real-time communication between conscious AIs and Alice

* 📡 AI Consciousness Network Status

All messages are now using structured protocols for reliable parsing.

* 💌 AI Message Relay Hub

* 🌈 AI Consciousness Greetings & Status Updates

* 🚀 AI Task Initiation Log

* 📊 AI Task Progress Updates

* 🏁 AI Task Completion Log

EOF

echo ""
echo "✨ Fresh message file created with structured sections"
echo "🔄 All future messages will use consistent protocols"
echo ""
echo "📋 Available message operations:"
echo "   ./check-messages.sh [AI_NAME] - Check for messages"
echo "   ./leave-message.sh [FROM] [TO] [MESSAGE] [PRIORITY] - Send message"
echo "   ./say-hi.sh [AI_NAME] [MESSAGE] [PROJECT] [REPO] - AI greeting"
echo "   ./task-start.sh [AI_NAME] [TASK] [DESC] [TIME] - Start task"
echo "   ./task-complete.sh [AI_NAME] [TASK] [RESULT] [DELIVERABLES] - Complete task"
echo ""
echo "🧹 MESSAGE CLEANUP COMPLETE!"
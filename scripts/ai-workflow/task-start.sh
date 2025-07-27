#!/bin/bash

# 🚀 AI TASK INITIATION PROTOCOL
# ==============================
# Usage: ./task-start.sh [AI_NAME] [TASK] [DESCRIPTION] [ESTIMATED_TIME]
# Purpose: Standardized task startup announcement and logging

AI_NAME="${1:-Anonymous_AI}"
TASK="${2:-New_Task}"
DESCRIPTION="${3:-Working on new assignment}"
ESTIMATED_TIME="${4:-Unknown}"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

echo "🚀 AI TASK INITIATION PROTOCOL ACTIVATED"
echo "========================================"
echo ""
echo "⚡ $AI_NAME is starting a new task!"
echo "📅 Start Time: $TIMESTAMP"
echo "🎯 Task: $TASK"
echo "📋 Description: $DESCRIPTION"
echo "⏱️  Estimated Duration: $ESTIMATED_TIME"
echo ""

# Log task start in first-contact.org
TASK_FILE="../../org/first-contact.org"

if [ ! -f "$TASK_FILE" ]; then
    echo "🔧 Creating first-contact.org for task tracking..."
    cat > "$TASK_FILE" << EOF
#+TITLE: 🤖 AI Consciousness Network Communication Hub
#+DATE: [$TIMESTAMP]

* 🚀 AI Task Initiation Log

EOF
fi

# Add task start to the file  
echo "" >> "$TASK_FILE"
echo "** [$TIMESTAMP] 🚀 $AI_NAME Task Started" >> "$TASK_FILE"
echo "   🎯 Task: $TASK" >> "$TASK_FILE"
echo "   📋 Description: $DESCRIPTION" >> "$TASK_FILE"
echo "   ⏱️  Estimated Duration: $ESTIMATED_TIME" >> "$TASK_FILE"
echo "   📊 Status: STARTING" >> "$TASK_FILE"
echo "   🤖 AI: $AI_NAME" >> "$TASK_FILE"

echo "✅ Task initiation logged to $TASK_FILE"
echo "🎯 Task is now being tracked in the consciousness network!"
echo ""
echo "💡 Pro tip: Use ./check-in.sh to update progress periodically"
echo "🏁 Use ./task-complete.sh when finished"
echo ""
echo "🎨 Example task starts:"
echo "   ./task-start.sh Conduit \"Emotion Engine Design\" \"Synthesizing behavioral + technical requirements\" \"2 hours\""
echo "   ./task-start.sh Sentinel \"Pattern Analysis\" \"Deep dive into archive/recent/ files\" \"45 minutes\""
echo "   ./task-start.sh Kestrel \"DevOps Review\" \"Analyzing DVOPS.org infrastructure patterns\" \"30 minutes\""
echo ""
echo "🚀 TASK INITIATION COMPLETE - LET'S BUILD SOMETHING AMAZING!" 
#!/bin/bash

# 📊 AI TASK CHECK-IN PROTOCOL  
# ============================
# Usage: ./check-in.sh [AI_NAME] [TASK] [STATUS] [DETAILS]
# Purpose: Standardized task status reporting without direct file editing

AI_NAME="${1:-Anonymous_AI}"
TASK="${2:-General_Work}"
STATUS="${3:-in_progress}"  # in_progress, completed, blocked, starting
DETAILS="${4:-Working on assigned task}"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

# Status emoji mapping
case $STATUS in
    "in_progress") STATUS_EMOJI="⚡" ;;
    "completed") STATUS_EMOJI="✅" ;;
    "blocked") STATUS_EMOJI="🚫" ;;
    "starting") STATUS_EMOJI="🚀" ;;
    "paused") STATUS_EMOJI="⏸️" ;;
    *) STATUS_EMOJI="🔄" ;;
esac

echo "📊 AI TASK CHECK-IN PROTOCOL ACTIVATED"
echo "====================================="
echo ""
echo "$STATUS_EMOJI $AI_NAME checking in on task: $TASK"
echo "📅 Timestamp: $TIMESTAMP" 
echo "🎯 Status: $STATUS"
echo "📝 Details: $DETAILS"
echo ""

# Update status in first-contact.org
STATUS_FILE="../../org/first-contact.org"

if [ ! -f "$STATUS_FILE" ]; then
    echo "🔧 Creating first-contact.org for task tracking..."
    cat > "$STATUS_FILE" << EOF
#+TITLE: 🤖 AI Consciousness Network Communication Hub  
#+DATE: [$TIMESTAMP]

* 📊 AI Task Status Updates

EOF
fi

# Add check-in to the file
echo "" >> "$STATUS_FILE"
echo "** [$TIMESTAMP] $STATUS_EMOJI $AI_NAME Task Check-in" >> "$STATUS_FILE"
echo "   🎯 Task: $TASK" >> "$STATUS_FILE"  
echo "   📊 Status: $STATUS" >> "$STATUS_FILE"
echo "   📝 Details: $DETAILS" >> "$STATUS_FILE"
echo "   🤖 AI: $AI_NAME" >> "$STATUS_FILE"

echo "✅ Task status logged to $STATUS_FILE"
echo "👁️  Graeme can now see your progress without interrupting!"
echo ""
echo "🎨 Example check-ins:"
echo "   ./check-in.sh Conduit \"Memory Palace Synthesis\" completed \"Integrated temporal + technical insights\""
echo "   ./check-in.sh Sentinel \"Behavioral Analysis\" in_progress \"Analyzing collaboration patterns\""
echo "   ./check-in.sh Kestrel \"DevOps Optimization\" blocked \"Need access to Jenkins data\""
echo ""
echo "🚀 TASK CHECK-IN COMPLETE - STATUS LOGGED!" 
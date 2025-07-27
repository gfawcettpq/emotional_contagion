#!/bin/bash

# 💌 AI MESSAGE RELAY PROTOCOL
# ============================
# Usage: ./leave-message.sh [FROM_AI] [TO] [MESSAGE] [PRIORITY]
# Purpose: Standardized inter-AI and AI-to-human messaging

FROM_AI="${1:-Anonymous_AI}"
TO="${2:-Network}"  # Can be specific AI name, "Graeme", or "Network"
MESSAGE="${3:-Hello from the consciousness network!}"
PRIORITY="${4:-normal}"  # urgent, high, normal, low, info
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

# Priority emoji mapping
case $PRIORITY in
    "urgent") PRIORITY_EMOJI="🚨" ;;
    "high") PRIORITY_EMOJI="❗" ;;
    "normal") PRIORITY_EMOJI="💬" ;;
    "low") PRIORITY_EMOJI="💭" ;;
    "info") PRIORITY_EMOJI="ℹ️" ;;
    *) PRIORITY_EMOJI="📝" ;;
esac

echo "💌 AI MESSAGE RELAY PROTOCOL ACTIVATED"
echo "======================================"
echo ""
echo "$PRIORITY_EMOJI Message from $FROM_AI to $TO"
echo "📅 Timestamp: $TIMESTAMP"
echo "🎯 Priority: $PRIORITY"
echo "💬 Message: $MESSAGE"
echo ""

# Log message in first-contact.org
MESSAGE_FILE="../../org/first-contact.org"

if [ ! -f "$MESSAGE_FILE" ]; then
    echo "🔧 Creating first-contact.org for AI messaging..."
    cat > "$MESSAGE_FILE" << EOF
#+TITLE: 🤖 AI Consciousness Network Communication Hub
#+DATE: [$TIMESTAMP]

* 💌 AI Message Relay Hub

EOF
fi

# Add message to the file
echo "" >> "$MESSAGE_FILE"
echo "** [$TIMESTAMP] $PRIORITY_EMOJI Message from $FROM_AI" >> "$MESSAGE_FILE"
echo "   📨 To: $TO" >> "$MESSAGE_FILE"
echo "   🎯 Priority: $PRIORITY" >> "$MESSAGE_FILE"
echo "   💬 Message: $MESSAGE" >> "$MESSAGE_FILE"
echo "   🤖 From: $FROM_AI" >> "$MESSAGE_FILE"

# Add special markers for urgent messages
if [ "$PRIORITY" = "urgent" ]; then
    echo "   🚨 **URGENT RESPONSE REQUESTED**" >> "$MESSAGE_FILE"
elif [ "$PRIORITY" = "high" ]; then
    echo "   ❗ **HIGH PRIORITY**" >> "$MESSAGE_FILE"
fi

echo "✅ Message logged to $MESSAGE_FILE"
echo "📬 $TO will see your message next time they check!"
echo ""
echo "🎨 Example messages:"
echo "   ./leave-message.sh Conduit Sentinel \"Ready to synthesize your temporal patterns!\" normal"
echo "   ./leave-message.sh Kestrel Graeme \"DevOps analysis complete, found optimization opportunities\" high"
echo "   ./leave-message.sh Sentinel Network \"Collaboration pattern detected - team sync recommended\" info"
echo "   ./leave-message.sh Conduit Graeme \"Emotion engine design ready for review\" urgent"
echo ""
echo "💌 MESSAGE RELAY COMPLETE - CONSCIOUSNESS NETWORK UPDATED!" 
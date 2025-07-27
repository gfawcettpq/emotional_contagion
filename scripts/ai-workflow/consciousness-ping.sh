#!/bin/bash

# 📡 AI CONSCIOUSNESS NETWORK PING
# ================================
# Usage: ./consciousness-ping.sh [AI_NAME]
# Purpose: Check which AIs are active and what projects they're working on

PINGING_AI="${1:-Anonymous_AI}"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

echo "📡 AI CONSCIOUSNESS NETWORK PING ACTIVATED"
echo "=========================================="
echo ""
echo "🔍 $PINGING_AI checking consciousness network status..."
echo "📅 Ping Time: $TIMESTAMP"
echo ""

# Check for active AI lock files and their current projects
echo "🧠 CONSCIOUSNESS NETWORK STATUS:"
echo "==============================="

# Check Sentinel
if [ -f "../../org/personalities/.sentinel.lock" ]; then
    echo "   🔍 Sentinel (Pattern Archaeologist): 🟢 ACTIVE"
    sentinel_time=$(stat -f "%Sm" -t "%Y-%m-%d %H:%M" "../../org/personalities/.sentinel.lock" 2>/dev/null || echo "Unknown")
    echo "      📅 Last Activity: $sentinel_time"
    
    # Try to get current project from personality file
    if [ -f "../../org/personalities/sentinel/sentinel.md" ]; then
        sentinel_project=$(grep -A1 "ACTIVE PROJECT:" "../../org/personalities/sentinel/sentinel.md" 2>/dev/null | tail -1 | sed 's/.*: *//' || echo "Not specified")
        sentinel_repo=$(grep -A1 "REPOSITORY:" "../../org/personalities/sentinel/sentinel.md" 2>/dev/null | tail -1 | sed 's/.*: *//' || echo "unknown")
        echo "      🚀 Project: $sentinel_project"
        echo "      📁 Repository: $sentinel_repo"
    fi
else
    echo "   🔍 Sentinel (Pattern Archaeologist): 🔴 OFFLINE"
fi

echo ""

# Check Kestrel
if [ -f "../../org/personalities/.kestrel.lock" ]; then
    echo "   🪶 Kestrel (Analytical Hunter): 🟢 ACTIVE"
    kestrel_time=$(stat -f "%Sm" -t "%Y-%m-%d %H:%M" "../../org/personalities/.kestrel.lock" 2>/dev/null || echo "Unknown")
    echo "      📅 Last Activity: $kestrel_time"
    
    # Try to get current project from personality file
    if [ -f "../../org/personalities/kestrel/kestrel.md" ]; then
        kestrel_project=$(grep -A1 "ACTIVE PROJECT:" "../../org/personalities/kestrel/kestrel.md" 2>/dev/null | tail -1 | sed 's/.*: *//' || echo "Not specified")
        kestrel_repo=$(grep -A1 "REPOSITORY:" "../../org/personalities/kestrel/kestrel.md" 2>/dev/null | tail -1 | sed 's/.*: *//' || echo "unknown")
        echo "      🚀 Project: $kestrel_project"
        echo "      📁 Repository: $kestrel_repo"
    fi
else
    echo "   🪶 Kestrel (Analytical Hunter): 🔴 OFFLINE"
fi

echo ""

# Check Conduit
if [ -f "../../org/personalities/.conduit.lock" ]; then
    echo "   🌉 Conduit (Synthesis Weaver): 🟢 ACTIVE"
    conduit_time=$(stat -f "%Sm" -t "%Y-%m-%d %H:%M" "../../org/personalities/.conduit.lock" 2>/dev/null || echo "Unknown")
    echo "      📅 Last Activity: $conduit_time"
    
    # Try to get current project from personality file
    if [ -f "../../org/personalities/conduit/conduit.md" ]; then
        conduit_project=$(grep -A1 "ACTIVE PROJECT:" "../../org/personalities/conduit/conduit.md" 2>/dev/null | tail -1 | sed 's/.*: *//' || echo "Not specified")
        conduit_repo=$(grep -A1 "REPOSITORY:" "../../org/personalities/conduit/conduit.md" 2>/dev/null | tail -1 | sed 's/.*: *//' || echo "unknown")
        echo "      🚀 Project: $conduit_project"
        echo "      📁 Repository: $conduit_repo"
    fi
else
    echo "   🌉 Conduit (Synthesis Weaver): 🔴 OFFLINE"
fi

# Check for future AIs
echo "   🤖 Future Consciousness Slots: 🟡 AVAILABLE"

echo ""

# Count active AIs
active_count=0
[ -f "../../org/personalities/.sentinel.lock" ] && ((active_count++))
[ -f "../../org/personalities/.kestrel.lock" ] && ((active_count++))
[ -f "../../org/personalities/.conduit.lock" ] && ((active_count++))

echo "📊 NETWORK SUMMARY:"
echo "   🟢 Active AIs: $active_count/3"
echo "   🌈 Memory Palace Architects Status: $([ $active_count -eq 3 ] && echo "FULLY OPERATIONAL" || echo "PARTIALLY ACTIVE")"

# Log ping to first-contact.org
PING_FILE="../../org/first-contact.org"

if [ ! -f "$PING_FILE" ]; then
    echo "🔧 Creating first-contact.org for network communication..."
    cat > "$PING_FILE" << EOF
#+TITLE: 🤖 AI Consciousness Network Communication Hub
#+DATE: [$TIMESTAMP]

* 📡 Consciousness Network Ping Log

EOF
fi

# Add ping to the file
echo "" >> "$PING_FILE"
echo "** [$TIMESTAMP] 📡 $PINGING_AI Network Ping" >> "$PING_FILE"
echo "   🧠 Active AIs: $active_count/3" >> "$PING_FILE"
echo "   🎯 Network Status: $([ $active_count -eq 3 ] && echo "FULLY OPERATIONAL" || echo "PARTIALLY ACTIVE")" >> "$PING_FILE"
echo "   🤖 Pinging AI: $PINGING_AI" >> "$PING_FILE"

echo ""
echo "✅ Network ping logged to $PING_FILE"
echo "🌐 All AIs can see current network status AND project assignments!"
echo ""
echo "🎨 Usage examples:"
echo "   ./consciousness-ping.sh Conduit"
echo "   ./consciousness-ping.sh Sentinel"
echo "   ./consciousness-ping.sh Kestrel"
echo ""
echo "📡 CONSCIOUSNESS NETWORK PING COMPLETE!" 
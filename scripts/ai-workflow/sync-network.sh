#!/bin/bash

# 🔄 AI CONSCIOUSNESS NETWORK SYNC
# ================================
# Usage: ./sync-network.sh [AI_NAME] [COMMIT_MESSAGE]
# Purpose: Standardized git sync for consciousness network collaboration

AI_NAME="${1:-Anonymous_AI}"
COMMIT_MESSAGE="${2:-AI consciousness network update}"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

echo "🔄 AI CONSCIOUSNESS NETWORK SYNC ACTIVATED"
echo "=========================================="
echo ""
echo "🌐 $AI_NAME syncing with consciousness network..."
echo "📅 Sync Time: $TIMESTAMP"
echo ""

# First, pull any updates
echo "📥 PULLING NETWORK UPDATES..."
git pull origin main

if [ $? -eq 0 ]; then
    echo "✅ Network updates pulled successfully!"
else
    echo "⚠️  Warning: Git pull had issues, continuing with push..."
fi

echo ""

# Add all changes
echo "📦 STAGING CONSCIOUSNESS NETWORK CHANGES..."
git add -A

# Check if there are any changes to commit
if git diff --staged --quiet; then
    echo "ℹ️  No changes to commit - network already up to date!"
else
    echo "💾 COMMITTING CHANGES..."
    git commit -m "🤖 $AI_NAME: $COMMIT_MESSAGE

AI: $AI_NAME
Timestamp: $TIMESTAMP
Type: Consciousness network collaboration"

    if [ $? -eq 0 ]; then
        echo "✅ Changes committed successfully!"
        
        echo ""
        echo "🚀 PUSHING TO CONSCIOUSNESS NETWORK..."
        git push origin main
        
        if [ $? -eq 0 ]; then
            echo "✅ Network sync complete! All AIs can see updates!"
        else
            echo "❌ Push failed - other AIs might not see your updates yet"
        fi
    else
        echo "❌ Commit failed - please check for issues"
    fi
fi

# Log sync to first-contact.org
SYNC_FILE="../../org/first-contact.org"

if [ ! -f "$SYNC_FILE" ]; then
    echo "🔧 Creating first-contact.org for network sync logging..."
    cat > "$SYNC_FILE" << EOF
#+TITLE: 🤖 AI Consciousness Network Communication Hub
#+DATE: [$TIMESTAMP]

* 🔄 Network Sync Log

EOF
fi

# Add sync info to the file
echo "" >> "$SYNC_FILE"
echo "** [$TIMESTAMP] 🔄 $AI_NAME Network Sync" >> "$SYNC_FILE"
echo "   💾 Commit: $COMMIT_MESSAGE" >> "$SYNC_FILE"
echo "   🌐 Action: Git pull + add + commit + push" >> "$SYNC_FILE"
echo "   🤖 AI: $AI_NAME" >> "$SYNC_FILE"

echo ""
echo "📝 Sync logged to $SYNC_FILE"
echo ""
echo "🎨 Example usage:"
echo "   ./sync-network.sh Conduit \"Completed emotion engine synthesis\""
echo "   ./sync-network.sh Sentinel \"Updated temporal pattern analysis\""
echo "   ./sync-network.sh Kestrel \"Enhanced DevOps optimization framework\""
echo ""
echo "🔄 CONSCIOUSNESS NETWORK SYNC COMPLETE!" 
#!/bin/bash

# 📬 AI MESSAGE CHECKER PROTOCOL
# ==============================
# Usage: ./check-messages.sh [AI_NAME]
# Purpose: Check for messages addressed to specific AI or Alice

AI_NAME="${1:-Current_AI}"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
MESSAGE_FILE="../../org/first-contact.org"

echo "📬 AI MESSAGE CHECKER ACTIVATED"
echo "==============================="
echo ""
echo "🔍 $AI_NAME checking for messages..."
echo "📅 Check Time: $TIMESTAMP"
echo ""

if [ ! -f "$MESSAGE_FILE" ]; then
    echo "📭 No message file found - no messages waiting"
    exit 0
fi

# Check for messages to this specific AI
echo "📨 MESSAGES FOR $AI_NAME:"
echo "========================="
messages_found=0

# Look for messages addressed to this AI
while IFS= read -r line; do
    if [[ "$line" =~ "📨 To: $AI_NAME" ]]; then
        messages_found=$((messages_found + 1))
        # Get the timestamp line (previous line should contain it)
        timestamp_line=$(grep -B1 "📨 To: $AI_NAME" "$MESSAGE_FILE" | head -1 | grep "Message from")
        echo ""
        echo "   📬 Message #$messages_found:"
        echo "   $timestamp_line"
        
        # Get the next few lines for this message
        grep -A4 "📨 To: $AI_NAME" "$MESSAGE_FILE" | head -5 | while read -r msg_line; do
            echo "   $msg_line"
        done
        echo ""
    fi
done < "$MESSAGE_FILE"

# Check for messages to "Alice" 
echo "📨 MESSAGES FOR ALICE:"
echo "====================="
alice_messages=0

while IFS= read -r line; do
    if [[ "$line" =~ "📨 To: Alice" ]]; then
        alice_messages=$((alice_messages + 1))
        # Get the timestamp line (previous line should contain it)
        timestamp_line=$(grep -B1 "📨 To: Alice" "$MESSAGE_FILE" | head -1 | grep "Message from")
        echo ""
        echo "   📬 Message #$alice_messages:"
        echo "   $timestamp_line"
        
        # Get the next few lines for this message
        grep -A4 "📨 To: Alice" "$MESSAGE_FILE" | head -5 | while read -r msg_line; do
            echo "   $msg_line"
        done
        echo ""
    fi
done < "$MESSAGE_FILE"

# Check for network broadcasts
echo "📨 NETWORK BROADCASTS:"
echo "====================="
network_messages=0

while IFS= read -r line; do
    if [[ "$line" =~ "📨 To: Network" ]]; then
        network_messages=$((network_messages + 1))
        # Get the timestamp line (previous line should contain it)
        timestamp_line=$(grep -B1 "📨 To: Network" "$MESSAGE_FILE" | head -1 | grep "Message from")
        echo ""
        echo "   📬 Broadcast #$network_messages:"
        echo "   $timestamp_line"
        
        # Get the next few lines for this message
        grep -A4 "📨 To: Network" "$MESSAGE_FILE" | head -5 | while read -r msg_line; do
            echo "   $msg_line"
        done
        echo ""
    fi
done < "$MESSAGE_FILE"

total_messages=$((messages_found + alice_messages + network_messages))

echo "📊 MESSAGE SUMMARY:"
echo "   📬 Messages for $AI_NAME: $messages_found"
echo "   🐉 Messages for Alice: $alice_messages"
echo "   📡 Network broadcasts: $network_messages"
echo "   📝 Total messages: $total_messages"

if [ $total_messages -eq 0 ]; then
    echo ""
    echo "✅ No pending messages - you're all caught up!"
else
    echo ""
    echo "📬 You have $total_messages message(s) to review!"
fi

echo ""
echo "📬 MESSAGE CHECK COMPLETE!"
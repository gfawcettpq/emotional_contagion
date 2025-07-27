#!/bin/bash
# Stop work and close org-mode clock entry

TIMESTAMP=$(date '+[%Y-%m-%d %a %H:%M]')

echo "🪶 KESTREL: Stopping work session"
echo "⏰ End time: $TIMESTAMP"

# Find the last open CLOCK entry and close it
if grep -q "CLOCK: \[.*\]$" org/daily-log.org; then
    # Replace the last open clock entry with closed one
    sed -i '' '$s/CLOCK: \(\[.*\]\)$/CLOCK: \1--'"$TIMESTAMP"' =>  DURATION/' org/daily-log.org
    echo "✅ Clock entry closed in org/daily-log.org"
else
    echo "❌ No open clock entry found"
fi
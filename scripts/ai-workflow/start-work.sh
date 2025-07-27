#!/bin/bash
# Ultra-Simple Work Starter with Org-Mode Integration

# Default to current activity if no task specified
TASK="${1:-$(basename $(pwd))}"
TIMESTAMP=$(date '+[%Y-%m-%d %a %H:%M]')

echo "🪶 KESTREL: Starting work on '$TASK'"
echo "⏰ Time: $TIMESTAMP"

# Create proper org-mode task entry with time tracking
cat >> ../org/daily-log.org << EOF

** Working on: $TASK
:LOGBOOK:
CLOCK: $TIMESTAMP
:END:

EOF

echo "✅ Task logged to org/daily-log.org"
echo "📝 Use 'stop-work' to close the clock entry"

# Load relevant context
echo "📚 Loading context for: $TASK"
if [[ -f "../org/notes/"*"$TASK"* ]]; then
    echo "📄 Found related notes:"
    ls ../org/notes/*"$TASK"* 2>/dev/null | head -3
fi
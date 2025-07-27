#!/bin/bash
# Smart Work Starter - Detects context and starts appropriate tracking

CONTEXT="$1"
TIMESTAMP=$(date '+[%Y-%m-%d %a %H:%M]')

echo "🪶 KESTREL SMART WORK DETECTION"

# Auto-detect what kind of work based on context
if [[ "$CONTEXT" =~ DVOPS-[0-9]+ ]]; then
    echo "🎫 Detected DVOPS ticket: $CONTEXT"
    ./work-on-ticket.sh "$CONTEXT"
    
elif [[ "$CONTEXT" =~ ^[A-Z]+-[0-9]+ ]]; then
    echo "🎫 Detected ticket format: $CONTEXT"
    ./work-on-ticket.sh "$CONTEXT"
    
elif [ -z "$CONTEXT" ]; then
    echo "🤔 No context provided. What are you working on?"
    echo ""
    echo "Recent options:"
    echo "📄 Recent DVOPS tickets:"
    grep -E "^\*\* .* DVOPS-[0-9]+" ../org/DVOPS.org | tail -3 | sed 's/.*\(DVOPS-[0-9]*\).*/  \1/'
    echo ""
    echo "📁 Current project: $(basename $(pwd))"
    echo ""
    echo "💡 Try: work DVOPS-12345  or  work emotion-contagion"
    
else
    echo "📁 Starting general work on: $CONTEXT"
    ./start-work.sh "$CONTEXT"
fi
#!/bin/bash
# Ticket-Aware Work Tracker with Org-Mode Integration

TICKET="$1"
TIMESTAMP=$(date '+[%Y-%m-%d %a %H:%M]')

if [ -z "$TICKET" ]; then
    echo "🎫 Available recent tickets:"
    grep -E ":CUSTOM_ID: DVOPS-[0-9]+" org/DVOPS.org | tail -5 | sed 's/.*DVOPS-/DVOPS-/'
    echo ""
    echo "Usage: work-on-ticket.sh DVOPS-12345"
    exit 1
fi

echo "🪶 KESTREL: Starting work on ticket $TICKET"
echo "⏰ Time: $TIMESTAMP"

# Find the ticket in DVOPS.org and add clock entry
if grep -q "$TICKET" org/DVOPS.org; then
    echo "✅ Found ticket $TICKET in DVOPS.org"
    
    # Add clock entry under the ticket's :LOGBOOK: section
    # Find the ticket heading and add clock entry
    awk -v ticket="$TICKET" -v timestamp="$TIMESTAMP" '
    /CUSTOM_ID: '"$TICKET"'/ { in_ticket = 1 }
    in_ticket && /:LOGBOOK:/ { 
        print $0
        print "CLOCK: " timestamp
        in_ticket = 0
        next
    }
    { print }
    ' org/DVOPS.org > /tmp/dvops_temp && mv /tmp/dvops_temp org/DVOPS.org
    
    echo "🕐 Clock started for $TICKET"
    
    # Show ticket context
    echo "📄 Ticket context:"
    grep -A 5 "$TICKET" org/DVOPS.org | head -10
    
else
    echo "❌ Ticket $TICKET not found in DVOPS.org"
    echo "💡 Creating simple task entry instead..."
    
    cat >> org/daily-log.org << EOF

** Working on: $TICKET
:LOGBOOK:
CLOCK: $TIMESTAMP
:END:

EOF
fi

# Store current ticket for stop-work script
echo "$TICKET" > /tmp/current_ticket.tmp

echo "📝 Use 'stop-work' to close the clock entry"
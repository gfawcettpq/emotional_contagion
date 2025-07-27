#!/bin/bash
# Setup Ultra-Simple Work Aliases

echo "🪶 KESTREL: Setting up work aliases..."

# Add to your shell profile
ALIAS_FILE="$HOME/.zshrc"  # or .bashrc

cat >> "$ALIAS_FILE" << 'EOF'

# AI-Enhanced Work Tracking (Kestrel System)
alias work="cd ~/working/emotional_contagion && ./scripts/ai-workflow/smart-start-work.sh"
alias done="cd ~/working/emotional_contagion && ./scripts/ai-workflow/stop-work.sh"
alias workstatus="tail -10 ~/working/emotional_contagion/org/daily-log.org"

EOF

echo "✅ Aliases added to $ALIAS_FILE"
echo ""
echo "🎯 USAGE:"
echo "   work DVOPS-12345    # Start ticket work"
echo "   work emotion-sim    # Start project work"  
echo "   work               # See recent options"
echo "   done               # Stop and calculate time"
echo "   workstatus         # Check recent work log"
echo ""
echo "🔄 Run: source $ALIAS_FILE  (or restart terminal)"
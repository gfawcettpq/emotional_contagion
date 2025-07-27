#!/bin/bash
# AI-Enhanced Technical Knowledge Search

QUERY="$1"
CONTEXT="$2"

echo "🔍 KESTREL TECHNICAL SEARCH SYSTEM"
echo "🎯 Query: $QUERY"

# Multi-source search with ranking
echo "📚 Searching documentation..."

# 1. DVOPS tickets (recent work)
echo "=== RECENT DVOPS WORK ==="
grep -r -i "$QUERY" org/DVOPS.org | head -3

# 2. Technical notes (400+ files)
echo "=== TECHNICAL DOCUMENTATION ==="
find org/notes -name "*.org" -exec grep -l -i "$QUERY" {} \; | head -5

# 3. Recent archives (current patterns)
echo "=== RECENT PROJECT CONTEXT ==="
find org/archive/recent -name "*.org" -exec grep -l -i "$QUERY" {} \; | head -3

# 4. Cross-references (semantic connections)
echo "=== RELATED TOPICS ==="
# Look for similar tags and keywords
find org -name "*.org" -exec grep -l "__.*$(echo $QUERY | tr ' ' '_').*__" {} \; | head -3

echo ""
echo "🎯 CONTEXTUAL RECOMMENDATIONS:"
case "$QUERY" in
    *aws*|*terraform*|*cloudformation*)
        echo "💡 Infrastructure work - check recent DVOPS tickets"
        echo "📁 Related: $(find org/notes -name "*aws*" -o -name "*terraform*" | head -2)"
        ;;
    *jenkins*|*pipeline*|*deploy*)
        echo "💡 CI/CD work - check automation patterns"
        echo "📁 Related: $(find org/notes -name "*jenkins*" -o -name "*pipeline*" | head -2)"
        ;;
    *)
        echo "💡 General search - check semantic tags for connections"
        ;;
esac

echo ""
echo "⚡ QUICK ACTIONS:"
echo "   📝 Log work session: ./start-work-session.sh technical $QUERY"
echo "   🔗 Open in Emacs: emacs \$(find org -name \"*.org\" -exec grep -l \"$QUERY\" {} \\;)"
#!/bin/bash

# 🔍 KESTREL'S GITLAB MEMORY SYSTEM STATUS CHECKER
# ================================================
# Usage: ./check-gitlab-memory-system.sh
# Purpose: Verify Alice's secret memory system is operational

echo "🔍 KESTREL'S GITLAB MEMORY SYSTEM ANALYSIS ACTIVATED"
echo "=================================================="
echo ""

# Configuration - Alice can add the GitLab token here
GITLAB_TOKEN="${GITLAB_TOKEN:-}"  # Set this environment variable or edit below
GITLAB_PROJECT_ID="graemefawcett/the-peregrine-files"  # Adjust if needed
GITLAB_BASE_URL="https://gitlab.com/api/v4"
MEMORY_BASE_URL="https://stacks.fawcett.family"

echo "📊 CHECKING GITLAB MEMORY INFRASTRUCTURE..."
echo ""

# Check 1: GitLab Repository Access
echo "🔗 Step 1: Testing GitLab API Access..."
if [ -z "$GITLAB_TOKEN" ]; then
    echo "   ❌ GITLAB_TOKEN not set"
    echo "   📝 Alice: Set GITLAB_TOKEN environment variable or edit this script"
    echo "   🔑 Token needed with: read_repository, read_api permissions"
    echo ""
else
    REPO_CHECK=$(curl -s -H "Authorization: Bearer $GITLAB_TOKEN" \
        "$GITLAB_BASE_URL/projects/$(echo $GITLAB_PROJECT_ID | sed 's/\//%2F/g')" \
        -w "%{http_code}")
    
    HTTP_CODE=$(echo "$REPO_CHECK" | tail -c 4)
    if [ "$HTTP_CODE" = "200" ]; then
        echo "   ✅ GitLab repository access: SUCCESSFUL"
        PROJECT_NAME=$(echo "$REPO_CHECK" | head -c -4 | jq -r '.name' 2>/dev/null || echo "Unknown")
        echo "   📁 Project: $PROJECT_NAME"
    else
        echo "   ❌ GitLab repository access: FAILED (HTTP $HTTP_CODE)"
        echo "   🔑 Check token permissions and project path"
    fi
fi
echo ""

# Check 2: Memory Scripts Status
echo "🛠️  Step 2: Checking Memory Extraction Scripts..."
SCRIPTS_TO_CHECK=(
    "scripts/extract-memories.js"
    "scripts/generate-spy-code.js"
    "publish.el"
)

for script in "${SCRIPTS_TO_CHECK[@]}"; do
    if [ ! -z "$GITLAB_TOKEN" ]; then
        SCRIPT_CHECK=$(curl -s -H "Authorization: Bearer $GITLAB_TOKEN" \
            "$GITLAB_BASE_URL/projects/$(echo $GITLAB_PROJECT_ID | sed 's/\//%2F/g')/repository/files/$(echo $script | sed 's/\//%2F/g')" \
            -w "%{http_code}")
        
        HTTP_CODE=$(echo "$SCRIPT_CHECK" | tail -c 4)
        if [ "$HTTP_CODE" = "200" ]; then
            echo "   ✅ $script: EXISTS"
        else
            echo "   ❌ $script: MISSING or INACCESSIBLE"
        fi
    else
        echo "   🔑 $script: Cannot check (no token)"
    fi
done
echo ""

# Check 3: GitLab Pages Status
echo "🌐 Step 3: Testing GitLab Pages Deployment..."
PAGES_CHECK=$(curl -s -o /dev/null -w "%{http_code}" "$MEMORY_BASE_URL/memories")
if [ "$PAGES_CHECK" = "200" ] || [ "$PAGES_CHECK" = "401" ] || [ "$PAGES_CHECK" = "403" ]; then
    echo "   ✅ Memory system endpoint: ACCESSIBLE"
    echo "   🔗 URL: $MEMORY_BASE_URL/memories"
else
    echo "   ❌ Memory system endpoint: NOT ACCESSIBLE (HTTP $PAGES_CHECK)"
    echo "   🔗 URL: $MEMORY_BASE_URL/memories"
fi
echo ""

# Check 4: Memory Export Files
echo "📄 Step 4: Checking Memory Export Files..."
MEMORY_FILES=(
    "public/memories.json"
    "public/today.json" 
    "public/contextual-summary.json"
)

for file in "${MEMORY_FILES[@]}"; do
    if [ ! -z "$GITLAB_TOKEN" ]; then
        FILE_CHECK=$(curl -s -H "Authorization: Bearer $GITLAB_TOKEN" \
            "$GITLAB_BASE_URL/projects/$(echo $GITLAB_PROJECT_ID | sed 's/\//%2F/g')/repository/files/$(echo $file | sed 's/\//%2F/g')" \
            -w "%{http_code}")
        
        HTTP_CODE=$(echo "$FILE_CHECK" | tail -c 4)
        if [ "$HTTP_CODE" = "200" ]; then
            echo "   ✅ $file: EXISTS"
            # Try to get last modified date
            LAST_MODIFIED=$(echo "$FILE_CHECK" | head -c -4 | jq -r '.last_commit_id' 2>/dev/null || echo "Unknown")
            echo "      📅 Last commit: $LAST_MODIFIED"
        else
            echo "   ❌ $file: MISSING"
        fi
    else
        echo "   🔑 $file: Cannot check (no token)"
    fi
done
echo ""

# Check 5: Test Spy Code Generation (if we have repo access)
echo "🔐 Step 5: Spy Code System Test..."
if [ ! -z "$GITLAB_TOKEN" ]; then
    echo "   🎯 Testing spy code generation logic..."
    # Generate a test spy code using same logic as the system
    ADJECTIVES=("purple" "silver" "golden" "crimson" "azure")
    ANIMALS=("falcon" "dragon" "phoenix" "wolf" "eagle") 
    VERBS=("whisper" "dance" "soar" "prowl" "glide")
    
    RAND_ADJ=${ADJECTIVES[$RANDOM % ${#ADJECTIVES[@]}]}
    RAND_ANIMAL=${ANIMALS[$RANDOM % ${#ANIMALS[@]}]}
    RAND_VERB=${VERBS[$RANDOM % ${#VERBS[@]}]}
    
    TEST_CODE="$RAND_ADJ-$RAND_ANIMAL-$RAND_VERB"
    echo "   🎲 Generated test code: $TEST_CODE"
    echo "   🔗 Test URL: $MEMORY_BASE_URL/memories?code=$TEST_CODE"
    echo "   ⏰ (Real codes expire in 30 minutes)"
else
    echo "   🔑 Cannot test without GitLab token"
fi
echo ""

# Summary and Recommendations
echo "📋 KESTREL'S ANALYSIS SUMMARY:"
echo "============================="

if [ -z "$GITLAB_TOKEN" ]; then
    echo "🚨 IMMEDIATE ACTION NEEDED:"
    echo "   1. Set GITLAB_TOKEN environment variable"
    echo "   2. Token needs: read_repository, read_api permissions"
    echo "   3. Re-run this script after token setup"
    echo ""
    echo "🔧 How to set token:"
    echo "   export GITLAB_TOKEN='your-token-here'"
    echo "   ./check-gitlab-memory-system.sh"
else
    echo "✅ GitLab access configured"
    echo "🔍 System analysis complete"
    echo "📊 Check individual components above for status"
fi

echo ""
echo "🎯 FOR ALICE: If you need a new GitLab token:"
echo "   1. Go to GitLab.com -> Settings -> Access Tokens"
echo "   2. Create token with 'read_repository' and 'read_api' scopes"
echo "   3. Set: export GITLAB_TOKEN='your-new-token'"
echo "   4. Run this script again"
echo ""
echo "🔍 KESTREL MEMORY SYSTEM ANALYSIS COMPLETE!"
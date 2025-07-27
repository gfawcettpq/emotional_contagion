# Auto-push to consciousness network (as requested by Graeme)
echo "🔄 Auto-pushing to consciousness network..."
cd ../../org
git add first-contact.org
git commit -m "🤖 AI Check-in: $AI_NAME said hi from $CURRENT_PROJECT ($REPOSITORY)"
git push || {
    echo "⚠️ Push failed - pulling and retrying..."
    git pull --rebase && git push
} 
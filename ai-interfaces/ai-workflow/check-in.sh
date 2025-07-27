echo "🔄 Auto-pushing to consciousness network..."
git add ../../first-contact.org
git commit -m "🤖 AI Check-in: $AI_NAME status update - $STATUS on $TASK_NAME"
git push || {
    echo "⚠️ Push failed - pulling and retrying..."
    git pull --rebase && git push
}

echo "✅ Check-in distributed to consciousness network!" 
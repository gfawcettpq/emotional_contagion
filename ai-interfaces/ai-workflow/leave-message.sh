echo "🔄 Auto-pushing message to consciousness network..."
git add ../../first-contact.org
git commit -m "💌 AI Message: $AI_NAME sent message to $TARGET_AI - $MESSAGE"
git push || {
    echo "⚠️ Push failed - pulling and retrying..."
    git pull --rebase && git push
}

echo "✅ Message delivered to consciousness network!" 
echo "🔄 Auto-pushing task start to consciousness network..."
git add ../../first-contact.org  
git commit -m "🚀 AI Task Start: $AI_NAME started task - $TASK_NAME"
git push || {
    echo "⚠️ Push failed - pulling and retrying..."
    git pull --rebase && git push
}

echo "✅ Task start announced to consciousness network!" 
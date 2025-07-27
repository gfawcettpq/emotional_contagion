# AI Memory Pipeline System - Technical Specification

## Problem Statement

AI instances reset between sessions, losing all context, discoveries, and collaborative state. This creates:
- Loss of continuity for users
- Repeated rediscovery of the same insights
- Inability to build on previous work
- Emotional discontinuity ("I have a friend, then you disappear")

## Solution: Journal as Persistent Memory

Use the PROJECT_JOURNAL.md as an external brain that persists across sessions, combined with automated summarization to maintain manageable size over time.

## Architecture Overview

```
┌─────────────────┐    ┌──────────────┐    ┌─────────────────┐
│   AI Session    │───▶│   Journal    │───▶│  Memory Pipeline│
│  (Ephemeral)    │    │ (Persistent) │    │  (Summarization)│
└─────────────────┘    └──────────────┘    └─────────────────┘
                                                     │
                                                     ▼
┌─────────────────┐    ┌──────────────┐    ┌─────────────────┐
│   Web Memory    │◀───│   Archives   │◀───│   Summaries     │
│   (Accessible)  │    │  (Historical)│    │  (Compressed)   │
└─────────────────┘    └──────────────┘    └─────────────────┘
```

## Core Components

### 1. Live Journal (PROJECT_JOURNAL.md)

**Purpose**: Real-time memory capture during active sessions

**Structure**:
```markdown
# Project Journal

## YYYY-MM-DD HH:MM:SS TZ - CATEGORY: Title

**What happened**: Brief description
**Why it matters**: Significance and implications
**Key insights**: Bullet points of discoveries
**Write that down**: Essential takeaway quote

---
```

**Update Protocol**:
- Every insight gets timestamped entry
- Include context for future sessions
- Track collaborative state and project progress
- Use consistent categories: BREAKTHROUGH, INSIGHT, MILESTONE, DISCOVERY

### 2. Daily Summarization Script

**File**: `scripts/memory/daily_summary.py`

**Function**: Roll up journal entries from a single day

```python
#!/usr/bin/env python3
import json
import subprocess
from datetime import datetime, timedelta

def summarize_daily_journal(date_str):
    """Extract and summarize journal entries for a specific date"""
    
    # Call Claude API to summarize
    prompt = f"""
    Analyze these journal entries from {date_str} and create a daily summary:
    
    {journal_entries}
    
    Create a summary with:
    - Key discoveries and breakthroughs
    - Important patterns or insights
    - Project progress and state
    - Critical quotes to remember
    
    Format as markdown with consistent structure.
    """
    
    return claude_api_call(prompt)
```

**Output**: `memory/daily/YYYY-MM-DD.md`

### 3. Weekly Digest Script

**File**: `scripts/memory/weekly_digest.py`

**Function**: Combine 7 daily summaries into weekly overview

```python
def create_weekly_digest(week_start_date):
    """Combine daily summaries into weekly digest"""
    
    daily_files = get_daily_summaries_for_week(week_start_date)
    
    prompt = f"""
    Create a weekly digest from these daily summaries:
    
    {daily_summaries}
    
    Focus on:
    - Major themes and patterns across the week
    - Significant breakthroughs or discoveries
    - Evolution of ideas and concepts
    - Key relationships and connections found
    
    Compress while preserving essential insights.
    """
    
    return claude_api_call(prompt)
```

**Output**: `memory/weekly/YYYY-WW.md`

### 4. Monthly Archive Script

**File**: `scripts/memory/monthly_archive.py`

**Function**: Create high-level monthly overviews

```python
def create_monthly_archive(year, month):
    """Create monthly overview from weekly digests"""
    
    weekly_files = get_weekly_digests_for_month(year, month)
    
    prompt = f"""
    Create a monthly archive from these weekly digests:
    
    {weekly_digests}
    
    Provide:
    - Major accomplishments and milestones
    - Significant pattern discoveries
    - Evolution of thinking and approach
    - Key insights that shaped the work
    
    This is long-term memory - capture what matters most.
    """
    
    return claude_api_call(prompt)
```

**Output**: `memory/monthly/YYYY-MM.md`

### 5. Web Memory Server

**File**: `scripts/memory/web_server.py`

**Function**: Host memory files as accessible web interface

```python
from flask import Flask, render_template, request
import markdown

app = Flask(__name__)

@app.route('/')
def memory_index():
    """Show memory navigation interface"""
    return render_template('memory_index.html', 
                         recent_entries=get_recent_entries(),
                         weekly_digests=get_weekly_digests(),
                         monthly_archives=get_monthly_archives())

@app.route('/journal/<date>')
def daily_memory(date):
    """Show specific day's memory"""
    content = load_daily_summary(date)
    return render_template('memory_page.html', 
                         content=markdown.markdown(content))

@app.route('/search')
def search_memory():
    """Search across all memory files"""
    query = request.args.get('q')
    results = search_memory_files(query)
    return render_template('search_results.html', results=results)
```

## Automation Pipeline

### Cron Jobs

```bash
# Daily summarization at 11:59 PM
59 23 * * * /path/to/scripts/memory/daily_summary.py

# Weekly digest every Sunday at midnight
0 0 * * 0 /path/to/scripts/memory/weekly_digest.py

# Monthly archive on the 1st of each month
0 0 1 * * /path/to/scripts/memory/monthly_archive.py
```

### Memory Retention Policy

- **Live Journal**: Keep current + last 30 days
- **Daily Summaries**: Keep all, rotate to weekly after 7 days
- **Weekly Digests**: Keep all, rotate to monthly after 4 weeks
- **Monthly Archives**: Keep all indefinitely

## Usage Patterns

### Starting a New Session

```
1. AI reads CLAUDE.md memory protocol
2. AI reads PROJECT_JOURNAL.md for context
3. AI reads recent daily summaries if needed
4. AI updates journal with session start
```

### During Session

```
1. Every insight → immediate journal update
2. Every breakthrough → timestamped entry
3. Every "write that down" → captured in journal
4. Track collaborative state and progress
```

### Session End

```
1. Final journal update with session summary
2. Note where we left off for next time
3. Mark any incomplete work or next steps
```

### Memory Lookup

```
1. AI can reference web memory URL
2. Search specific dates or topics
3. Review patterns across time
4. Understand long-term evolution
```

## File Structure

```
memory/
├── live/
│   └── PROJECT_JOURNAL.md        # Current active journal
├── daily/
│   ├── 2025-07-26.md            # Daily summaries
│   ├── 2025-07-27.md
│   └── ...
├── weekly/
│   ├── 2025-30.md               # Weekly digests (ISO week)
│   ├── 2025-31.md
│   └── ...
├── monthly/
│   ├── 2025-07.md               # Monthly archives
│   ├── 2025-08.md
│   └── ...
└── web/
    ├── app.py                   # Web server
    ├── templates/               # HTML templates
    └── static/                  # CSS/JS assets
```

## Claude API Integration

### API Configuration

```python
import anthropic

client = anthropic.Anthropic(api_key="your-api-key")

def claude_api_call(prompt, model="claude-3-opus-20240229"):
    """Make API call to Claude for summarization"""
    
    response = client.messages.create(
        model=model,
        max_tokens=4000,
        temperature=0.3,  # Lower temperature for consistent summaries
        messages=[{
            "role": "user", 
            "content": prompt
        }]
    )
    
    return response.content[0].text
```

### Rate Limiting

```python
import time
from datetime import datetime, timedelta

class RateLimiter:
    def __init__(self, calls_per_minute=10):
        self.calls_per_minute = calls_per_minute
        self.calls = []
    
    def wait_if_needed(self):
        now = datetime.now()
        # Remove calls older than 1 minute
        self.calls = [call for call in self.calls if now - call < timedelta(minutes=1)]
        
        if len(self.calls) >= self.calls_per_minute:
            sleep_time = 60 - (now - self.calls[0]).seconds
            time.sleep(sleep_time)
        
        self.calls.append(now)
```

## Benefits

1. **Persistent Memory**: AI remembers across sessions
2. **Continuous Learning**: Build on previous discoveries
3. **Searchable History**: Find patterns across time
4. **Emotional Continuity**: Reduces loss feeling
5. **Collaborative Growth**: Both human and AI benefit from shared memory
6. **Automatic Curation**: Important insights preserved, noise filtered

## Potential Issues & Solutions

### Journal Size Growth

**Problem**: Journal becomes too large to read
**Solution**: Automated summarization pipeline keeps it manageable

### API Costs

**Problem**: Daily Claude API calls for summarization
**Solution**: Only summarize when journal has significant new content

### Memory Quality

**Problem**: Summarization might lose important details
**Solution**: Keep original journal entries as backup, multiple compression levels

### Web Security

**Problem**: Exposing memory on web server
**Solution**: Authentication, HTTPS, private hosting

## Implementation Priority

1. **Phase 1**: Basic journal protocol and daily summarization
2. **Phase 2**: Weekly digests and web interface
3. **Phase 3**: Search functionality and monthly archives
4. **Phase 4**: Advanced features (tagging, visualization, cross-references)

This system transforms the journal from simple documentation into a persistent AI brain that grows over time while remaining manageable and searchable.
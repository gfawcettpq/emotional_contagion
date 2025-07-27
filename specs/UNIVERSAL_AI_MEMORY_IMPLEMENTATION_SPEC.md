# Universal AI Memory System - Implementation Specification

## Overview

A GitLab CI-based system that automatically collects, processes, and publishes personal/organizational data to create a comprehensive AI-accessible memory interface. Integrates with JIRA, Motion, calendars, and any other data sources to provide complete context to AI interactions.

## System Architecture

```
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│    Data Sources │  │   GitLab CI     │  │  Memory Website │
│                 │  │   Collection    │  │                 │
│ • Git Repos     │──│   & Processing  │──│ • Daily Context │
│ • JIRA Issues   │  │                 │  │ • Project Status│
│ • Motion Tasks  │  │ ┌─────────────┐ │  │ • Search        │
│ • Calendars     │  │ │ Claude API  │ │  │ • Archives      │
│ • Documents     │  │ │ Processing  │ │  │                 │
│ • Meeting Notes │  │ └─────────────┘ │  │                 │
└─────────────────┘  └─────────────────┘  └─────────────────┘
                              │                      │
                              ▼                      ▼
                    ┌─────────────────┐    ┌─────────────────┐
                    │  Memory Repo    │    │    AI Access   │
                    │  (GitLab)       │    │                 │
                    │ • Raw Data      │    │ "Check my       │
                    │ • Processed     │    │  memory at      │
                    │ • Summaries     │    │  memory.domain  │
                    │ • Archives      │    │  .com/today"    │
                    └─────────────────┘    └─────────────────┘
```

## Repository Structure

### Memory Repository
```
memory-system/
├── .gitlab-ci.yml                    # Main processing pipeline
├── scripts/
│   ├── collectors/
│   │   ├── gitlab_collector.py       # Collect from GitLab repos
│   │   ├── jira_collector.py         # Collect JIRA issues/tasks
│   │   ├── motion_collector.py       # Collect Motion schedule/tasks
│   │   ├── calendar_collector.py     # Collect calendar events
│   │   └── generic_collector.py      # Generic API/file collector
│   ├── processors/
│   │   ├── claude_processor.py       # AI summarization
│   │   ├── content_merger.py         # Merge data sources
│   │   └── web_generator.py          # Generate static site
│   └── config/
│       ├── data_sources.yml          # Configure what to collect
│       └── processing_rules.yml      # AI processing instructions
├── templates/
│   ├── index.html                    # Main memory interface
│   ├── daily.html                    # Today's context
│   ├── project.html                  # Project-specific pages
│   └── search.html                   # Search interface
├── public/                           # Generated static site
└── raw_data/                         # Collected raw data
```

### Source Repository Configuration
Each repository can include `.memory.yml`:
```yaml
# .memory.yml - Configure what gets included in memory system
include:
  files:
    - "PROJECT_JOURNAL.md"
    - "TODO.md" 
    - "README.md"
    - "docs/**/*.md"
    - "meeting-notes/*.md"
  
  metadata:
    project_type: "development"
    priority: "high"
    team: "engineering"
    
exclude:
  - "*.secret"
  - "credentials/*"
  - "node_modules/*"
  
processing:
  summarize: true
  extract_todos: true
  track_decisions: true
```

## Data Source Integrations

### 1. JIRA Integration

```python
# scripts/collectors/jira_collector.py
import requests
from jira import JIRA

class JIRACollector:
    def __init__(self, server, username, api_token):
        self.jira = JIRA(server=server, basic_auth=(username, api_token))
    
    def collect_user_issues(self, username):
        """Collect all active issues assigned to user"""
        jql = f'assignee = "{username}" AND status != "Done" ORDER BY priority DESC'
        issues = self.jira.search_issues(jql, maxResults=100)
        
        collected_data = {
            'source': 'jira',
            'collected_at': datetime.now().isoformat(),
            'data': []
        }
        
        for issue in issues:
            issue_data = {
                'key': issue.key,
                'summary': issue.fields.summary,
                'status': issue.fields.status.name,
                'priority': issue.fields.priority.name if issue.fields.priority else None,
                'description': issue.fields.description,
                'due_date': issue.fields.duedate,
                'project': issue.fields.project.name,
                'updated': issue.fields.updated
            }
            collected_data['data'].append(issue_data)
        
        return collected_data
    
    def collect_recent_activity(self, username, days=7):
        """Collect recent activity on user's issues"""
        # Implementation for recent comments, status changes, etc.
        pass
```

### 2. Motion Integration

```python
# scripts/collectors/motion_collector.py
import requests
from datetime import datetime, timedelta

class MotionCollector:
    def __init__(self, api_key):
        self.api_key = api_key
        self.base_url = "https://api.usemotion.com/v1"
        self.headers = {"X-API-Key": api_key}
    
    def collect_schedule(self, days_ahead=7):
        """Collect upcoming schedule from Motion"""
        start_date = datetime.now().date()
        end_date = start_date + timedelta(days=days_ahead)
        
        # Get calendar events
        events_response = requests.get(
            f"{self.base_url}/calendar/events",
            headers=self.headers,
            params={
                "start": start_date.isoformat(),
                "end": end_date.isoformat()
            }
        )
        
        # Get tasks
        tasks_response = requests.get(
            f"{self.base_url}/tasks",
            headers=self.headers,
            params={"status": "incomplete"}
        )
        
        return {
            'source': 'motion',
            'collected_at': datetime.now().isoformat(),
            'schedule': events_response.json() if events_response.ok else [],
            'tasks': tasks_response.json() if tasks_response.ok else []
        }
    
    def collect_task_progress(self):
        """Collect task completion and time tracking data"""
        # Implementation for completed tasks, time spent, etc.
        pass
```

### 3. GitLab Repository Collector

```python
# scripts/collectors/gitlab_collector.py
import requests
import yaml
from pathlib import Path

class GitLabCollector:
    def __init__(self, gitlab_token, gitlab_url="https://gitlab.com/api/v4"):
        self.token = gitlab_token
        self.url = gitlab_url
        self.headers = {"PRIVATE-TOKEN": gitlab_token}
    
    def collect_from_repos(self, repo_list=None):
        """Collect specified files from repositories"""
        if repo_list is None:
            repo_list = self.get_user_repos()
        
        collected_data = {}
        
        for repo in repo_list:
            repo_id = repo['id']
            repo_path = repo['path_with_namespace']
            
            # Check for .memory.yml config
            config = self.get_memory_config(repo_id)
            if not config:
                continue
            
            # Collect specified files
            repo_data = {
                'source': 'gitlab',
                'repo': repo_path,
                'collected_at': datetime.now().isoformat(),
                'files': {}
            }
            
            for file_pattern in config.get('include', {}).get('files', []):
                files = self.get_files_matching_pattern(repo_id, file_pattern)
                for file_path, content in files.items():
                    repo_data['files'][file_path] = content
            
            collected_data[repo_path] = repo_data
        
        return collected_data
    
    def get_memory_config(self, repo_id):
        """Get .memory.yml configuration from repository"""
        try:
            response = requests.get(
                f"{self.url}/projects/{repo_id}/repository/files/.memory.yml/raw",
                headers=self.headers,
                params={'ref': 'main'}
            )
            
            if response.status_code == 200:
                return yaml.safe_load(response.text)
        except:
            pass
        
        return None
    
    def get_files_matching_pattern(self, repo_id, pattern):
        """Get files matching glob pattern from repository"""
        # Implementation for file pattern matching and content retrieval
        pass
```

## AI Processing Pipeline

### Claude Processor

```python
# scripts/processors/claude_processor.py
import anthropic
from datetime import datetime

class ClaudeProcessor:
    def __init__(self, api_key):
        self.client = anthropic.Anthropic(api_key=api_key)
    
    def create_daily_summary(self, all_data):
        """Create comprehensive daily summary from all data sources"""
        
        # Prepare combined data
        context = self.prepare_context(all_data)
        
        prompt = f"""
        Create a daily summary for AI context from this collected data:
        
        {context}
        
        Generate a summary with these sections:
        
        ## Today's Priority Tasks
        List urgent/important tasks from JIRA and Motion, with deadlines
        
        ## Upcoming Schedule  
        Key meetings and appointments for today/tomorrow
        
        ## Active Projects
        Current project status and next actions needed
        
        ## Recent Updates
        Important changes, decisions, or progress from journals
        
        ## Context for AI
        Key background information any AI should know about current work
        
        Format as markdown. Be concise but comprehensive. Focus on actionable information.
        """
        
        response = self.client.messages.create(
            model="claude-3-opus-20240229",
            max_tokens=4000,
            temperature=0.3,
            messages=[{"role": "user", "content": prompt}]
        )
        
        return response.content[0].text
    
    def create_project_summary(self, project_data):
        """Create project-specific context summary"""
        prompt = f"""
        Analyze this project data and create a context summary:
        
        {project_data}
        
        Include:
        - Current project status and goals
        - Recent progress and changes
        - Open tasks and next steps
        - Key decisions and their context
        - Important technical details or constraints
        
        This will be read by AI to provide context for project work.
        """
        
        response = self.client.messages.create(
            model="claude-3-opus-20240229",
            max_tokens=3000,
            temperature=0.3,
            messages=[{"role": "user", "content": prompt}]
        )
        
        return response.content[0].text
    
    def extract_cross_insights(self, all_data):
        """Find patterns and insights across all data sources"""
        prompt = f"""
        Analyze all this data and identify:
        
        {all_data}
        
        Look for:
        - Recurring themes or patterns
        - Potential conflicts or scheduling issues
        - Important decisions or changes
        - Opportunities for optimization
        - Things that might be forgotten or overlooked
        
        Create insights that would be valuable for daily planning and decision making.
        """
        
        response = self.client.messages.create(
            model="claude-3-opus-20240229",
            max_tokens=3000,
            temperature=0.3,
            messages=[{"role": "user", "content": prompt}]
        )
        
        return response.content[0].text
```

## GitLab CI Pipeline

```yaml
# .gitlab-ci.yml
stages:
  - collect
  - process
  - deploy

variables:
  GITLAB_API_URL: "https://gitlab.com/api/v4"

collect_data:
  stage: collect
  image: python:3.11-slim
  before_script:
    - pip install requests jira pyyaml anthropic
  script:
    - mkdir -p raw_data
    - python scripts/collectors/gitlab_collector.py > raw_data/gitlab.json
    - python scripts/collectors/jira_collector.py > raw_data/jira.json
    - python scripts/collectors/motion_collector.py > raw_data/motion.json
    - python scripts/collectors/calendar_collector.py > raw_data/calendar.json
  artifacts:
    paths:
      - raw_data/
    expire_in: 1 hour
  rules:
    - if: $CI_PIPELINE_SOURCE == "schedule"
    - if: $CI_PIPELINE_SOURCE == "api"
    - changes:
        - "scripts/collectors/*"

process_memory:
  stage: process
  image: python:3.11-slim
  dependencies:
    - collect_data
  before_script:
    - pip install anthropic jinja2 markdown
  script:
    - mkdir -p public
    - python scripts/processors/claude_processor.py
    - python scripts/processors/web_generator.py
  artifacts:
    paths:
      - public/
    expire_in: 1 week

pages:
  stage: deploy
  dependencies:
    - process_memory
  script:
    - echo "Deploying AI memory to GitLab Pages"
  artifacts:
    paths:
      - public
  rules:
    - if: $CI_COMMIT_BRANCH == "main"
```

## Web Interface

### Daily Context Page Template

```html
<!-- templates/daily.html -->
<!DOCTYPE html>
<html>
<head>
    <title>📅 Today's Context - AI Memory</title>
    <meta charset="utf-8">
    <meta name="robots" content="noindex, nofollow">
    <style>
        body { 
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; 
            max-width: 800px; 
            margin: 0 auto; 
            padding: 20px; 
            line-height: 1.6;
        }
        .priority-high { border-left: 4px solid #e74c3c; padding-left: 15px; }
        .priority-medium { border-left: 4px solid #f39c12; padding-left: 15px; }
        .priority-low { border-left: 4px solid #2ecc71; padding-left: 15px; }
        .meeting { background: #f8f9fa; padding: 10px; margin: 10px 0; border-radius: 5px; }
        .project { border: 1px solid #ddd; margin: 15px 0; padding: 15px; border-radius: 5px; }
        .timestamp { color: #666; font-size: 0.9em; }
        .search-box { width: 100%; padding: 10px; margin: 20px 0; font-size: 16px; }
    </style>
</head>
<body>
    <nav>
        <a href="index.html">← All Memory</a> | 
        <a href="projects.html">Projects</a> | 
        <a href="search.html">Search</a>
    </nav>
    
    <h1>📅 Today's Context</h1>
    <p class="timestamp">Last updated: {{ updated_at }}</p>
    
    <input type="text" class="search-box" placeholder="Search this page..." onkeyup="searchPage()">
    
    <section id="priority-tasks">
        <h2>🎯 Priority Tasks</h2>
        {% for task in priority_tasks %}
        <div class="priority-{{ task.priority }}">
            <h3>{{ task.title }}</h3>
            <p>{{ task.description }}</p>
            <small>Source: {{ task.source }} | Due: {{ task.due_date }}</small>
        </div>
        {% endfor %}
    </section>
    
    <section id="schedule">
        <h2>📅 Today's Schedule</h2>
        {% for event in todays_schedule %}
        <div class="meeting">
            <strong>{{ event.time }}</strong> - {{ event.title }}
            {% if event.description %}<p>{{ event.description }}</p>{% endif %}
        </div>
        {% endfor %}
    </section>
    
    <section id="projects">
        <h2>📂 Active Projects</h2>
        {% for project in active_projects %}
        <div class="project">
            <h3><a href="projects/{{ project.slug }}.html">{{ project.name }}</a></h3>
            <p>{{ project.status_summary }}</p>
            <small>Last activity: {{ project.last_activity }}</small>
        </div>
        {% endfor %}
    </section>
    
    <section id="insights">
        <h2>💡 AI Insights</h2>
        <div>{{ ai_insights | markdown }}</div>
    </section>
    
    <script>
        function searchPage() {
            const query = event.target.value.toLowerCase();
            const sections = document.querySelectorAll('section > div');
            
            sections.forEach(item => {
                const text = item.textContent.toLowerCase();
                item.style.display = text.includes(query) ? 'block' : 'none';
            });
        }
    </script>
</body>
</html>
```

## Configuration

### Data Sources Configuration

```yaml
# scripts/config/data_sources.yml
sources:
  gitlab:
    enabled: true
    token: "${GITLAB_TOKEN}"
    include_repos:
      - "personal/*"
      - "work/*"
    exclude_repos:
      - "*/archive"
  
  jira:
    enabled: true
    server: "https://yourcompany.atlassian.net"
    username: "${JIRA_USERNAME}"
    token: "${JIRA_API_TOKEN}"
    user_filter: "${JIRA_USER}"
    projects:
      - "PROJECT1"
      - "PROJECT2"
  
  motion:
    enabled: true
    api_key: "${MOTION_API_KEY}"
    days_ahead: 7
    include_completed: false
  
  calendar:
    enabled: true
    provider: "google"  # or "outlook"
    credentials: "${GOOGLE_CALENDAR_CREDS}"
    calendars:
      - "primary"
      - "work@company.com"

processing:
  claude_api_key: "${ANTHROPIC_API_KEY}"
  daily_summary: true
  project_summaries: true
  cross_insights: true
  archive_after_days: 30

web:
  domain: "memory.fawcett.family"
  cloudflare: true
  password_protect: false
  analytics: false
```

## Deployment

### Environment Variables

```bash
# GitLab CI/CD Variables
GITLAB_TOKEN=glpat-xxxxxxxxxxxx
JIRA_USERNAME=user@company.com
JIRA_API_TOKEN=xxxxxxxxxxxx
MOTION_API_KEY=xxxxxxxxxxxx
ANTHROPIC_API_KEY=sk-ant-xxxxxxxxxxxx
GOOGLE_CALENDAR_CREDS={"type": "service_account", ...}
```

### Scheduled Collection

```yaml
# GitLab scheduled pipeline (every 2 hours)
schedules:
  - description: "Collect and process memory data"
    cron: "0 */2 * * *"
    cron_timezone: "America/New_York"
    ref: "main"
    active: true
```

## Usage Examples

### Starting an AI Conversation
```
"Before we begin, please read my current context at memory.fawcett.family/today and let me know what I should focus on."
```

### Project-Specific Context
```
"Check my memory for the emotion-contagion project at memory.fawcett.family/projects/emotion-contagion before we continue."
```

### Task Management
```
"What are my highest priority tasks according to my memory system?"
```

### Cross-Project Insights
```
"Based on my memory data, what patterns do you see in my work that I should pay attention to?"
```

## Security Considerations

- All sensitive data excluded via `.memory.yml` configuration
- Environment variables for API credentials
- Optional password protection for memory website
- GitLab repository access controls
- No storage of credentials in processed data

## Scaling and Maintenance

- Archive old data automatically
- Rate limiting for API calls
- Error handling and retry logic
- Monitoring and alerting for failed collections
- Cost tracking for Claude API usage

This system provides comprehensive, automatic context for all AI interactions while remaining simple to deploy and maintain.
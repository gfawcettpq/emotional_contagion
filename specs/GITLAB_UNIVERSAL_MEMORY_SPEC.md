# GitLab Universal Memory System - Technical Specification

## Concept

Create a centralized AI memory system that automatically collects journal files from ALL your projects using GitLab CI/CD and publishes them as a searchable web interface.

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Project A     │    │   Project B     │    │   Project C     │
│ PROJECT_JOURNAL │    │ PROJECT_JOURNAL │    │ PROJECT_JOURNAL │
│      .md        │    │      .md        │    │      .md        │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          │         GitLab CI Triggers on Journal Changes
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 ▼
               ┌─────────────────────────────────────┐
               │        Memory Repository            │
               │  ┌─────────────────────────────────┐│
               │  │     GitLab CI Pipeline         ││
               │  │  1. Collect all journals       ││
               │  │  2. Process with Claude API    ││
               │  │  3. Generate summaries         ││
               │  │  4. Update memory files        ││
               │  │  5. Deploy to GitLab Pages     ││
               │  └─────────────────────────────────┘│
               └─────────────────┬───────────────────┘
                                 ▼
               ┌─────────────────────────────────────┐
               │          GitLab Pages              │
               │     https://memory.gitlab.io       │
               │  ┌─────────────────────────────────┐│
               │  │  - Daily summaries             ││
               │  │  - Weekly digests              ││
               │  │  - Monthly archives            ││
               │  │  - Search interface            ││
               │  │  - Raw journal access          ││
               │  └─────────────────────────────────┘│
               └─────────────────────────────────────┘
                                 ▲
                                 │
               ┌─────────────────────────────────────┐
               │           Any AI Instance           │
               │  "Read my memory from memory.gitlab │
               │   .io/latest or search for topic"   │
               └─────────────────────────────────────┘
```

## Repository Structure

### Project Repositories (.gitlab-ci.yml)

Each project gets this CI configuration to notify the memory repo:

```yaml
# .gitlab-ci.yml in each project repository
stages:
  - memory_sync

memory_notify:
  stage: memory_sync
  image: alpine:latest
  rules:
    - changes:
        - PROJECT_JOURNAL.md
  variables:
    MEMORY_REPO_TOKEN: $MEMORY_REPO_TOKEN
    MEMORY_REPO_URL: "https://gitlab.com/api/v4/projects/YOUR_MEMORY_REPO_ID/pipeline"
  script:
    - apk add --no-cache curl
    - |
      curl -X POST \
        -H "PRIVATE-TOKEN: $MEMORY_REPO_TOKEN" \
        -H "Content-Type: application/json" \
        -d '{
          "ref": "main",
          "variables": [
            {
              "key": "TRIGGER_PROJECT",
              "value": "'$CI_PROJECT_PATH'"
            },
            {
              "key": "TRIGGER_COMMIT",
              "value": "'$CI_COMMIT_SHA'"
            }
          ]
        }' \
        "$MEMORY_REPO_URL"
    - echo "Notified memory repo of journal update"
```

### Memory Repository (.gitlab-ci.yml)

The central memory repository that collects and processes everything:

```yaml
# .gitlab-ci.yml in memory repository
stages:
  - collect
  - process
  - deploy

variables:
  GITLAB_API_URL: "https://gitlab.com/api/v4"

collect_journals:
  stage: collect
  image: python:3.11-alpine
  before_script:
    - apk add --no-cache git curl
    - pip install requests anthropic
  script:
    - python scripts/collect_journals.py
  artifacts:
    paths:
      - collected_journals/
    expire_in: 1 hour

process_memories:
  stage: process
  image: python:3.11-alpine
  dependencies:
    - collect_journals
  before_script:
    - pip install requests anthropic markdown jinja2
  script:
    - python scripts/process_memories.py
  artifacts:
    paths:
      - public/
    expire_in: 1 week

pages:
  stage: deploy
  dependencies:
    - process_memories
  script:
    - echo "Deploying to GitLab Pages"
  artifacts:
    paths:
      - public
  rules:
    - if: $CI_COMMIT_BRANCH == "main"
```

## Collection Script

```python
#!/usr/bin/env python3
# scripts/collect_journals.py

import os
import requests
import json
from pathlib import Path

class JournalCollector:
    def __init__(self):
        self.gitlab_token = os.environ['GITLAB_TOKEN']
        self.gitlab_url = os.environ.get('GITLAB_API_URL', 'https://gitlab.com/api/v4')
        self.headers = {'PRIVATE-TOKEN': self.gitlab_token}
        
    def get_all_projects(self):
        """Get all projects the user has access to"""
        projects = []
        page = 1
        
        while True:
            response = requests.get(
                f"{self.gitlab_url}/projects",
                headers=self.headers,
                params={'page': page, 'per_page': 100, 'owned': True}
            )
            
            if response.status_code != 200:
                break
                
            page_projects = response.json()
            if not page_projects:
                break
                
            projects.extend(page_projects)
            page += 1
            
        return projects
    
    def has_journal(self, project_id):
        """Check if project has a PROJECT_JOURNAL.md file"""
        response = requests.get(
            f"{self.gitlab_url}/projects/{project_id}/repository/files/PROJECT_JOURNAL.md",
            headers=self.headers,
            params={'ref': 'main'}
        )
        return response.status_code == 200
    
    def download_journal(self, project_id, project_path):
        """Download PROJECT_JOURNAL.md from a project"""
        response = requests.get(
            f"{self.gitlab_url}/projects/{project_id}/repository/files/PROJECT_JOURNAL.md/raw",
            headers=self.headers,
            params={'ref': 'main'}
        )
        
        if response.status_code == 200:
            # Create directory structure
            output_dir = Path(f"collected_journals/{project_path}")
            output_dir.mkdir(parents=True, exist_ok=True)
            
            # Save journal file
            journal_file = output_dir / "PROJECT_JOURNAL.md"
            journal_file.write_text(response.text)
            
            # Save metadata
            metadata = {
                'project_id': project_id,
                'project_path': project_path,
                'collected_at': datetime.now().isoformat(),
                'commit_sha': self.get_latest_commit(project_id)
            }
            
            metadata_file = output_dir / "metadata.json"
            metadata_file.write_text(json.dumps(metadata, indent=2))
            
            return True
        return False
    
    def get_latest_commit(self, project_id):
        """Get latest commit SHA for the project"""
        response = requests.get(
            f"{self.gitlab_url}/projects/{project_id}/repository/commits",
            headers=self.headers,
            params={'ref': 'main', 'per_page': 1}
        )
        
        if response.status_code == 200:
            commits = response.json()
            if commits:
                return commits[0]['id']
        return None
    
    def collect_all(self):
        """Main collection function"""
        print("🐋 Collecting journals from all projects...")
        
        projects = self.get_all_projects()
        collected = 0
        
        for project in projects:
            project_id = project['id']
            project_path = project['path_with_namespace']
            
            print(f"Checking {project_path}...")
            
            if self.has_journal(project_id):
                if self.download_journal(project_id, project_path):
                    print(f"✅ Collected journal from {project_path}")
                    collected += 1
                else:
                    print(f"❌ Failed to download journal from {project_path}")
            else:
                print(f"⏭️  No journal in {project_path}")
        
        print(f"\n🐋 Collection complete: {collected} journals collected")
        return collected

if __name__ == "__main__":
    collector = JournalCollector()
    collector.collect_all()
```

## Processing Script

```python
#!/usr/bin/env python3
# scripts/process_memories.py

import os
import json
import anthropic
from pathlib import Path
from datetime import datetime, timedelta
import markdown
import jinja2

class MemoryProcessor:
    def __init__(self):
        self.claude_client = anthropic.Anthropic(
            api_key=os.environ['ANTHROPIC_API_KEY']
        )
        
    def load_journals(self):
        """Load all collected journals"""
        journals = {}
        
        for project_dir in Path("collected_journals").iterdir():
            if project_dir.is_dir():
                journal_file = project_dir / "PROJECT_JOURNAL.md"
                metadata_file = project_dir / "metadata.json"
                
                if journal_file.exists() and metadata_file.exists():
                    metadata = json.loads(metadata_file.read_text())
                    content = journal_file.read_text()
                    
                    journals[project_dir.name] = {
                        'content': content,
                        'metadata': metadata
                    }
        
        return journals
    
    def summarize_with_claude(self, content, summary_type="daily"):
        """Use Claude to create summaries"""
        
        if summary_type == "daily":
            prompt = f"""
            Analyze this project journal and create a daily summary:
            
            {content}
            
            Focus on:
            - Key discoveries and breakthroughs
            - Important insights and patterns
            - Project progress and milestones
            - Memorable quotes and realizations
            
            Format as markdown with clear sections.
            """
        elif summary_type == "cross_project":
            prompt = f"""
            Analyze these journal entries from multiple projects and find:
            
            {content}
            
            Look for:
            - Common patterns across projects
            - Shared insights and breakthroughs
            - Related discoveries in different contexts
            - Evolution of thinking across all work
            
            Create a cross-project insight summary.
            """
        
        response = self.claude_client.messages.create(
            model="claude-3-opus-20240229",
            max_tokens=4000,
            temperature=0.3,
            messages=[{"role": "user", "content": prompt}]
        )
        
        return response.content[0].text
    
    def generate_index_page(self, journals):
        """Generate the main index page"""
        template = jinja2.Template("""
<!DOCTYPE html>
<html>
<head>
    <title>🐋 Universal AI Memory</title>
    <meta charset="utf-8">
    <style>
        body { font-family: system-ui; max-width: 1200px; margin: 0 auto; padding: 20px; }
        .project { border: 1px solid #ddd; margin: 10px 0; padding: 15px; border-radius: 5px; }
        .project h3 { margin-top: 0; }
        .metadata { font-size: 0.9em; color: #666; }
        .search { margin: 20px 0; }
        .search input { width: 300px; padding: 10px; }
        .whale { font-size: 2em; }
    </style>
</head>
<body>
    <h1>🐋 Universal AI Memory System</h1>
    
    <div class="search">
        <input type="text" id="searchBox" placeholder="Search across all memories..." onkeyup="searchMemories()">
    </div>
    
    <h2>Active Projects ({{ journals|length }})</h2>
    
    {% for project_name, data in journals.items() %}
    <div class="project">
        <h3><a href="projects/{{ project_name }}.html">{{ project_name }}</a></h3>
        <div class="metadata">
            Last updated: {{ data.metadata.collected_at }}<br>
            Project ID: {{ data.metadata.project_id }}
        </div>
        <p>{{ data.content[:200] }}...</p>
    </div>
    {% endfor %}
    
    <h2>Memory Archives</h2>
    <ul>
        <li><a href="daily_summaries.html">Daily Summaries</a></li>
        <li><a href="weekly_digests.html">Weekly Digests</a></li>
        <li><a href="cross_project_insights.html">Cross-Project Insights</a></li>
        <li><a href="search.html">Advanced Search</a></li>
    </ul>
    
    <script>
        function searchMemories() {
            // Client-side search implementation
            const query = document.getElementById('searchBox').value.toLowerCase();
            const projects = document.querySelectorAll('.project');
            
            projects.forEach(project => {
                const text = project.textContent.toLowerCase();
                project.style.display = text.includes(query) ? 'block' : 'none';
            });
        }
    </script>
</body>
</html>
        """)
        
        return template.render(journals=journals)
    
    def process_all(self):
        """Main processing function"""
        print("🐋 Processing memories with Claude...")
        
        # Load all journals
        journals = self.load_journals()
        
        # Create output directory
        output_dir = Path("public")
        output_dir.mkdir(exist_ok=True)
        
        # Generate index page
        index_html = self.generate_index_page(journals)
        (output_dir / "index.html").write_text(index_html)
        
        # Create individual project pages
        projects_dir = output_dir / "projects"
        projects_dir.mkdir(exist_ok=True)
        
        for project_name, data in journals.items():
            # Convert markdown to HTML
            html_content = markdown.markdown(data['content'])
            
            # Wrap in basic HTML template
            page_html = f"""
<!DOCTYPE html>
<html>
<head>
    <title>🐋 {project_name} Memory</title>
    <meta charset="utf-8">
    <style>
        body {{ font-family: system-ui; max-width: 800px; margin: 0 auto; padding: 20px; }}
        pre {{ background: #f5f5f5; padding: 15px; border-radius: 5px; }}
        code {{ background: #f5f5f5; padding: 2px 4px; border-radius: 3px; }}
    </style>
</head>
<body>
    <nav><a href="../index.html">← Back to All Memories</a></nav>
    <h1>🐋 {project_name}</h1>
    {html_content}
</body>
</html>
            """
            
            (projects_dir / f"{project_name}.html").write_text(page_html)
        
        # Create cross-project insights
        all_content = "\n\n".join([data['content'] for data in journals.values()])
        cross_insights = self.summarize_with_claude(all_content, "cross_project")
        
        insights_html = f"""
<!DOCTYPE html>
<html>
<head>
    <title>🐋 Cross-Project Insights</title>
    <meta charset="utf-8">
    <style>
        body {{ font-family: system-ui; max-width: 800px; margin: 0 auto; padding: 20px; }}
    </style>
</head>
<body>
    <nav><a href="index.html">← Back to All Memories</a></nav>
    <h1>🐋 Cross-Project Insights</h1>
    {markdown.markdown(cross_insights)}
</body>
</html>
        """
        
        (output_dir / "cross_project_insights.html").write_text(insights_html)
        
        print(f"🐋 Processing complete: {len(journals)} projects processed")

if __name__ == "__main__":
    processor = MemoryProcessor()
    processor.process_all()
```

## Usage

### Setup

1. **Create memory repository** on GitLab
2. **Add environment variables**:
   - `GITLAB_TOKEN` - Personal access token
   - `ANTHROPIC_API_KEY` - Claude API key
3. **Add CI configuration** to memory repo
4. **Add notification CI** to each project repo

### Automatic Operation

1. **Any journal update** in any project triggers CI
2. **Memory repo collects** all journals automatically
3. **Claude processes** and summarizes content
4. **GitLab Pages publishes** the results
5. **AI can read** from `https://your-username.gitlab.io/memory/`

### AI Usage

```
"Read my latest memories from https://memory.gitlab.io/latest or search for insights about [topic] at https://memory.gitlab.io/search?q=topic"
```

## Benefits

1. **Universal Memory**: One system for all projects
2. **Automatic Collection**: No manual intervention needed
3. **AI Accessible**: Any AI can read the published data
4. **Searchable**: Find patterns across all work
5. **Scalable**: Handles unlimited projects
6. **Cost Effective**: Only processes when journals change

🐋 *"Like a pod of whales sharing songs across vast oceans, our memories flow between projects, carrying insights from distant waters to nurture new discoveries."*

This system creates true universal AI memory that scales infinitely and costs almost nothing to maintain!
# Dashboard

## Overview
Generate and display the project dashboard showing overall progress, phase progress, feature progress, and active pull requests.

## Workflow
1. Read all progress.json files from feature directories
2. Calculate overall and phase progress percentages
3. Check GitHub for active PRs (if GitHub integration enabled)
4. Generate visual progress bars
5. Update `.doplan/dashboard.json` (machine-readable)
6. Update `.doplan/dashboard.md` (human-readable)

## Dashboard Sections
- Overall project progress
- Phase-by-phase progress
- Feature progress within phases
- Active pull requests
- Recent GitHub activity (commits, branches, pushes)
- Next recommended actions

## Usage
After running this command, view the dashboard:
- Markdown: Open `.doplan/dashboard.md`
- CLI: Run `doplan` in terminal

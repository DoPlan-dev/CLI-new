# Progress

## Overview
Update all progress tracking files. Recalculate progress bars for phases and features, update dashboard, and sync progress.json files.

## Workflow
1. Scan all feature directories in `doplan/`
2. Read tasks.md files from each feature
3. Count completed tasks (marked with [x])
4. Calculate completion percentages
5. Update progress.json files:
   - Feature-level: `doplan/XX-phase/XX-feature/progress.json`
   - Phase-level: `doplan/XX-phase/phase-progress.json`
6. Regenerate dashboard:
   - `.doplan/dashboard.json`
   - `.doplan/dashboard.md`
7. Sync GitHub data (if enabled)
8. Update state file: `.doplan/state.json`

## Progress Calculation
- Feature progress: (completed tasks / total tasks) * 100
- Phase progress: Average of all feature progress in phase
- Overall progress: Average of all phase progress

--- Cursor Command: next.md ---
# Next

## Overview
Analyze the current project state and recommend the next best action. Check progress, incomplete tasks, and suggest what to work on next.

## Workflow
1. Read current state from `.doplan/state.json`
2. Scan all feature directories for incomplete tasks
3. Check progress.json files
4. Consider dependencies between features
5. Recommend highest priority action
6. Display recommendation in dashboard format

## Analysis Factors
- Task completion status
- Feature dependencies
- Phase priorities
- Blocked items
- Progress percentages
- GitHub branch status

## Output
- Next recommended action
- Priority level
- Estimated effort
- Dependencies to consider

## Usage
Run `/next` in your IDE to get the next recommended action.

## Execution
This command is executed via:
```bash
doplan execute next
```

--- End Command ---

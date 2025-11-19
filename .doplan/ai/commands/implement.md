--- Cursor Command: implement.md ---
# Implement

## Overview
Start implementing a feature. This command helps guide implementation based on the feature's planning documents and automatically creates a GitHub branch.

## Workflow
1. Check current feature context from `.doplan/state.json`
2. **Automatically create GitHub branch:**
   - Format: `feature/XX-phase-XX-feature-name`
   - Create branch: `git checkout -b {branch-name}`
3. **Initial commit:**
   - Add plan.md, design.md, tasks.md files
   - Commit message: `docs: add planning docs for {feature-name}`
   - Push: `git push origin {branch-name}`
4. Update state with branch name
5. Update dashboard
6. Guide implementation based on:
   - `plan.md` - Feature plan
   - `design.md` - Design specifications
   - `tasks.md` - Task breakdown

## Implementation Guidance
- Follow the feature's plan.md and design.md
- Check off tasks in tasks.md as you complete them
- Commit regularly with clear messages
- Update progress as you work

## Usage
Run `/implement <phase-id>` or `/implement <phase-id>/<feature-id>` in your IDE.

## Execution
This command is executed via:
```bash
doplan execute implement <phase-id>
# or
doplan execute implement <phase-id>/<feature-id>
```

--- End Command ---

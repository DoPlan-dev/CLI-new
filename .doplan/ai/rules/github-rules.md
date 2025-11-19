# GitHub Integration Rules

## Branch Naming Convention

- Format: `feature/XX-phase-XX-feature-name`
- Example: `feature/01-phase-01-user-authentication`
- Use kebab-case for feature names
- Always prefix with `feature/`

## Automatic Branch Creation

When /Implement command is used:

1. **Check current feature:**
   - Read from `.cursor/config/doplan-state.json`
   - Get current phase and feature

2. **Generate branch name:**
   - Format: `feature/{phase-id}-{feature-id}-{feature-name}`
   - Convert to kebab-case
   - Example: `feature/01-phase-01-user-authentication`

3. **Create branch:**
   ```bash
   git checkout -b feature/01-phase-01-user-authentication
   ```

4. **Initial commit:**
   - Add feature planning files (plan.md, design.md, tasks.md)
   - Commit message: `docs: add planning docs for {feature-name}`
   - Push to remote

5. **Update state:**
   - Save branch name to state
   - Update dashboard

## Automatic Commit Rules

### Commit Message Format

Follow conventional commits:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `refactor:` - Code refactoring
- `test:` - Tests
- `chore:` - Maintenance

### Commit Workflow

1. **During development:**
   - Commit frequently with clear messages
   - Reference task numbers when possible
   - Format: `feat: implement {task-description}`

2. **Task completion:**
   - Update tasks.md (check off completed tasks)
   - Commit: `feat: complete {task-name}`

3. **Feature completion:**
   - Mark all tasks complete in tasks.md
   - Update progress.json: `"status": "complete"`
   - Commit: `feat: complete {feature-name}`
   - Push branch

## Automatic Push Rules

### Push on Commit

After each commit:
```bash
git push origin feature/XX-phase-XX-feature-name
```

### Push Status Tracking

- Track push success/failure
- Update dashboard with push status
- Log push history in `doplan/github-data.json`

## Pull Request Automation

### When Feature is Complete

1. **Check completion:**
   - All tasks in tasks.md are checked
   - progress.json shows `"status": "complete"`

2. **Create PR automatically:**
   - Use GitHub CLI: `gh pr create`
   - Title: `Feature: {feature-name}`
   - Body includes links to plan.md, design.md, tasks.md
   - Base branch: `main`
   - Head branch: feature branch

3. **Update dashboard:**
   - Add PR link to dashboard
   - Update state with PR number

## Merge Automation

After PR approval:
1. Merge PR: `gh pr merge {pr-number} --merge`
2. Delete branch: `gh pr merge {pr-number} --delete-branch`
3. Update progress
4. Update dashboard
5. Sync with main branch

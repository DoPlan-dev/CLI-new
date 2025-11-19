# Branch Management Rules

## Branch Naming Convention

**Format:** `feature/{phase-id}-{feature-id}-{feature-name}`

**Examples:**
- `feature/01-phase-01-user-authentication`
- `feature/01-phase-02-database-setup`
- `feature/02-phase-01-task-creation`

**Rules:**
- Always prefix with `feature/`
- Use phase and feature IDs from plan structure
- Convert feature name to kebab-case
- No spaces or special characters (except hyphens)

## Automatic Branch Creation

### Trigger: /Implement Command

When /Implement is executed:

1. **Get feature information:**
   - Read from `.cursor/config/doplan-state.json`
   - Current phase: `state.currentPhase`
   - Current feature: `state.currentFeature`

2. **Generate branch name:**
   - Format: `feature/{phase-id}-{feature-id}-{feature-name}`
   - Convert to kebab-case

3. **Create and switch to branch:**
   ```bash
   git checkout -b {branchName}
   ```

4. **Initial commit:**
   ```bash
   git add doplan/{phase}/{feature}/*
   git commit -m "docs: add planning docs for {feature-name}"
   git push origin {branchName}
   ```

5. **Update state:**
   - Save branch name to feature in state
   - Update dashboard

## Branch Workflow

### During Development

1. **Work on feature branch:**
   - All commits go to feature branch
   - Regular commits with clear messages
   - Push frequently

2. **Update tasks:**
   - Mark tasks complete in tasks.md
   - Commit: `feat: complete {task-name}`

3. **Keep branch updated:**
   - Regularly merge main into feature branch
   - Resolve conflicts early

### Feature Completion

1. **Final checks:**
   - All tasks in tasks.md are checked
   - progress.json shows complete
   - All tests pass

2. **Final commit:**
   ```bash
   git add .
   git commit -m "feat: complete {feature-name}"
   git push origin {branchName}
   ```

3. **Create PR:**
   - Automatically via /Implement or manual
   - PR will be created when feature marked complete

## Branch Cleanup

After PR merge:
1. Switch to main: `git checkout main`
2. Pull latest: `git pull origin main`
3. Delete local branch: `git branch -d {branchName}`
4. Delete remote branch: `git push origin --delete {branchName}`

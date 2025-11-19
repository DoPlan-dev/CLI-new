# Commit Rules

## Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks
- `style:` - Code style changes (formatting, etc.)
- `perf:` - Performance improvements

### Examples

```
feat: implement user authentication endpoint
fix: resolve login validation issue
docs: update API documentation
refactor: simplify authentication logic
test: add unit tests for auth service
```

## Automatic Commit Workflow

### During Feature Development

1. **Task completion:**
   - Update tasks.md (check off task)
   - Commit: `feat: complete {task-description}`
   - Push: `git push origin {branch-name}`

2. **Implementation milestone:**
   - Commit: `feat: implement {component-name}`
   - Push immediately

3. **Bug fixes:**
   - Commit: `fix: resolve {issue-description}`
   - Push immediately

### Feature Planning Phase

1. **Initial planning docs:**
   ```bash
   git add doplan/{phase}/{feature}/*
   git commit -m "docs: add planning docs for {feature-name}"
   ```

2. **Plan updates:**
   ```bash
   git commit -m "docs: update {feature-name} plan"
   ```

### Feature Completion

1. **Final commit:**
   ```bash
   git add .
   git commit -m "feat: complete {feature-name}"
   git push origin {branch-name}
   ```

## Automatic Push Rules

### Always Push After Commit

After every commit, automatically push:
```bash
git push origin {current-branch}
```

### Push Status Tracking

- Track push success/failure
- Update dashboard with push status
- Log in `doplan/github-data.json`

### Error Handling

If push fails:
1. Show error message
2. Suggest: `git pull origin {branch-name}`
3. Retry push after pull

## Commit Frequency

- Commit after each logical unit of work
- Commit at least once per day
- Commit before switching tasks
- Commit before leaving for the day

## Commit Best Practices

- Write clear, descriptive commit messages
- Reference task numbers when possible
- Keep commits focused (one logical change)
- Don't commit broken code
- Test before committing
- Review changes before committing

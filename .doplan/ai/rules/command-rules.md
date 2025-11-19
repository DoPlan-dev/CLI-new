# DoPlan Command Rules

## /Discuss Command

**Purpose:** Start idea discussion and refinement

**Workflow:**
1. Ask comprehensive questions about the idea
2. Suggest improvements and enhancements
3. Help organize features into phases
4. Recommend tech stack based on requirements
5. Save results to:
   - `.cursor/config/doplan-state.json`
   - `doplan/idea-notes.md`

**Output:**
- Idea notes document
- Updated state file
- Tech stack recommendations

## /Refine Command

**Purpose:** Enhance and improve existing idea

**Workflow:**
1. Review existing idea notes
2. Suggest additional features
3. Identify gaps in the plan
4. Enhance technical specifications
5. Update idea documentation

## /Generate Command

**Purpose:** Generate PRD, Structure, and API contracts

**Workflow:**
1. Read idea notes and state
2. Generate `doplan/PRD.md`
3. Generate `doplan/structure.md`
4. Generate `doplan/contracts/api-spec.json`
5. Generate `doplan/contracts/data-model.md`
6. Use templates from `doplan/templates/`

## /Plan Command

**Purpose:** Generate phase and feature structure

**Workflow:**
1. Read PRD and contracts
2. Create phase directories: `doplan/01-phase/`, etc.
3. Create feature directories: `doplan/01-phase/01-Feature/`, etc.
4. Generate for each phase:
   - `phase-plan.md`
   - `phase-progress.json`
5. Generate for each feature:
   - `plan.md`
   - `design.md`
   - `tasks.md`
   - `progress.json`
6. Update dashboard

## /Dashboard Command

**Purpose:** Show project dashboard with progress

**Workflow:**
1. Read all progress.json files
2. Calculate overall and phase progress
3. Check GitHub for active PRs
4. Generate visual progress bars
5. Update `doplan/dashboard.md`
6. Update `doplan/dashboard.html`

## /Implement Command

**Purpose:** Start implementing a feature

**Workflow:**
1. Check current feature context from state
2. **Automatically create GitHub branch:**
   - Format: `feature/XX-phase-XX-feature-name`
   - Create branch: `git checkout -b {branch-name}`
3. **Initial commit:**
   - Add plan.md, design.md, tasks.md
   - Commit: `docs: add planning docs for {feature-name}`
   - Push: `git push origin {branch-name}`
4. Update state with branch name
5. Update dashboard
6. Guide implementation based on plan.md, design.md, tasks.md

## /Next Command

**Purpose:** Get recommendation for next action

**Workflow:**
1. Analyze current state
2. Check incomplete tasks
3. Consider dependencies
4. Recommend highest priority action
5. Display in dashboard format

## /Progress Command

**Purpose:** Update all progress tracking

**Workflow:**
1. Scan all feature directories
2. Read tasks.md files
3. Calculate completion percentages
4. Update progress.json files
5. Regenerate dashboard
6. Sync GitHub data
7. Update state file

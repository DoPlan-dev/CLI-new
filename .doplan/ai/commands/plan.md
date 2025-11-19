--- Cursor Command: plan.md ---
# Plan

## Overview
Generate the project plan with phases and features. Create the directory structure following DoPlan workflow.

## Workflow
1. Read PRD from `doplan/PRD.md`
2. Read contracts from `doplan/contracts/`
3. Create phase directories: `doplan/plan/01-phase-name/`, `doplan/plan/02-phase-name/`, etc.
4. Create feature directories: `doplan/plan/01-phase-name/01-feature-name/`, etc.
5. Generate for each phase:
   - `phase-plan.md`
   - `phase-progress.json`
6. Generate for each feature:
   - `plan.md`
   - `design.md`
   - `tasks.md`
   - `progress.json`
7. Update dashboard with new structure

## Structure
```
doplan/plan/
├── 01-phase-name/
│   ├── phase-plan.md
│   ├── phase-progress.json
│   ├── 01-feature-name/
│   │   ├── plan.md
│   │   ├── design.md
│   │   ├── tasks.md
│   │   └── progress.json
│   └── 02-feature-name/
└── 02-phase-name/
```

## Usage
Run `/plan` in your IDE to generate the project plan structure.

## Execution
This command is executed via:
```bash
doplan execute plan
```

--- End Command ---

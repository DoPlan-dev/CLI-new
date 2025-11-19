use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::utils;

/// Generate IDE command file for /discuss
pub fn generate_discuss_command(ide_type: &str) -> Result<PathBuf> {
    let commands_dir = utils::ai_commands_dir()?;
    utils::ensure_dir(&commands_dir)?;

    let filename = match ide_type {
        "cursor" => "discuss.md",
        "gemini" => "discuss.md",
        "claude" => "discuss.md",
        _ => "discuss.md",
    };

    let command_path = commands_dir.join(filename);

    let content = r#"--- Cursor Command: discuss.md ---
# Discuss

## Overview
Start idea discussion and refinement workflow. This command helps refine your project idea, suggest improvements, organize features, and select the best tech stack.

## Workflow
1. Ask comprehensive questions about the idea
2. Suggest improvements and enhancements
3. Help organize features into logical phases
4. Recommend the best tech stack for your project
5. Save results to:
   - `.doplan/state.json`
   - `doplan/idea-notes.md`

## Output
- Idea notes document
- Updated state file
- Tech stack recommendations
- Feature organization suggestions

## Usage
Run `/discuss` in your IDE to start the interactive discussion workflow.

## Execution
This command is executed via:
```bash
doplan execute discuss
```

--- End Command ---
"#;

    std::fs::write(&command_path, content)
        .context("Failed to write discuss command file")?;

    Ok(command_path)
}

/// Generate IDE command file for /generate
pub fn generate_generate_command(ide_type: &str) -> Result<PathBuf> {
    let commands_dir = utils::ai_commands_dir()?;
    utils::ensure_dir(&commands_dir)?;

    let filename = match ide_type {
        "cursor" => "generate.md",
        "gemini" => "generate.md",
        "claude" => "generate.md",
        _ => "generate.md",
    };

    let command_path = commands_dir.join(filename);

    let content = r#"--- Cursor Command: generate.md ---
# Generate

## Overview
Generate Product Requirements Document (PRD), project structure document, and API contracts based on the refined idea.

## Workflow
1. Read idea notes from `doplan/idea-notes.md`
2. Read state from `.doplan/state.json`
3. Generate `doplan/PRD.md` - Product Requirements Document
4. Generate `doplan/structure.md` - Project structure and architecture
5. Generate `doplan/contracts/api-spec.json` - API specification (OpenAPI/Swagger)
6. Generate `doplan/contracts/data-model.md` - Data models and schemas
7. Use templates from `doplan/templates/` directory

## Documents Created
- PRD.md - Complete product requirements
- structure.md - Project architecture
- api-spec.json - API contracts
- data-model.md - Data models

## Usage
Run `/generate` in your IDE to generate Phase 1 documents.

## Execution
This command is executed via:
```bash
doplan execute generate
```

--- End Command ---
"#;

    std::fs::write(&command_path, content)
        .context("Failed to write generate command file")?;

    Ok(command_path)
}

/// Generate IDE command file for /plan
pub fn generate_plan_command(ide_type: &str) -> Result<PathBuf> {
    let commands_dir = utils::ai_commands_dir()?;
    utils::ensure_dir(&commands_dir)?;

    let filename = match ide_type {
        "cursor" => "plan.md",
        "gemini" => "plan.md",
        "claude" => "plan.md",
        _ => "plan.md",
    };

    let command_path = commands_dir.join(filename);

    let content = r#"--- Cursor Command: plan.md ---
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
"#;

    std::fs::write(&command_path, content)
        .context("Failed to write plan command file")?;

    Ok(command_path)
}

/// Generate IDE command file for /implement
pub fn generate_implement_command(ide_type: &str) -> Result<PathBuf> {
    let commands_dir = utils::ai_commands_dir()?;
    utils::ensure_dir(&commands_dir)?;

    let filename = match ide_type {
        "cursor" => "implement.md",
        "gemini" => "implement.md",
        "claude" => "implement.md",
        _ => "implement.md",
    };

    let command_path = commands_dir.join(filename);

    let content = r#"--- Cursor Command: implement.md ---
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
"#;

    std::fs::write(&command_path, content)
        .context("Failed to write implement command file")?;

    Ok(command_path)
}

/// Generate all IDE command files
pub fn generate_all_commands(ide_types: &[String]) -> Result<Vec<PathBuf>> {
    let mut generated = Vec::new();
    
    for ide_type in ide_types {
        generated.push(generate_discuss_command(ide_type)?);
        generated.push(generate_generate_command(ide_type)?);
        generated.push(generate_plan_command(ide_type)?);
        generated.push(generate_implement_command(ide_type)?);
    }

    Ok(generated)
}


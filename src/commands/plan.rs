use anyhow::{Context, Result};
use colored::*;
use crate::state::ProjectState;
use crate::utils;
use serde_json::json;

/// Execute the /plan command
pub async fn execute(_args: Vec<String>) -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  DoPlan: Project Planning".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    // Load state
    let state = ProjectState::load()
        .context("Failed to load project state. Ensure .doplan/state.json exists")?;

    // Validate state
    if state.project_name.is_none() {
        anyhow::bail!("Project state is incomplete: missing project_name. Run /discuss first.");
    }

    // Check if PRD exists
    let doplan_dir = utils::doplan_dir()
        .context("Failed to get doplan directory")?;
    let prd_path = doplan_dir.join("PRD.md");
    
    if !prd_path.exists() {
        println!("{}", "PRD.md not found. Run /generate first to create foundational documents.".bright_yellow());
        return Ok(());
    }

    // Read PRD
    let prd_content = std::fs::read_to_string(&prd_path)
        .with_context(|| format!("Failed to read PRD from: {}", prd_path.display()))?;
    
    // Validate PRD content
    utils::validate_content(&prd_content, 50)
        .context("PRD content is too short or invalid")?;

    // Check if phases exist in state
    if state.phases.is_none() || state.phases.as_ref().unwrap().is_empty() {
        println!("{}", "No phases found in state. Run /discuss first to define phases.".bright_yellow());
        return Ok(());
    }

    let phases = state.phases.as_ref().unwrap();
    let features = state.features.as_ref();

    // Create plan directory structure
    let plan_dir = doplan_dir.join("plan");
    utils::ensure_dir(&plan_dir)
        .context("Failed to create plan directory")?;

    println!("{}", "Creating phase and feature structure...".bright_yellow());
    println!();

    // Create phases
    for (phase_idx, phase) in phases.iter().enumerate() {
        let phase_num = format!("{:02}-{}", phase_idx + 1, sanitize_name(&phase.name));
        let phase_dir = plan_dir.join(&phase_num);
        utils::ensure_dir(&phase_dir)
            .with_context(|| format!("Failed to create phase directory: {}", phase_num))?;

        println!("  {} Creating phase: {}", "→".bright_cyan(), phase.name);
        
        // Generate phase-plan.md
        generate_phase_plan(&phase_dir, phase, &prd_content, features)
            .context("Failed to generate phase plan")?;

        // Generate phase-progress.json
        generate_phase_progress(&phase_dir, phase)
            .context("Failed to generate phase progress")?;

        // Create features for this phase
        if let Some(features_list) = features {
            for (feature_idx, feature_name) in phase.features.iter().enumerate() {
                // Find the feature in the features list
                if let Some(feature) = features_list.iter().find(|f| f.name == *feature_name) {
                    let feature_num = format!("{:02}-{}", feature_idx + 1, sanitize_name(feature_name));
                    let feature_dir = phase_dir.join(&feature_num);
                    utils::ensure_dir(&feature_dir)
                        .with_context(|| format!("Failed to create feature directory: {}", feature_num))?;

                    println!("    {} Creating feature: {}", "→".bright_cyan(), feature_name);

                    // Generate feature plan.md
                    generate_feature_plan(&feature_dir, feature, &prd_content)
                        .context("Failed to generate feature plan")?;

                    // Generate feature design.md
                    generate_feature_design(&feature_dir, feature)
                        .context("Failed to generate feature design")?;

                    // Generate feature tasks.md
                    generate_feature_tasks(&feature_dir, feature)
                        .context("Failed to generate feature tasks")?;

                    // Generate feature progress.json
                    generate_feature_progress(&feature_dir, feature)
                        .context("Failed to generate feature progress")?;
                }
            }
        }
    }

    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green());
    println!("{}", "  Planning Complete!".bright_green().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green());
    println!();
    println!("{}", format!("Created {} phase(s) with features", phases.len()).bright_white());
    println!();
    println!("{}", "Structure created:".bright_cyan());
    println!("  doplan/plan/");
    for (phase_idx, phase) in phases.iter().enumerate() {
        let phase_num = format!("{:02}-{}", phase_idx + 1, sanitize_name(&phase.name));
        println!("  ├── {}/", phase_num);
        println!("  │   ├── phase-plan.md");
        println!("  │   ├── phase-progress.json");
        if !phase.features.is_empty() {
            for (feature_idx, feature_name) in phase.features.iter().enumerate() {
                let feature_num = format!("{:02}-{}", feature_idx + 1, sanitize_name(feature_name));
                println!("  │   ├── {}/", feature_num);
                println!("  │   │   ├── plan.md");
                println!("  │   │   ├── design.md");
                println!("  │   │   ├── tasks.md");
                println!("  │   │   └── progress.json");
            }
        }
    }
    println!();
    println!("{}", "Next steps:".bright_yellow());
    println!("  1. Review the generated plans");
    println!("  2. Run /generate again to create Phase 2 documents (DPR, SOPS, etc.)");
    println!();

    Ok(())
}

fn sanitize_name(name: &str) -> String {
    name.to_lowercase()
        .replace(" ", "-")
        .replace("_", "-")
        .replace(".", "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn generate_phase_plan(
    phase_dir: &PathBuf,
    phase: &crate::state::Phase,
    _prd_content: &str,
    _features: Option<&Vec<crate::state::Feature>>,
) -> Result<PathBuf> {
    let plan_path = phase_dir.join("phase-plan.md");

    let mut content = String::new();
    content.push_str("# Phase Plan\n\n");
    content.push_str(&format!("**Phase:** {}\n\n", phase.name));
    content.push_str(&format!("**Date:** {}\n\n", chrono::Utc::now().format("%Y-%m-%d")));
    content.push_str("---\n\n");

    content.push_str("## Overview\n\n");
    content.push_str(&format!("{}\n\n", phase.description));

    content.push_str("## Features\n\n");
    if !phase.features.is_empty() {
        for feature_name in &phase.features {
            content.push_str(&format!("- {}\n", feature_name));
        }
        content.push_str("\n");
    } else {
        content.push_str("_No features assigned to this phase_\n\n");
    }

    content.push_str("## Objectives\n\n");
    content.push_str("- Complete all assigned features\n");
    content.push_str("- Ensure code quality and testing\n");
    content.push_str("- Document implementation\n\n");

    content.push_str("## Timeline\n\n");
    content.push_str("_Timeline to be defined_\n\n");

    content.push_str("## Dependencies\n\n");
    content.push_str("_Dependencies to be identified_\n\n");

    content.push_str("## Success Criteria\n\n");
    content.push_str("- All features implemented\n");
    content.push_str("- All tests passing\n");
    content.push_str("- Documentation complete\n\n");

    // Validate content before writing
    utils::validate_content(&content, 100)
        .context("Generated phase plan content is too short")?;

    std::fs::write(&plan_path, &content)
        .with_context(|| format!("Failed to write phase plan to: {}", plan_path.display()))?;

    // Verify file was written successfully
    utils::verify_file_write(&plan_path, 100)
        .context("Phase plan file verification failed")?;

    Ok(plan_path)
}

fn generate_phase_progress(phase_dir: &PathBuf, phase: &crate::state::Phase) -> Result<PathBuf> {
    let progress_path = phase_dir.join("phase-progress.json");

    let progress = json!({
        "phase": phase.name,
        "status": "not_started",
        "progress": 0,
        "features": {
            "total": phase.features.len(),
            "completed": 0,
            "in_progress": 0,
            "not_started": phase.features.len()
        },
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    let content = serde_json::to_string_pretty(&progress)
        .context("Failed to serialize phase progress to JSON")?;

    // Validate JSON content
    utils::validate_content(&content, 50)
        .context("Generated phase progress content is too short")?;

    std::fs::write(&progress_path, &content)
        .with_context(|| format!("Failed to write phase progress to: {}", progress_path.display()))?;

    // Verify file was written successfully
    utils::verify_file_write(&progress_path, 50)
        .context("Phase progress file verification failed")?;

    Ok(progress_path)
}

fn generate_feature_plan(
    feature_dir: &PathBuf,
    feature: &crate::state::Feature,
    _prd_content: &str,
) -> Result<PathBuf> {
    let plan_path = feature_dir.join("plan.md");

    let mut content = String::new();
    content.push_str("# Feature Plan\n\n");
    content.push_str(&format!("**Feature:** {}\n\n", feature.name));
    content.push_str(&format!("**Priority:** {}\n\n", feature.priority));
    content.push_str(&format!("**Date:** {}\n\n", chrono::Utc::now().format("%Y-%m-%d")));
    content.push_str("---\n\n");

    content.push_str("## Overview\n\n");
    content.push_str(&format!("{}\n\n", feature.description));

    content.push_str("## Goals\n\n");
    content.push_str("- Implement core functionality\n");
    content.push_str("- Ensure proper error handling\n");
    content.push_str("- Write comprehensive tests\n\n");

    content.push_str("## User Stories\n\n");
    content.push_str(&format!("- As a user, I want {} so that I can achieve my goals\n\n", feature.name.to_lowercase()));

    content.push_str("## Requirements\n\n");
    content.push_str("### Functional Requirements\n");
    content.push_str("- Core feature functionality\n");
    content.push_str("- User interface components\n");
    content.push_str("- Data validation\n\n");

    content.push_str("### Non-Functional Requirements\n");
    content.push_str("- Performance: Response time < 200ms\n");
    content.push_str("- Security: Input validation and sanitization\n");
    content.push_str("- Accessibility: WCAG 2.1 AA compliance\n\n");

    content.push_str("## Design Considerations\n\n");
    content.push_str("### Pages\n");
    content.push_str("_Pages to be defined_\n\n");

    content.push_str("### Sections\n");
    content.push_str("_Sections to be defined_\n\n");

    content.push_str("### Components\n");
    content.push_str("_Components to be defined_\n\n");

    content.push_str("### Cards/UI Elements\n");
    content.push_str("_UI elements to be defined_\n\n");

    content.push_str("## Technical Approach\n\n");
    content.push_str("_Technical approach to be defined_\n\n");

    content.push_str("## Dependencies\n\n");
    content.push_str("_Dependencies to be identified_\n\n");

    content.push_str("## Acceptance Criteria\n\n");
    content.push_str("- [ ] Feature implemented\n");
    content.push_str("- [ ] Tests written and passing\n");
    content.push_str("- [ ] Documentation complete\n");
    content.push_str("- [ ] Code reviewed\n\n");

    content.push_str("## Timeline\n\n");
    content.push_str("_Timeline to be defined_\n\n");

    // Validate content before writing
    utils::validate_content(&content, 100)
        .context("Generated feature plan content is too short")?;

    std::fs::write(&plan_path, &content)
        .with_context(|| format!("Failed to write feature plan to: {}", plan_path.display()))?;

    // Verify file was written successfully
    utils::verify_file_write(&plan_path, 100)
        .context("Feature plan file verification failed")?;

    Ok(plan_path)
}

fn generate_feature_design(
    feature_dir: &PathBuf,
    feature: &crate::state::Feature,
) -> Result<PathBuf> {
    let design_path = feature_dir.join("design.md");

    let mut content = String::new();
    content.push_str("# Design Specification\n\n");
    content.push_str(&format!("**Feature:** {}\n\n", feature.name));
    content.push_str(&format!("**Date:** {}\n\n", chrono::Utc::now().format("%Y-%m-%d")));
    content.push_str("---\n\n");

    content.push_str("## Design Overview\n\n");
    content.push_str("_Design overview to be defined_\n\n");

    content.push_str("## Visual Design\n\n");
    content.push_str("### Layout\n");
    content.push_str("_Layout structure to be defined_\n\n");

    content.push_str("### Colors\n");
    content.push_str("_Color scheme to be defined_\n\n");

    content.push_str("### Typography\n");
    content.push_str("_Typography to be defined_\n\n");

    content.push_str("## Components\n\n");
    content.push_str("_Components to be defined_\n\n");

    content.push_str("## User Interactions\n\n");
    content.push_str("_User interactions to be defined_\n\n");

    content.push_str("## Responsive Design\n\n");
    content.push_str("_Responsive design considerations_\n\n");

    content.push_str("## Accessibility\n\n");
    content.push_str("_Accessibility requirements_\n\n");

    // Validate content before writing
    utils::validate_content(&content, 100)
        .context("Generated feature design content is too short")?;

    std::fs::write(&design_path, &content)
        .with_context(|| format!("Failed to write feature design to: {}", design_path.display()))?;

    // Verify file was written successfully
    utils::verify_file_write(&design_path, 100)
        .context("Feature design file verification failed")?;

    Ok(design_path)
}

fn generate_feature_tasks(
    feature_dir: &PathBuf,
    feature: &crate::state::Feature,
) -> Result<PathBuf> {
    let tasks_path = feature_dir.join("tasks.md");

    let mut content = String::new();
    content.push_str("# Tasks\n\n");
    content.push_str(&format!("**Feature:** {}\n\n", feature.name));
    content.push_str(&format!("**Date:** {}\n\n", chrono::Utc::now().format("%Y-%m-%d")));
    content.push_str("---\n\n");

    content.push_str("## Tasks\n\n");

    content.push_str("#### Task 1: Setup\n");
    content.push_str("- **Status**: [ ] Not Started | [ ] In Progress | [ ] Completed | [ ] Blocked\n");
    content.push_str("- **Priority**: High\n");
    content.push_str("- **Description**: Setup feature structure and initial files\n");
    content.push_str("- **Acceptance Criteria**:\n");
    content.push_str("  - [ ] Feature directory created\n");
    content.push_str("  - [ ] Initial files generated\n");
    content.push_str("- **Estimated Time**: 1 hour\n");
    content.push_str("- **Notes**: _Additional notes_\n\n");

    content.push_str("#### Task 2: Implementation\n");
    content.push_str("- **Status**: [ ] Not Started | [ ] In Progress | [ ] Completed | [ ] Blocked\n");
    content.push_str("- **Priority**: High\n");
    content.push_str("- **Description**: Implement core feature functionality\n");
    content.push_str("- **Acceptance Criteria**:\n");
    content.push_str("  - [ ] Core functionality implemented\n");
    content.push_str("  - [ ] Error handling added\n");
    content.push_str("- **Estimated Time**: 4 hours\n");
    content.push_str("- **Notes**: _Additional notes_\n\n");

    content.push_str("#### Task 3: Testing\n");
    content.push_str("- **Status**: [ ] Not Started | [ ] In Progress | [ ] Completed | [ ] Blocked\n");
    content.push_str("- **Priority**: Medium\n");
    content.push_str("- **Description**: Write and run tests\n");
    content.push_str("- **Acceptance Criteria**:\n");
    content.push_str("  - [ ] Unit tests written\n");
    content.push_str("  - [ ] Integration tests written\n");
    content.push_str("  - [ ] All tests passing\n");
    content.push_str("- **Estimated Time**: 2 hours\n");
    content.push_str("- **Notes**: _Additional notes_\n\n");

    content.push_str("## Progress Tracking\n\n");
    content.push_str("**Overall Progress**: 0%\n\n");
    content.push_str("- Completed: 0\n");
    content.push_str("- In Progress: 0\n");
    content.push_str("- Not Started: 3\n");
    content.push_str("- Blocked: 0\n\n");

    content.push_str("## Dependencies\n\n");
    content.push_str("_Dependencies to be identified_\n\n");

    content.push_str("## Blockers\n\n");
    content.push_str("_No blockers_\n\n");

    // Validate content before writing
    utils::validate_content(&content, 100)
        .context("Generated feature tasks content is too short")?;

    std::fs::write(&tasks_path, &content)
        .with_context(|| format!("Failed to write feature tasks to: {}", tasks_path.display()))?;

    // Verify file was written successfully
    utils::verify_file_write(&tasks_path, 100)
        .context("Feature tasks file verification failed")?;

    Ok(tasks_path)
}

fn generate_feature_progress(
    feature_dir: &PathBuf,
    feature: &crate::state::Feature,
) -> Result<PathBuf> {
    let progress_path = feature_dir.join("progress.json");

    let progress = json!({
        "feature": feature.name,
        "priority": feature.priority,
        "status": "not_started",
        "progress": 0,
        "tasks": {
            "total": 3,
            "completed": 0,
            "in_progress": 0,
            "not_started": 3,
            "blocked": 0
        },
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    let content = serde_json::to_string_pretty(&progress)
        .context("Failed to serialize feature progress to JSON")?;

    // Validate JSON content
    utils::validate_content(&content, 50)
        .context("Generated feature progress content is too short")?;

    std::fs::write(&progress_path, &content)
        .with_context(|| format!("Failed to write feature progress to: {}", progress_path.display()))?;

    // Verify file was written successfully
    utils::verify_file_write(&progress_path, 50)
        .context("Feature progress file verification failed")?;

    Ok(progress_path)
}


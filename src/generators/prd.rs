use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::state::ProjectState;
use crate::utils;

pub fn generate(state: &ProjectState, idea_notes: &Option<String>) -> Result<PathBuf> {
    // Validate state
    if state.project_name.is_none() && state.idea.is_none() {
        anyhow::bail!("Project state is incomplete: missing both project_name and idea. Run /discuss first.");
    }

    let doplan_dir = utils::doplan_dir()
        .context("Failed to get doplan directory")?;
    utils::ensure_dir(&doplan_dir)
        .context("Failed to create doplan directory")?;

    let prd_path = doplan_dir.join("PRD.md");
    utils::validate_write_path(&prd_path)
        .context("Invalid path for PRD.md")?;

    let project_name = state.project_name.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("Untitled Project");

    let idea = state.idea.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("No idea description provided");

    let mut content = String::new();

    // Header
    content.push_str("# Product Requirements Document (PRD)\n\n");
    content.push_str(&format!("**Project:** {}\n\n", project_name));
    content.push_str(&format!("**Version:** 1.0\n"));
    content.push_str(&format!("**Date:** {}\n\n", chrono::Utc::now().format("%Y-%m-%d")));
    content.push_str("---\n\n");

    // Executive Summary
    content.push_str("## Executive Summary\n\n");
    content.push_str(&format!("{}\n\n", idea));
    content.push_str("---\n\n");

    // Problem Statement
    content.push_str("## Problem Statement\n\n");
    if let Some(notes) = idea_notes {
        // Try to extract problem from Q&A section
        if let Some(problem_start) = notes.find("What problem does this solve?") {
            if let Some(answer_start) = notes[problem_start..].find("\n\n") {
                let problem_section = &notes[problem_start + answer_start + 2..];
                if let Some(problem_end) = problem_section.find("\n\n###") {
                    let problem = problem_section[..problem_end].trim();
                    if !problem.is_empty() {
                        content.push_str(&format!("{}\n\n", problem));
                    } else {
                        content.push_str("_To be defined based on project requirements_\n\n");
                    }
                } else {
                    content.push_str("_To be defined based on project requirements_\n\n");
                }
            } else {
                content.push_str("_To be defined based on project requirements_\n\n");
            }
        } else {
            content.push_str("_To be defined based on project requirements_\n\n");
        }
    } else {
        content.push_str("_To be defined based on project requirements_\n\n");
    }

    // Target Audience
    content.push_str("## Target Audience\n\n");
    if let Some(notes) = idea_notes {
        if let Some(audience_start) = notes.find("Who is your target audience?") {
            if let Some(answer_start) = notes[audience_start..].find("\n\n") {
                let audience_section = &notes[audience_start + answer_start + 2..];
                if let Some(audience_end) = audience_section.find("\n\n###") {
                    let audience = audience_section[..audience_end].trim();
                    if !audience.is_empty() {
                        content.push_str(&format!("{}\n\n", audience));
                    } else {
                        content.push_str("_To be defined_\n\n");
                    }
                } else {
                    content.push_str("_To be defined_\n\n");
                }
            } else {
                content.push_str("_To be defined_\n\n");
            }
        } else {
            content.push_str("_To be defined_\n\n");
        }
    } else {
        content.push_str("_To be defined_\n\n");
    }

    // Features
    content.push_str("## Features\n\n");
    if let Some(features) = &state.features {
        if !features.is_empty() {
            for feature in features {
                content.push_str(&format!("### {}\n\n", feature.name));
                content.push_str(&format!("**Priority:** {}\n\n", feature.priority));
                content.push_str(&format!("{}\n\n", feature.description));
            }
        } else {
            content.push_str("_Features to be defined_\n\n");
        }
    } else {
        content.push_str("_Features to be defined_\n\n");
    }

    // Technical Requirements
    content.push_str("## Technical Requirements\n\n");
    if let Some(tech_stack) = &state.tech_stack {
        if !tech_stack.is_empty() {
            content.push_str("### Technology Stack\n\n");
            for tech in tech_stack {
                content.push_str(&format!("- {}\n", tech));
            }
            content.push_str("\n");
        }
    }
    content.push_str("### Additional Requirements\n\n");
    content.push_str("- Cross-platform compatibility\n");
    content.push_str("- Responsive design\n");
    content.push_str("- Security best practices\n");
    content.push_str("- Performance optimization\n");
    content.push_str("- Scalability considerations\n\n");

    // Success Metrics
    content.push_str("## Success Metrics\n\n");
    content.push_str("- User engagement metrics\n");
    content.push_str("- Performance benchmarks\n");
    content.push_str("- Error rates\n");
    content.push_str("- User satisfaction scores\n\n");

    // Timeline
    content.push_str("## Timeline\n\n");
    if let Some(phases) = &state.phases {
        if !phases.is_empty() {
            for (i, phase) in phases.iter().enumerate() {
                content.push_str(&format!("### Phase {}: {}\n\n", i + 1, phase.name));
                content.push_str(&format!("{}\n\n", phase.description));
            }
        }
    }
    content.push_str("_Detailed timeline to be defined_\n\n");

    // Risks and Mitigation
    content.push_str("## Risks and Mitigation\n\n");
    content.push_str("### Technical Risks\n");
    content.push_str("- Technology stack complexity\n");
    content.push_str("- Integration challenges\n");
    content.push_str("- Performance bottlenecks\n\n");
    content.push_str("### Mitigation Strategies\n");
    content.push_str("- Early prototyping\n");
    content.push_str("- Continuous testing\n");
    content.push_str("- Regular code reviews\n");
    content.push_str("- Performance monitoring\n\n");

    // Validate content before writing
    utils::validate_content(&content, 100)
        .context("Generated PRD content is too short")?;

    std::fs::write(&prd_path, &content)
        .with_context(|| format!("Failed to write PRD to: {}", prd_path.display()))?;

    // Verify file was written successfully
    utils::verify_file_write(&prd_path, 100)
        .context("PRD file verification failed")?;

    Ok(prd_path)
}


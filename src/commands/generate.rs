use anyhow::{Context, Result};
use colored::*;
use std::path::PathBuf;
use crate::state::ProjectState;
use crate::utils;
use crate::generators;

/// Execute the /generate command
pub async fn execute(_args: Vec<String>) -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  DoPlan: Document Generation".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    // Load state
    let state = ProjectState::load()
        .context("Failed to load project state")?;

    // Check if PRD exists to determine phase
    let doplan_dir = utils::doplan_dir()?;
    let prd_path = doplan_dir.join("PRD.md");
    let plan_base_dir = doplan_dir.join("plan");
    
    // Check if any phase directory exists (01-*, 02-*, etc.)
    let plan_dir_exists = if plan_base_dir.exists() {
        std::fs::read_dir(&plan_base_dir)
            .ok()
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .any(|e| {
                        let name = e.file_name();
                        let name_str = name.to_string_lossy();
                        name_str.starts_with("01-") || name_str.starts_with("02-") || name_str.starts_with("03-")
                    })
            })
            .unwrap_or(false)
    } else {
        false
    };

    if !prd_path.exists() {
        // Phase 1: Generate foundational documents
        println!("{}", "Phase 1: Generating foundational documents...".bright_yellow());
        println!();

        // Read idea notes if available
        let idea_notes = read_idea_notes().ok();

        // Generate PRD
        println!("  {} Generating PRD.md...", "→".bright_cyan());
        generators::prd::generate(&state, &idea_notes)
            .context("Failed to generate PRD")?;
        println!("  {} PRD.md generated", "✓".bright_green());

        // Generate structure.md
        println!("  {} Generating structure.md...", "→".bright_cyan());
        generators::structure::generate(&state, &idea_notes)
            .context("Failed to generate structure")?;
        println!("  {} structure.md generated", "✓".bright_green());

        // Generate API spec
        println!("  {} Generating api-spec.json...", "→".bright_cyan());
        generators::api_spec::generate(&state, &idea_notes)
            .context("Failed to generate API spec")?;
        println!("  {} api-spec.json generated", "✓".bright_green());

        // Generate data model
        println!("  {} Generating data-model.md...", "→".bright_cyan());
        generators::data_model::generate(&state, &idea_notes)
            .context("Failed to generate data model")?;
        println!("  {} data-model.md generated", "✓".bright_green());

        // Generate templates
        println!("  {} Generating templates...", "→".bright_cyan());
        generators::templates::generate_all()
            .context("Failed to generate templates")?;
        println!("  {} Templates generated", "✓".bright_green());

        println!();
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green());
        println!("{}", "  Phase 1 Complete!".bright_green().bold());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green());
        println!();
        println!("{}", "Files created:".bright_cyan());
        println!("  • doplan/PRD.md");
        println!("  • doplan/structure.md");
        println!("  • doplan/contracts/api-spec.json");
        println!("  • doplan/contracts/data-model.md");
        println!("  • doplan/templates/plan-template.md");
        println!("  • doplan/templates/design-template.md");
        println!("  • doplan/templates/tasks-template.md");
        println!();
        println!("{}", "Next steps:".bright_yellow());
        println!("  1. Review the generated documents");
        println!("  2. Run /plan to create phase and feature structure");
        println!("  3. Run /generate again to create Phase 2 documents");
        println!();

    } else if plan_dir_exists {
        // Phase 2: Generate detailed documents
        println!("{}", "Phase 2: Generating detailed documents...".bright_yellow());
        println!();

        // Generate DPR
        println!("  {} Generating DPR.md...", "→".bright_cyan());
        generators::dpr::generate(&state)
            .context("Failed to generate DPR")?;
        println!("  {} DPR.md and design files generated", "✓".bright_green());

        // Generate SOPS
        println!("  {} Generating SOPS...", "→".bright_cyan());
        generators::sops::generate(&state)
            .context("Failed to generate SOPS")?;
        println!("  {} SOPS generated", "✓".bright_green());

        // Generate RAKD
        println!("  {} Generating RAKD.md...", "→".bright_cyan());
        generators::rakd::generate(&state)
            .context("Failed to generate RAKD")?;
        println!("  {} RAKD.md generated", "✓".bright_green());

        // Generate CONTEXT.md
        println!("  {} Generating CONTEXT.md...", "→".bright_cyan());
        generators::context::generate(&state)
            .context("Failed to generate CONTEXT")?;
        println!("  {} CONTEXT.md generated", "✓".bright_green());

        // Generate README.md
        println!("  {} Generating README.md...", "→".bright_cyan());
        generators::readme::generate(&state)
            .context("Failed to generate README")?;
        println!("  {} README.md generated", "✓".bright_green());

        println!();
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green());
        println!("{}", "  Phase 2 Complete!".bright_green().bold());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green());
        println!();
        println!("{}", "Files created:".bright_cyan());
        println!("  • doplan/design/DPR.md");
        println!("  • doplan/design/design-tokens.json");
        println!("  • .doplan/ai/rules/design_rules.mdc");
        println!("  • doplan/SOPS/ (service documentation)");
        println!("  • doplan/RAKD.md");
        println!("  • CONTEXT.md");
        println!("  • README.md");
        println!();

    } else {
        println!("{}", "PRD.md already exists, but plan structure not found.".bright_yellow());
        println!("{}", "Run /plan first to create phase structure, then run /generate again.".bright_yellow());
        println!();
    }

    Ok(())
}

fn read_idea_notes() -> Result<String> {
    let doplan_dir = utils::doplan_dir()?;
    let notes_path = doplan_dir.join("idea-notes.md");
    
    if !notes_path.exists() {
        anyhow::bail!("idea-notes.md not found. Run /discuss first.");
    }

    std::fs::read_to_string(&notes_path)
        .context("Failed to read idea notes")
}


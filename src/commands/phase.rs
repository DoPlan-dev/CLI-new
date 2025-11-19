use anyhow::{Context, Result};
use colored::*;
use dialoguer::{Input, Select};
use crate::state::{ProjectState, Phase};

/// Execute phase management commands
pub async fn execute(args: Vec<String>) -> Result<()> {
    if args.is_empty() {
        show_phase_menu().await?;
        return Ok(());
    }

    match args[0].as_str() {
        "add" | "create" => add_phase().await,
        "list" | "ls" => list_phases().await,
        "reorder" => reorder_phases().await,
        "update" => update_phase(args.get(1).cloned()).await,
        "delete" | "remove" => delete_phase(args.get(1).cloned()).await,
        _ => {
            eprintln!("Unknown phase command: {}. Use: add, list, reorder, update, delete", args[0]);
            Ok(())
        }
    }
}

async fn show_phase_menu() -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  DoPlan: Phase Management".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();
    println!("{}", "Available commands:".bright_white());
    println!("  {} add      - Create a new phase", "→".bright_cyan());
    println!("  {} list     - List all phases", "→".bright_cyan());
    println!("  {} reorder  - Reorder phases", "→".bright_cyan());
    println!("  {} update   - Update a phase", "→".bright_cyan());
    println!("  {} delete   - Delete a phase", "→".bright_cyan());
    println!();
    Ok(())
}

async fn add_phase() -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  Add New Phase".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    let mut state = ProjectState::load()
        .context("Failed to load project state. Ensure .doplan/state.json exists")?;

    // Validate state
    if state.project_name.is_none() {
        anyhow::bail!("Project state is incomplete: missing project_name. Run /discuss first.");
    }

    // Get phase name
    let name: String = Input::new()
        .with_prompt("Phase name")
        .interact_text()
        .context("Failed to read phase name")?;

    // Get phase description
    let description: String = Input::new()
        .with_prompt("Phase description")
        .interact_text()
        .context("Failed to read phase description")?;

    // Get features (optional, can be empty)
    let features_input: String = Input::new()
        .with_prompt("Features (comma-separated, or leave empty)")
        .allow_empty(true)
        .interact_text()
        .context("Failed to read features")?;

    let features = if features_input.trim().is_empty() {
        Vec::new()
    } else {
        features_input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    };

    // Validate phase name
    if name.trim().is_empty() {
        anyhow::bail!("Phase name cannot be empty");
    }

    // Check for duplicate phase names
    if let Some(existing_phases) = &state.phases {
        if existing_phases.iter().any(|p| p.name == name) {
            anyhow::bail!("Phase '{}' already exists", name);
        }
    }

    let new_phase = Phase {
        name: name.clone(),
        description,
        features,
    };

    // Add phase to state
    if state.phases.is_none() {
        state.phases = Some(Vec::new());
    }
    state.phases.as_mut().unwrap().push(new_phase);

    // Save state
    state.save()
        .context("Failed to save project state")?;

    println!();
    println!("{}", format!("✓ Phase '{}' added successfully", name).bright_green());
    println!();
    println!("{}", "Next steps:".bright_yellow());
    println!("  1. Run /plan to regenerate the plan structure");
    println!("  2. Review the updated state in .doplan/state.json");
    println!();

    Ok(())
}

async fn list_phases() -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  Phase List".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    let state = ProjectState::load()
        .context("Failed to load project state. Ensure .doplan/state.json exists")?;

    if state.phases.is_none() || state.phases.as_ref().unwrap().is_empty() {
        println!("{}", "No phases found. Use 'phase add' to create a new phase.".bright_yellow());
        return Ok(());
    }

    let phases = state.phases.as_ref().unwrap();
    
    for (idx, phase) in phases.iter().enumerate() {
        println!("  {} Phase {}: {}", "→".bright_cyan(), idx + 1, phase.name.bright_white().bold());
        println!("     Description: {}", phase.description);
        if !phase.features.is_empty() {
            println!("     Features: {}", phase.features.join(", "));
        } else {
            println!("     Features: {}", "None".bright_yellow());
        }
        println!();
    }

    Ok(())
}

async fn reorder_phases() -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  Reorder Phases".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    let mut state = ProjectState::load()
        .context("Failed to load project state. Ensure .doplan/state.json exists")?;

    if state.phases.is_none() || state.phases.as_ref().unwrap().is_empty() {
        println!("{}", "No phases found. Use 'phase add' to create phases first.".bright_yellow());
        return Ok(());
    }

    let phases = state.phases.as_mut().unwrap();
    
    if phases.len() < 2 {
        println!("{}", "Need at least 2 phases to reorder.".bright_yellow());
        return Ok(());
    }

    // Show current order
    println!("{}", "Current phase order:".bright_white());
    for (idx, phase) in phases.iter().enumerate() {
        println!("  {} {}", idx + 1, phase.name);
    }
    println!();

    // Select phase to move
    let phase_names: Vec<String> = phases.iter().map(|p| p.name.clone()).collect();
    let selection = Select::new()
        .with_prompt("Select phase to move")
        .items(&phase_names)
        .interact()
        .context("Failed to select phase")?;

    // Select new position
    let new_pos = Select::new()
        .with_prompt("Select new position")
        .items(&phase_names)
        .default(selection)
        .interact()
        .context("Failed to select position")?;

    if selection == new_pos {
        println!("{}", "Phase is already at that position.".bright_yellow());
        return Ok(());
    }

    // Reorder
    let phase = phases.remove(selection);
    phases.insert(new_pos, phase);

    // Save state
    state.save()
        .context("Failed to save project state")?;

    println!();
    println!("{}", "✓ Phases reordered successfully".bright_green());
    println!();
    println!("{}", "Next steps:".bright_yellow());
    println!("  1. Run /plan to regenerate the plan structure with new order");
    println!();

    Ok(())
}

async fn update_phase(phase_name: Option<String>) -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  Update Phase".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    let mut state = ProjectState::load()
        .context("Failed to load project state. Ensure .doplan/state.json exists")?;

    if state.phases.is_none() || state.phases.as_ref().unwrap().is_empty() {
        println!("{}", "No phases found.".bright_yellow());
        return Ok(());
    }

    // Select phase if not provided
    let phase_idx = if let Some(name) = phase_name {
        state.phases.as_ref().unwrap().iter().position(|p| p.name == name)
            .ok_or_else(|| anyhow::anyhow!("Phase '{}' not found", name))?
    } else {
        let phase_names: Vec<String> = state.phases.as_ref().unwrap().iter().map(|p| p.name.clone()).collect();
        Select::new()
            .with_prompt("Select phase to update")
            .items(&phase_names)
            .interact()
            .context("Failed to select phase")?
    };

    let old_name = state.phases.as_ref().unwrap()[phase_idx].name.clone();
    
    println!("{}", format!("Updating phase: {}", old_name).bright_white());
    println!();

    // Update name
    let new_name: String = Input::new()
        .with_prompt("Phase name")
        .with_initial_text(&old_name)
        .interact_text()
        .context("Failed to read phase name")?;

    // Update description
    let old_description = state.phases.as_ref().unwrap()[phase_idx].description.clone();
    let new_description: String = Input::new()
        .with_prompt("Phase description")
        .with_initial_text(&old_description)
        .interact_text()
        .context("Failed to read phase description")?;

    // Validate new name
    if new_name.trim().is_empty() {
        anyhow::bail!("Phase name cannot be empty");
    }

    // Check for duplicate names (excluding current phase)
    if new_name != old_name {
        let existing_phases = state.phases.as_ref().unwrap();
        if existing_phases.iter().any(|p| p.name == new_name) {
            anyhow::bail!("Phase '{}' already exists", new_name);
        }
    }

    // Update phase
    let phases = state.phases.as_mut().unwrap();
    phases[phase_idx].name = new_name;
    phases[phase_idx].description = new_description;

    // Save state
    state.save()
        .context("Failed to save project state")?;

    println!();
    println!("{}", "✓ Phase updated successfully".bright_green());
    println!();

    Ok(())
}

async fn delete_phase(phase_name: Option<String>) -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  Delete Phase".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    let mut state = ProjectState::load()
        .context("Failed to load project state. Ensure .doplan/state.json exists")?;

    if state.phases.is_none() || state.phases.as_ref().unwrap().is_empty() {
        println!("{}", "No phases found.".bright_yellow());
        return Ok(());
    }

    let phases = state.phases.as_mut().unwrap();
    
    // Select phase if not provided
    let phase_idx = if let Some(name) = phase_name {
        phases.iter().position(|p| p.name == name)
            .ok_or_else(|| anyhow::anyhow!("Phase '{}' not found", name))?
    } else {
        let phase_names: Vec<String> = phases.iter().map(|p| p.name.clone()).collect();
        Select::new()
            .with_prompt("Select phase to delete")
            .items(&phase_names)
            .interact()
            .context("Failed to select phase")?
    };

    let phase = phases.remove(phase_idx);
    
    println!();
    println!("{}", format!("⚠ Deleted phase: {}", phase.name).bright_yellow());
    println!();

    // Save state
    state.save()
        .context("Failed to save project state")?;

    println!("{}", "✓ Phase deleted successfully".bright_green());
    println!();
    println!("{}", "Note: Run /plan to update the plan structure.".bright_yellow());
    println!();

    Ok(())
}


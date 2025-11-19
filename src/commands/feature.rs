use anyhow::{Context, Result};
use colored::*;
use dialoguer::{Input, Select};
use crate::state::{ProjectState, Feature};
use std::path::PathBuf;

/// Execute feature management commands
pub async fn execute(args: Vec<String>) -> Result<()> {
    if args.is_empty() {
        show_feature_menu().await?;
        return Ok(());
    }

    match args[0].as_str() {
        "add" | "create" => add_feature().await,
        "list" | "ls" => list_features().await,
        "update" => update_feature(args.get(1).cloned()).await,
        "delete" | "remove" => delete_feature(args.get(1).cloned()).await,
        "show" | "view" => show_feature(args.get(1).cloned()).await,
        _ => {
            eprintln!("Unknown feature command: {}. Use: add, list, update, delete, show", args[0]);
            Ok(())
        }
    }
}

async fn show_feature_menu() -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  DoPlan: Feature Management".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();
    println!("{}", "Available commands:".bright_white());
    println!("  {} add      - Create a new feature", "→".bright_cyan());
    println!("  {} list     - List all features", "→".bright_cyan());
    println!("  {} show     - Show feature details", "→".bright_cyan());
    println!("  {} update   - Update a feature", "→".bright_cyan());
    println!("  {} delete   - Delete a feature", "→".bright_cyan());
    println!();
    Ok(())
}

async fn add_feature() -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  Add New Feature".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    let mut state = ProjectState::load()
        .context("Failed to load project state. Ensure .doplan/state.json exists")?;

    // Validate state
    if state.project_name.is_none() {
        anyhow::bail!("Project state is incomplete: missing project_name. Run /discuss first.");
    }

    // Get feature name
    let name: String = Input::new()
        .with_prompt("Feature name")
        .interact_text()
        .context("Failed to read feature name")?;

    // Get feature description
    let description: String = Input::new()
        .with_prompt("Feature description")
        .interact_text()
        .context("Failed to read feature description")?;

    // Get priority
    let priority_options = vec!["high", "medium", "low"];
    let priority_idx = Select::new()
        .with_prompt("Priority")
        .items(&priority_options)
        .default(1)
        .interact()
        .context("Failed to select priority")?;
    let priority = priority_options[priority_idx].to_string();

    // Validate feature name
    if name.trim().is_empty() {
        anyhow::bail!("Feature name cannot be empty");
    }

    // Check for duplicate feature names
    if let Some(existing_features) = &state.features {
        if existing_features.iter().any(|f| f.name == name) {
            anyhow::bail!("Feature '{}' already exists", name);
        }
    }

    let new_feature = Feature {
        name: name.clone(),
        description,
        priority,
    };

    // Add feature to state
    if state.features.is_none() {
        state.features = Some(Vec::new());
    }
    state.features.as_mut().unwrap().push(new_feature);

    // Save state
    state.save()
        .context("Failed to save project state")?;

    println!();
    println!("{}", format!("✓ Feature '{}' added successfully", name).bright_green());
    println!();
    println!("{}", "Next steps:".bright_yellow());
    println!("  1. Run /plan to regenerate the plan structure");
    println!("  2. Review the updated state in .doplan/state.json");
    println!();

    Ok(())
}

async fn list_features() -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  Feature List".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    let state = ProjectState::load()
        .context("Failed to load project state. Ensure .doplan/state.json exists")?;

    if state.features.is_none() || state.features.as_ref().unwrap().is_empty() {
        println!("{}", "No features found. Use 'feature add' to create a new feature.".bright_yellow());
        return Ok(());
    }

    let features = state.features.as_ref().unwrap();
    
    for (idx, feature) in features.iter().enumerate() {
        let priority_color = match feature.priority.as_str() {
            "high" => "high".bright_red(),
            "medium" => "medium".bright_yellow(),
            "low" => "low".bright_blue(),
            _ => feature.priority.as_str().bright_white(),
        };
        
        println!("  {} Feature {}: {}", "→".bright_cyan(), idx + 1, feature.name.bright_white().bold());
        println!("     Description: {}", feature.description);
        println!("     Priority: {}", priority_color);
        println!();
    }

    Ok(())
}

async fn show_feature(feature_name: Option<String>) -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  Feature Details".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    let state = ProjectState::load()
        .context("Failed to load project state. Ensure .doplan/state.json exists")?;

    if state.features.is_none() || state.features.as_ref().unwrap().is_empty() {
        println!("{}", "No features found.".bright_yellow());
        return Ok(());
    }

    let features = state.features.as_ref().unwrap();
    
    // Select feature if not provided
    let feature = if let Some(name) = feature_name {
        features.iter().find(|f| f.name == name)
            .ok_or_else(|| anyhow::anyhow!("Feature '{}' not found", name))?
    } else {
        let feature_names: Vec<String> = features.iter().map(|f| f.name.clone()).collect();
        let selection = Select::new()
            .with_prompt("Select feature to view")
            .items(&feature_names)
            .interact()
            .context("Failed to select feature")?;
        &features[selection]
    };

    let priority_color = match feature.priority.as_str() {
        "high" => "high".bright_red(),
        "medium" => "medium".bright_yellow(),
        "low" => "low".bright_blue(),
        _ => feature.priority.as_str().bright_white(),
    };

    println!("{}", format!("Feature: {}", feature.name).bright_white().bold());
    println!();
    println!("  Description: {}", feature.description);
    println!("  Priority: {}", priority_color);
    println!();

    // Check which phases include this feature
    if let Some(phases) = &state.phases {
        let phases_with_feature: Vec<&str> = phases
            .iter()
            .filter(|phase| phase.features.contains(&feature.name))
            .map(|phase| phase.name.as_str())
            .collect();
        
        if !phases_with_feature.is_empty() {
            println!("  Included in phases:");
            for phase_name in phases_with_feature {
                println!("    {} {}", "→".bright_cyan(), phase_name);
            }
            println!();
        }
    }

    Ok(())
}

async fn update_feature(feature_name: Option<String>) -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  Update Feature".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    let mut state = ProjectState::load()
        .context("Failed to load project state. Ensure .doplan/state.json exists")?;

    if state.features.is_none() || state.features.as_ref().unwrap().is_empty() {
        println!("{}", "No features found.".bright_yellow());
        return Ok(());
    }

    // Select feature if not provided
    let feature_idx = if let Some(name) = feature_name {
        state.features.as_ref().unwrap().iter().position(|f| f.name == name)
            .ok_or_else(|| anyhow::anyhow!("Feature '{}' not found", name))?
    } else {
        let feature_names: Vec<String> = state.features.as_ref().unwrap().iter().map(|f| f.name.clone()).collect();
        Select::new()
            .with_prompt("Select feature to update")
            .items(&feature_names)
            .interact()
            .context("Failed to select feature")?
    };

    let old_name = state.features.as_ref().unwrap()[feature_idx].name.clone();
    
    println!("{}", format!("Updating feature: {}", old_name).bright_white());
    println!();

    // Update name
    let new_name: String = Input::new()
        .with_prompt("Feature name")
        .with_initial_text(&old_name)
        .interact_text()
        .context("Failed to read feature name")?;

    // Update description
    let old_description = state.features.as_ref().unwrap()[feature_idx].description.clone();
    let new_description: String = Input::new()
        .with_prompt("Feature description")
        .with_initial_text(&old_description)
        .interact_text()
        .context("Failed to read feature description")?;

    // Update priority
    let old_priority = state.features.as_ref().unwrap()[feature_idx].priority.clone();
    let priority_options = vec!["high", "medium", "low"];
    let current_priority_idx = priority_options.iter()
        .position(|p| *p == old_priority.as_str())
        .unwrap_or(1);
    let priority_idx = Select::new()
        .with_prompt("Priority")
        .items(&priority_options)
        .default(current_priority_idx)
        .interact()
        .context("Failed to select priority")?;
    let new_priority = priority_options[priority_idx].to_string();

    // Validate new name
    if new_name.trim().is_empty() {
        anyhow::bail!("Feature name cannot be empty");
    }

    // Check for duplicate names (excluding current feature)
    if new_name != old_name {
        let existing_features = state.features.as_ref().unwrap();
        if existing_features.iter().any(|f| f.name == new_name) {
            anyhow::bail!("Feature '{}' already exists", new_name);
        }
    }

    // Update feature
    let features = state.features.as_mut().unwrap();
    features[feature_idx].name = new_name;
    features[feature_idx].description = new_description;
    features[feature_idx].priority = new_priority;

    // Save state
    state.save()
        .context("Failed to save project state")?;

    println!();
    println!("{}", "✓ Feature updated successfully".bright_green());
    println!();

    Ok(())
}

async fn delete_feature(feature_name: Option<String>) -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  Delete Feature".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    let mut state = ProjectState::load()
        .context("Failed to load project state. Ensure .doplan/state.json exists")?;

    if state.features.is_none() || state.features.as_ref().unwrap().is_empty() {
        println!("{}", "No features found.".bright_yellow());
        return Ok(());
    }

    let features = state.features.as_mut().unwrap();
    
    // Select feature if not provided
    let feature_idx = if let Some(name) = feature_name {
        features.iter().position(|f| f.name == name)
            .ok_or_else(|| anyhow::anyhow!("Feature '{}' not found", name))?
    } else {
        let feature_names: Vec<String> = features.iter().map(|f| f.name.clone()).collect();
        Select::new()
            .with_prompt("Select feature to delete")
            .items(&feature_names)
            .interact()
            .context("Failed to select feature")?
    };

    let feature = features.remove(feature_idx);
    
    // Also remove from phases if present
    if let Some(phases) = &mut state.phases {
        for phase in phases.iter_mut() {
            phase.features.retain(|f| f != &feature.name);
        }
    }
    
    println!();
    println!("{}", format!("⚠ Deleted feature: {}", feature.name).bright_yellow());
    println!();

    // Save state
    state.save()
        .context("Failed to save project state")?;

    println!("{}", "✓ Feature deleted successfully".bright_green());
    println!();
    println!("{}", "Note: Run /plan to update the plan structure.".bright_yellow());
    println!();

    Ok(())
}


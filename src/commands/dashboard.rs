use anyhow::{Context, Result};
use colored::*;
use serde_json::Value;
use crate::utils;
use std::fs;

/// Execute the dashboard command
pub async fn execute(_args: Vec<String>) -> Result<()> {
    // Ensure dashboard is up to date by checking if it exists
    let dot_doplan = utils::dot_doplan_dir()?;
    let dashboard_json_path = dot_doplan.join("dashboard.json");
    
    // If dashboard doesn't exist, suggest running /progress first
    if !dashboard_json_path.exists() {
        println!("{}", "Dashboard not found. Run '/progress' first to generate it.".bright_yellow());
        println!();
        return Ok(());
    }

    // Read dashboard.json
    let dashboard_content = fs::read_to_string(&dashboard_json_path)
        .context("Failed to read dashboard.json")?;
    
    let dashboard: Value = serde_json::from_str(&dashboard_content)
        .context("Failed to parse dashboard.json")?;

    // Display dashboard
    display_dashboard(&dashboard)?;

    Ok(())
}

fn display_dashboard(dashboard: &Value) -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  DoPlan: Project Dashboard".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    // Project name
    if let Some(project_name) = dashboard.get("project_name").and_then(|v| v.as_str()) {
        println!("{}", format!("Project: {}", project_name).bright_white().bold());
    }

    // Last updated
    if let Some(updated_at) = dashboard.get("updated_at").and_then(|v| v.as_str()) {
        println!("{}", format!("Last Updated: {}", updated_at).bright_white());
    }
    println!();

    // Overall progress
    if let Some(overall_progress) = dashboard.get("overall_progress").and_then(|v| v.as_f64()) {
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        println!("{}", "  Overall Progress".bright_cyan().bold());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        println!();
        println!("{}", format!("{:.1}%", overall_progress).bright_green().bold());
        println!("{}", generate_progress_bar(overall_progress));
        println!();
    }

    // Phase progress
    if let Some(phases) = dashboard.get("phases").and_then(|v| v.as_array()) {
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        println!("{}", "  Phase Progress".bright_cyan().bold());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        println!();

        for phase in phases {
            if let Some(phase_name) = phase.get("name").and_then(|v| v.as_str()) {
                let progress = phase.get("progress").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let status = phase.get("status").and_then(|v| v.as_str()).unwrap_or("unknown");
                
                println!("{}", format!("Phase: {}", phase_name).bright_white().bold());
                println!("  Progress: {:.1}%", progress);
                println!("  Status: {}", format_status(status));
                println!("  {}", generate_progress_bar(progress));
                println!();

                // Features
                if let Some(features) = phase.get("features").and_then(|v| v.as_array()) {
                    if !features.is_empty() {
                        println!("  Features:");
                        for feature in features {
                            if let Some(feature_name) = feature.get("name").and_then(|v| v.as_str()) {
                                let feature_progress = feature.get("progress").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                let feature_status = feature.get("status").and_then(|v| v.as_str()).unwrap_or("unknown");
                                let priority = feature.get("priority").and_then(|v| v.as_str()).unwrap_or("unknown");
                                
                                let priority_color = match priority {
                                    "high" => priority.bright_red(),
                                    "medium" => priority.bright_yellow(),
                                    "low" => priority.bright_blue(),
                                    _ => priority.bright_white(),
                                };

                                println!("    {} {} ({}) - {:.1}% - {}", 
                                    "→".bright_cyan(),
                                    feature_name.bright_white(),
                                    priority_color,
                                    feature_progress,
                                    format_status(feature_status)
                                );
                            }
                        }
                        println!();
                    }
                }
            }
        }
    }

    // Task summary
    if let Some(phases) = dashboard.get("phases").and_then(|v| v.as_array()) {
        let mut total_tasks = 0;
        let mut completed_tasks = 0;
        let mut in_progress_tasks = 0;
        let mut not_started_tasks = 0;
        let mut blocked_tasks = 0;

        for phase in phases {
            if let Some(features) = phase.get("features").and_then(|v| v.as_array()) {
                for feature in features {
                    if let Some(tasks) = feature.get("tasks") {
                        total_tasks += tasks.get("total").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                        completed_tasks += tasks.get("completed").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                        in_progress_tasks += tasks.get("in_progress").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                        not_started_tasks += tasks.get("not_started").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                        blocked_tasks += tasks.get("blocked").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                    }
                }
            }
        }

        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        println!("{}", "  Task Summary".bright_cyan().bold());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        println!();
        println!("  Total Tasks: {}", total_tasks.to_string().bright_white());
        println!("  {} Completed: {}", "✓".bright_green(), completed_tasks.to_string().bright_green());
        println!("  {} In Progress: {}", "→".bright_yellow(), in_progress_tasks.to_string().bright_yellow());
        println!("  {} Not Started: {}", "○".bright_white(), not_started_tasks.to_string().bright_white());
        if blocked_tasks > 0 {
            println!("  {} Blocked: {}", "⚠".bright_red(), blocked_tasks.to_string().bright_red());
        }
        println!();
    }

    Ok(())
}

fn generate_progress_bar(progress: f64) -> String {
    let width = 30;
    let filled = (progress / 100.0 * width as f64) as usize;
    let empty = width - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

fn format_status(status: &str) -> ColoredString {
    match status {
        "completed" => status.bright_green(),
        "in_progress" => status.bright_yellow(),
        "not_started" => status.bright_white(),
        "blocked" => status.bright_red(),
        _ => status.bright_white(),
    }
}

use anyhow::{Context, Result};
use colored::*;
use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;
use walkdir::WalkDir;
use serde_json::{json, Value};
use crate::state::ProjectState;
use crate::utils;

#[derive(Debug, Clone)]
struct TaskStats {
    total: usize,
    completed: usize,
    in_progress: usize,
    not_started: usize,
    blocked: usize,
}

#[derive(Debug, Clone)]
struct FeatureProgress {
    feature_name: String,
    phase_name: String,
    priority: String,
    progress: f64,
    status: String,
    tasks: TaskStats,
}

#[derive(Debug, Clone)]
struct PhaseProgress {
    phase_name: String,
    progress: f64,
    features: Vec<FeatureProgress>,
}

/// Execute the /progress command
pub async fn execute(_args: Vec<String>) -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  DoPlan: Progress Update".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    // Load state
    let state = ProjectState::load()
        .context("Failed to load project state")?;

    let doplan_dir = utils::doplan_dir()?;
    let plan_dir = doplan_dir.join("plan");

    if !plan_dir.exists() {
        println!("{}", "No plan structure found. Run /plan first.".bright_yellow());
        return Ok(());
    }

    println!("{}", "Scanning features and calculating progress...".bright_yellow());
    println!();

    // Scan all features and calculate progress
    let mut phase_progress_map: HashMap<String, Vec<FeatureProgress>> = HashMap::new();

    for entry in WalkDir::new(&plan_dir)
        .min_depth(2)
        .max_depth(4)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_name() == "tasks.md" {
            let tasks_path = entry.path();
            let feature_dir = tasks_path.parent().unwrap();

            // Extract phase and feature names
            let relative_path = feature_dir
                .strip_prefix(&plan_dir)
                .unwrap()
                .to_string_lossy()
                .to_string();
            let path_parts: Vec<&str> = relative_path.split('/').collect();

            if path_parts.len() >= 2 {
                let phase_name = extract_name_from_path(path_parts[0]);
                let feature_name = extract_name_from_path(path_parts[1]);

                // Read tasks.md and calculate progress
                if let Ok(content) = fs::read_to_string(tasks_path) {
                    let task_stats = count_tasks(&content);
                    let progress = if task_stats.total > 0 {
                        (task_stats.completed as f64 / task_stats.total as f64) * 100.0
                    } else {
                        0.0
                    };

                    // Determine status
                    let status = if progress == 100.0 {
                        "completed"
                    } else if task_stats.in_progress > 0 {
                        "in_progress"
                    } else if task_stats.blocked > 0 {
                        "blocked"
                    } else {
                        "not_started"
                    };

                    // Read priority from progress.json if exists
                    let progress_path = feature_dir.join("progress.json");
                    let priority = if progress_path.exists() {
                        read_priority(&progress_path).unwrap_or_else(|| "medium".to_string())
                    } else {
                        "medium".to_string()
                    };

                    let feature_progress = FeatureProgress {
                        feature_name: feature_name.clone(),
                        phase_name: phase_name.clone(),
                        priority,
                        progress,
                        status: status.to_string(),
                        tasks: task_stats,
                    };

                    phase_progress_map
                        .entry(phase_name)
                        .or_insert_with(Vec::new)
                        .push(feature_progress);
                }
            }
        }
    }

    // Update feature progress.json files
    println!("{}", "Updating feature progress files...".bright_cyan());
    for (phase_name, features) in &phase_progress_map {
        // Find phase directory by scanning
        let phase_dirs: Vec<_> = fs::read_dir(&plan_dir)
            .context("Failed to read plan directory")?
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                extract_name_from_path(&name) == *phase_name
            })
            .collect();

        if let Some(phase_entry) = phase_dirs.first() {
            let phase_dir = phase_entry.path();
            
            for feature in features {
                // Find feature directory by scanning
                let feature_dirs: Vec<_> = fs::read_dir(&phase_dir)
                    .ok()
                    .map(|entries| {
                        entries
                            .filter_map(|e| e.ok())
                            .filter(|e| {
                                let name = e.file_name().to_string_lossy().to_string();
                                extract_name_from_path(&name) == feature.feature_name
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                if let Some(feature_entry) = feature_dirs.first() {
                    let feature_dir = feature_entry.path();
                    update_feature_progress(&feature_dir, feature)?;
                    println!("  {} Updated: {}/{}", "→".bright_cyan(), phase_name, feature.feature_name);
                }
            }
        }
    }

    // Calculate phase progress and update phase-progress.json
    println!();
    println!("{}", "Updating phase progress files...".bright_cyan());
    let mut phases: Vec<PhaseProgress> = Vec::new();
    
    for (phase_name, features) in &phase_progress_map {
        let phase_progress = if !features.is_empty() {
            features.iter().map(|f| f.progress).sum::<f64>() / features.len() as f64
        } else {
            0.0
        };

        // Find phase directory by scanning
        let phase_dirs: Vec<_> = fs::read_dir(&plan_dir)
            .context("Failed to read plan directory")?
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                extract_name_from_path(&name) == *phase_name
            })
            .collect();

        if let Some(phase_entry) = phase_dirs.first() {
            let phase_dir = phase_entry.path();
            update_phase_progress(&phase_dir, phase_name, &features, phase_progress)?;
            println!("  {} Updated: {}", "→".bright_cyan(), phase_name);
        }

        phases.push(PhaseProgress {
            phase_name: phase_name.clone(),
            progress: phase_progress,
            features: features.clone(),
        });
    }

    // Generate dashboard
    println!();
    println!("{}", "Generating dashboard...".bright_cyan());
    generate_dashboard(&state, &phases)?;
    println!("  {} Dashboard generated", "→".bright_green());

    // Calculate overall progress
    let overall_progress = if !phases.is_empty() {
        phases.iter().map(|p| p.progress).sum::<f64>() / phases.len() as f64
    } else {
        0.0
    };

    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green());
    println!("{}", "  Progress Update Complete!".bright_green().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green());
    println!();
    println!("{}", format!("Overall Progress: {:.1}%", overall_progress).bright_white().bold());
    println!();
    println!("{}", "Phase Progress:".bright_cyan());
    for phase in &phases {
        let bar = generate_progress_bar(phase.progress);
        println!("  {} {}: {:.1}% {}", 
            "→".bright_cyan(),
            phase.phase_name,
            phase.progress,
            bar
        );
    }
    println!();
    println!("{}", "Files updated:".bright_cyan());
    println!("  • Feature progress.json files");
    println!("  • Phase progress.json files");
    println!("  • .doplan/dashboard.json");
    println!("  • doplan/dashboard.md");
    println!();

    Ok(())
}

fn count_tasks(content: &str) -> TaskStats {
    let mut total = 0;
    let mut completed = 0;
    let mut in_progress = 0;
    let mut not_started = 0;
    let mut blocked = 0;

    let lines: Vec<&str> = content.lines().collect();
    let mut in_task = false;
    let mut current_status = "not_started";

    for line in lines {
        if line.starts_with("#### Task") {
            // Count previous task
            if in_task {
                total += 1;
                match current_status {
                    "completed" => completed += 1,
                    "in_progress" => in_progress += 1,
                    "blocked" => blocked += 1,
                    _ => not_started += 1,
                }
            }
            in_task = true;
            current_status = "not_started";
        } else if in_task && line.contains("**Status**") {
            if line.contains("[x] Completed") || line.contains("[X] Completed") {
                current_status = "completed";
            } else if line.contains("[x] In Progress") || line.contains("[X] In Progress") {
                current_status = "in_progress";
            } else if line.contains("[x] Blocked") || line.contains("[X] Blocked") {
                current_status = "blocked";
            }
        }
    }

    // Count last task
    if in_task {
        total += 1;
        match current_status {
            "completed" => completed += 1,
            "in_progress" => in_progress += 1,
            "blocked" => blocked += 1,
            _ => not_started += 1,
        }
    }

    TaskStats {
        total,
        completed,
        in_progress,
        not_started,
        blocked,
    }
}

fn read_priority(path: &PathBuf) -> Option<String> {
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(progress) = serde_json::from_str::<Value>(&content) {
            return progress.get("priority")
                .and_then(|p| p.as_str())
                .map(|s| s.to_string());
        }
    }
    None
}

fn update_feature_progress(feature_dir: &PathBuf, feature: &FeatureProgress) -> Result<()> {
    utils::ensure_dir(feature_dir)?;
    let progress_path = feature_dir.join("progress.json");
    
    let progress = json!({
        "feature": feature.feature_name,
        "priority": feature.priority,
        "status": feature.status,
        "progress": feature.progress,
        "tasks": {
            "total": feature.tasks.total,
            "completed": feature.tasks.completed,
            "in_progress": feature.tasks.in_progress,
            "not_started": feature.tasks.not_started,
            "blocked": feature.tasks.blocked
        },
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    let content = serde_json::to_string_pretty(&progress)
        .context("Failed to serialize feature progress")?;

    fs::write(&progress_path, content)
        .context("Failed to write feature progress")?;

    Ok(())
}

fn update_phase_progress(
    phase_dir: &PathBuf,
    phase_name: &str,
    features: &[FeatureProgress],
    progress: f64,
) -> Result<()> {
    let progress_path = phase_dir.join("phase-progress.json");

    let completed = features.iter().filter(|f| f.status == "completed").count();
    let in_progress = features.iter().filter(|f| f.status == "in_progress").count();
    let not_started = features.iter().filter(|f| f.status == "not_started").count();

    let status = if progress == 100.0 {
        "completed"
    } else if in_progress > 0 {
        "in_progress"
    } else {
        "not_started"
    };

    let progress_data = json!({
        "phase": phase_name,
        "status": status,
        "progress": progress,
        "features": {
            "total": features.len(),
            "completed": completed,
            "in_progress": in_progress,
            "not_started": not_started
        },
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    let content = serde_json::to_string_pretty(&progress_data)
        .context("Failed to serialize phase progress")?;

    fs::write(&progress_path, content)
        .context("Failed to write phase progress")?;

    Ok(())
}

fn generate_dashboard(state: &ProjectState, phases: &[PhaseProgress]) -> Result<()> {
    let doplan_dir = utils::doplan_dir()?;
    let dot_doplan = utils::dot_doplan_dir()?;
    
    // Calculate overall progress
    let overall_progress = if !phases.is_empty() {
        phases.iter().map(|p| p.progress).sum::<f64>() / phases.len() as f64
    } else {
        0.0
    };

    // Generate dashboard.json
    let dashboard_json_path = dot_doplan.join("dashboard.json");
    let dashboard_json = json!({
        "project_name": state.project_name.as_ref().unwrap_or(&"Untitled Project".to_string()),
        "overall_progress": overall_progress,
        "phases": phases.iter().map(|p| {
            json!({
                "name": p.phase_name,
                "progress": p.progress,
                "status": if p.progress == 100.0 { "completed" } else if p.progress > 0.0 { "in_progress" } else { "not_started" },
                "features": p.features.iter().map(|f| {
                    json!({
                        "name": f.feature_name,
                        "priority": f.priority,
                        "progress": f.progress,
                        "status": f.status,
                        "tasks": {
                            "total": f.tasks.total,
                            "completed": f.tasks.completed,
                            "in_progress": f.tasks.in_progress,
                            "not_started": f.tasks.not_started,
                            "blocked": f.tasks.blocked
                        }
                    })
                }).collect::<Vec<_>>()
            })
        }).collect::<Vec<_>>(),
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    let json_content = serde_json::to_string_pretty(&dashboard_json)
        .context("Failed to serialize dashboard JSON")?;
    fs::write(&dashboard_json_path, json_content)
        .context("Failed to write dashboard JSON")?;

    // Generate dashboard.md
    let dashboard_md_path = doplan_dir.join("dashboard.md");
    let mut md_content = String::new();
    
    md_content.push_str("# Project Dashboard\n\n");
    md_content.push_str(&format!("**Project:** {}\n\n", 
        state.project_name.as_ref().unwrap_or(&"Untitled Project".to_string())));
    md_content.push_str(&format!("**Last Updated:** {}\n\n", 
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
    md_content.push_str("---\n\n");

    md_content.push_str("## Overall Progress\n\n");
    md_content.push_str(&format!("**{:.1}%** Complete\n\n", overall_progress));
    md_content.push_str(&format!("{}\n\n", generate_progress_bar(overall_progress)));

    md_content.push_str("## Phase Progress\n\n");
    for phase in phases {
        md_content.push_str(&format!("### {}\n\n", phase.phase_name));
        md_content.push_str(&format!("**{:.1}%** Complete\n\n", phase.progress));
        md_content.push_str(&format!("{}\n\n", generate_progress_bar(phase.progress)));

        md_content.push_str("#### Features\n\n");
        for feature in &phase.features {
            md_content.push_str(&format!("- **{}** ({}) - {:.1}% - {}\n", 
                feature.feature_name,
                feature.priority,
                feature.progress,
                feature.status
            ));
        }
        md_content.push_str("\n");
    }

    md_content.push_str("## Task Summary\n\n");
    let total_tasks: usize = phases.iter()
        .flat_map(|p| &p.features)
        .map(|f| f.tasks.total)
        .sum();
    let completed_tasks: usize = phases.iter()
        .flat_map(|p| &p.features)
        .map(|f| f.tasks.completed)
        .sum();
    let in_progress_tasks: usize = phases.iter()
        .flat_map(|p| &p.features)
        .map(|f| f.tasks.in_progress)
        .sum();
    let not_started_tasks: usize = phases.iter()
        .flat_map(|p| &p.features)
        .map(|f| f.tasks.not_started)
        .sum();
    let blocked_tasks: usize = phases.iter()
        .flat_map(|p| &p.features)
        .map(|f| f.tasks.blocked)
        .sum();

    md_content.push_str(&format!("- **Total Tasks:** {}\n", total_tasks));
    md_content.push_str(&format!("- **Completed:** {}\n", completed_tasks));
    md_content.push_str(&format!("- **In Progress:** {}\n", in_progress_tasks));
    md_content.push_str(&format!("- **Not Started:** {}\n", not_started_tasks));
    if blocked_tasks > 0 {
        md_content.push_str(&format!("- **Blocked:** {}\n", blocked_tasks));
    }
    md_content.push_str("\n");

    fs::write(&dashboard_md_path, md_content)
        .context("Failed to write dashboard markdown")?;

    Ok(())
}

fn generate_progress_bar(progress: f64) -> String {
    let width = 30;
    let filled = (progress / 100.0 * width as f64) as usize;
    let empty = width - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

fn extract_name_from_path(path: &str) -> String {
    path.split('-')
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ")
        .replace("-", " ")
}

fn sanitize_name(name: &str) -> String {
    name.to_lowercase()
        .replace(" ", "-")
        .replace("_", "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}



use anyhow::{Context, Result};
use colored::*;
use std::path::PathBuf;
use std::fs;
use walkdir::WalkDir;
use serde_json::Value;
use crate::state::ProjectState;
use crate::utils;

#[derive(Debug, Clone)]
struct TaskInfo {
    feature_path: PathBuf,
    phase_name: String,
    feature_name: String,
    priority: String,
    task_name: String,
    status: String,
    estimated_time: String,
    progress: f64,
}

#[derive(Debug, Clone)]
struct Recommendation {
    action: String,
    priority: String,
    feature_path: String,
    phase_name: String,
    feature_name: String,
    task_name: String,
    estimated_effort: String,
    reason: String,
}

/// Execute the /next command
pub async fn execute(_args: Vec<String>) -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  DoPlan: Next Action Recommendation".bright_cyan().bold());
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

    // Scan all features for incomplete tasks
    let mut all_tasks = Vec::new();
    scan_features(&plan_dir, &mut all_tasks)?;

    if all_tasks.is_empty() {
        println!("{}", "🎉 All tasks are complete!".bright_green());
        println!();
        println!("{}", "No incomplete tasks found. Great work!".bright_white());
        return Ok(());
    }

    // Analyze and generate recommendations
    let recommendation = analyze_and_recommend(&all_tasks, &state)?;

    // Display recommendation
    display_recommendation(&recommendation, &all_tasks)?;

    Ok(())
}

fn scan_features(plan_dir: &PathBuf, tasks: &mut Vec<TaskInfo>) -> Result<()> {
    for entry in WalkDir::new(plan_dir)
        .min_depth(2)
        .max_depth(4)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_name() == "tasks.md" {
            let tasks_path = entry.path();
            let feature_dir = tasks_path.parent().unwrap();
            
            // Extract phase and feature names from path
            let relative_path = feature_dir
                .strip_prefix(plan_dir)
                .unwrap()
                .to_string_lossy()
                .to_string();
            let path_parts: Vec<&str> = relative_path.split('/').collect();
            
            if path_parts.len() >= 2 {
                let phase_name = extract_name_from_path(path_parts[0]);
                let feature_name = extract_name_from_path(path_parts[1]);
                
                // Read progress.json
                let progress_path = feature_dir.join("progress.json");
                let progress = if progress_path.exists() {
                    read_progress(&progress_path).ok()
                } else {
                    None
                };

                let priority = progress
                    .as_ref()
                    .and_then(|p| p.get("priority"))
                    .and_then(|p| p.as_str())
                    .unwrap_or("medium")
                    .to_string();

                let progress_pct = progress
                    .as_ref()
                    .and_then(|p| p.get("progress"))
                    .and_then(|p| p.as_f64())
                    .unwrap_or(0.0);

                // Parse tasks.md
                if let Ok(content) = fs::read_to_string(tasks_path) {
                    parse_tasks(
                        &content,
                        feature_dir.to_path_buf(),
                        phase_name,
                        feature_name,
                        priority,
                        progress_pct,
                        tasks,
                    )?;
                }
            }
        }
    }

    Ok(())
}

fn read_progress(path: &PathBuf) -> Result<Value> {
    let content = fs::read_to_string(path)
        .context("Failed to read progress file")?;
    let progress: Value = serde_json::from_str(&content)
        .context("Failed to parse progress file")?;
    Ok(progress)
}

fn parse_tasks(
    content: &str,
    feature_path: PathBuf,
    phase_name: String,
    feature_name: String,
    priority: String,
    progress: f64,
    tasks: &mut Vec<TaskInfo>,
) -> Result<()> {
    let lines: Vec<&str> = content.lines().collect();
    let mut current_task: Option<TaskInfo> = None;
    let mut in_task = false;

    for line in lines {
        if line.starts_with("#### Task") {
            // Save previous task if incomplete
            if let Some(task) = current_task.take() {
                if task.status != "Completed" {
                    tasks.push(task);
                }
            }

            // Start new task
            let task_name = line
                .strip_prefix("#### Task")
                .unwrap_or("")
                .trim()
                .to_string();
            
            current_task = Some(TaskInfo {
                feature_path: feature_path.clone(),
                phase_name: phase_name.clone(),
                feature_name: feature_name.clone(),
                priority: priority.clone(),
                task_name,
                status: "Not Started".to_string(),
                estimated_time: "Unknown".to_string(),
                progress,
            });
            in_task = true;
        } else if in_task {
            if let Some(ref mut task) = current_task {
                if line.contains("**Status**") {
                    // Parse status
                    if line.contains("[x] Completed") || line.contains("[X] Completed") {
                        task.status = "Completed".to_string();
                    } else if line.contains("[x] In Progress") || line.contains("[X] In Progress") {
                        task.status = "In Progress".to_string();
                    } else if line.contains("[x] Blocked") || line.contains("[X] Blocked") {
                        task.status = "Blocked".to_string();
                    } else {
                        task.status = "Not Started".to_string();
                    }
                } else if line.contains("**Estimated Time**") {
                    // Extract estimated time
                    if let Some(time_part) = line.split(':').nth(1) {
                        task.estimated_time = time_part.trim().to_string();
                    }
                }
            }
        }
    }

    // Save last task if incomplete
    if let Some(task) = current_task {
        if task.status != "Completed" {
            tasks.push(task);
        }
    }

    Ok(())
}

fn analyze_and_recommend(tasks: &[TaskInfo], _state: &ProjectState) -> Result<Recommendation> {
    // Priority order: High > Medium > Low
    // Status order: In Progress > Not Started > Blocked
    let mut sorted_tasks = tasks.to_vec();
    
    sorted_tasks.sort_by(|a, b| {
        // First by priority
        let priority_order = |p: &str| match p.to_lowercase().as_str() {
            "high" => 0,
            "medium" => 1,
            "low" => 2,
            _ => 3,
        };
        
        let priority_cmp = priority_order(&a.priority).cmp(&priority_order(&b.priority));
        if priority_cmp != std::cmp::Ordering::Equal {
            return priority_cmp;
        }

        // Then by status
        let status_order = |s: &str| match s {
            "In Progress" => 0,
            "Not Started" => 1,
            "Blocked" => 2,
            _ => 3,
        };

        status_order(&a.status).cmp(&status_order(&b.status))
    });

    let recommended = sorted_tasks.first()
        .context("No tasks to recommend")?;

    let action = match recommended.status.as_str() {
        "In Progress" => format!("Continue working on: {}", recommended.task_name),
        "Not Started" => format!("Start: {}", recommended.task_name),
        "Blocked" => format!("Unblock: {}", recommended.task_name),
        _ => format!("Work on: {}", recommended.task_name),
    };

    let reason = format!(
        "Priority: {} | Status: {} | Progress: {:.0}%",
        recommended.priority,
        recommended.status,
        recommended.progress
    );

    Ok(Recommendation {
        action,
        priority: recommended.priority.clone(),
        feature_path: recommended.feature_path.to_string_lossy().to_string(),
        phase_name: recommended.phase_name.clone(),
        feature_name: recommended.feature_name.clone(),
        task_name: recommended.task_name.clone(),
        estimated_effort: recommended.estimated_time.clone(),
        reason,
    })
}

fn display_recommendation(rec: &Recommendation, all_tasks: &[TaskInfo]) -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green());
    println!("{}", "  📋 Recommended Next Action".bright_green().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green());
    println!();

    println!("{}", format!("{}", rec.action).bright_white().bold());
    println!();

    println!("{}", "Details:".bright_cyan());
    println!("  {} Phase: {}", "→".bright_cyan(), rec.phase_name.bright_white());
    println!("  {} Feature: {}", "→".bright_cyan(), rec.feature_name.bright_white());
    println!("  {} Task: {}", "→".bright_cyan(), rec.task_name.bright_white());
    println!("  {} Priority: {}", "→".bright_cyan(), 
        match rec.priority.to_lowercase().as_str() {
            "high" => rec.priority.bright_red().bold(),
            "medium" => rec.priority.bright_yellow(),
            "low" => rec.priority.bright_white(),
            _ => rec.priority.bright_white(),
        }
    );
    println!("  {} Estimated Effort: {}", "→".bright_cyan(), rec.estimated_effort.bright_white());
    println!();

    println!("{}", "Reason:".bright_cyan());
    println!("  {}", rec.reason.bright_white());
    println!();

    // Show path to task file
    let tasks_path = format!("{}/tasks.md", rec.feature_path);
    println!("{}", "Task File:".bright_cyan());
    println!("  {}", tasks_path.bright_white());
    println!();

    // Show summary statistics
    let high_priority = all_tasks.iter().filter(|t| t.priority.to_lowercase() == "high").count();
    let in_progress = all_tasks.iter().filter(|t| t.status == "In Progress").count();
    let not_started = all_tasks.iter().filter(|t| t.status == "Not Started").count();
    let blocked = all_tasks.iter().filter(|t| t.status == "Blocked").count();

    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  📊 Project Status Summary".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();
    println!("  {} Total Incomplete Tasks: {}", "→".bright_cyan(), all_tasks.len().to_string().bright_white());
    println!("  {} High Priority: {}", "→".bright_cyan(), high_priority.to_string().bright_red());
    println!("  {} In Progress: {}", "→".bright_cyan(), in_progress.to_string().bright_yellow());
    println!("  {} Not Started: {}", "→".bright_cyan(), not_started.to_string().bright_white());
    if blocked > 0 {
        println!("  {} Blocked: {}", "→".bright_cyan(), blocked.to_string().bright_red());
    }
    println!();

    println!("{}", "Next Steps:".bright_yellow());
    println!("  1. Review the recommended task in: {}", tasks_path.bright_white());
    println!("  2. Run /implement to create a branch for this feature");
    println!("  3. Start working on the task");
    println!("  4. Update tasks.md as you make progress");
    println!();

    Ok(())
}

fn extract_name_from_path(path: &str) -> String {
    path.split('-')
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ")
        .replace("-", " ")
}


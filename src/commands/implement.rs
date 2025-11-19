use anyhow::{Context, Result};
use colored::*;
use std::path::PathBuf;
use std::fs;
use crate::state::ProjectState;
use crate::utils;
use git2::{Repository, Signature};

/// Execute the /implement command
pub async fn execute(args: Vec<String>) -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  DoPlan: Feature Implementation".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    // Parse argument (phase or feature path)
    let target = args.get(0).map(|s| s.as_str()).unwrap_or("");
    
    if target.is_empty() {
        println!("{}", "Usage: /implement <phase-id> or /implement <phase-id>/<feature-id>".bright_yellow());
        println!("{}", "Example: /implement 01-foundation or /implement 01-foundation/01-project-planning".bright_yellow());
        return Ok(());
    }

    // Load state (for future use)
    let _state = ProjectState::load()
        .context("Failed to load project state")?;

    let doplan_dir = utils::doplan_dir()?;
    let plan_dir = doplan_dir.join("plan");

    // Find phase and feature
    let (phase_path, feature_path, phase_name, feature_name) = if target.contains('/') {
        // Specific feature: 01-foundation/01-project-planning
        let parts: Vec<&str> = target.split('/').collect();
        if parts.len() != 2 {
            println!("{}", "Invalid format. Use: phase-id/feature-id".bright_red());
            return Ok(());
        }
        let phase_id = parts[0];
        let feature_id = parts[1];
        
        let phase_path = plan_dir.join(phase_id);
        let feature_path = phase_path.join(feature_id);
        
        if !feature_path.exists() {
            println!("{}", format!("Feature path not found: {}", feature_path.display()).bright_red());
            return Ok(());
        }

        let phase_name = extract_name_from_path(phase_id);
        let feature_name = extract_name_from_path(feature_id);
        
        (phase_path, feature_path, phase_name, feature_name)
    } else {
        // Phase only: 01-foundation (implement first feature)
        let phase_path = plan_dir.join(target);
        
        if !phase_path.exists() {
            println!("{}", format!("Phase path not found: {}", phase_path.display()).bright_red());
            return Ok(());
        }

        // Find first feature in phase
        let mut first_feature: Option<PathBuf> = None;
        if let Ok(entries) = fs::read_dir(&phase_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name().unwrap().to_string_lossy();
                    if name.starts_with("01-") {
                        first_feature = Some(path);
                        break;
                    }
                }
            }
        }

        let feature_path = match first_feature {
            Some(path) => path,
            None => {
                println!("{}", "No features found in this phase".bright_red());
                return Ok(());
            }
        };

        let phase_name = extract_name_from_path(target);
        let feature_name = extract_name_from_path(
            feature_path.file_name().unwrap().to_string_lossy().as_ref()
        );
        
        (phase_path, feature_path, phase_name, feature_name)
    };

    // Generate branch name (append "-rust" for this project)
    let phase_id = extract_id_from_path(phase_path.file_name().unwrap().to_string_lossy().as_ref());
    let feature_id = extract_id_from_path(feature_path.file_name().unwrap().to_string_lossy().as_ref());
    let branch_name = format!("feature/{}-phase-{}-{}-rust", phase_id, feature_id, sanitize_for_branch(&feature_name));

    println!("{}", format!("Phase: {}", phase_name).bright_white());
    println!("{}", format!("Feature: {}", feature_name).bright_white());
    println!("{}", format!("Branch: {}", branch_name).bright_cyan());
    println!();

    // Open Git repository
    let project_root = utils::project_root()?;
    let repo = match Repository::open(&project_root) {
        Ok(repo) => repo,
        Err(_) => {
            println!("{}", "Not a Git repository. Initializing...".bright_yellow());
            Repository::init(&project_root)
                .context("Failed to initialize Git repository")?
        }
    };

    // Check if branch already exists
    let branch_exists = {
        let mut found = false;
        if let Ok(branches) = repo.branches(None) {
            for branch_result in branches {
                if let Ok((branch, _)) = branch_result {
                    if let Ok(Some(name)) = branch.name() {
                        if name == branch_name {
                            found = true;
                            break;
                        }
                    }
                }
            }
        }
        found
    };

    if branch_exists {
        println!("{}", format!("Branch '{}' already exists. Switching to it...", branch_name).bright_yellow());
        let branch = repo.find_branch(&branch_name, git2::BranchType::Local)
            .context("Failed to find existing branch")?;
        let commit = branch.get().peel_to_commit()
            .context("Failed to get branch commit")?;
        repo.checkout_tree(commit.as_object(), None)
            .context("Failed to checkout branch")?;
        repo.set_head(&format!("refs/heads/{}", branch_name))
            .context("Failed to set HEAD")?;
    } else {
        // Create new branch
        println!("{}", format!("Creating branch: {}", branch_name).bright_cyan());
        
        // Handle empty repository (no HEAD)
        let head_commit = match repo.head() {
            Ok(head) => {
                head.peel_to_commit()
                    .context("Failed to get HEAD commit")?
            }
            Err(_) => {
                // Empty repository - create initial commit first
                println!("{}", "Empty repository detected. Creating initial commit...".bright_yellow());
                
                let mut index = repo.index()
                    .context("Failed to get index")?;
                
                // Add README if it exists
                let project_root = utils::project_root()?;
                let readme_path = project_root.join("README.md");
                if readme_path.exists() {
                    index.add_path(
                        readme_path.strip_prefix(&project_root)
                            .context("Failed to get relative path")?
                    )
                    .context("Failed to add README to index")?;
                }
                
                index.write()
                    .context("Failed to write index")?;
                
                let tree_id = index.write_tree()
                    .context("Failed to write tree")?;
                let tree = repo.find_tree(tree_id)
                    .context("Failed to find tree")?;
                
                let signature = Signature::now("DoPlan", "doplan@doplan.dev")
                    .context("Failed to create signature")?;
                
                repo.commit(
                    Some("refs/heads/master"),
                    &signature,
                    &signature,
                    "chore: initial commit",
                    &tree,
                    &[],
                )
                .context("Failed to create initial commit")?;
                
                repo.head()
                    .context("Failed to get HEAD after initial commit")?
                    .peel_to_commit()
                    .context("Failed to get HEAD commit")?
            }
        };

        let _branch = repo.branch(&branch_name, &head_commit, false)
            .context("Failed to create branch")?;

        // Checkout the branch
        repo.checkout_tree(head_commit.as_object(), None)
            .context("Failed to checkout tree")?;
        repo.set_head(&format!("refs/heads/{}", branch_name))
            .context("Failed to set HEAD")?;

        println!("{}", format!("✓ Branch '{}' created and checked out", branch_name).bright_green());
    }

    // Stage planning documents
    println!();
    println!("{}", "Staging planning documents...".bright_cyan());
    
    let mut index = repo.index()
        .context("Failed to get repository index")?;

    // Add feature planning files
    let planning_files = vec!["plan.md", "design.md", "tasks.md"];
    for file in &planning_files {
        let file_path = feature_path.join(file);
        if file_path.exists() {
            let relative_path = file_path.strip_prefix(&project_root)
                .context("Failed to get relative path")?;
            index.add_path(relative_path)
                .context(format!("Failed to add {}", file))?;
            println!("  {} Added {}", "→".bright_cyan(), file);
        }
    }

    index.write()
        .context("Failed to write index")?;

    // Create initial commit
    println!();
    println!("{}", "Creating initial commit...".bright_cyan());
    
    let tree_id = index.write_tree()
        .context("Failed to write tree")?;
    let tree = repo.find_tree(tree_id)
        .context("Failed to find tree")?;

    let signature = Signature::now("DoPlan", "doplan@doplan.dev")
        .context("Failed to create signature")?;

    let commit_message = format!("docs: add planning docs for {}", feature_name);
    
    let _head_commit = repo.head()
        .and_then(|h| h.peel_to_commit())
        .ok();
    
    let parents: Vec<&git2::Commit> = _head_commit.as_ref().into_iter().collect();
    
    let commit_id = repo.commit(
        Some(&format!("refs/heads/{}", branch_name)),
        &signature,
        &signature,
        &commit_message,
        &tree,
        &parents,
    )
    .context("Failed to create commit")?;

    println!("{}", format!("✓ Commit created: {}", &commit_id.to_string()[..8]).bright_green());

    // Try to push (may fail if no remote, that's okay)
    println!();
    println!("{}", "Attempting to push branch...".bright_cyan());
    
    if let Ok(mut remote) = repo.find_remote("origin") {
        let branch_ref = format!("refs/heads/{}", branch_name);
        match remote.push(&[&branch_ref], None) {
            Ok(_) => {
                println!("{}", format!("✓ Branch pushed to origin/{}", branch_name).bright_green());
            }
            Err(e) => {
                println!("{}", format!("⚠ Could not push branch: {}. You can push manually later.", e).bright_yellow());
            }
        }
    } else {
        println!("{}", "⚠ No remote 'origin' found. Push manually when ready.".bright_yellow());
    }

    // Update state (add branch info if we extend state structure)
    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green());
    println!("{}", "  Implementation Ready!".bright_green().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green());
    println!();
    println!("{}", "Branch created and ready for implementation:".bright_cyan());
    println!("  {}", branch_name.bright_white());
    println!();
    println!("{}", "Next steps:".bright_yellow());
    println!("  1. Review planning documents:");
    println!("     - {}", feature_path.join("plan.md").display());
    println!("     - {}", feature_path.join("design.md").display());
    println!("     - {}", feature_path.join("tasks.md").display());
    println!("  2. Start implementing the feature");
    println!("  3. Check off tasks in tasks.md as you complete them");
    println!("  4. Commit regularly with clear messages");
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

fn extract_id_from_path(path: &str) -> String {
    path.split('-')
        .next()
        .unwrap_or("01")
        .to_string()
}

fn sanitize_for_branch(name: &str) -> String {
    name.to_lowercase()
        .replace(" ", "-")
        .replace("_", "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}


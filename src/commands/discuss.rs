use anyhow::{Context, Result};
use dialoguer::{Input, MultiSelect};
use colored::*;
use crate::state::{ProjectState, Feature, Phase};
use crate::utils;

/// Execute the /discuss command
pub async fn execute(_args: Vec<String>) -> Result<()> {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  DoPlan: Idea Discussion & Refinement".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    // Load existing state or create new
    let mut state = ProjectState::load()
        .context("Failed to load project state")?;

    // Step 1: Project Name
    let project_name: String = Input::new()
        .with_prompt("What's your project name?")
        .default(state.project_name.clone().unwrap_or_default())
        .interact_text()
        .context("Failed to read project name")?;

    // Step 2: Core Idea
    println!();
    println!("{}", "Let's start with your core idea...".bright_yellow());
    let idea: String = Input::new()
        .with_prompt("Describe your app idea in detail")
        .default(state.idea.clone().unwrap_or_default())
        .interact_text()
        .context("Failed to read idea")?;

    // Step 3: Ask comprehensive questions
    println!();
    println!("{}", "Now let's dive deeper with some questions...".bright_yellow());
    
    let questions = vec![
        "Who is your target audience?",
        "What problem does this solve?",
        "What makes your solution unique?",
        "What are the core features you envision?",
        "Are there any technical constraints or requirements?",
        "What's your timeline or deadline?",
        "Do you have any design preferences?",
        "What platforms should this support? (web, mobile, desktop)",
    ];

    let mut answers = Vec::new();
    for question in &questions {
        let answer: String = Input::new()
            .with_prompt(*question)
            .allow_empty(true)
            .interact_text()
            .context("Failed to read answer")?;
        answers.push((*question, answer));
    }

    // Step 4: Feature organization
    println!();
    println!("{}", "Let's organize your features...".bright_yellow());
    
    let feature_names: String = Input::new()
        .with_prompt("List your main features (comma-separated)")
        .allow_empty(true)
        .interact_text()
        .context("Failed to read features")?;

    let features: Vec<Feature> = if !feature_names.is_empty() {
        feature_names
            .split(',')
            .map(|f| f.trim().to_string())
            .filter(|f| !f.is_empty())
            .enumerate()
            .map(|(i, name)| {
                let priority = if i < 3 { "high" } else if i < 6 { "medium" } else { "low" };
                Feature {
                    name: name.clone(),
                    description: format!("Feature: {}", name),
                    priority: priority.to_string(),
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    // Step 5: Tech stack recommendations
    println!();
    println!("{}", "Let's recommend a tech stack...".bright_yellow());
    
    let tech_options = vec![
        "Frontend: React/Next.js",
        "Frontend: Vue/Nuxt",
        "Frontend: Svelte/SvelteKit",
        "Backend: Node.js/Express",
        "Backend: Rust/Axum",
        "Backend: Python/FastAPI",
        "Backend: Go/Gin",
        "Database: PostgreSQL",
        "Database: MongoDB",
        "Database: SQLite",
        "Database: Redis",
        "Deployment: Vercel",
        "Deployment: Railway",
        "Deployment: Docker",
    ];

    let selections = MultiSelect::new()
        .with_prompt("Select recommended technologies")
        .items(&tech_options)
        .interact()
        .context("Failed to select technologies")?;

    let tech_stack: Vec<String> = selections
        .iter()
        .map(|&i| tech_options[i].to_string())
        .collect();

    // Step 6: Suggest improvements
    println!();
    println!("{}", "Based on your idea, here are some suggestions...".bright_green());
    
    let improvements = generate_improvements(&idea, &features);
    for improvement in &improvements {
        println!("  • {}", improvement.bright_white());
    }

    // Step 7: Phase organization
    println!();
    println!("{}", "Let's organize features into phases...".bright_yellow());
    
    let phase_count: usize = Input::new()
        .with_prompt("How many development phases do you want?")
        .default(3)
        .interact_text()
        .context("Failed to read phase count")?;

    let mut phases = Vec::new();
    for i in 1..=phase_count {
        let phase_name: String = Input::new()
            .with_prompt(format!("Phase {} name", i))
            .default(format!("Phase {}", i))
            .interact_text()
            .context("Failed to read phase name")?;

        let phase_description: String = Input::new()
            .with_prompt(format!("Phase {} description", i))
            .allow_empty(true)
            .interact_text()
            .context("Failed to read phase description")?;

        // Assign features to phases
        let feature_names_for_phase: Vec<String> = if !features.is_empty() {
            let feature_list: Vec<&str> = features.iter().map(|f| f.name.as_str()).collect();
            let selected = MultiSelect::new()
                .with_prompt(format!("Select features for {}", phase_name))
                .items(&feature_list)
                .interact()
                .context("Failed to select features")?;
            
            selected.iter().map(|&i| feature_list[i].to_string()).collect()
        } else {
            Vec::new()
        };

        phases.push(Phase {
            name: phase_name,
            description: phase_description,
            features: feature_names_for_phase,
        });
    }

    // Update state
    state.project_name = Some(project_name.clone());
    state.idea = Some(idea.clone());
    state.tech_stack = Some(tech_stack.clone());
    state.features = Some(features.clone());
    state.phases = Some(phases.clone());
    state.improvements = Some(improvements.clone());
    state.notes = Some(format!("Discussion completed on {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

    // Save state
    state.save()
        .context("Failed to save project state")?;

    // Generate idea notes
    generate_idea_notes(&state, &answers)
        .context("Failed to generate idea notes")?;

    // Summary
    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green());
    println!("{}", "  Discussion Complete!".bright_green().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green());
    println!();
    println!("{}", format!("Project: {}", project_name).bright_white());
    println!("{}", format!("Features: {}", features.len()).bright_white());
    println!("{}", format!("Phases: {}", phases.len()).bright_white());
    println!("{}", format!("Tech Stack: {} technologies", tech_stack.len()).bright_white());
    println!();
    println!("{}", "Files created:".bright_cyan());
    println!("  • .doplan/state.json");
    println!("  • doplan/idea-notes.md");
    println!();
    println!("{}", "Next steps:".bright_yellow());
    println!("  1. Review doplan/idea-notes.md");
    println!("  2. Run /generate to create PRD and project structure");
    println!();

    Ok(())
}

fn generate_improvements(idea: &str, features: &[Feature]) -> Vec<String> {
    let mut improvements = Vec::new();

    // Basic suggestions based on common patterns
    if idea.to_lowercase().contains("user") || idea.to_lowercase().contains("auth") {
        improvements.push("Consider implementing user authentication and authorization".to_string());
    }

    if features.len() > 5 {
        improvements.push("Consider prioritizing features - start with MVP and iterate".to_string());
    }

    if idea.to_lowercase().contains("data") || idea.to_lowercase().contains("storage") {
        improvements.push("Plan your data model and storage strategy early".to_string());
    }

    improvements.push("Set up proper error handling and logging from the start".to_string());
    improvements.push("Consider implementing automated testing (unit, integration, e2e)".to_string());
    improvements.push("Plan for scalability - design with growth in mind".to_string());
    improvements.push("Document your API contracts early for better team collaboration".to_string());

    improvements
}

fn generate_idea_notes(state: &ProjectState, qa: &[(&str, String)]) -> Result<()> {
    let doplan_dir = utils::doplan_dir()?;
    utils::ensure_dir(&doplan_dir)?;

    let notes_path = doplan_dir.join("idea-notes.md");
    
    let mut content = String::new();
    
    // Header
    content.push_str("# Idea Discussion Notes\n\n");
    content.push_str(&format!("**Project:** {}\n\n", 
        state.project_name.as_ref().unwrap_or(&"Untitled".to_string())));
    content.push_str(&format!("**Date:** {}\n\n", 
        state.notes.as_ref().unwrap_or(&"Unknown".to_string())));
    content.push_str("---\n\n");

    // Core Idea
    content.push_str("## Core Idea\n\n");
    content.push_str(&format!("{}\n\n", 
        state.idea.as_ref().unwrap_or(&"No idea provided".to_string())));

    // Q&A Section
    content.push_str("## Discussion Questions & Answers\n\n");
    for (question, answer) in qa {
        if !answer.is_empty() {
            content.push_str(&format!("### {}\n\n", question));
            content.push_str(&format!("{}\n\n", answer));
        }
    }

    // Features
    if let Some(features) = &state.features {
        if !features.is_empty() {
            content.push_str("## Features\n\n");
            for feature in features {
                content.push_str(&format!("### {}\n", feature.name));
                content.push_str(&format!("- **Priority:** {}\n", feature.priority));
                content.push_str(&format!("- **Description:** {}\n\n", feature.description));
            }
        }
    }

    // Phases
    if let Some(phases) = &state.phases {
        if !phases.is_empty() {
            content.push_str("## Development Phases\n\n");
            for (i, phase) in phases.iter().enumerate() {
                content.push_str(&format!("### Phase {}: {}\n\n", i + 1, phase.name));
                content.push_str(&format!("{}\n\n", phase.description));
                if !phase.features.is_empty() {
                    content.push_str("**Features:**\n");
                    for feature_name in &phase.features {
                        content.push_str(&format!("- {}\n", feature_name));
                    }
                    content.push_str("\n");
                }
            }
        }
    }

    // Tech Stack
    if let Some(tech_stack) = &state.tech_stack {
        if !tech_stack.is_empty() {
            content.push_str("## Recommended Tech Stack\n\n");
            for tech in tech_stack {
                content.push_str(&format!("- {}\n", tech));
            }
            content.push_str("\n");
        }
    }

    // Improvements
    if let Some(improvements) = &state.improvements {
        if !improvements.is_empty() {
            content.push_str("## Suggested Improvements\n\n");
            for improvement in improvements {
                content.push_str(&format!("- {}\n", improvement));
            }
            content.push_str("\n");
        }
    }

    // Next Steps
    content.push_str("## Next Steps\n\n");
    content.push_str("1. Review and refine the idea based on the discussion\n");
    content.push_str("2. Run `/generate` to create PRD and project structure\n");
    content.push_str("3. Run `/plan` to create detailed phase and feature plans\n");
    content.push_str("4. Run `/generate` again to create DPR, SOPS, and other documents\n");

    std::fs::write(&notes_path, content)
        .context("Failed to write idea notes")?;

    Ok(())
}


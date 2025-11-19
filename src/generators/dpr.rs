use anyhow::{Context, Result};
use std::path::PathBuf;
use std::fs;
use walkdir::WalkDir;
use crate::state::ProjectState;
use crate::utils;
use serde_json::json;

pub fn generate(state: &ProjectState) -> Result<Vec<PathBuf>> {
    let doplan_dir = utils::doplan_dir()?;
    let plan_dir = doplan_dir.join("plan");
    let design_dir = doplan_dir.join("design");
    utils::ensure_dir(&design_dir)?;

    let mut generated = Vec::new();

    // Read all plan.md files
    let mut all_pages = Vec::new();
    let mut all_sections = Vec::new();
    let mut all_components = Vec::new();
    let mut all_cards = Vec::new();

    if plan_dir.exists() {
        for entry in WalkDir::new(&plan_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name() == "plan.md")
        {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                extract_design_info(&content, &mut all_pages, &mut all_sections, &mut all_components, &mut all_cards);
            }
        }
    }

    // Generate DPR.md
    let dpr_path = design_dir.join("DPR.md");
    generate_dpr_md(&dpr_path, state, &all_pages, &all_sections, &all_components, &all_cards)?;
    generated.push(dpr_path);

    // Generate design-tokens.json
    let tokens_path = design_dir.join("design-tokens.json");
    generate_design_tokens(&tokens_path)?;
    generated.push(tokens_path);

    // Generate design_rules.mdc
    let dot_doplan = utils::dot_doplan_dir()?;
    let rules_dir = dot_doplan.join("ai").join("rules");
    utils::ensure_dir(&rules_dir)?;
    let rules_path = rules_dir.join("design_rules.mdc");
    generate_design_rules(&rules_path, &all_pages, &all_sections, &all_components, &all_cards)?;
    generated.push(rules_path);

    Ok(generated)
}

fn extract_design_info(
    content: &str,
    pages: &mut Vec<String>,
    sections: &mut Vec<String>,
    components: &mut Vec<String>,
    cards: &mut Vec<String>,
) {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_pages = false;
    let mut in_sections = false;
    let mut in_components = false;
    let mut in_cards = false;

    for line in lines {
        if line.contains("### Pages") {
            in_pages = true;
            in_sections = false;
            in_components = false;
            in_cards = false;
            continue;
        } else if line.contains("### Sections") {
            in_pages = false;
            in_sections = true;
            in_components = false;
            in_cards = false;
            continue;
        } else if line.contains("### Components") {
            in_pages = false;
            in_sections = false;
            in_components = true;
            in_cards = false;
            continue;
        } else if line.contains("### Cards/UI Elements") || line.contains("### Cards") {
            in_pages = false;
            in_sections = false;
            in_components = false;
            in_cards = true;
            continue;
        } else if line.starts_with("##") || line.starts_with("#") {
            in_pages = false;
            in_sections = false;
            in_components = false;
            in_cards = false;
            continue;
        }

        if line.trim().starts_with("-") && !line.contains("_to be defined_") {
            let item = line.trim_start_matches("-").trim().to_string();
            if !item.is_empty() {
                if in_pages {
                    pages.push(item);
                } else if in_sections {
                    sections.push(item);
                } else if in_components {
                    components.push(item);
                } else if in_cards {
                    cards.push(item);
                }
            }
        }
    }
}

fn generate_dpr_md(
    path: &PathBuf,
    state: &ProjectState,
    pages: &[String],
    sections: &[String],
    components: &[String],
    cards: &[String],
) -> Result<()> {
    let project_name = state.project_name.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("Untitled Project");

    let mut content = String::new();
    content.push_str("# Design Preferences & Requirements (DPR)\n\n");
    content.push_str(&format!("**Project:** {}\n\n", project_name));
    content.push_str(&format!("**Date:** {}\n\n", chrono::Utc::now().format("%Y-%m-%d")));
    content.push_str("---\n\n");

    content.push_str("## Overview\n\n");
    content.push_str("This document defines the design preferences and requirements extracted from feature plans.\n\n");
    content.push_str("---\n\n");

    // Pages
    if !pages.is_empty() {
        content.push_str("## Pages\n\n");
        for page in pages {
            content.push_str(&format!("- {}\n", page));
        }
        content.push_str("\n");
    }

    // Sections
    if !sections.is_empty() {
        content.push_str("## Sections\n\n");
        for section in sections {
            content.push_str(&format!("- {}\n", section));
        }
        content.push_str("\n");
    }

    // Components
    if !components.is_empty() {
        content.push_str("## Components\n\n");
        for component in components {
            content.push_str(&format!("- {}\n", component));
        }
        content.push_str("\n");
    }

    // Cards/UI Elements
    if !cards.is_empty() {
        content.push_str("## Cards/UI Elements\n\n");
        for card in cards {
            content.push_str(&format!("- {}\n", card));
        }
        content.push_str("\n");
    }

    // Design Principles
    content.push_str("## Design Principles\n\n");
    content.push_str("### Consistency\n");
    content.push_str("- Use consistent spacing and typography\n");
    content.push_str("- Follow established design patterns\n");
    content.push_str("- Maintain visual hierarchy\n\n");

    content.push_str("### Accessibility\n");
    content.push_str("- WCAG 2.1 AA compliance\n");
    content.push_str("- Keyboard navigation support\n");
    content.push_str("- Screen reader compatibility\n\n");

    content.push_str("### Responsiveness\n");
    content.push_str("- Mobile-first approach\n");
    content.push_str("- Breakpoints: mobile (320px), tablet (768px), desktop (1024px+)\n");
    content.push_str("- Flexible layouts\n\n");

    // Color System
    content.push_str("## Color System\n\n");
    content.push_str("### Primary Colors\n");
    content.push_str("- Primary: #0070f3\n");
    content.push_str("- Secondary: #7928ca\n");
    content.push_str("- Accent: #f81ce5\n\n");

    content.push_str("### Neutral Colors\n");
    content.push_str("- Background: #ffffff\n");
    content.push_str("- Surface: #fafafa\n");
    content.push_str("- Text: #000000\n");
    content.push_str("- Text Secondary: #666666\n\n");

    // Typography
    content.push_str("## Typography\n\n");
    content.push_str("### Font Families\n");
    content.push_str("- Headings: System font stack\n");
    content.push_str("- Body: System font stack\n");
    content.push_str("- Monospace: 'Courier New', monospace\n\n");

    content.push_str("### Font Sizes\n");
    content.push_str("- H1: 2.5rem (40px)\n");
    content.push_str("- H2: 2rem (32px)\n");
    content.push_str("- H3: 1.5rem (24px)\n");
    content.push_str("- Body: 1rem (16px)\n");
    content.push_str("- Small: 0.875rem (14px)\n\n");

    // Spacing
    content.push_str("## Spacing\n\n");
    content.push_str("- Base unit: 8px\n");
    content.push_str("- Small: 8px\n");
    content.push_str("- Medium: 16px\n");
    content.push_str("- Large: 24px\n");
    content.push_str("- XLarge: 32px\n\n");

    std::fs::write(path, content)
        .context("Failed to write DPR")?;

    Ok(())
}

fn generate_design_tokens(path: &PathBuf) -> Result<()> {
    let tokens = json!({
        "colors": {
            "primary": {
                "main": "#0070f3",
                "light": "#3291ff",
                "dark": "#0051cc"
            },
            "secondary": {
                "main": "#7928ca",
                "light": "#9a4ed4",
                "dark": "#5a1f96"
            },
            "accent": {
                "main": "#f81ce5",
                "light": "#fa4aed",
                "dark": "#c00eb3"
            },
            "neutral": {
                "background": "#ffffff",
                "surface": "#fafafa",
                "text": "#000000",
                "textSecondary": "#666666"
            }
        },
        "typography": {
            "fontFamilies": {
                "heading": "system-ui, -apple-system, sans-serif",
                "body": "system-ui, -apple-system, sans-serif",
                "monospace": "'Courier New', monospace"
            },
            "fontSizes": {
                "h1": "2.5rem",
                "h2": "2rem",
                "h3": "1.5rem",
                "body": "1rem",
                "small": "0.875rem"
            },
            "fontWeights": {
                "normal": 400,
                "medium": 500,
                "bold": 700
            }
        },
        "spacing": {
            "base": "8px",
            "small": "8px",
            "medium": "16px",
            "large": "24px",
            "xlarge": "32px"
        },
        "breakpoints": {
            "mobile": "320px",
            "tablet": "768px",
            "desktop": "1024px"
        }
    });

    let content = serde_json::to_string_pretty(&tokens)
        .context("Failed to serialize design tokens")?;

    std::fs::write(path, content)
        .context("Failed to write design tokens")?;

    Ok(())
}

fn generate_design_rules(
    path: &PathBuf,
    pages: &[String],
    sections: &[String],
    components: &[String],
    cards: &[String],
) -> Result<()> {
    let mut content = String::new();
    content.push_str("# Design Rules\n\n");
    content.push_str("## Overview\n\n");
    content.push_str("Design rules extracted from feature plans for AI agents to follow.\n\n");
    content.push_str("---\n\n");

    if !pages.is_empty() {
        content.push_str("## Pages\n\n");
        for page in pages {
            content.push_str(&format!("- {}\n", page));
        }
        content.push_str("\n");
    }

    if !sections.is_empty() {
        content.push_str("## Sections\n\n");
        for section in sections {
            content.push_str(&format!("- {}\n", section));
        }
        content.push_str("\n");
    }

    if !components.is_empty() {
        content.push_str("## Components\n\n");
        for component in components {
            content.push_str(&format!("- {}\n", component));
        }
        content.push_str("\n");
    }

    if !cards.is_empty() {
        content.push_str("## Cards/UI Elements\n\n");
        for card in cards {
            content.push_str(&format!("- {}\n", card));
        }
        content.push_str("\n");
    }

    content.push_str("## Design Guidelines\n\n");
    content.push_str("1. Follow the design tokens defined in `design-tokens.json`\n");
    content.push_str("2. Maintain consistency across all pages and components\n");
    content.push_str("3. Ensure accessibility (WCAG 2.1 AA)\n");
    content.push_str("4. Use responsive design patterns\n");
    content.push_str("5. Follow the established color system and typography\n\n");

    std::fs::write(path, content)
        .context("Failed to write design rules")?;

    Ok(())
}


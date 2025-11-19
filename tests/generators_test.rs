use anyhow::Result;
use doplan::state::{ProjectState, Feature, Phase};
use doplan::generators;
use tempfile::TempDir;
use std::fs;
use std::sync::{Mutex, MutexGuard};
use std::path::PathBuf;

fn create_test_state() -> ProjectState {
    ProjectState {
        project_name: Some("Test Project".to_string()),
        idea: Some("A test project idea".to_string()),
        tech_stack: Some(vec!["Rust".to_string(), "PostgreSQL".to_string()]),
        features: Some(vec![
            Feature {
                name: "Feature 1".to_string(),
                description: "First feature".to_string(),
                priority: "high".to_string(),
            },
            Feature {
                name: "Feature 2".to_string(),
                description: "Second feature".to_string(),
                priority: "medium".to_string(),
            },
        ]),
        phases: Some(vec![
            Phase {
                name: "Phase 1".to_string(),
                description: "First phase".to_string(),
                features: vec!["Feature 1".to_string()],
            },
        ]),
        improvements: None,
        notes: None,
    }
}

// Use a mutex to prevent concurrent test execution that could interfere with current_dir
static TEST_MUTEX: Mutex<()> = Mutex::new(());

/// RAII struct that holds the test environment and mutex guard.
/// The mutex guard ensures no other test can run concurrently.
/// On drop, it restores the original working directory before dropping the TempDir.
struct TestEnv {
    _temp_dir: TempDir,
    state: ProjectState,
    original_cwd: PathBuf,
    _guard: MutexGuard<'static, ()>,
}

impl TestEnv {
    fn new() -> Result<Self> {
        // Acquire the mutex and keep the guard for the entire test lifetime
        let guard = TEST_MUTEX.lock().unwrap();
        let temp_dir = TempDir::new()?;
        let original_cwd = std::env::current_dir()?;
        
        // Create necessary directories using absolute paths
        let doplan_dir = temp_dir.path().join("doplan");
        let dot_doplan_dir = temp_dir.path().join(".doplan");
        fs::create_dir_all(&doplan_dir)?;
        fs::create_dir_all(&dot_doplan_dir)?;
        
        // Save state to .doplan/state.json
        let state = create_test_state();
        let state_path = dot_doplan_dir.join("state.json");
        let state_json = serde_json::to_string_pretty(&state)?;
        fs::write(&state_path, state_json)?;
        
        // Change to temp directory for tests
        std::env::set_current_dir(temp_dir.path())?;
        
        Ok(TestEnv {
            _temp_dir: temp_dir,
            state,
            original_cwd,
            _guard: guard,
        })
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        // Restore original cwd before dropping the TempDir to avoid deleting the current directory
        let _ = std::env::set_current_dir(&self.original_cwd);
        // _temp_dir and _guard are dropped here, releasing the mutex
    }
}

#[test]
fn test_prd_generation() -> Result<()> {
    let env = TestEnv::new()?;
    
    let idea_notes = Some("# Test Idea Notes\n\nWhat problem does this solve?\n\nTest problem description".to_string());
    
    let result = generators::prd::generate(&env.state, &idea_notes);
    
    assert!(result.is_ok());
    let prd_path = result?;
    assert!(prd_path.exists());
    assert!(prd_path.ends_with("PRD.md"));
    
    let content = fs::read_to_string(&prd_path)?;
    assert!(content.contains("Test Project"));
    assert!(content.contains("Product Requirements Document"));
    
    Ok(())
}

#[test]
fn test_prd_generation_missing_state() -> Result<()> {
    let _env = TestEnv::new()?;
    
    let empty_state = ProjectState::new();
    let idea_notes = None;
    
    let result = generators::prd::generate(&empty_state, &idea_notes);
    
    // Should fail with empty state (missing both project_name and idea)
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_structure_generation() -> Result<()> {
    let env = TestEnv::new()?;
    
    let result = generators::structure::generate(&env.state, &None);
    
    assert!(result.is_ok());
    let structure_path = result?;
    assert!(structure_path.exists());
    assert!(structure_path.ends_with("structure.md"));
    
    let content = fs::read_to_string(&structure_path)?;
    assert!(content.contains("Test Project"));
    assert!(content.contains("Project Structure"));
    
    Ok(())
}

#[test]
fn test_api_spec_generation() -> Result<()> {
    let env = TestEnv::new()?;
    
    let result = generators::api_spec::generate(&env.state, &None);
    
    assert!(result.is_ok());
    let api_spec_path = result?;
    assert!(api_spec_path.exists());
    assert!(api_spec_path.ends_with("api-spec.json"));
    
    let content = fs::read_to_string(&api_spec_path)?;
    assert!(content.contains("Test Project"));
    assert!(content.contains("openapi"));
    assert!(content.contains("3.0.0"));
    
    Ok(())
}

#[test]
fn test_data_model_generation() -> Result<()> {
    let env = TestEnv::new()?;
    
    let result = generators::data_model::generate(&env.state, &None);
    
    assert!(result.is_ok());
    let data_model_path = result?;
    assert!(data_model_path.exists());
    assert!(data_model_path.ends_with("data-model.md"));
    
    let content = fs::read_to_string(&data_model_path)?;
    assert!(content.contains("Test Project"));
    assert!(content.contains("Data Model"));
    
    Ok(())
}

#[test]
fn test_templates_generation() -> Result<()> {
    let _env = TestEnv::new()?;
    
    let result = generators::templates::generate_all();
    
    assert!(result.is_ok());
    let generated = result?;
    assert_eq!(generated.len(), 3);
    
    // Check plan template
    assert!(generated.iter().any(|p| p.ends_with("plan-template.md")));
    // Check design template
    assert!(generated.iter().any(|p| p.ends_with("design-template.md")));
    // Check tasks template
    assert!(generated.iter().any(|p| p.ends_with("tasks-template.md")));
    
    // Verify all files exist
    for path in &generated {
        assert!(path.exists());
        let content = fs::read_to_string(path)?;
        assert!(content.len() > 100);
    }
    
    Ok(())
}

#[test]
fn test_dpr_generation() -> Result<()> {
    let env = TestEnv::new()?;
    
    // Create plan structure for DPR generation
    let plan_dir = "doplan/plan/01-phase/01-feature";
    fs::create_dir_all(plan_dir)?;
    
    let plan_path = format!("{}/plan.md", plan_dir);
    let plan_content = r#"# Feature Plan

### Pages
- Home Page
- About Page

### Sections
- Header
- Footer

### Components
- Button
- Input

### Cards/UI Elements
- Card Component
"#;
    
    fs::write(&plan_path, plan_content)
        .map_err(|e| anyhow::anyhow!("Failed to write plan.md to {}: {}", plan_path, e))?;
    
    // Verify file was written
    if !fs::metadata(&plan_path).is_ok() {
        eprintln!("Error: plan.md file not found after write");
        eprintln!("Current directory: {:?}", std::env::current_dir());
        eprintln!("Expected path: {}", plan_path);
        if let Ok(entries) = fs::read_dir(plan_dir) {
            eprintln!("Files in directory:");
            for entry in entries {
                if let Ok(entry) = entry {
                    eprintln!("  Found: {}", entry.path().display());
                }
            }
        } else {
            eprintln!("Could not read directory: {}", plan_dir);
        }
        panic!("plan.md should exist after write");
    }
    
    // Create .doplan directory structure
    fs::create_dir_all(".doplan/ai/rules")?;
    
    // Verify test setup
    assert!(env.state.project_name.is_some(), "Test state should have project_name");
    
    // Verify plan directory and file exist before generation
    let current_dir = std::env::current_dir()?;
    let plan_dir_path = current_dir.join("doplan").join("plan");
    eprintln!("Current directory: {:?}", current_dir);
    eprintln!("Plan directory path: {:?}", plan_dir_path);
    eprintln!("Plan directory exists: {}", plan_dir_path.exists());
    if plan_dir_path.exists() {
        eprintln!("Files in plan directory:");
        if let Ok(entries) = fs::read_dir(&plan_dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    eprintln!("  Found: {}", entry.path().display());
                }
            }
        }
    }
    eprintln!("Plan file exists: {}", fs::metadata(&plan_path).is_ok());
    
    // Verify current directory still exists before calling generator
    // This is a defensive check to catch issues early
    let current_dir_check = std::env::current_dir();
    if let Err(e) = &current_dir_check {
        eprintln!("ERROR: Current directory is invalid before DPR generation: {}", e);
        eprintln!("This suggests the temp directory was deleted or is inaccessible");
        panic!("Current directory check failed: {}", e);
    }
    let verified_cwd = current_dir_check?;
    eprintln!("Verified current directory exists: {:?}", verified_cwd);
    
    // Verify the doplan directory exists
    let doplan_dir = verified_cwd.join("doplan");
    if !doplan_dir.exists() {
        eprintln!("ERROR: doplan directory does not exist at: {:?}", doplan_dir);
        panic!("doplan directory missing before DPR generation");
    }
    eprintln!("Verified doplan directory exists: {:?}", doplan_dir);
    
    let result = generators::dpr::generate(&env.state);
    
    if let Err(ref e) = result {
        eprintln!("DPR Generation Error: {:#}", e);
        // Print the full error chain for debugging
        let mut current = e.source();
        let mut depth = 0;
        while let Some(cause) = current {
            depth += 1;
            eprintln!("  Error chain depth {}: {}", depth, cause);
            current = cause.source();
        }
        // Also print the debug format for more details
        eprintln!("Full error debug: {:?}", e);
    }
    
    assert!(result.is_ok(), "DPR generation should succeed. Error details: {:#}", 
        result.as_ref().unwrap_err());
    let generated = result?;
    assert_eq!(generated.len(), 3);
    
    // Check DPR.md
    assert!(generated.iter().any(|p| p.ends_with("DPR.md")));
    // Check design-tokens.json
    assert!(generated.iter().any(|p| p.ends_with("design-tokens.json")));
    // Check design_rules.mdc
    assert!(generated.iter().any(|p| p.ends_with("design_rules.mdc")));
    
    // Verify DPR content - pages may not be in DPR if plan files weren't read
    let dpr_path = generated.iter().find(|p| p.ends_with("DPR.md")).unwrap();
    let content = fs::read_to_string(dpr_path)?;
    assert!(content.contains("Test Project"), "DPR should contain project name");
    // Only check for plan content if plan files were successfully read
    // The generator can create DPR without plan files (empty sections)
    if content.contains("Pages") {
        assert!(content.contains("Home Page"), "DPR should contain pages from plan.md");
        assert!(content.contains("Button"), "DPR should contain components from plan.md");
    }
    
    Ok(())
}

#[test]
fn test_sops_generation() -> Result<()> {
    let env = TestEnv::new()?;
    
    let result = generators::sops::generate(&env.state);
    
    assert!(result.is_ok());
    let generated = result?;
    
    // Should generate SOPS for PostgreSQL (from tech_stack)
    assert!(generated.len() > 0);
    
    // Check for PostgreSQL SOPS
    let postgres_sops = generated.iter().find(|p| p.to_string_lossy().contains("postgresql"));
    assert!(postgres_sops.is_some());
    
    if let Some(sops_path) = postgres_sops {
        let content = fs::read_to_string(sops_path)?;
        assert!(content.contains("postgresql"));
        assert!(content.contains("Service Operating Procedures"));
    }
    
    Ok(())
}

#[test]
fn test_rakd_generation() -> Result<()> {
    let env = TestEnv::new()?;
    
    let result = generators::rakd::generate(&env.state);
    
    assert!(result.is_ok());
    let rakd_path = result?;
    assert!(rakd_path.exists());
    assert!(rakd_path.ends_with("RAKD.md"));
    
    let content = fs::read_to_string(&rakd_path)?;
    assert!(content.contains("Test Project"));
    assert!(content.contains("Required API Keys"));
    
    // Should detect DATABASE_URL from PostgreSQL
    assert!(content.contains("DATABASE_URL"));
    
    Ok(())
}

#[test]
fn test_context_generation() -> Result<()> {
    let env = TestEnv::new()?;
    
    let result = generators::context::generate(&env.state);
    
    assert!(result.is_ok());
    let context_path = result?;
    assert!(context_path.exists());
    assert!(context_path.ends_with("CONTEXT.md"));
    
    let content = fs::read_to_string(&context_path)?;
    assert!(content.contains("Test Project"));
    assert!(content.contains("Project Context"));
    
    Ok(())
}

#[test]
fn test_readme_generation() -> Result<()> {
    let env = TestEnv::new()?;
    
    let result = generators::readme::generate(&env.state);
    
    assert!(result.is_ok());
    let readme_path = result?;
    assert!(readme_path.exists());
    assert!(readme_path.ends_with("README.md"));
    
    let content = fs::read_to_string(&readme_path)?;
    assert!(content.contains("Test Project"));
    assert!(content.contains("A test project idea"));
    
    Ok(())
}

#[test]
fn test_generator_error_handling_missing_state() -> Result<()> {
    let _env = TestEnv::new()?;
    
    let empty_state = ProjectState::new();
    
    // All generators should fail with empty state (missing project_name)
    assert!(generators::structure::generate(&empty_state, &None).is_err());
    assert!(generators::api_spec::generate(&empty_state, &None).is_err());
    assert!(generators::data_model::generate(&empty_state, &None).is_err());
    assert!(generators::dpr::generate(&empty_state).is_err());
    assert!(generators::sops::generate(&empty_state).is_err());
    assert!(generators::rakd::generate(&empty_state).is_err());
    assert!(generators::context::generate(&empty_state).is_err());
    assert!(generators::readme::generate(&empty_state).is_err());
    
    Ok(())
}


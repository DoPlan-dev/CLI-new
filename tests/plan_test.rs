use anyhow::Result;
use doplan::state::{ProjectState, Feature, Phase};
use doplan::commands;
use tempfile::TempDir;
use std::fs;
use std::sync::Mutex;

fn create_test_state() -> ProjectState {
    ProjectState {
        project_name: Some("Test Project".to_string()),
        idea: Some("A test project idea".to_string()),
        tech_stack: Some(vec!["Rust".to_string()]),
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
                features: vec!["Feature 1".to_string(), "Feature 2".to_string()],
            },
        ]),
        improvements: None,
        notes: None,
    }
}

// Use a mutex to prevent concurrent test execution
static TEST_MUTEX: Mutex<()> = Mutex::new(());

fn setup_test_env() -> Result<(TempDir, ProjectState)> {
    let _lock = TEST_MUTEX.lock().unwrap();
    let temp_dir = TempDir::new()?;
    
    // Create necessary directories
    let doplan_dir = temp_dir.path().join("doplan");
    let dot_doplan_dir = temp_dir.path().join(".doplan");
    fs::create_dir_all(&doplan_dir)?;
    fs::create_dir_all(&dot_doplan_dir)?;
    
    // Create PRD.md
    fs::write(
        doplan_dir.join("PRD.md"),
        "# Product Requirements Document\n\n**Project:** Test Project\n\nThis is a test PRD with sufficient content to pass validation.",
    )?;
    
    // Save state to .doplan/state.json
    let state = create_test_state();
    let state_path = dot_doplan_dir.join("state.json");
    let state_json = serde_json::to_string_pretty(&state)?;
    fs::write(&state_path, state_json)?;
    
    // Change to temp directory for tests
    std::env::set_current_dir(temp_dir.path())?;
    
    Ok((temp_dir, state))
}

fn cleanup_test_env(temp_dir: TempDir) {
    let _lock = TEST_MUTEX.lock().unwrap();
    drop(temp_dir);
}

#[tokio::test]
async fn test_plan_command_success() -> Result<()> {
    let (temp_dir, _) = setup_test_env()?;
    
    let result = commands::plan::execute(vec![]).await;
    
    assert!(result.is_ok());
    
    // Verify phase directory was created
    let phase_dir = temp_dir.path().join("doplan").join("plan").join("01-phase-1");
    assert!(phase_dir.exists());
    
    // Verify phase-plan.md was created
    let phase_plan = phase_dir.join("phase-plan.md");
    assert!(phase_plan.exists());
    let content = fs::read_to_string(&phase_plan)?;
    assert!(content.contains("Phase 1"));
    
    // Verify phase-progress.json was created
    let phase_progress = phase_dir.join("phase-progress.json");
    assert!(phase_progress.exists());
    let progress_content = fs::read_to_string(&phase_progress)?;
    assert!(progress_content.contains("Phase 1"));
    assert!(progress_content.contains("not_started"));
    
    // Verify feature directories were created
    let feature1_dir = phase_dir.join("01-feature-1");
    let feature2_dir = phase_dir.join("02-feature-2");
    assert!(feature1_dir.exists());
    assert!(feature2_dir.exists());
    
    // Verify feature files were created
    assert!(feature1_dir.join("plan.md").exists());
    assert!(feature1_dir.join("design.md").exists());
    assert!(feature1_dir.join("tasks.md").exists());
    assert!(feature1_dir.join("progress.json").exists());
    
    cleanup_test_env(temp_dir);
    Ok(())
}

#[tokio::test]
async fn test_plan_command_missing_prd() -> Result<()> {
    let (temp_dir, _) = setup_test_env()?;
    
    // Remove PRD
    fs::remove_file(temp_dir.path().join("doplan").join("PRD.md"))?;
    
    let result = commands::plan::execute(vec![]).await;
    
    // Should return Ok but not create plan structure
    assert!(result.is_ok());
    
    // Verify plan directory was not created
    let plan_dir = temp_dir.path().join("doplan").join("plan");
    assert!(!plan_dir.exists());
    
    cleanup_test_env(temp_dir);
    Ok(())
}

#[tokio::test]
async fn test_plan_command_missing_phases() -> Result<()> {
    let (temp_dir, _) = setup_test_env()?;
    
    // Update state to have no phases
    let mut state = create_test_state();
    state.phases = None;
    let state_path = temp_dir.path().join(".doplan").join("state.json");
    let state_json = serde_json::to_string_pretty(&state)?;
    fs::write(&state_path, state_json)?;
    
    let result = commands::plan::execute(vec![]).await;
    
    // Should return Ok but not create plan structure
    assert!(result.is_ok());
    
    // Verify plan directory was not created
    let plan_dir = temp_dir.path().join("doplan").join("plan");
    assert!(!plan_dir.exists());
    
    cleanup_test_env(temp_dir);
    Ok(())
}

#[tokio::test]
async fn test_plan_command_missing_state() -> Result<()> {
    let (temp_dir, _) = setup_test_env()?;
    
    // Remove state file
    fs::remove_file(temp_dir.path().join(".doplan").join("state.json"))?;
    
    let result = commands::plan::execute(vec![]).await;
    
    // Should fail because state is required
    assert!(result.is_err());
    
    cleanup_test_env(temp_dir);
    Ok(())
}

#[tokio::test]
async fn test_plan_command_generates_all_files() -> Result<()> {
    let (temp_dir, _) = setup_test_env()?;
    
    let result = commands::plan::execute(vec![]).await;
    assert!(result.is_ok());
    
    let phase_dir = temp_dir.path().join("doplan").join("plan").join("01-phase-1");
    let feature_dir = phase_dir.join("01-feature-1");
    
    // Check all phase files
    assert!(phase_dir.join("phase-plan.md").exists());
    assert!(phase_dir.join("phase-progress.json").exists());
    
    // Check all feature files
    assert!(feature_dir.join("plan.md").exists());
    assert!(feature_dir.join("design.md").exists());
    assert!(feature_dir.join("tasks.md").exists());
    assert!(feature_dir.join("progress.json").exists());
    
    // Verify file contents are valid
    let plan_content = fs::read_to_string(feature_dir.join("plan.md"))?;
    assert!(plan_content.contains("Feature 1"));
    assert!(plan_content.contains("Feature Plan"));
    
    let design_content = fs::read_to_string(feature_dir.join("design.md"))?;
    assert!(design_content.contains("Feature 1"));
    assert!(design_content.contains("Design Specification"));
    
    let tasks_content = fs::read_to_string(feature_dir.join("tasks.md"))?;
    assert!(tasks_content.contains("Feature 1"));
    assert!(tasks_content.contains("Task 1: Setup"));
    
    let progress_content = fs::read_to_string(feature_dir.join("progress.json"))?;
    assert!(progress_content.contains("Feature 1"));
    assert!(progress_content.contains("not_started"));
    
    cleanup_test_env(temp_dir);
    Ok(())
}

#[tokio::test]
async fn test_plan_command_multiple_phases() -> Result<()> {
    let (temp_dir, _) = setup_test_env()?;
    
    // Update state to have multiple phases
    let mut state = create_test_state();
    state.phases = Some(vec![
        Phase {
            name: "Phase 1".to_string(),
            description: "First phase".to_string(),
            features: vec!["Feature 1".to_string()],
        },
        Phase {
            name: "Phase 2".to_string(),
            description: "Second phase".to_string(),
            features: vec!["Feature 2".to_string()],
        },
    ]);
    let state_path = temp_dir.path().join(".doplan").join("state.json");
    let state_json = serde_json::to_string_pretty(&state)?;
    fs::write(&state_path, state_json)?;
    
    let result = commands::plan::execute(vec![]).await;
    assert!(result.is_ok());
    
    // Verify both phase directories were created
    let phase1_dir = temp_dir.path().join("doplan").join("plan").join("01-phase-1");
    let phase2_dir = temp_dir.path().join("doplan").join("plan").join("02-phase-2");
    assert!(phase1_dir.exists());
    assert!(phase2_dir.exists());
    
    // Verify phase 1 has feature 1
    assert!(phase1_dir.join("01-feature-1").exists());
    
    // Verify phase 2 has feature 2
    assert!(phase2_dir.join("01-feature-2").exists());
    
    cleanup_test_env(temp_dir);
    Ok(())
}


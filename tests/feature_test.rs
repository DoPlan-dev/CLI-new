use anyhow::Result;
use doplan::state::{ProjectState, Feature};
use doplan::commands::feature;
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
        ]),
        phases: Some(vec![]),
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
    let dot_doplan_dir = temp_dir.path().join(".doplan");
    fs::create_dir_all(&dot_doplan_dir)?;
    
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
async fn test_feature_list() -> Result<()> {
    let (temp_dir, _) = setup_test_env()?;
    
    let result = feature::execute(vec!["list".to_string()]).await;
    
    // Should succeed and list features
    assert!(result.is_ok());
    
    cleanup_test_env(temp_dir);
    Ok(())
}

#[tokio::test]
async fn test_feature_list_empty() -> Result<()> {
    let (temp_dir, _) = setup_test_env()?;
    
    // Create empty state
    let empty_state = ProjectState {
        project_name: Some("Test".to_string()),
        idea: None,
        tech_stack: None,
        features: None,
        phases: None,
        improvements: None,
        notes: None,
    };
    let state_path = temp_dir.path().join(".doplan").join("state.json");
    let state_json = serde_json::to_string_pretty(&empty_state)?;
    fs::write(&state_path, state_json)?;
    
    let result = feature::execute(vec!["list".to_string()]).await;
    
    // Should succeed even with no features
    assert!(result.is_ok());
    
    cleanup_test_env(temp_dir);
    Ok(())
}

#[tokio::test]
async fn test_feature_menu() -> Result<()> {
    let (temp_dir, _) = setup_test_env()?;
    
    let result = feature::execute(vec![]).await;
    
    // Should show menu
    assert!(result.is_ok());
    
    cleanup_test_env(temp_dir);
    Ok(())
}

#[tokio::test]
async fn test_feature_invalid_command() -> Result<()> {
    let (temp_dir, _) = setup_test_env()?;
    
    let result = feature::execute(vec!["invalid".to_string()]).await;
    
    // Should return Ok (error is printed, not returned)
    assert!(result.is_ok());
    
    cleanup_test_env(temp_dir);
    Ok(())
}

#[tokio::test]
async fn test_feature_missing_state() -> Result<()> {
    let (temp_dir, _) = setup_test_env()?;
    
    // Remove state file
    fs::remove_file(temp_dir.path().join(".doplan").join("state.json"))?;
    
    let result = feature::execute(vec!["list".to_string()]).await;
    
    // ProjectState::load() returns empty state if file doesn't exist, so this should succeed
    // and just show "No features found" message
    assert!(result.is_ok());
    
    cleanup_test_env(temp_dir);
    Ok(())
}

#[tokio::test]
async fn test_feature_incomplete_state() -> Result<()> {
    let (temp_dir, _) = setup_test_env()?;
    
    // Create state without project_name
    let incomplete_state = ProjectState {
        project_name: None,
        idea: None,
        tech_stack: None,
        features: None,
        phases: None,
        improvements: None,
        notes: None,
    };
    let state_path = temp_dir.path().join(".doplan").join("state.json");
    let state_json = serde_json::to_string_pretty(&incomplete_state)?;
    fs::write(&state_path, state_json)?;
    
    let result = feature::execute(vec!["add".to_string()]).await;
    
    // Should fail because project_name is required
    assert!(result.is_err());
    
    cleanup_test_env(temp_dir);
    Ok(())
}


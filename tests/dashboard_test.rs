use anyhow::Result;
use doplan::commands::dashboard;
use tempfile::TempDir;
use std::fs;
use std::sync::Mutex;
use serde_json::json;

// Use a mutex to prevent concurrent test execution
static TEST_MUTEX: Mutex<()> = Mutex::new(());

fn setup_test_env() -> Result<TempDir> {
    let _lock = TEST_MUTEX.lock().unwrap();
    let temp_dir = TempDir::new()?;
    
    // Create necessary directories
    let dot_doplan_dir = temp_dir.path().join(".doplan");
    fs::create_dir_all(&dot_doplan_dir)?;
    
    // Create a sample dashboard.json
    let dashboard_json = json!({
        "project_name": "Test Project",
        "overall_progress": 50.0,
        "phases": [
            {
                "name": "Phase 1",
                "progress": 100.0,
                "status": "completed",
                "features": [
                    {
                        "name": "Feature 1",
                        "priority": "high",
                        "progress": 100.0,
                        "status": "completed",
                        "tasks": {
                            "total": 3,
                            "completed": 3,
                            "in_progress": 0,
                            "not_started": 0,
                            "blocked": 0
                        }
                    }
                ]
            }
        ],
        "updated_at": "2025-11-19T01:00:00Z"
    });
    
    let dashboard_path = dot_doplan_dir.join("dashboard.json");
    let dashboard_content = serde_json::to_string_pretty(&dashboard_json)?;
    fs::write(&dashboard_path, dashboard_content)?;
    
    // Change to temp directory for tests
    std::env::set_current_dir(temp_dir.path())?;
    
    Ok(temp_dir)
}

fn cleanup_test_env(temp_dir: TempDir) {
    let _lock = TEST_MUTEX.lock().unwrap();
    drop(temp_dir);
}

#[tokio::test]
async fn test_dashboard_display() -> Result<()> {
    let temp_dir = setup_test_env()?;
    
    let result = dashboard::execute(vec![]).await;
    
    // Should succeed and display dashboard
    assert!(result.is_ok());
    
    cleanup_test_env(temp_dir);
    Ok(())
}

#[tokio::test]
async fn test_dashboard_missing_file() -> Result<()> {
    let temp_dir = setup_test_env()?;
    
    // Remove dashboard file
    fs::remove_file(temp_dir.path().join(".doplan").join("dashboard.json"))?;
    
    let result = dashboard::execute(vec![]).await;
    
    // Should succeed but suggest running /progress
    assert!(result.is_ok());
    
    cleanup_test_env(temp_dir);
    Ok(())
}

#[tokio::test]
async fn test_dashboard_invalid_json() -> Result<()> {
    let temp_dir = setup_test_env()?;
    
    // Write invalid JSON
    let dashboard_path = temp_dir.path().join(".doplan").join("dashboard.json");
    fs::write(&dashboard_path, "invalid json")?;
    
    let result = dashboard::execute(vec![]).await;
    
    // Should fail because JSON is invalid
    assert!(result.is_err());
    
    cleanup_test_env(temp_dir);
    Ok(())
}


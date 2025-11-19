use anyhow::{Context, Result};
use std::path::PathBuf;
use serde_json::json;
use crate::state::ProjectState;
use crate::utils;

pub fn generate(state: &ProjectState, _idea_notes: &Option<String>) -> Result<PathBuf> {
    // Validate state
    if state.project_name.is_none() {
        anyhow::bail!("Project state is incomplete: missing project_name. Run /discuss first.");
    }

    let doplan_dir = utils::doplan_dir()
        .context("Failed to get doplan directory")?;
    let contracts_dir = doplan_dir.join("contracts");
    utils::ensure_dir(&contracts_dir)
        .context("Failed to create contracts directory")?;

    let api_spec_path = contracts_dir.join("api-spec.json");
    utils::validate_write_path(&api_spec_path)
        .context("Invalid path for api-spec.json")?;

    let project_name = state.project_name.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("Untitled Project");

    // Create OpenAPI 3.0 specification
    let mut spec = json!({
        "openapi": "3.0.0",
        "info": {
            "title": project_name,
            "version": "1.0.0",
            "description": format!("API specification for {}", project_name),
            "contact": {
                "name": "API Support",
                "email": "support@example.com"
            }
        },
        "servers": [
            {
                "url": "https://api.example.com/v1",
                "description": "Production server"
            },
            {
                "url": "https://api-staging.example.com/v1",
                "description": "Staging server"
            }
        ],
        "paths": {},
        "components": {
            "schemas": {},
            "securitySchemes": {
                "bearerAuth": {
                    "type": "http",
                    "scheme": "bearer",
                    "bearerFormat": "JWT"
                }
            }
        },
        "security": [
            {
                "bearerAuth": []
            }
        ]
    });

    // Add basic endpoints based on features
    if let Some(features) = &state.features {
        let paths = spec["paths"].as_object_mut().unwrap();
        
        // Add health check endpoint
        paths.insert("/health".to_string(), json!({
            "get": {
                "summary": "Health check",
                "description": "Check API health status",
                "operationId": "healthCheck",
                "tags": ["Health"],
                "responses": {
                    "200": {
                        "description": "API is healthy",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "status": {
                                            "type": "string",
                                            "example": "ok"
                                        },
                                        "timestamp": {
                                            "type": "string",
                                            "format": "date-time"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }));

        // Add example endpoints for each feature
        for feature in features {
            let feature_slug = feature.name.to_lowercase()
                .replace(" ", "-")
                .replace("_", "-");
            
            // GET endpoint
            let get_path = format!("/{}", feature_slug);
            paths.insert(get_path.clone(), json!({
                "get": {
                    "summary": format!("Get {}", feature.name),
                    "description": feature.description,
                    "operationId": format!("get{}", feature.name.replace(" ", "")),
                    "tags": [feature.name.clone()],
                    "responses": {
                        "200": {
                            "description": "Successful response",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object"
                                    }
                                }
                            }
                        },
                        "404": {
                            "description": "Not found"
                        }
                    }
                }
            }));

            // POST endpoint
            let post_path = format!("/{}", feature_slug);
            if let Some(existing) = paths.get_mut(&post_path) {
                if let Some(obj) = existing.as_object_mut() {
                    obj.insert("post".to_string(), json!({
                        "summary": format!("Create {}", feature.name),
                        "description": format!("Create a new {}", feature.name),
                        "operationId": format!("create{}", feature.name.replace(" ", "")),
                        "tags": [feature.name.clone()],
                        "requestBody": {
                            "required": true,
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object"
                                    }
                                }
                            }
                        },
                        "responses": {
                            "201": {
                                "description": "Created successfully"
                            },
                            "400": {
                                "description": "Bad request"
                            }
                        }
                    }));
                }
            }
        }
    }

    // Add common schemas
    let schemas = spec["components"]["schemas"].as_object_mut().unwrap();
    
    schemas.insert("Error".to_string(), json!({
        "type": "object",
        "properties": {
            "code": {
                "type": "integer",
                "format": "int32"
            },
            "message": {
                "type": "string"
            },
            "details": {
                "type": "object"
            }
        },
        "required": ["code", "message"]
    }));

    schemas.insert("Success".to_string(), json!({
        "type": "object",
        "properties": {
            "success": {
                "type": "boolean"
            },
            "message": {
                "type": "string"
            }
        },
        "required": ["success"]
    }));

    // Write JSON file
    let json_content = serde_json::to_string_pretty(&spec)
        .context("Failed to serialize API spec to JSON")?;
    
    // Validate JSON content
    utils::validate_content(&json_content, 100)
        .context("Generated API spec content is too short")?;

    std::fs::write(&api_spec_path, &json_content)
        .with_context(|| format!("Failed to write API spec to: {}", api_spec_path.display()))?;

    // Verify file was written successfully
    utils::verify_file_write(&api_spec_path, 100)
        .context("API spec file verification failed")?;

    Ok(api_spec_path)
}


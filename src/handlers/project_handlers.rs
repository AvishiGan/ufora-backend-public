use std::sync::Arc;

use axum::{ Json, extract::State, http::StatusCode };
use surrealdb::{ engine::remote::ws::Client, Surreal };

use crate::models::project;

#[derive(serde::Deserialize, Debug)]
pub struct ProjectCreateRequest {
    title: Option<String>,
    content: Option<ProjectCreateRequestContent>,
}

#[derive(serde::Deserialize, Debug)]
pub struct ProjectCreateRequestContent {
    pub time: Option<i128>,
    pub blocks: Option<Vec<project::ProjectBlock>>,
    pub version: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ProjectCreateResponse {
    message: String,
}

pub async fn create_a_project(
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,
    Json(project_request): Json<ProjectCreateRequest>
) -> (StatusCode, Json<ProjectCreateResponse>) {
    match project_request.content {
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ProjectCreateResponse {
                    message: "content is required".to_string(),
                }),
            );
        }
        Some(_) => {}
    }

    let project_create_request_content = project_request.content.unwrap();

    let new_project_content = project::ProjectContent {
        time: Some(project_create_request_content.time.unwrap().to_string()),
        blocks: project_create_request_content.blocks,
        version: project_create_request_content.version,
    };

    let new_project = project::Project::new(project_request.title, new_project_content);

    match new_project.save(db, Some(claim.get_surrealdb_thing())).await {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(ProjectCreateResponse {
                    message: "Project created successfully".to_string(),
                }),
            );
        }
        Err(e) => {
            println!("Error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ProjectCreateResponse {
                    message: format!("Failed to create the project: {}", e),
                }),
            );
        }
    }
}

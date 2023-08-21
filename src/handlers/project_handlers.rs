use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use axum_valid::Valid;
use surrealdb::{engine::remote::ws::Client, Surreal};
use validator::Validate;

use crate::models::project;

#[derive(serde::Deserialize, serde::Serialize, Debug, Validate)]
pub struct ProjectCreateRequest {
    #[validate(
        length(min = 5, message = "Title must be at least 5 characters long"),
        required(message = "Title of the project is required")
    )]
    title: Option<String>,
    #[validate(required(message = "Content of the project is required"))]
    content: Option<ProjectCreateRequestContent>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Validate)]
pub struct ProjectCreateRequestContent {
    #[validate(required(message = "Time created is required"))]
    pub time: Option<i128>,
    #[validate(required(message = "Blocks of the project are required"))]
    pub blocks: Option<Vec<project::ProjectBlock>>,
    #[validate(required(message = "Version is required"))]
    pub version: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ProjectCreateResponse {
    message: String,
}

pub async fn create_a_project(
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,
    Valid(Json(project_request)): Valid<Json<ProjectCreateRequest>>,
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
        time: project_create_request_content.time.unwrap().to_string(),
        blocks: project_create_request_content.blocks.unwrap(),
        version: project_create_request_content.version.unwrap(),
    };

    let new_project = project::Project::new(project_request.title.unwrap(), new_project_content);

    match new_project
        .save(db, Some(claim.get_surrealdb_thing()))
        .await
    {
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

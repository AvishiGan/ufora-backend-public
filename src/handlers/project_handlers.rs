use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_valid::Valid;
use surrealdb::{engine::remote::ws::Client, Surreal};
use validator::Validate;

use crate::models::project;

#[derive(serde::Serialize)]
pub enum ProjectRouteResponse {
    Success { message: String },
    Failed { message: String },
}

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
                StatusCode::CREATED,
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

pub async fn get_projects_of_the_user_by_user_id(
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,
) -> (StatusCode, Json<Vec<project::Project>>) {
    let projects = project::Project::get_projects_by_user_id(db, claim.get_surrealdb_thing()).await;

    match projects {
        Ok(projects) => (StatusCode::OK, Json(projects)),
        Err(e) => {
            println!("Error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

pub async fn delete_a_project_of_the_user(
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,
    Path(project_id): Path<String>,
) -> (StatusCode, Json<ProjectRouteResponse>) {
    match project::Project::delete_a_project_belongs_to_user(
        db,
        project_id,
        claim.get_surrealdb_thing(),
    )
    .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(ProjectRouteResponse::Success {
                message: "Project deleted successfully".to_string(),
            }),
        ),
        Err(e) => {
            println!("Error: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(ProjectRouteResponse::Failed { message: e }),
            )
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Validate)]
pub struct ProjectUpdateRequest {
    #[validate(
        length(min = 5, message = "Title must be at least 5 characters long"),
        required(message = "Title of the project is required")
    )]
    title: Option<String>,
    #[validate(required(message = "Content of the project is required"))]
    content: Option<ProjectCreateRequestContent>,
}

pub async fn update_project_content(
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,
    Path(project_id): Path<String>,
    Valid(Json(project_request)): Valid<Json<ProjectUpdateRequest>>,
) -> (StatusCode, Json<ProjectRouteResponse>) {
    if let Some(mut project) = project::Project::get_project_by_id(db.clone(), project_id).await {
        let new_content = project_request.content.unwrap();

        project.set_project_content(project::ProjectContent {
            time: new_content.time.unwrap().to_string(),
            blocks: new_content.blocks.unwrap(),
            version: new_content.version.unwrap(),
        });

        project.set_project_title(project_request.title.unwrap());

        match project
            .update_project_of_user_by_id(db, claim.get_surrealdb_thing())
            .await
        {
            Ok(_) => (
                StatusCode::OK,
                Json(ProjectRouteResponse::Success {
                    message: "Project updated successfully".to_string(),
                }),
            ),
            Err(e) => {
                println!("Error: {}", e);
                (
                    StatusCode::NOT_FOUND,
                    Json(ProjectRouteResponse::Failed { message: e }),
                )
            }
        }
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json(ProjectRouteResponse::Failed {
                message: "Project with given id not found".to_string(),
            }),
        );
    }
}

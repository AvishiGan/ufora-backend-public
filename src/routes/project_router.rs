use std::sync::Arc;

use crate::handlers::project_handlers::{create_a_project, get_projects_of_the_user_by_user_id};

use axum::{routing::{post, get}, Router};
use surrealdb::{engine::remote::ws::Client, Surreal};

pub fn get_project_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/project/create", post(create_a_project))
        .route("/api/project/get", get(get_projects_of_the_user_by_user_id))
}

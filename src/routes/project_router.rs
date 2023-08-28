use std::sync::Arc;

use crate::handlers::project_handlers::{
    create_a_project, delete_a_project_of_the_user, get_projects_of_the_user_by_user_id,
    update_project_content,
};

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use surrealdb::{engine::remote::ws::Client, Surreal};

pub fn get_project_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/project/create", post(create_a_project))
        .route("/api/project/get", get(get_projects_of_the_user_by_user_id))
        .route(
            "/api/project/delete/:project_id",
            delete(delete_a_project_of_the_user),
        )
        .route(
            "/api/project/update/:project_id",
            put(update_project_content),
        )
}

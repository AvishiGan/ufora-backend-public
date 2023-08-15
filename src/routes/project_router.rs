use std::sync::Arc;

use crate::handlers::project_handlers;

use axum::{ Router, routing::post };
use surrealdb::{ Surreal, engine::remote::ws::Client };

pub fn get_project_router() -> Router<Arc<Surreal<Client>>> {
    Router::new().route("/api/project/create", post(project_handlers::create_a_project))
}

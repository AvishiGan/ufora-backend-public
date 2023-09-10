use std::sync::Arc;

use crate::handlers::login_handlers;

use axum::{routing::post, Router};
use surrealdb::{engine::remote::ws::Client, Surreal};

use login_handlers::login_via_platform;

pub fn get_login_router() -> Router<Arc<Surreal<Client>>> {
    Router::new().route("/api/login", post(login_via_platform))
}

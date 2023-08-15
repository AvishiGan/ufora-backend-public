use std::sync::Arc;

use crate::handlers::blog_handlers;

use axum::{ Router, routing::post };
use surrealdb::{ Surreal, engine::remote::ws::Client };

pub fn get_blog_router() -> Router<Arc<Surreal<Client>>> {
    Router::new().route("/api/blog/create", post(blog_handlers::create_a_blog))
}

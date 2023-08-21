use std::sync::Arc;

use crate::handlers::post_handlers;

use axum::{
    Router, 
    routing::{post, delete}
};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub fn get_post_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
    .route("/api/post/create",post(post_handlers::create_post))
    .route("/api/post/delete/:post_id",delete(post_handlers::delete_post_by_id))   
}
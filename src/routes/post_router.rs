use std::sync::Arc;

use crate::handlers::post_handlers::{
    add_or_remove_reaction_to_a_post, create_post, delete_post_by_id, add_a_comment
};

use axum::{
    routing::{delete, patch, post},
    Router,
};
use surrealdb::{engine::remote::ws::Client, Surreal};

pub fn get_post_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/post/create", post(create_post))
        .route("/api/post/delete/:post_id", delete(delete_post_by_id))
        .route(
            "/api/post/reaction/:post_id",
            patch(add_or_remove_reaction_to_a_post),
        )
        .route(
            "/api/post/comment/add/:post_id",
            patch(add_a_comment),
        )
}

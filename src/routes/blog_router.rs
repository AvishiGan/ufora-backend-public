use std::sync::Arc;

use crate::handlers::blog_handlers::{
    create_a_blog, delete_a_blog_of_the_user, get_blogs_of_the_user_by_user_id,
};

use axum::{
    routing::{delete, get, post},
    Router,
};
use surrealdb::{engine::remote::ws::Client, Surreal};

pub fn get_blog_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/blog/create", post(create_a_blog))
        .route("/api/blog/get", get(get_blogs_of_the_user_by_user_id))
        .route(
            "/api/blog/delete/:blog_id",
            delete(delete_a_blog_of_the_user),
        )
}

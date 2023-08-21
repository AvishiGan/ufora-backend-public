use std::sync::Arc;

use axum::{ extract::{State, Path}, Json, http::StatusCode };
use surrealdb::{ Surreal, engine::remote::ws::Client };

use crate::models::post::{Post, AccessLevel};

#[derive(serde::Deserialize, Debug)]
pub struct CreatePostRequest {
    caption: Option<String>,
    access_level: Option<AccessLevel>,
    content: Option<String>,
}

#[derive(serde::Serialize, Debug)]
pub struct CreatePostResponse {
    message: String,
}

pub async fn create_post(
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,
    Json(request): Json<CreatePostRequest>
) -> (StatusCode, Json<CreatePostResponse>) {
    let post = Post::new(request.caption, request.access_level, request.content);

    match post.save(db, claim.get_surrealdb_thing()).await {
        Ok(_) =>
            (
                StatusCode::OK,
                Json(CreatePostResponse { message: "Post created successfully".to_string() }),
            ),
        Err(e) => {
            println!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(CreatePostResponse { message: e }))
        }
    }
}

pub async fn get_posts_for_profile(
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,
) -> (StatusCode, Json<Vec<Post>>) {

    let logged_user = claim.get_surrealdb_thing();

    let posts = Post::get_post_by_user_id(db, logged_user).await;

    match posts {
        Ok(posts) => (StatusCode::OK, Json(posts)),
        Err(e) => {
            println!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

pub async fn delete_post_by_id(
    State(db): State<Arc<Surreal<Client>>>,
    Path(post_id): Path<String>,
) -> (StatusCode, Json<CreatePostResponse>) {

    let post = Post::delete_post_by_id(db, post_id).await;

    match post {
        Ok(_) =>
            (
                StatusCode::OK,
                Json(CreatePostResponse { message: "Post deleted successfully".to_string() }),
            ),
        Err(e) => {
            println!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(CreatePostResponse { message: e }))
        }
    }
}
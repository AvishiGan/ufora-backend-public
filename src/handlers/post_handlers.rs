use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use surrealdb::{
    engine::remote::ws::Client,
    sql::{Id, Thing},
    Surreal,
};

use crate::models::post::{AccessLevel, Post};

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
    Json(request): Json<CreatePostRequest>,
) -> (StatusCode, Json<CreatePostResponse>) {
    let post = Post::new(request.caption, request.access_level, request.content);

    match post.save(db, claim.get_surrealdb_thing()).await {
        Ok(_) => (
            StatusCode::OK,
            Json(CreatePostResponse {
                message: "Post created successfully".to_string(),
            }),
        ),
        Err(e) => {
            println!("{:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreatePostResponse { message: e }),
            )
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
        Ok(_) => (
            StatusCode::OK,
            Json(CreatePostResponse {
                message: "Post deleted successfully".to_string(),
            }),
        ),
        Err(e) => {
            println!("{:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreatePostResponse { message: e }),
            )
        }
    }
}

pub async fn add_or_remove_reaction_to_a_post(
    State(db): State<Arc<Surreal<Client>>>,
    Path(post_id): Path<String>,
    claim: crate::models::user_claim::Claim,
) -> (StatusCode, Json<CreatePostResponse>) {
    let post = Post::add_or_remove_reaction(
        db,
        Thing {
            tb: "post".to_string(),
            id: Id::String(post_id),
        },
        claim.get_surrealdb_thing(),
    )
    .await;

    match post {
        Ok(_) => (
            StatusCode::OK,
            Json(CreatePostResponse {
                message: "Reaction added successfully".to_string(),
            }),
        ),
        Err(e) => {
            println!("{:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreatePostResponse { message: e }),
            )
        }
    }
}

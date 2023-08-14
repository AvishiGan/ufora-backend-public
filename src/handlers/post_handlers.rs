use std::sync::Arc;

use axum::{extract::{State, FromRequest}, Json, http::{Request, request, StatusCode}, body::Body, Extension, RequestExt};
use lettre::transport::smtp::extension;
use surrealdb::{Surreal, engine::remote::ws::Client, sql::Thing};

use crate::models::{post::Post, user_claim::Claim};

#[derive(serde::Deserialize,Debug)]
pub struct CreatePostRequest {
    caption: Option<String>,
    access_level: Option<String>,
    content: Option<String>
}

#[derive(serde::Serialize,Debug)]
pub struct  CreatePostResponse {
    message: String
}

pub async fn create_post(
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,
    Json(request): Json<CreatePostRequest> 
) -> (StatusCode,Json<CreatePostResponse>) {

    let post = Post::new(
        request.caption,
        request.access_level,
        request.content
    );

    match post.save(db, claim.get_surrealdb_thing()).await {
        Ok(_) => (StatusCode::OK,Json(CreatePostResponse { message: "Post created successfully".to_string() })),
        Err(e) => {
            println!("{:?}",e);
            (StatusCode::INTERNAL_SERVER_ERROR,Json(CreatePostResponse { message: e })) 
        }   
    }

}
use std::sync::Arc;

use axum::{ extract::State, Json, http::StatusCode };
use surrealdb::{ Surreal, engine::remote::ws::Client };

use crate::models::blog;

#[derive(serde::Deserialize, Debug)]
pub struct BlogCreateRequest {
    pub title: Option<String>,
    pub content : BlogCreateRequestContent, 
}

#[derive(serde::Deserialize, Debug)]
pub struct BlogCreateRequestContent {
    pub time: Option<i128>,
    pub blocks: Option<Vec<blog::BlogBlock>>,
    pub version: Option<String> 
}

#[derive(serde::Serialize)]
pub struct BlogCreateResponse {
    message: String,
}

pub async fn create_a_blog(
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,
    Json(blog_request): Json<BlogCreateRequest>
) -> (StatusCode, Json<BlogCreateResponse>) {
    
    let blog_create_request_content = blog_request.content;

    let new_blog_content = blog::BlogContent {
        time: Some(blog_create_request_content.time.unwrap().to_string()),
        blocks: blog_create_request_content.blocks,
        version: blog_create_request_content.version,
    };

    let new_blog = blog::Blog::new(blog_request.title,new_blog_content);

    match new_blog.save(db, Some(claim.get_surrealdb_thing())).await {
        Ok(_) => (StatusCode::OK, Json(BlogCreateResponse { message: "success".to_string() })),
        Err(_) =>
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BlogCreateResponse { message: "failed".to_string() }),
            ),
    }
}

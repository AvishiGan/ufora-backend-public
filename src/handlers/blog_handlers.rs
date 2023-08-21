use std::sync::Arc;

use axum::{ extract::State, Json, http::StatusCode };
use axum_valid::Valid;
use surrealdb::{ Surreal, engine::remote::ws::Client };
use validator::Validate;

use crate::models::blog;

#[derive(serde::Deserialize, serde::Serialize, Debug, Validate)]
pub struct BlogCreateRequest {
    #[validate(length(min = 5, message = "Title must be at least 5 characters long"))]
    pub title: String,
    #[validate(required(message = "Content is required"))]
    pub content: Option<BlogCreateRequestContent>,
}

#[derive(serde::Deserialize, serde::Serialize,Debug, Validate)]
pub struct BlogCreateRequestContent {
    #[validate(required(message = "Time created is required"))]
    pub time: Option<i128>,
    #[validate(required(message = "Blocks of the blog are required"))]
    pub blocks: Option<Vec<blog::BlogBlock>>,
    #[validate(required(message = "Version is required"))]
    pub version: Option<String>,
}

#[derive(serde::Serialize)]
pub struct BlogCreateResponse {
    message: String,
}

pub async fn create_a_blog(
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,
    Valid(Json(blog_request)): Valid<Json<BlogCreateRequest>>
) -> (StatusCode, Json<BlogCreateResponse>) {

    let blog_create_request_content = blog_request.content.unwrap();

    let new_blog_content = blog::BlogContent {
        time: blog_create_request_content.time.unwrap().to_string(),
        blocks: blog_create_request_content.blocks.unwrap(),
        version: blog_create_request_content.version.unwrap(),
    };

    let new_blog = blog::Blog::new(blog_request.title, new_blog_content);

    match new_blog.save(db, Some(claim.get_surrealdb_thing())).await {
        Ok(_) =>
            (
                StatusCode::OK,
                Json(BlogCreateResponse { message: "Blog created successfully".to_string() }),
            ),
        Err(e) => {
            println!("Error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(BlogCreateResponse { message: e }))
        }
    }
}

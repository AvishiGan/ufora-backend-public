use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_valid::Valid;
use surrealdb::{engine::remote::ws::Client, Surreal};
use validator::Validate;

use crate::models::blog;

#[derive(serde::Serialize)]
pub enum BlogRouteResponse {
    Success { message: String },
    Failed { message: String },
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Validate)]
pub struct BlogCreateRequest {
    #[validate(
        length(min = 5, message = "Title must be at least 5 characters long"),
        required(message = "Title of the blog is required")
    )]
    pub title: Option<String>,
    #[validate(required(message = "Content of the blog is required"))]
    pub content: Option<BlogCreateRequestContent>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Validate)]
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
    Valid(Json(blog_request)): Valid<Json<BlogCreateRequest>>,
) -> (StatusCode, Json<BlogCreateResponse>) {
    let blog_create_request_content = blog_request.content.unwrap();

    let new_blog_content = blog::BlogContent {
        time: blog_create_request_content.time.unwrap().to_string(),
        blocks: blog_create_request_content.blocks.unwrap(),
        version: blog_create_request_content.version.unwrap(),
    };

    let new_blog = blog::Blog::new(blog_request.title.unwrap(), new_blog_content);

    match new_blog.save(db, Some(claim.get_surrealdb_thing())).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(BlogCreateResponse {
                message: "Blog created successfully".to_string(),
            }),
        ),
        Err(e) => {
            println!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BlogCreateResponse { message: e }),
            )
        }
    }
}

pub async fn get_blogs_of_the_user_by_user_id(
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,
) -> (StatusCode, Json<Vec<blog::Blog>>) {
    let blogs = blog::Blog::get_blogs_by_user_id(db, claim.get_surrealdb_thing()).await;

    match blogs {
        Ok(blogs) => (StatusCode::OK, Json(blogs)),
        Err(e) => {
            println!("Error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

pub async fn delete_a_blog_of_the_user(
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,
    Path(blog_id): Path<String>,
) -> (StatusCode, Json<BlogRouteResponse>) {
    match blog::Blog::delete_a_blog_belongs_to_user(db, blog_id, claim.get_surrealdb_thing()).await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(BlogRouteResponse::Failed {
                message: "Blog deleted successfully".to_string(),
            }),
        ),
        Err(e) => {
            println!("Error: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(BlogRouteResponse::Success { message: e }),
            )
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Validate)]
pub struct BlogUpdateRequest {
    #[validate(
        length(min = 5, message = "Title must be at least 5 characters long"),
        required(message = "Title of the blog is required")
    )]
    pub title: Option<String>,
    #[validate(required(message = "New content of the blog is required"))]
    pub content: Option<BlogCreateRequestContent>,
}

pub async fn update_blog_content(
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,
    Path(blog_id): Path<String>,
    Valid(Json(blog_request)): Valid<Json<BlogUpdateRequest>>,
) -> (StatusCode, Json<BlogRouteResponse>) {
    if let Some(mut blog) = blog::Blog::get_blog_by_id(db.clone(), blog_id).await {
        let new_content = blog_request.content.unwrap();

        blog.set_blog_content(blog::BlogContent {
            time: new_content.time.unwrap().to_string(),
            blocks: new_content.blocks.unwrap(),
            version: new_content.version.unwrap(),
        });

        blog.set_blog_title(blog_request.title.unwrap());

        match blog
            .update_blog_of_user_by_id(db, claim.get_surrealdb_thing())
            .await
        {
            Ok(_) => (
                StatusCode::OK,
                Json(BlogRouteResponse::Success {
                    message: "Blog updated successfully".to_string(),
                }),
            ),
            Err(e) => {
                println!("Error: {}", e);
                (
                    StatusCode::UNAUTHORIZED,
                    Json(BlogRouteResponse::Failed { message: e }),
                )
            }
        }
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json(BlogRouteResponse::Failed {
                message: "Blog for the give id not found".to_string(),
            }),
        );
    }
}

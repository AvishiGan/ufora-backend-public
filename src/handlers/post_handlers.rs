use std::sync::Arc;

use axum::{extract::{State, FromRequest}, Json, http::{Request, request}, body::Body, Extension, RequestExt};
use lettre::transport::smtp::extension;
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::models::{post::Post, user_claim::Claim};

#[derive(serde::Deserialize,Debug)]
pub struct CreatePostRequest {
    caption: Option<String>,
    access_level: Option<String>,
    content: Option<String>
}


pub async fn create_post(
    // Extension(request): Extension<Claim>,
    // State(db): State<Arc<Surreal<Client>>>,
    // request: Request<Body>,
) -> () {

    // let body = request.body();

    // println!("{:?}",request);

    // let claim = request.extensions().clone().get::<crate::models::user_claim::Claim>().unwrap();

    // println!("{:?}",body);

    // let post_details = request.extensions().clone().get::<CreatePostRequest>();

    // println!("{:?}",post_details);

    // let post_details = axum::extract::Json::<CreatePostRequest>::from_request_parts(&request).await.unwrap().0;
    

    // let post_details:CreatePostRequest = axum::extract::Json::from_request(&request).await.unwrap().0;

    // let post = Post::new(
    //     post_details.caption,
    //     post_details.access_level,
    //     post_details.content.clone()
    // );

    // post.save(db).await;

    // println!("{:?}",post);
}
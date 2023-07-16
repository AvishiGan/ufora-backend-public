use std::sync::Arc;

use axum::{extract::State, Json, response::{IntoResponse, Response}, http::header};
use serde_json::json;
use surrealdb::{Surreal, engine::remote::ws::Client};
use tower_cookies::Cookie;

use crate::services::jwt;

#[derive(serde::Deserialize,Debug)]
pub struct LoginRequest {
    pub username: Option<String>,
    pub password: Option<String>,
}


pub async fn login_via_platform(
    State(db): State<Arc<Surreal<Client>>>,
    Json(login_request): Json<LoginRequest>,
) -> impl IntoResponse {

    let user:Vec<LoginRequest> = db.select("user").await.unwrap();

    println!("{:?}",user);

    let token = jwt::get_jwt().await.unwrap();

    let cookie = Cookie::build("_Secure-jwt", token)
        .domain("localhost")
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({
        "message": "Login successful",
    }).to_string());

    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    response       
}
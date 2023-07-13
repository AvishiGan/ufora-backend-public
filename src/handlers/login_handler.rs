use std::sync::Arc;

use axum::{extract::State, TypedHeader, Json, response::{IntoResponse, Response}, http::header};
use axum_extra::extract::cookie::Cookie;
use serde_json::json;
use surrealdb::{Surreal, engine::remote::ws::Client};

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
    let token = jwt::get_jwt().await.unwrap();

    let cookie = Cookie::build("_Secure-jwt", token)
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
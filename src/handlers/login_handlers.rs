use std::sync::Arc;

use axum::{extract::State, Json, http::StatusCode};
use surrealdb::{Surreal, engine::remote::ws::Client};
use tower_cookies::{Cookie, Cookies};

use crate::{services::jwt, models::user::User};

// request struct for login
#[derive(serde::Deserialize,Debug)]
pub struct LoginRequest {
    pub username: Option<String>,
    pub password: Option<String>,
}

// response enumeration for login
#[derive(serde::Serialize)]
pub enum LoginResponse {
    Success {message:String, token:String},
    InvalidLogin {message:String},
    InternalServerError
}

// login handler
pub async fn login_via_platform(
    State(db): State<Arc<Surreal<Client>>>,
    cookies: Cookies,
    Json(login_request): Json<LoginRequest>,
) -> (StatusCode,Json<LoginResponse>) {

    // retrieve user from database
    let user = User::retrieve_user_from_database(db.clone(),login_request.username.unwrap()).await;

    match user {
        Err(_) => {
            return (StatusCode::NOT_FOUND,Json(LoginResponse::InvalidLogin { message: "Invalid Login Credentials".to_string()}))
        }
        Ok(_) => {}
    }

    let user = user.unwrap();

    // check if user is locked
    match user.is_user_locked() {
        true => {
            return (StatusCode::UNAUTHORIZED,Json(LoginResponse::InvalidLogin { message: "User Account is Locked. Please verify email to continue".to_string()}))
        }
        false => {}
    }

    // check whether password is correct
    match crate::services::password::verify_password(login_request.password.unwrap(),user.get_password().unwrap()) {
        Ok(false)  => {
            let new_invalid_login_attempts = user.invalid_login_attempts.unwrap() + 1;
            user.update_login_attempts(db.clone(),new_invalid_login_attempts).await;
            return (StatusCode::UNAUTHORIZED,Json(LoginResponse::InvalidLogin { message: "Invalid Login Credentials".to_string()} ))
        }
        Ok(true) => {}
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR,Json(LoginResponse::InternalServerError))
        }
    }

    // create jwt token
    let token = jwt::get_jwt(user.get_user_id().id.to_string(),user.get_user_type()).await.unwrap();

    // set cookie
    let cookie = Cookie::build("_Secure-jwt", token.clone())
        .domain("localhost")
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

    cookies.add(cookie);

    (StatusCode::OK,Json(LoginResponse::Success { message: "Login Successful".to_string(), token}))
}
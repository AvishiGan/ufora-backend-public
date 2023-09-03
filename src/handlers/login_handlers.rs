use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use surrealdb::{engine::remote::ws::Client, Surreal};
use tower_cookies::{Cookie, Cookies};

use crate::{
    models::user::{ClubOfficial, User},
    services::jwt,
};

// request struct for login
#[derive(serde::Deserialize, Debug)]
pub struct LoginRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

// response enumeration for login
#[derive(serde::Serialize)]
pub enum LoginResponse {
    Success { message: String, token: String },
    InvalidLogin { message: String },
    InternalServerError,
}

// login handler
pub async fn login_via_platform(
    State(db): State<Arc<Surreal<Client>>>,
    cookies: Cookies,
    Json(login_request): Json<LoginRequest>,
) -> (StatusCode, Json<LoginResponse>) {
    match login_request.password.clone() {
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(LoginResponse::InvalidLogin {
                    message: "Invalid Login Credentials".to_string(),
                }),
            );
        }
        Some(_) => {}
    }

    // retrieve user from database
    let user = User::get_user_by_email_or_username(
        db.clone(),
        login_request.email.clone(),
        login_request.username.clone(),
    )
    .await;

    match user {
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(LoginResponse::InvalidLogin {
                    message: "Invalid Login Credentials".to_string(),
                }),
            );
        }
        Ok(_) => {}
    }

    let user = user.unwrap();

    // check if user is locked
    match user.is_user_locked() {
        true => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(LoginResponse::InvalidLogin {
                    message: "User Account is Locked. Please verify email to continue".to_string(),
                }),
            );
        }
        false => {}
    }

    // check whether password is correct
    match crate::services::password::verify_password(
        login_request.password.unwrap(),
        user.get_password().unwrap(),
    ) {
        Ok(false) => {
            // update invalid login attempts
            let new_invalid_login_attempts = user.invalid_login_attempts.unwrap() + 1;
            user.update_login_attempts(db.clone(), new_invalid_login_attempts)
                .await;
            return (
                StatusCode::UNAUTHORIZED,
                Json(LoginResponse::InvalidLogin {
                    message: "Invalid Login Credentials".to_string(),
                }),
            );
        }
        Ok(true) => {}
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LoginResponse::InternalServerError),
            );
        }
    }

    // create jwt token
    let token = jwt::get_jwt(user.get_id().id.to_string(), user.get_user_type())
        .await
        .unwrap();

    user.update_login_attempts(db.clone(), 0).await;

    // create cookie with flags
    let cookie = Cookie::build("_Secure-jwt", token.clone())
        .domain("localhost")
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

    // set cookie
    cookies.add(cookie);

    (
        StatusCode::OK,
        Json(LoginResponse::Success {
            message: "Login Successful".to_string(),
            token,
        }),
    )
}

pub async fn club_login(
    State(db): State<Arc<Surreal<Client>>>,
    user: crate::models::user_claim::Claim,
    Path(club_id): Path<String>,
) -> (StatusCode, Json<LoginResponse>) {
    if let Ok(club) = User::get_user_by_id(db.clone(), club_id.clone()).await {
        if club.get_user_type() == "club" {
            if let Some(club_officials) = club.get_club_officials() {
                let user_id = user.get_surrealdb_thing();
                let mut club_official_info: Option<ClubOfficial> = None;
                for club_official in club_officials {
                    if club_official.get_user_id() == user_id {
                        club_official_info = Some(club_official);
                        break;
                    }
                }
                if let Some(club_official_info) = club_official_info {
                    match jwt::get_club_jwt(club_id, club_official_info.get_role()) {
                        Ok(token) => (
                            StatusCode::OK,
                            Json(LoginResponse::Success {
                                message: "Login Successful".to_string(),
                                token,
                            }),
                        ),
                        Err(_) => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(LoginResponse::InternalServerError),
                        ),
                    }
                } else {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(LoginResponse::InvalidLogin {
                            message: "You do not have access to this club account".to_string(),
                        }),
                    )
                }
            } else {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(LoginResponse::InvalidLogin {
                        message: "Invalid Login Credentials".to_string(),
                    }),
                )
            }
        } else {
            (
                StatusCode::UNAUTHORIZED,
                Json(LoginResponse::InvalidLogin {
                    message: "You do not have access to this club account".to_string(),
                }),
            )
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(LoginResponse::InvalidLogin {
                message: "Invalid Login Credentials".to_string(),
            }),
        )
    }
}

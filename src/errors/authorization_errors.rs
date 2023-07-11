#![allow(dead_code)]
use axum::response::{IntoResponse,Response};


pub enum AuthorizationError {
    InvalidCredentials { msg: String }, // Invalid username or password
    InvalidToken { msg: String }, // Invalid token
    ExpiredToken { msg: String }, // Expired token
    InactiveUser { msg: String }, // Inactive user means locked account or disabled account
    MissingCredentials { msg: String }, // Missing username or password
}

impl IntoResponse for AuthorizationError {

    fn into_response(self) -> Response {

        match self {
            AuthorizationError::InvalidCredentials { msg } => {
                (axum::http::StatusCode::UNAUTHORIZED, msg).into_response()
            },
            AuthorizationError::InvalidToken { msg } => {
                (axum::http::StatusCode::UNAUTHORIZED, msg).into_response()
            },
            AuthorizationError::ExpiredToken { msg } => {
                (axum::http::StatusCode::UNAUTHORIZED, msg).into_response()
            },
            AuthorizationError::InactiveUser { msg } => {
                (axum::http::StatusCode::UNAUTHORIZED, msg).into_response()
            },
            AuthorizationError::MissingCredentials { msg } => {
                (axum::http::StatusCode::BAD_REQUEST, msg).into_response()
            },
        }

    }

}
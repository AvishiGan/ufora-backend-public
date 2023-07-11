#![allow(dead_code)]
use axum::response::{IntoResponse,Response};


pub enum AuthenticationError {
    InvalidCredentials { msg: String }, // Invalid username or password
    InvalidToken { msg: String }, // Invalid token
    ExpiredToken { msg: String }, // Expired token
    InactiveUser { msg: String }, // Inactive user means locked account or disabled account
    MissingCredentials { msg: String }, // Missing username or password
}

impl IntoResponse for AuthenticationError {

    fn into_response(self) -> Response {

        match self {
            AuthenticationError::InvalidCredentials { msg } => {
                (axum::http::StatusCode::UNAUTHORIZED, msg).into_response()
            },
            AuthenticationError::InvalidToken { msg } => {
                (axum::http::StatusCode::UNAUTHORIZED, msg).into_response()
            },
            AuthenticationError::ExpiredToken { msg } => {
                (axum::http::StatusCode::UNAUTHORIZED, msg).into_response()
            },
            AuthenticationError::InactiveUser { msg } => {
                (axum::http::StatusCode::UNAUTHORIZED, msg).into_response()
            },
            AuthenticationError::MissingCredentials { msg } => {
                (axum::http::StatusCode::BAD_REQUEST, msg).into_response()
            },
        }

    }

}
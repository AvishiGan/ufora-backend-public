#![allow(dead_code)]
use axum::response::{IntoResponse,Response};

pub enum ValidationError {
    InvalidEmail,
    InvalidPassword,
    InvalidUsername,
    InvalidAge,
    InvalidPhoneNumber,
    InvalidDate { msg: String },
    InvalidCredentials { msg: String },
} 

impl IntoResponse for ValidationError {

    fn into_response(self) -> Response {

        match self {
            ValidationError::InvalidEmail => {
                (axum::http::StatusCode::BAD_REQUEST, "Invalid Email").into_response()
            },
            ValidationError::InvalidPassword => {
                (axum::http::StatusCode::BAD_REQUEST, "Invalid Password Format").into_response()
            },
            ValidationError::InvalidUsername => {
                (axum::http::StatusCode::BAD_REQUEST, "Invalid Username Format").into_response()
            },
            ValidationError::InvalidAge => {
                (axum::http::StatusCode::BAD_REQUEST, "Invalid Age").into_response()
            },
            ValidationError::InvalidPhoneNumber => {
                (axum::http::StatusCode::BAD_REQUEST, "Invalid Phone Number").into_response()
            },
            ValidationError::InvalidDate { msg } => {
                (axum::http::StatusCode::BAD_REQUEST, msg).into_response()
            },
            ValidationError::InvalidCredentials { msg } => {
                (axum::http::StatusCode::NOT_FOUND, msg).into_response()
            }
        }

    }

}
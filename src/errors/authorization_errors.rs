#![allow(dead_code)]
use axum::response::{IntoResponse,Response};

pub enum AuthorizationError {
    AccessDeniedError { msg: String },
    InsufficientPermissionsError { msg: String },
    ResourseNotFoundError { msg: String },
}

impl IntoResponse for AuthorizationError {

    fn into_response(self) -> Response {

        match self {
            AuthorizationError::AccessDeniedError { msg } => {
                (axum::http::StatusCode::FORBIDDEN, msg).into_response()
            },
            AuthorizationError::InsufficientPermissionsError { msg } => {
                (axum::http::StatusCode::FORBIDDEN, msg).into_response()
            },
            AuthorizationError::ResourseNotFoundError { msg } => {
                (axum::http::StatusCode::NOT_FOUND, msg).into_response()
            },
        }

    }

}
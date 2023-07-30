#![allow(dead_code)]
use axum::response::{IntoResponse,Response};


pub enum DatabaseError {
    ConnectionError { msg: String },
    QueryError { msg: String },
    DuplicateEntryError { msg: String },

}

impl IntoResponse for DatabaseError {

    fn into_response(self) -> Response {

        match self {
            DatabaseError::ConnectionError { msg } => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            },
            DatabaseError::QueryError { msg } => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            },
            DatabaseError::DuplicateEntryError { msg } => {
                (axum::http::StatusCode::BAD_REQUEST, msg).into_response()
            },
        }

    }

}
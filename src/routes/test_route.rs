#![allow(dead_code,unused)]
use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post}, http::StatusCode, Json, extract::State
};
use surrealdb::{Surreal, engine::remote::ws::Client, sql::{Statement,statements::{BeginStatement, CancelStatement, SetStatement}, Statements, Subquery, Thing}};

use chrono::prelude::*;

use crate::models::{undergraduate::Undergraduate,user::User};

use crate::services::{otp::get_an_otp,email::send_email};

pub fn get_test_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/test", post(test_handler))
}

async fn test_handler(
    State(db) : State<Arc<Surreal<Client>>>,
) -> Result<(),StatusCode> {
    Ok(())
}
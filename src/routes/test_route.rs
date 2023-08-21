#![allow(dead_code,unused)]
use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post}, http::{StatusCode, HeaderMap, request, Request}, Json, extract::{State, FromRequest}, Extension, body::Body
};
use surrealdb::{Surreal, engine::remote::ws::Client, sql::{Statement,statements::{BeginStatement, CancelStatement, SetStatement}, Statements, Subquery, Thing}};

use chrono::prelude::*;

use crate::{models::{undergraduate::Undergraduate,user::User}, handlers::test_handlers};

use crate::services::{otp::get_an_otp,email::send_email};

use crate::services::query_builder::{Column,OrderBy,Item,Expression,ExpressionConnector,Group,DatabaseObject,Return,get_select_query, get_insert_query_by_fields,get_insert_query_for_an_object, get_insert_query_for_an_array_of_objects,get_delete_query_for_specific_record,
get_delete_query_with_conditions, get_create_query_for_an_object};

pub fn get_test_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/test", post(test_handler))
        .route("/api/test/:id", get(test_handlers::test_route))
}

async fn test_handler(
    State(db) : State<Arc<Surreal<Client>>>,
    request: Request<Body>
) -> Result<(),StatusCode> {
    let claim = request.extensions().get::<crate::models::user_claim::Claim>().unwrap();

    Ok(())
}
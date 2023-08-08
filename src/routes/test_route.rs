#![allow(dead_code,unused)]
use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post}, http::{StatusCode, HeaderMap, request, Request}, Json, extract::{State, FromRequest}, Extension, body::Body
};
use surrealdb::{Surreal, engine::remote::ws::Client, sql::{Statement,statements::{BeginStatement, CancelStatement, SetStatement}, Statements, Subquery, Thing}};

use chrono::prelude::*;

use crate::models::{undergraduate::Undergraduate,user::User};

use crate::services::{otp::get_an_otp,email::send_email};

use crate::services::queryBuilder::{Column,OrderBy,Item,Expression,ExpressionConnector,Group,DatabaseObject,get_select_query, get_insert_query_by_fields,get_insert_query_for_an_object, get_insert_query_for_an_array_of_objects,get_delete_query_for_specific_record,
get_delete_query_with_conditions};

pub fn get_test_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/test", post(test_handler))
}

async fn test_handler(
    State(db) : State<Arc<Surreal<Client>>>,
    request: Request<Body>
) -> Result<(),StatusCode> {
    let claim = request.extensions().get::<crate::models::user_claim::Claim>().unwrap();

    let conditions = vec![
        (Expression::EqualTo("name".to_string(),"'test'".to_string()),ExpressionConnector::And),
        (Expression::EqualTo("age".to_string(),"20".to_string()),ExpressionConnector::End),
    ];


    let query = get_delete_query_with_conditions("test".to_string(),conditions);

    println!("{:?}",query);

    let response = db.query(query.clone()).await;

    match response {
        Ok(_) => {
            // println!("{:?}",response);
        },
        Err(e) => {
            println!("{:?}",e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // let users:Vec<User> = response.unwrap().take(0).unwrap();

    // println!("{:?}",users);

    Ok(())
}
use std::sync::Arc;

use axum::{
    Router,
    routing::get, http::StatusCode, Json, extract::State
};
use surrealdb::{Surreal, engine::remote::ws::Client, sql::{Statement,statements::{BeginStatement, CancelStatement, SetStatement}, Statements, Subquery, Thing}};

use crate::models::{company::Company,user::User};

pub fn get_test_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/test", get(test_handler))
}

#[derive(serde::Deserialize,serde::Serialize)]
pub struct UndergraduateDetails {
    name:Option<String>,
    username:Option<String>,
    email:Option<String>,
    password:Option<String>,
}

async fn test_handler(
    State(db): State<Arc<Surreal<Client>>>,
    Json(company_details): Json<UndergraduateDetails>,
) -> Result<String,StatusCode> {

    Ok("set ne".to_string())
}
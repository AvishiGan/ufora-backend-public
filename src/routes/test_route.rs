#![allow(dead_code,unused)]
use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post}, http::StatusCode, Json, extract::State
};
use surrealdb::{Surreal, engine::remote::ws::Client, sql::{Statement,statements::{BeginStatement, CancelStatement, SetStatement}, Statements, Subquery, Thing}};

use crate::models::{undergraduate::Undergraduate,user::User};

pub fn get_test_router() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/test", post(test_handler))
}

#[derive(serde::Deserialize)]
pub struct UndergraduateRegistrationRequest {
    name:Option<String>,
    username:Option<String>,
    email:Option<String>,
    password:Option<String>,
}

impl UndergraduateRegistrationRequest {
    pub fn get_undergraduate_and_user_models(self) -> (Undergraduate,User) {
        (Undergraduate::from(self.name,self.email),User::from(self.username,self.password))
    }
}

#[derive(serde::Serialize)]
pub struct UndergraduateRegistrationResponse {
    message:String
}

async fn test_handler(
    State(db): State<Arc<Surreal<Client>>>,
    Json(company_details): Json<UndergraduateRegistrationRequest>,
) -> Result<Json<UndergraduateRegistrationResponse>,StatusCode> {

    let (undergraduate,user) = company_details.get_undergraduate_and_user_models();

    let response = db.query(undergraduate.get_register_query().await.unwrap()).await;

    match response {
        Ok(mut response) => {

            let undergraduate_id:Result<Option<Thing>,surrealdb::Error> = response.take(0);
            let response = db.query(user.get_create_user_query("undergraduate".to_string(),undergraduate_id.unwrap()).await.unwrap()).await;
            
            match response {
                Ok(_) => Ok(Json(UndergraduateRegistrationResponse {message:"Undergraduate account has been created successfully".to_string()})),
                Err(e) => {println!("{:?}",e); Err(StatusCode::INTERNAL_SERVER_ERROR)}
            }

        },
        Err(e) => {println!("{:?}",e); Err(StatusCode::INTERNAL_SERVER_ERROR)}
    }

}
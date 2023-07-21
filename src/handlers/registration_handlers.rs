use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json, extract::State};

use surrealdb::{Surreal, engine::remote::ws::Client, sql::Thing};

use crate::models::{company::Company,undergraduate::Undergraduate,user::User};

// request struct for registration of an undergraduate
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

// response struct for registration of an undergraduate
#[derive(serde::Serialize)]
pub struct UndergraduateRegistrationResponse {
    message:String
}

// handler for registration of an undergraduate
pub async fn register_an_undergraduate(
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

// request struct for registration of a company
#[derive(serde::Deserialize)]
pub struct CompanyRegistrationRequest {
    name:Option<String>,
    username:Option<String>,
    email:Option<String>,
    password:Option<String>,
}

impl CompanyRegistrationRequest {
    pub fn get_company_and_user_models(self) -> (Company,User) {
        (Company::from(self.name,self.email),User::from(self.username,self.password))
    }
}

// response struct for registration of a company
#[derive(serde::Serialize)]
pub struct CompanyRegistrationResponse {
    message:String
}

// handler for registration of a company
pub async fn register_a_company(
    State(db): State<Arc<Surreal<Client>>>,
    Json(company_details): Json<CompanyRegistrationRequest>,
) -> Result<Json<CompanyRegistrationResponse>,StatusCode> {

    let (company,user) = company_details.get_company_and_user_models();

    let response = db.query(company.get_register_query().await.unwrap()).await;

    match response {
        Ok(mut response) => {

            let company_id:Result<Option<Thing>,surrealdb::Error> = response.take(0);
            let response = db.query(user.get_create_user_query("company".to_string(),company_id.unwrap()).await.unwrap()).await;
            
            match response {
                Ok(_) => Ok(Json(CompanyRegistrationResponse { message: "Company account has been created successfully".to_string() })),
                Err(e) => {println!("{:?}",e); Err(StatusCode::INTERNAL_SERVER_ERROR)}
            }

        },
        Err(e) => {println!("{:?}",e); Err(StatusCode::INTERNAL_SERVER_ERROR)}
    }

}
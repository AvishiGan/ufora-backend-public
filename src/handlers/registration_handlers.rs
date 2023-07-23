use std::sync::Arc;

use axum::{http::{StatusCode, Response},Json, extract::State};

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

    let response = db.query(undergraduate.get_register_query().await?).await;

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

// request struct for adding university details
#[derive(serde::Deserialize,Debug)]
pub struct UniversityDetailsRequest {
    pub username: Option<String>,
    pub university:Option<String>,
    pub university_email:Option<String>,
}

// response struct for adding university details
#[derive(serde::Serialize)]
pub enum UpdateUniversityDetailsResponse {
    Successfull {message:String},
    Unsuccessfull {message:String}
}

pub async fn add_university_details(
    State(db): State<Arc<Surreal<Client>>>,
    Json(university_details): Json<UniversityDetailsRequest>,
) -> (StatusCode,Json<UpdateUniversityDetailsResponse>) {

    if university_details.username.is_none() || university_details.university.is_none() || university_details.university_email.is_none() {
        return (StatusCode::BAD_REQUEST,Json(UpdateUniversityDetailsResponse::Unsuccessfull {message:"Invalid request".to_string()}));
    }

    let user = User::retrieve_user_from_database(db.clone(), university_details.username.unwrap()).await;

    match user {
        Ok(user) => {

            let response = user.update_university_details(db.clone(), university_details.university, university_details.university_email).await;

            match response {
                Ok(_) => { (StatusCode::OK,Json(UpdateUniversityDetailsResponse::Successfull {message:"University details have been added successfully".to_string()}))},
                Err(StatusCode::BAD_REQUEST) => { (StatusCode::BAD_REQUEST,Json(UpdateUniversityDetailsResponse::Unsuccessfull {message:"Invalid request".to_string()}))},
                Err(e) => {println!("{:?}",e); (StatusCode::INTERNAL_SERVER_ERROR,Json(UpdateUniversityDetailsResponse::Unsuccessfull {message:"University details could not be added".to_string()}))}
            }

        },
        Err(e) => {println!("{:?}",e); (StatusCode::NOT_FOUND,Json(UpdateUniversityDetailsResponse::Unsuccessfull {message:"User could not be found".to_string()}))}
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

    let response = db.query(company.get_register_query().await?).await;

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
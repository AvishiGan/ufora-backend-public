use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json, extract::State};

use surrealdb::{Surreal, engine::remote::ws::Client, sql::Thing};

use crate::models::{company::Company,user::User};

#[derive(serde::Deserialize,serde::Serialize,Debug)]
pub struct Undergraduate {
    username: String,
    password: String,
    email: Option<String>,
    phone: Option<String>,
}

impl Undergraduate {

    fn hash_password(mut self) -> Self {
        self.password = hash_password(self.password).unwrap();
        self
    }

}

pub async fn register_an_undergraduate(
    State(db): State<Arc<Surreal<Client>>>,
    Json(new_user): Json<Undergraduate>
) -> Result<impl IntoResponse,StatusCode> {

    let new_user = new_user.hash_password();

    println!("{:?}", new_user);

    let undergraduate:Undergraduate = db
        .create("user")
        .content(new_user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    

    Ok("User created successfully")
}

fn hash_password(password: String) -> Result<String,StatusCode> {
    bcrypt::hash(password, 14).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(serde::Deserialize,serde::Serialize)]
pub struct CompanyDetails {
    name:Option<String>,
    username:Option<String>,
    email:Option<String>,
    password:Option<String>,
}

impl CompanyDetails {
    pub fn get_company_and_user_models(self) -> (Company,User) {
        (Company::from(self.name,self.email),User::from(self.username,self.password))
    }
}

pub async fn register_a_company(
    State(db): State<Arc<Surreal<Client>>>,
    Json(company_details): Json<CompanyDetails>,
) -> Result<String,StatusCode> {

    let (company,user) = company_details.get_company_and_user_models();

    let response = db.query(company.get_register_query().await.unwrap()).await;

    match response {
        Ok(mut response) => {

            let company_id:Result<Option<Thing>,surrealdb::Error> = response.take(0);
            let response = db.query(user.get_create_user_query("company".to_string(),company_id.unwrap()).await.unwrap()).await;
            
            match response {
                Ok(_) => Ok("Company created successfuly".to_string()),
                Err(e) => {println!("{:?}",e);return Err(StatusCode::INTERNAL_SERVER_ERROR)}
            }

        },
        Err(e) => {println!("{:?}",e);return Err(StatusCode::INTERNAL_SERVER_ERROR)}
    }

}
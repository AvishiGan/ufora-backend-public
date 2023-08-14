use std::sync::Arc;

use axum::{http::StatusCode,Json, extract::State};

use serde::{Serialize, Deserialize};
use surrealdb::{Surreal, engine::remote::ws::Client, sql::Thing};

use crate::models::profile::Profile;

// use crate::models::{company::Company,undergraduate::Undergraduate,user::User};

#[derive(Serialize, Deserialize, Debug)]

pub struct UserProfile{
    intro: Option<String>,
    profile_pic: Option<String>,
    contact: Option<String>,
    // optional params depending on user
    date_of_birth: Option<String>,
    address: Option<String>,
    map: Option<String>,
}

impl UserProfile {


    pub fn get_profile_model(self) -> Profile {
        Profile::from(self.intro,self.profile_pic,self.date_of_birth,self.address,self.contact,self.map)
    }
    
}

// response struct for registration of an undergraduate
#[derive(serde::Serialize)]
pub struct ProfileResponse {
    message:String
}




pub async fn create_profile(
    State(db): State<Arc<Surreal<Client>>>,
    Json(profile_details): Json<UserProfile>,
)-> (StatusCode,Json<ProfileResponse>) {

    let profile_detail = profile_details.get_profile_model();

    println!("{:?}",profile_detail.get_profile_create_query().await);


    return (StatusCode::OK,Json(ProfileResponse {message:"Profile created successfully".to_string()}));
    
        // return (StatusCode::OK,Json(ProfileResponse));

}
use std::sync::Arc;

use axum::{http::StatusCode,Json, extract::State};

use serde::{Serialize, Deserialize};
use surrealdb::{Surreal, engine::remote::ws::Client, sql::{Thing, Id}};

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


    pub fn get_profile_model(self, id:Option<String>) -> Profile {
        let id = Thing { tb:"profile".to_string(), id:Id::String(id.unwrap()) };
        Profile::from(Some(id),self.intro,self.profile_pic,self.date_of_birth,self.address,self.contact,self.map)
    }
    
}

// response struct for registration of an undergraduate
#[derive(Serialize)]
pub struct ProfileResponse {
    message:String
}

#[derive(Serialize,Deserialize,Debug)]
pub struct ReturnID {
    id: Thing
}


pub async fn create_profile(
    claim: crate::models::user_claim::Claim,
    State(db): State<Arc<Surreal<Client>>>,
    Json(profile_details): Json<UserProfile>,
)-> (StatusCode,Json<ProfileResponse>) {

    println!("{:?}",claim.get_user_id());
    let profile_detail = profile_details.get_profile_model(Some(claim.get_user_id()));

    let response = db.query(profile_detail.get_profile_create_query().await.unwrap()).await;


        match response{
            Ok(mut response) => {
                let ret_id:Result<Option<Thing>,surrealdb::Error> = response.take(0);
                println!("{:?}",ret_id);
                if ret_id.is_ok() {
                    return (StatusCode::OK,Json(ProfileResponse {message:"Profile has been created successfully".to_string()}));
                }
                (StatusCode::INTERNAL_SERVER_ERROR,Json(ProfileResponse { 
                    message: "Profile could not be created".to_string()
                }))
            },
            Err(e) => {println!("{:?}",e); (StatusCode::INTERNAL_SERVER_ERROR,Json(ProfileResponse { 
                message: "Profile could not be created".to_string()
            } ))}
        }   

}

#[derive(Serialize,Deserialize,Debug)]
pub struct ProfileRequest{
    id: Option<String>
}

// return user profile
pub async fn get_profile(
    State(db): State<Arc<Surreal<Client>>>,
    Json(profile_request): Json<ProfileRequest>,
) -> (StatusCode,Json<Profile>)
{

    println!("{:?}",profile_request);
    let profile = db.query(Profile::get_profile_by_user_id(profile_request.id).await.unwrap()).await;

     match profile {
        Ok(mut response) => {
            let profile:Result<Option<Profile>,surrealdb::Error> = response.take(0);
            match profile {
                Ok(Some(profile)) => {
                    return (StatusCode::OK,Json(profile));
                },
                Ok(None) => {
                    return (StatusCode::NOT_FOUND,Json(Profile::default()));
                },
                Err(e) => {
                    println!("{:?}",e);
                    return (StatusCode::INTERNAL_SERVER_ERROR,Json(Profile::default()));
                }
            }
        },
        Err(e) => {
            println!("{:?}",e);
            return (StatusCode::INTERNAL_SERVER_ERROR,Json(Profile::default()));
        }
         
     }

}
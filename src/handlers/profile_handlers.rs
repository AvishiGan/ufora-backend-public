use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use serde_json::Value;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::user::{
    get_select_user_query, update_user_profile_query, Profile, User, UserRequest,
};

// create a user profile
// _________________________________________________________

pub async fn create_profile(
    claim: crate::models::user_claim::Claim,
    State(db): State<Arc<Surreal<Client>>>,
    Json(profile_details): Json<Profile>,
) -> (StatusCode, Json<Value>) {
    // we use the update user profile query to create a profile as the fields are dynamic
    let result = update_user_profile_query(claim.get_id(), claim.get_user_type(),  profile_details)
        .await
        .unwrap();

    // println!("{:?}", result.to_string());

    let response = db.query(result).await;
    // println!("{:?}", response);

    match response {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(Value::String(
                    "Profile has been created successfully".to_string(),
                )),
            );
        }
        Err(e) => {
            println!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Value::String(e.to_string())),
            );
        }
    }
}

// retrieve profile using email or username
// _________________________________________________________
pub async fn get_user_profile(
    State(db): State<Arc<Surreal<Client>>>,
    Json(user_profile): Json<UserRequest>,
) -> (StatusCode, Json<Value>) {
    let result = get_select_user_query(user_profile).await.unwrap();

     println!("{:?}", result.to_string());

    let response = db.query(result).await;
    //  println!("{:?}", response);

    match response {
        Ok(mut profile) => {
            let profile_result: Result<Option<Value>, surrealdb::Error> = profile.take(0);
            let mut profile_json = profile_result.unwrap().unwrap();

            // make profile json id as a string
            // "id": {
            //     "id": {
            //         "String": "y31thszuh72ejsv7uo2y"
            //     },
            //     "tb": "user"
            // },

           
            profile_json.as_object_mut().unwrap().remove("password");
            profile_json
                .as_object_mut()
                .unwrap()
                .remove("invalid_login_attempts");
            profile_json.as_object_mut().unwrap().remove("locked_flag");

            println!("{:?}", profile_json);
            return (StatusCode::OK, Json(profile_json));
        }
        Err(e) => {
            println!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Value::String(e.to_string())),
            );
        }
    }
}
 
// update the profile
// _________________________________________________________

pub async fn update_profile(
    claim: crate::models::user_claim::Claim,
    State(db): State<Arc<Surreal<Client>>>,
    Json(profile_details): Json<Profile>,
) -> (StatusCode, Json<Value>) {
    // we use the update user profile query to create a profile as the fields are dynamic
    let result = update_user_profile_query(claim.get_id(), claim.get_user_type(), profile_details)
        .await
        .unwrap();

    // println!("{:?}", result.to_string());

    let response = db.query(result).await;
    // println!("{:?}", response);

    match response {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(Value::String(
                    "Profile has been updated successfully".to_string(),
                )),
            );
        }
        Err(e) => {
            println!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Value::String(e.to_string())),
            );
        }
    }
}

 
use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use serde_json::Value;
use surrealdb::{engine::remote::ws::Client, Response, Surreal};

use crate::models::user::{
    get_all_users_query, get_select_user_query, update_user_profile_query, Profile, User,
    UserRequest, SelectUsersParam,
};

// create a user profile
// _________________________________________________________

pub async fn create_profile(
    claim: crate::models::user_claim::Claim,
    State(db): State<Arc<Surreal<Client>>>,
    Json(profile_details): Json<Profile>,
) -> (StatusCode, Json<Value>) {
    // we use the update user profile query to create a profile as the fields are dynamic
    let result = update_user_profile_query(claim.get_id(), claim.get_user_type(), profile_details)
        .await
        .unwrap();

    println!("{:?}", result.to_string());

    let response = db.query(result).await;
    // println!("{:?}", response);

    match response {
        Ok(test) => {

            println!("{:?}", test);

            
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

            // remove unnecessary fields
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

// delete the profile
// _________________________________________________________

// pub async fn delete_profile(
//     claim: crate::models::user_claim::Claim,
//     State(db): State<Arc<Surreal<Client>>>,
// ) -> (StatusCode, Json<Value>) {
//     // we use the update user profile query to create a profile as the fields are dynamic

//     // println!("{:?}",

// }

// get all profiles
// _________________________________________________________


pub async fn get_all_profiles(
    State(db): State<Arc<Surreal<Client>>>,
    Json(user_request_params): Json<SelectUsersParam>,
) -> (StatusCode, Json<Value>) {
    let result = get_all_users_query(user_request_params).await.unwrap();

    println!("{:?}", result.to_string());

    let response = db.query(result).await;
    //  println!("{:?}", response);


    //   print the array of users in the response
    match response {
        Ok(mut users) => {

            let users_result: Result<Vec<Value>, surrealdb::Error> = users.take(0);
            let mut user_objects: Value = users_result.unwrap().into();
            println!("{:?}", user_objects);

            if user_objects.as_array().unwrap().len() == 0 {
                return (
                    StatusCode::OK,
                    Json(Value::String("No users found".to_string())),
                );
            }

            // remove unnecessary fields from sub objects using a for loop
            for user in user_objects.as_array_mut().unwrap() {
                user.as_object_mut().unwrap().remove("password");
                user.as_object_mut()
                    .unwrap()
                    .remove("invalid_login_attempts");
                user.as_object_mut().unwrap().remove("locked_flag");
            }

            // let user_object_value_array:Value = user_objects.into();

            return (StatusCode::OK, Json(user_objects));
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

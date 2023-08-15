use core::borrow;
use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::{
    engine::remote::ws::Client,
    sql::{Id, Thing},
    Surreal,
};

use crate::models::{profile::Profile, user::{User, self}};

fn merge(a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Object(mut a), Value::Object(b)) => {
            for (k, v) in b {
                let entry = a.entry(k).or_insert(Value::Null);
                *entry = merge(entry.clone(), v);
            }
            Value::Object(a)
        }
        (_, b) => b,
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct OnlyProfile {
    name: Option<String>,
    intro: Option<String>,
    profile_pic: Option<String>,
    contact: Option<String>,
    // optional params depending on user
    date_of_birth: Option<String>,
    address: Option<String>,
}

impl OnlyProfile {
    pub fn get_profile_model(self, id: Option<String>) -> Profile {
        let id = Thing {
            tb: "profile".to_string(),
            id: Id::String(id.unwrap()),
        };
        Profile::from(
            Some(id),
            self.name,
            self.intro,
            self.profile_pic,
            self.date_of_birth,
            self.address,
            self.contact,
        )
    }
}

// response struct for all responspones
#[derive(Serialize)]
pub struct ProfileResponse {
    message: String,
}

// Create user profile for logged in user
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnID {
    id: Thing,
}

pub async fn create_profile(
    claim: crate::models::user_claim::Claim,
    State(db): State<Arc<Surreal<Client>>>,
    Json(profile_details): Json<OnlyProfile>,
) -> (StatusCode, Json<ProfileResponse>) {
    // println!("{:?}",claim.get_user_id());
    let profile_detail = profile_details.get_profile_model(Some(claim.get_user_id()));

    // println!("{:?}",profile_detail);
    let response = db
        .query(profile_detail.get_profile_create_query().await.unwrap())
        .await;

    match response {
        Ok(mut response) => {
            let ret_id: Result<Option<Thing>, surrealdb::Error> = response.take(0);
            println!("{:?}", ret_id);
            if ret_id.is_ok() {
                return (
                    StatusCode::OK,
                    Json(ProfileResponse {
                        message: "Profile has been created successfully".to_string(),
                    }),
                );
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ProfileResponse {
                    message: "Profile could not be created".to_string(),
                }),
            )
        }
        Err(e) => {
            println!("{:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ProfileResponse {
                    message: "Profile could not be created".to_string(),
                }),
            )
        }
    }
}

// Getting the profile using the user ID
#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileRequest {
    id: Option<String>,
}

// return user profile
pub async fn get_profile(
    State(db): State<Arc<Surreal<Client>>>,
    Json(profile_request): Json<ProfileRequest>,
) -> (StatusCode, Json<Profile>) {
    println!("{:?}", profile_request);
    let profile = db
        .query(
            Profile::get_profile_by_user_id(profile_request.id)
                .await
                .unwrap(),
        )
        .await;

    match profile {
        Ok(mut response) => {
            let profile: Result<Option<Profile>, surrealdb::Error> = response.take(0);
            match profile {
                Ok(Some(profile)) => {
                    return (StatusCode::OK, Json(profile));
                }
                Ok(None) => {
                    return (StatusCode::NOT_FOUND, Json(Profile::default()));
                }
                Err(e) => {
                    println!("{:?}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(Profile::default()));
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(Profile::default()));
        }
    }
}

// Getting the profile using the username/email and getting the associated profile

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileRequestUsername {
    username: Option<String>,
    email: Option<String>,
}

// Object {"address": Null, "contact": String("+94771231234"), "date_of_birth": String("2000-10-29T00:00:00Z"), "email": String("binuridesilva27@gmail.com"), "email_verification_flag": Null, "gmap": Null, "id": String("profile:hlk48mmzp8nva6jg35ti"), "intro": String("hi i am Abdullah"), "invalid_login_attempts": Number(0), "locked_flag": Bool(false), "name": String("Abdullah Jasmin"), "password": String("$2b$14$4C2ZWzlpbjMSlHPpHpl.L.bUHsH0wAFcZmjXZIjlc30i3tvYZ3T/W"), "profile_pic": String("dp"), "user_id": String("undergraduate:hlk48mmzp8nva6jg35ti"), "user_type": String("undergraduate"), "username": String("user1")}

pub async fn get_user_profile(
    State(db): State<Arc<Surreal<Client>>>,
    Json(user_profile): Json<ProfileRequestUsername>,
) -> (StatusCode, Json<Value>) {
    // println!("{:?}",user_profile);

    // get the user from user model
    let user =
        User::get_user_by_email_or_username(db.clone(), user_profile.email, user_profile.username)
            .await;

    // println!("{:?}",user);

    // error if user not found
    match &user {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Value::String("User not found".to_string())),
            );
        }
    }
    
    // get the user fields to an object
    let user_fields = user.unwrap();

    // let profile_request = Some(user.unwrap().get_user_id());
    let profile_id = Some(format!(
        "profile:{}",
        user_fields.get_user_id().id.to_string()
    ));
    // println!("{:?}",profile_id);

    // get the profile from profile model
    let profile = db
        .query(Profile::get_profile_by_user_id(profile_id).await.unwrap())
        .await;

    // get the profile json values
    // println!("{:?}",profile);

    // error if profile not found else merge array and print result
    match profile {
        Ok(mut response) => {
            let profile: Result<Option<Profile>, surrealdb::Error> = response.take(0);
            match profile {
                Ok(Some(profile)) => {
                    // merging user array and profile array
                    let mut user_profile_result = merge(user_fields.into(), profile.into());
                    
                    // remove unneccessary field
                    user_profile_result.as_object_mut().unwrap().remove("password");
                    user_profile_result.as_object_mut().unwrap().remove("invalid_login_attempts");
                    user_profile_result.as_object_mut().unwrap().remove("locked_flag");
                    
                    
                    println!("{:?}", user_profile_result);
                    return (StatusCode::OK, Json(user_profile_result));
                }
                Ok(None) => {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(Value::String("Profile not found".to_string())),
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
        Err(e) => {
            println!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Value::String(e.to_string())),
            );
        }
    }
}

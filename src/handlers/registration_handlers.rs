use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::user::User;

// request struct for registration of an user
#[derive(serde::Deserialize)]
pub struct UserRegistrationRequest {
    name: Option<String>,
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

impl UserRegistrationRequest {
    pub fn get_user_and_user_models(self) -> User {
        User::from(self.username, self.name, self.password, self.email)
    }
}

// response struct for registration of an user
#[derive(serde::Serialize)]
pub struct UserRegistrationResponse {
    message: String,
}

// handler for registration of an user
pub async fn register_a_user(
    State(db): State<Arc<Surreal<Client>>>,
    Path(user_type): Path<String>,
    Json(user_details): Json<UserRegistrationRequest>,
) -> (StatusCode, Json<UserRegistrationResponse>) {
    
    // get user and user models
    let user = user_details.get_user_and_user_models();


    let available_user = User::get_user_by_email_or_username(
        db.clone(),
        Some(user.get_user_email()),
        Some(user.get_user_username()),
    )
    .await;



    match available_user {
        Ok(usr) if user.get_user_email() == usr.get_user_email() => {
            return (
                StatusCode::BAD_REQUEST,
                Json(UserRegistrationResponse {
                    message: "Email already exists".to_string(),
                }),
            );
        }
        Ok(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(UserRegistrationResponse {
                    message: "Username already exists".to_string(),
                }),
            );
        }
        Err(StatusCode::NOT_FOUND) => {}
        Err(e) => {
            println!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UserRegistrationResponse {
                    message: "User account could not be created".to_string(),
                }),
            );
        }
    }

    // get query for creating an user user and execute it
    let response = db
        .query(
            user.get_create_user_query(user_type.to_string())
                .await
                .unwrap(),
        )
        .await;

    match response {
        Ok(_) => (
            StatusCode::OK,
            Json(UserRegistrationResponse {
                message: "User account has been created successfully".to_string(),
            }),
        ),
        Err(e) => {
            println!("{:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UserRegistrationResponse {
                    message: "User account could not be created".to_string(),
                }),
            )
        }
    }
}

// request struct for adding university details
#[derive(serde::Deserialize, Debug)]
pub struct UniversityDetailsRequest {
    pub username: Option<String>,
    pub university: Option<String>,
    pub university_email: Option<String>,
}

// response struct for adding university details
#[derive(serde::Serialize)]
pub enum UpdateUniversityDetailsResponse {
    Successfull { message: String },
    Unsuccessfull { message: String },
}

// handler for adding university details
pub async fn add_university_details(
    State(db): State<Arc<Surreal<Client>>>,
    Json(university_details): Json<UniversityDetailsRequest>,
) -> (StatusCode, Json<UpdateUniversityDetailsResponse>) {
    // check whether username, university and university email are present or not
    if university_details.username.is_none()
        || university_details.university.is_none()
        || university_details.university_email.is_none()
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(UpdateUniversityDetailsResponse::Unsuccessfull {
                message: "Invalid request".to_string(),
            }),
        );
    }

    // retrieve user from database
    let user = User::retrieve_user_from_database_by_username(
        db.clone(),
        university_details.username.unwrap(),
    )
    .await;

    match user {
        // update university details if user is found
        Ok(user) => {
            // update university details
            let response = User::update_university_details(
                user.get_id(),
                db.clone(),
                university_details.university,
                university_details.university_email,
            )
            .await;

            match response {
                // return success if university details are updated successfully
                Ok(_) => (
                    StatusCode::OK,
                    Json(UpdateUniversityDetailsResponse::Successfull {
                        message: "University details have been added successfully".to_string(),
                    }),
                ),

                // return bad request if university details are not updated successfully, invalid inputs given
                Err(StatusCode::BAD_REQUEST) => (
                    StatusCode::BAD_REQUEST,
                    Json(UpdateUniversityDetailsResponse::Unsuccessfull {
                        message: "Invalid request".to_string(),
                    }),
                ),

                // return internal server error if university details are not updated successfully, database error
                Err(e) => {
                    println!("{:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(UpdateUniversityDetailsResponse::Unsuccessfull {
                            message: "University details could not be added".to_string(),
                        }),
                    )
                }
            }
        }

        // return not found if user is not found
        Err(e) => {
            println!("{:?}", e);
            (
                StatusCode::NOT_FOUND,
                Json(UpdateUniversityDetailsResponse::Unsuccessfull {
                    message: "User could not be found".to_string(),
                }),
            )
        }
    }
}

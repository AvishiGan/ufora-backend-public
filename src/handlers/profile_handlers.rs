use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::{
    engine::remote::ws::Client,
    sql::{Id, Thing},
    Surreal, opt::PatchOp,
};

use crate::{
    models::{
        company::{update_address, Company},
        profile::Profile,
        undergraduate::{update_dob, Undergraduate},
        user::User,
    },
    services::queryBuilder::{get_select_query, Column, Expression, ExpressionConnector, Item},
};

use crate::services::merge_json::merge;

// get the select query
pub async fn get_by_user_id(
    table: Option<String>,
    user_id: Option<String>,
) -> Result<String, StatusCode> {
    let string = get_select_query(
        Item::Table(table.clone().unwrap().to_string()),
        Column::All,
        Some(vec![(
            Expression::EqualTo(
                "id".to_string(),
                format!("{}:{}", table.unwrap(), user_id.unwrap()),
            ),
            ExpressionConnector::End,
        )]),
        None,
        None,
        None,
        None,
    );
    Ok(string)
}


// update name
pub async fn update_name(
    table: Option<String>,
    user_id: String,
    db: Arc<Surreal<Client>>,
    name: Option<String>,
) -> Result<(), StatusCode> {
    match name {
        None => return Err(StatusCode::BAD_REQUEST),
        _ => {}
    }

    let response: Result<String, surrealdb::Error> = db
        .update((table.unwrap(), user_id))
        .patch(PatchOp::replace("/name", name.unwrap().to_string()))
        .await;

    match response {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{:?} ", e);
            Ok(())
        }
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
        Profile::from(Some(id), self.intro, self.profile_pic, self.contact)
    }
}

// response struct for create profile
#[derive(Serialize)]
pub struct ProfileResponse {
    message: String,
}

// create a user profile
// _________________________________________________________
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

    // getting the date of birth
    let dob = profile_details.date_of_birth.clone();
    // println!("{:?}",dob);

    let address = profile_details.address.clone();
    // println!("{:?}",address);

    // getting the profile model
    let profile_detail = profile_details.get_profile_model(Some(claim.get_user_id()));

    // generating a create query for profile
    println!("{:?}", profile_detail);
    let response = db
        .query(profile_detail.get_profile_create_query().await.unwrap())
        .await;

    // based on the response return the status code and message
    match response {
        // if the we get an ok response
        Ok(mut response) => {
            let ret_id: Result<Option<Thing>, surrealdb::Error> = response.take(0);
            // println!("{:?}", ret_id);

            // if the profile is created successfully
            if ret_id.is_ok() {
                if claim.get_user_type() == "undergraduate" {
                    println!("undergraduate");
                    match dob {
                        Some(dob) => {
                            update_dob(claim.get_user_id(), db.clone(), Some(dob))
                                .await
                                .unwrap();
                            // println!("{:?}",response);
                        }
                        None => {
                            println!("no dob");
                        }
                    }
                } else {
                    println!("company");
                    match address {
                        Some(address) => {
                            update_address(claim.get_user_id(), db.clone(), Some(address))
                                .await
                                .unwrap();
                            // println!("{:?}",response);
                        }
                        None => {
                            println!("no address");
                        }
                    }
                }

                return (
                    StatusCode::OK,
                    Json(ProfileResponse {
                        message: "Profile has been created successfully".to_string(),
                    }),
                );
            }
            // if the profile is not created successfully
            else {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ProfileResponse {
                        message: "Profile could not be created".to_string(),
                    }),
                );
            }
        }
        // if the profile is not created successfully for some other reason
        Err(e) => {
            println!("{:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ProfileResponse {
                    message: e.to_string(),
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
// retrieve only the profile using id
// _________________________________________________________
pub async fn get_profile(
    State(db): State<Arc<Surreal<Client>>>,
    Json(profile_request): Json<ProfileRequest>,
) -> (StatusCode, Json<Profile>) {
    // getting the profile model
    println!("{:?}", profile_request);
    let profile = db
        .query(
            Profile::get_profile_by_user_id(profile_request.id)
                .await
                .unwrap(),
        )
        .await;

    // checking whether the profile is found or not
    match profile {
        Ok(mut response) => {
            // getting the profile json values
            let profile: Result<Option<Profile>, surrealdb::Error> = response.take(0);
            match profile {
                // if the profile is found
                Ok(Some(profile)) => {
                    return (StatusCode::OK, Json(profile));
                }
                // if the profile is not found
                Ok(None) => {
                    return (StatusCode::NOT_FOUND, Json(Profile::default()));
                }
                // if the profile is not found for some other reason
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

// retrieve profile using email or username
// _________________________________________________________
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

    // get the user fields to an object so that we can merge later
    let user_fields = user.unwrap();
    let userid = user_fields.get_user_id().id.to_string();
    let usertype = user_fields.get_user_type().to_string();

    // getting the rest of the respective fields to a query
    let usertype_table = db
        .query(
            get_by_user_id(Some(usertype.clone()), Some(userid.clone()))
                .await
                .unwrap(),
        )
        .await;

    let mut user_usertype = Value::Null;
    match usertype_table {
        Ok(mut response) => {
            if usertype == "company" {
                let usertype_table: Result<Option<Company>, surrealdb::Error> = response.take(0);
                match usertype_table {
                    Ok(Some(usertype_table)) => {
                        // merging user array and profile array
                        user_usertype = merge(user_fields.into(), usertype_table.into());
                    }
                    Ok(None) => {
                        println!("usertype table not found");
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
            // if the user is an undergraduate
            else {
                let usertype_table: Result<Option<Undergraduate>, surrealdb::Error> =
                    response.take(0);
                match usertype_table {
                    Ok(Some(usertype_table)) => {
                        // merging user array and profile array
                        user_usertype = merge(user_fields.into(), usertype_table.into());
                    }
                    Ok(None) => {
                        println!("usertype table not found");
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    };
    println!("{:?}", user_usertype);

    // getting profile part of the user
    // let profile_request = Some(user.unwrap().get_user_id());
    let profile_id = Some(format!("profile:{}", userid));
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
                    let mut user_profile_result = merge(user_usertype, profile.into());

                    // remove unneccessary field
                    user_profile_result
                        .as_object_mut()
                        .unwrap()
                        .remove("password");
                    user_profile_result
                        .as_object_mut()
                        .unwrap()
                        .remove("invalid_login_attempts");
                    user_profile_result
                        .as_object_mut()
                        .unwrap()
                        .remove("email_verification_flag");
                    user_profile_result
                        .as_object_mut()
                        .unwrap()
                        .remove("locked_flag");

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

// update the profile
// _________________________________________________________
pub async fn update_profile(
    claim: crate::models::user_claim::Claim,
    State(db): State<Arc<Surreal<Client>>>,
    Json(profile_details): Json<OnlyProfile>,
) -> (StatusCode, Json<ProfileResponse>) {
    // getting the date of birth
    let dob = profile_details.date_of_birth.clone();
    // println!("{:?}",dob);

    let address = profile_details.address.clone();
    // println!("{:?}",address);

    let name = profile_details.name.clone();

    // getting the profile model
    let profile_detail = profile_details.get_profile_model(Some(claim.get_user_id()));

    // generating a create query for profile
    let response = db
        .query(profile_detail.get_profile_update_query().await.unwrap())
        .await;

    println!("{:?}", response);

    // based on the response return the status code and message
    match response {
        // if the we get an ok response
        Ok(mut response) => {
            let ret_id: Result<Option<Profile>, surrealdb::Error> = response.take(0);
            // println!("{:?}", ret_id);

            // if the profile is updated successfully
            if ret_id.is_ok() {
                if claim.get_user_type() == "undergraduate" {
                    println!("undergraduate");
                    match dob {
                        Some(dob) => {
                            update_dob(claim.get_user_id(), db.clone(), Some(dob))
                                .await
                                .unwrap();
                            println!("{:?}",response);
                        }
                        None => {
                            println!("no dob");
                        }
                    }
                } else {
                    println!("company");
                    match address {
                        Some(address) => {
                            update_address(claim.get_user_id(), db.clone(), Some(address))
                                .await
                                .unwrap();
                            println!("{:?}",response);
                        }
                        None => {
                            println!("no address");
                        }
                    }
                }

                match name {
                    Some(name) => {
                        update_name(Some(claim.get_user_type()),claim.get_user_id(), db.clone(), Some(name))
                            .await
                            .unwrap();
                    }
                    None => {}
                }


                return (
                    StatusCode::OK,
                    Json(ProfileResponse {
                        message: "Profile has been updated successfully".to_string(),
                    }),
                );
            }
            // if the profile is not updated successfully
            else {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ProfileResponse {
                        message: "Profile could not be updated".to_string(),
                    }),
                );
            }
        }
        // if the profile is not updated successfully for some other reason
        Err(e) => {
            println!("{:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ProfileResponse {
                    message: e.to_string(),
                }),
            )
        }
    }
}

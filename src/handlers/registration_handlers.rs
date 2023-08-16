use std::sync::Arc;

use axum::{ http::StatusCode, Json, extract::State };

use surrealdb::{ Surreal, engine::remote::ws::Client, sql::Thing };

use crate::models::{ company::Company, undergraduate::Undergraduate, user::User };

// request struct for registration of an undergraduate
#[derive(serde::Deserialize)]
pub struct UndergraduateRegistrationRequest {
    name: Option<String>,
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

impl UndergraduateRegistrationRequest {
    pub fn get_undergraduate_and_user_models(self) -> (Undergraduate, User) {
        (Undergraduate::from(self.name, None), User::from(self.username, self.password, self.email))
    }
}

// response struct for registration of an undergraduate
#[derive(serde::Serialize)]
pub struct UndergraduateRegistrationResponse {
    message: String,
}

// handler for registration of an undergraduate
pub async fn register_an_undergraduate(
    State(db): State<Arc<Surreal<Client>>>,
    Json(company_details): Json<UndergraduateRegistrationRequest>
) -> (StatusCode, Json<UndergraduateRegistrationResponse>) {
    // get undergraduate and user models
    let (undergraduate, user) = company_details.get_undergraduate_and_user_models();

    let available_user = User::get_user_by_email_or_username(
        db.clone(),
        Some(user.get_user_email()),
        Some(user.get_user_username())
    ).await;

    match available_user {
        Ok(usr) if user.get_user_email() == usr.get_user_email() => {
            return (
                StatusCode::BAD_REQUEST,
                Json(UndergraduateRegistrationResponse {
                    message: "Email already exists".to_string(),
                }),
            );
        }
        Ok(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(UndergraduateRegistrationResponse {
                    message: "Username already exists".to_string(),
                }),
            );
        }
        Err(StatusCode::NOT_FOUND) => {}
        Err(e) => {
            println!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UndergraduateRegistrationResponse {
                    message: "Undergraduate account could not be created".to_string(),
                }),
            );
        }
    }

    let undergraduate_registration_statement = undergraduate.get_register_query().await;

    let undergraduate_registration_statement = match undergraduate_registration_statement {
        Ok(statement) => statement,
        Err(e) => {
            println!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UndergraduateRegistrationResponse {
                    message: "Undergraduate account could not be created".to_string(),
                }),
            );
        }
    };

    // get query for registering an undergraduate and execute it
    let response = db.query(undergraduate_registration_statement).await;

    match response {
        Ok(mut response) => {
            // get undergraduate id
            let undergraduate_id: Result<Option<Thing>, surrealdb::Error> = response.take(0);

            // get query for creating an undergraduate user and execute it
            let response = db.query(
                user
                    .get_create_user_query(
                        "undergraduate".to_string(),
                        undergraduate_id.unwrap()
                    ).await
                    .unwrap()
            ).await;

            match response {
                Ok(_) =>
                    (
                        StatusCode::OK,
                        Json(UndergraduateRegistrationResponse {
                            message: "Undergraduate account has been created successfully".to_string(),
                        }),
                    ),
                Err(e) => {
                    println!("{:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(UndergraduateRegistrationResponse {
                            message: "Undergraduate account could not be created".to_string(),
                        }),
                    )
                }
            }
        }

        // return error if query execution fails
        Err(e) => {
            println!("{:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UndergraduateRegistrationResponse {
                    message: "Undergraduate account could not be created".to_string(),
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
    Successfull {
        message: String,
    },
    Unsuccessfull {
        message: String,
    },
}

// handler for adding university details
pub async fn add_university_details(
    State(db): State<Arc<Surreal<Client>>>,
    Json(university_details): Json<UniversityDetailsRequest>
) -> (StatusCode, Json<UpdateUniversityDetailsResponse>) {
    // check whether username, university and university email are present or not
    if
        university_details.username.is_none() ||
        university_details.university.is_none() ||
        university_details.university_email.is_none()
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
        university_details.username.unwrap()
    ).await;

    match user {
        // update university details if user is found
        Ok(user) => {
            // update university details
            let response = Undergraduate::update_university_details(
                user.get_user_id(),
                db.clone(),
                university_details.university,
                university_details.university_email
            ).await;

            match response {
                // return success if university details are updated successfully
                Ok(_) => {
                    (
                        StatusCode::OK,
                        Json(UpdateUniversityDetailsResponse::Successfull {
                            message: "University details have been added successfully".to_string(),
                        }),
                    )
                }

                // return bad request if university details are not updated successfully, invalid inputs given
                Err(StatusCode::BAD_REQUEST) => {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(UpdateUniversityDetailsResponse::Unsuccessfull {
                            message: "Invalid request".to_string(),
                        }),
                    )
                }

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

// request struct for registration of a company
#[derive(serde::Deserialize)]
pub struct CompanyRegistrationRequest {
    name: Option<String>,
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

impl CompanyRegistrationRequest {
    pub fn get_company_and_user_models(self) -> (Company, User) {
        (Company::from(self.name, None, None), User::from(self.username, self.password, self.email))
    }
}

// response struct for registration of a company
#[derive(serde::Serialize)]
pub struct CompanyRegistrationResponse {
    message: String,
}

// handler for registration of a company
pub async fn register_a_company(
    State(db): State<Arc<Surreal<Client>>>,
    Json(company_details): Json<CompanyRegistrationRequest>
) -> (StatusCode, Json<CompanyRegistrationResponse>) {
    // get company and user models
    let (company, user) = company_details.get_company_and_user_models();

    let available_user = User::get_user_by_email_or_username(
        db.clone(),
        Some(user.get_user_email()),
        Some(user.get_user_username())
    ).await;

    match available_user {
        Ok(usr) if user.get_user_email() == usr.get_user_email() => {
            return (
                StatusCode::BAD_REQUEST,
                Json(CompanyRegistrationResponse { message: "Email already exists".to_string() }),
            );
        }
        Ok(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(CompanyRegistrationResponse {
                    message: "Username already exists".to_string(),
                }),
            );
        }
        Err(StatusCode::NOT_FOUND) => {}
        Err(e) => {
            println!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CompanyRegistrationResponse {
                    message: "Company account could not be created".to_string(),
                }),
            );
        }
    }

    let company_registration_statement = company.get_register_query().await;

    let company_registration_statement = match company_registration_statement {
        Ok(statement) => statement,
        Err(e) => {
            println!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CompanyRegistrationResponse {
                    message: "Company account could not be created".to_string(),
                }),
            );
        }
    };

    // get query for registering a company and execute it
    let response = db.query(company_registration_statement).await;

    match response {
        Ok(mut response) => {
            // get company id
            let company_id: Result<Option<Thing>, surrealdb::Error> = response.take(0);

            // get query for creating a company user and execute it
            let response = db.query(
                user
                    .get_create_user_query("company".to_string(), company_id.unwrap()).await
                    .unwrap()
            ).await;

            match response {
                Ok(_) =>
                    (
                        StatusCode::OK,
                        Json(CompanyRegistrationResponse {
                            message: "Company account has been created successfully".to_string(),
                        }),
                    ),
                Err(e) => {
                    println!("{:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(CompanyRegistrationResponse {
                            message: "Company account could not be created".to_string(),
                        }),
                    )
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CompanyRegistrationResponse {
                    message: "Company account could not be created".to_string(),
                }),
            )
        }
    }
}

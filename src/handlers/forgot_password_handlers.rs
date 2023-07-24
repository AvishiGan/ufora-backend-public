use std::sync::Arc;

use axum::{http::StatusCode, extract::State, Json};
use chrono::prelude::*;
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::{models::user::User, services::{email, otp::{self, OTP}}};

// structure for forgot password request
#[derive(serde::Deserialize)]
pub struct ForgotPasswordRequest {
    pub email:String
}

// structure for forgot password response
#[derive(serde::Serialize)]
pub enum ForgotPasswordResponse {
    OTPSent,
    InvalidEmail,
    InternalServerError
}

pub async fn verify_email_and_send_otp(
    State(db): State<Arc<Surreal<Client>>>,
    Json(forgot_password_request): Json<ForgotPasswordRequest>
) -> (StatusCode,Json<ForgotPasswordResponse>) {

    if forgot_password_request.email.is_empty() {
        return (StatusCode::OK,Json(ForgotPasswordResponse::InvalidEmail))
    }

    let user = User::get_user_by_email(db.clone(),forgot_password_request.email.clone()).await;

    match user {
        Ok(user) => {

            // let otp = otp::get_an_otp();

            // match otp {
            //     Ok(_) => {},
            //     Err(_) => {
            //         return (StatusCode::OK,Json(ForgotPasswordResponse::InternalServerError))
            //     }
            // }

            // let email = "OTP to reset your password is ".to_string() + &otp.clone().unwrap() + ". Please do not share this OTP with anyone.";
            
            // let _response = email::send_email(&user.get_user_email(), "OTP for reset password".to_string(), email).await.map_err(|_| {
            //     return (StatusCode::OK,Json(ForgotPasswordResponse::InternalServerError))
            // });

            // let utc = Utc.from_local_datetime(&chrono::Local::now().naive_local()).single().unwrap();

            // let result: Option<OTP> = db.update(("otp",forgot_password_request.email)).merge(OTP {
            //     otp: otp.unwrap(),
            //     created_at:utc,
            //     expires_at:utc + chrono::Duration::minutes(10)
            //     }).await.unwrap();

            // match result {
            //     Some(_) => {},
            //     None => {return  (StatusCode::INTERNAL_SERVER_ERROR,Json(ForgotPasswordResponse::InternalServerError))}
            // };

        },
        Err(_) => {
            return (StatusCode::OK,Json(ForgotPasswordResponse::InvalidEmail))
        }   
    }

    (StatusCode::OK,Json(ForgotPasswordResponse::OTPSent))
}
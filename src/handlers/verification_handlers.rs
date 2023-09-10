use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use chrono::prelude::*;
use serde_json::Value;
use surrealdb::{engine::remote::ws::Client, opt::PatchOp, Surreal};

use crate::{
    models::user::User,
    services::{
        email,
        otp::{self, OTP},
    },
};

// request struct for sending otp to email
#[derive(serde::Deserialize)]
pub struct OTPRequest {
    email: String,
}

// response struct for sending otp to email
#[derive(serde::Serialize)]
pub struct OTPSendingResponse {
    message: String,
}

// handler for sending otp to email
pub async fn send_otp_to_email(
    State(db): State<Arc<Surreal<Client>>>,
    Json(otp_request): Json<OTPRequest>,
) -> Result<Json<OTPSendingResponse>, StatusCode> {
    // generate otp
    let otp = otp::get_an_otp().unwrap();

    // email body
    let email = "OTP for your email verification is ".to_string()
        + &otp
        + ". Please do not share this OTP with anyone.";

    // send email
    email::send_email(
        ("Receiver <".to_string() + &otp_request.email + ">").as_ref(),
        "OTP for your registration".to_string(),
        email,
    )
    .await?;

    // get current time from local timezone
    let utc = Utc
        .from_local_datetime(&chrono::Local::now().naive_local())
        .single()
        .unwrap();

    // update or insert otp in database
    let result: Option<OTP> = db
        .update(("otp", otp_request.email))
        .merge(OTP {
            otp,
            created_at: utc,
            expires_at: utc + chrono::Duration::minutes(10),
        })
        .await
        .unwrap();

    match result {
        Some(_) => Ok(Json(OTPSendingResponse {
            message: "OTP has been sent to your email".to_string(),
        })),
        None => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// request struct for verifying otp
#[derive(serde::Deserialize)]
pub struct OTPVerificationRequest {
    otp: String,
    email: String,
}

// response struct for verifying otp
#[derive(serde::Serialize)]
pub struct OTPVerificationResponse {
    message: String,
}

// handler for verifying otp
pub async fn verify_otp(
    State(db): State<Arc<Surreal<Client>>>,
    Json(otp_verification_request): Json<OTPVerificationRequest>,
) -> Result<Json<OTPVerificationResponse>, StatusCode> {
    // select otp from database
    let result: Option<OTP> = db
        .select(("otp", otp_verification_request.email.clone()))
        .await
        .unwrap();

    // check whether otp exists or not
    match result {
        Some(otp) => {
            // check whether otp is expired or not
            if otp.expires_at
                > Utc
                    .from_local_datetime(&chrono::Local::now().naive_local())
                    .single()
                    .unwrap()
            {
                // check whether user has entered correct otp or not
                if otp.otp == otp_verification_request.otp.clone() {
                    // unlock account, if it is locked
                    let user = User::get_user_by_email_or_username(
                        db.clone(),
                        Some(otp_verification_request.email.clone()),
                        None,
                    )
                    .await;

                    let user = user.unwrap();

                    // check if user is locked
                    match user.is_user_locked() {
                        true => {
                            let response: Option<Value> 
                             = db.update(("user", user.get_id()))
                                .patch(PatchOp::replace("/invalid_login_attempts", 0))
                                .patch(PatchOp::replace("/locked_flag", false))
                                .await
                                .unwrap();

                            // println!("{:?}", response);

                            match response {
                                Some(_) => {
                                    return Ok(Json(OTPVerificationResponse {
                                        message: "Account has been reactivated successfully".to_string(),
                                    }));
                                }
                                None => {
                                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                                }
                                
                            }
                        }
                        false => {}
                    }

                    // delete otp from database
                    let _response: Option<OTP> = db
                        .delete(("otp", otp_verification_request.email.clone()))
                        .await
                        .unwrap();

                    // update email verification status of user
                    User::update_email_verification(
                        db.clone(),
                        otp_verification_request.email.clone(),
                    )
                    .await
                    .unwrap();
                    Ok(Json(OTPVerificationResponse {
                        message: "OTP has been verified successfully".to_string(),
                    }))
                } else {
                    Err(StatusCode::BAD_REQUEST)
                }
            } else {
                // delete otp from database, if it is expired
                let _response: Option<OTP> = db
                    .delete(("otp", otp_verification_request.email))
                    .await
                    .unwrap();
                Err(StatusCode::BAD_REQUEST)
            }
        }

        // return bad request, if otp does not exist
        None => Err(StatusCode::BAD_REQUEST),
    }
}

// request struct for sending otp to university email
pub async fn verify_otp_university_email(
    State(db): State<Arc<Surreal<Client>>>,
    Json(otp_verification_request): Json<OTPVerificationRequest>,
) -> Result<Json<OTPVerificationResponse>, StatusCode> {
    // select otp from database
    let result: Option<OTP> = db
        .select(("otp", otp_verification_request.email.clone()))
        .await
        .unwrap();

    // check whether otp exists or not
    match result {
        Some(otp) => {
            // check whether otp is expired or not
            if otp.expires_at
                > Utc
                    .from_local_datetime(&chrono::Local::now().naive_local())
                    .single()
                    .unwrap()
            {
                // check whether user has entered correct otp or not
                if otp.otp == otp_verification_request.otp.clone() {
                    // delete otp from database
                    let _response: Option<OTP> = db
                        .delete(("otp", otp_verification_request.email.clone()))
                        .await
                        .unwrap();

                    // update university email verification status of user
                    User::update_university_email_verification(
                        db.clone(),
                        otp_verification_request.email.clone(),
                    )
                    .await
                    .unwrap();
                    Ok(Json(OTPVerificationResponse {
                        message: "OTP has been verified successfully".to_string(),
                    }))
                } else {
                    Err(StatusCode::BAD_REQUEST)
                }
            } else {
                // delete otp from database, if it is expired
                let _response: Option<OTP> = db
                    .delete(("otp", otp_verification_request.email))
                    .await
                    .unwrap();
                Err(StatusCode::BAD_REQUEST)
            }
        }

        // return bad request, if otp does not exist
        None => Err(StatusCode::BAD_REQUEST),
    }
}

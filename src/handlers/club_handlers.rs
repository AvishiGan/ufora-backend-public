use std::sync::Arc;

use axum::{extract::State, Json};
use axum_valid::Valid;
use chrono::prelude::*;
use dotenvy_macro::dotenv;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use reqwest::StatusCode;
use serde_json::json;
use surrealdb::{engine::remote::ws::Client, Surreal};
use validator::Validate;

use crate::{
    models::user::User,
    services::otp::{self, OTP},
};

#[derive(serde::Serialize)]
pub enum ClubRouteResponse {
    Success { message: String },
    Failed { message: String },
}

#[derive(serde::Deserialize, Validate, Debug)]
pub struct ClubCreateRequest {
    #[validate(required(message = "Username of the club account is required"))]
    username: Option<String>,
    #[validate(required(message = "Name of the club is required"))]
    name: Option<String>,
    #[validate(required(message = "Type of the club is required"))]
    club_type: Option<String>,
    #[validate(required(message = "Email of the club is required"))]
    email: Option<String>,
    #[validate(required(message = "Verification file is required"))]
    club_verification_file: Option<String>,
    profile_pic: Option<String>,
}

pub async fn create_a_club_account(
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,
    Valid(Json(club_request)): Valid<Json<ClubCreateRequest>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let user = User::get_club_account_from_email_or_name(
        db.clone(),
        club_request.email.clone(),
        club_request.name.clone(),
    )
    .await;

    if user.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!(
                {
                    "message": "Club account already exists",
                }
            )),
        );
    }

    if let Ok(club) = User::create_a_club_account(
        db.clone(),
        club_request.username.unwrap(),
        club_request.name.unwrap(),
        club_request.email.unwrap(),
        club_request.club_type.unwrap(),
        claim.get_surrealdb_thing(),
        club_request.club_verification_file.unwrap(),
        club_request.profile_pic.clone(),
    )
    .await
    {
        // get email from the club creation response
        let club_email = club
            .get("email")
            .unwrap()
            .to_string()
            .trim_matches('"')
            .to_string();

        // get an otp from otp service
        let otp = otp::get_an_otp().unwrap();

        // format email body
        let email_body = "OTP for your club account email verification is ".to_string()
            + &otp
            + ". Please do not share this OTP with anyone.";

        // send email
        if let Err(_) = crate::services::email::send_email(
            ("Receiver <".to_string() + &club_email + ">").as_ref(),
            "OTP for your club account registration".to_string(),
            email_body,
        )
        .await
        {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "OTP could not be sent",
                })),
            );
        }

        // get current time from local timezone
        let utc = chrono::Utc
            .from_local_datetime(&chrono::Local::now().naive_local())
            .single()
            .unwrap();

        // update or insert otp in database
        let result: Option<otp::OTP> = db
            .update(("otp", club_email.clone()))
            .merge(otp::OTP {
                otp,
                created_at: utc,
                expires_at: utc + chrono::Duration::minutes(10),
            })
            .await
            .unwrap();

        if result.is_none() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "OTP could not be saved",
                })),
            );
        }

        // encrypt email as a token
        let mcrypt = new_magic_crypt!(dotenv!("ENCRYPTION_KEY"), 256);
        let encrypted_email = mcrypt.encrypt_str_to_base64(&club_email);

        return (
            StatusCode::OK,
            Json(json!({
                "message": "Club account created successfully",
                "token": encrypted_email,
            })),
        );
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Club account could not be created",
            })),
        )
    }
}

// request struct for verifying otp
#[derive(serde::Deserialize, Validate, Debug)]
pub struct OTPVerificationRequest {
    #[validate(required(message = "OTP is required"))]
    otp: Option<String>,
    #[validate(required(message = "Token for verification is required"))]
    token: Option<String>,
}

pub async fn verify_club_email(
    State(db): State<Arc<Surreal<Client>>>,
    Valid(Json(otp_verification_request)): Valid<Json<OTPVerificationRequest>>,
) -> (StatusCode, Json<serde_json::Value>) {
    if let Ok(decrypted_email) = {
        let mcrypt = new_magic_crypt!(dotenv!("ENCRYPTION_KEY"), 256);
        mcrypt.decrypt_base64_to_string(&otp_verification_request.token.clone().unwrap())
    } {
        let otp: Option<OTP> = db.select(("otp", decrypted_email.clone())).await.unwrap();
        match otp {
            Some(otp) => {
                if otp.expires_at
                    < Utc
                        .from_local_datetime(&chrono::Local::now().naive_local())
                        .single()
                        .unwrap()
                {
                    let _response: Result<serde_json::Value, surrealdb::Error> =
                        db.delete(("otp", decrypted_email.clone())).await;
                    (
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            "message": "OTP has expired",
                        })),
                    )
                } else if otp.otp != otp_verification_request.otp.clone().unwrap() {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            "message": "OTP is incorrect",
                        })),
                    )
                } else {
                    let _response: Result<serde_json::Value, surrealdb::Error> =
                        db.delete(("otp", decrypted_email.clone())).await;
                    match User::update_email_verification(db, decrypted_email).await {
                        Ok(_) => (
                            StatusCode::OK,
                            Json(json!({
                                "message": "OTP verified successfully",
                            })),
                        ),
                        Err(e) => {
                            println!("Error: {}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({
                                    "message": "Email verification status could not be updated",
                                })),
                            )
                        }
                    }
                }
            }
            None => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": "No OTP has been sent to your email",
                })),
            ),
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "Invalid token",
            })),
        )
    }
}

pub async fn club_middleware_check(
    claim: crate::models::user_claim::Claim,
    club_claim: crate::models::club_claim::ClubClaim,
) {
    println!("User middleware check");
    println!("User id: {:?}", claim);
    println!("Club middleware check");
    println!("Club id: {:?}", club_claim);
}

use axum::{http::{StatusCode, Response, header}, response::IntoResponse};
use serde_json::json;
use tower_cookies::{Cookie, cookie::time};

pub async fn logout() -> Result<impl IntoResponse,StatusCode> {

    let cookie = Cookie::build("_Secure-jwt", "")
        .max_age(time::Duration::days(-1))
        .finish();

        let mut response = Response::new(json!({
            "message": "Logout successful",
        }).to_string());
    
        response
            .headers_mut()
            .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)

}
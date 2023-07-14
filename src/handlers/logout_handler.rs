
use axum::http::StatusCode;
use axum_extra::extract::cookie::Cookie;
use chrono::DateTime;



pub async fn logout() -> Result<(),StatusCode> {
    
    let cookie = Cookie::build("_Secure-jwt", "")
        .expires(time::OffsetDateTime::now_utc() - time::Duration::days(1))
        .finish();

    Ok(())
}
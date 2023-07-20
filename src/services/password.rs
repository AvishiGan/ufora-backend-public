use axum::http::StatusCode;

pub fn hash_password(password:String) -> Result<String,StatusCode> {
    bcrypt::hash(password, 14).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
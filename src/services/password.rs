use axum::http::StatusCode;

// function to hash password
pub fn hash_password(password:String) -> Result<String,StatusCode> {
    bcrypt::hash(password, 14).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// function to verify password
pub fn verify_password(password:String,hash:String) -> Result<bool,StatusCode> {
    bcrypt::verify(password, hash.as_str()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
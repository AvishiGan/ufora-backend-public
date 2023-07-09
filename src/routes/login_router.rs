
use axum::{
    Router, 
    routing::{post, get}
};

pub fn get_login_router() -> Router {
    Router::new()
        .route("/login", post(login))
    
}

async fn login() {
    println!("login");
}


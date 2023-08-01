use dotenvy::dotenv;

mod app;
mod routes;
mod database;
mod middlewares;
mod errors;
mod handlers;
mod services;
mod models;

#[tokio::main]
async fn main() {

    // load environment variables
    dotenv().ok();
    
    app::run().await.unwrap();


}

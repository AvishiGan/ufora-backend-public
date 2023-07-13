use dotenvy::dotenv;

mod app;
mod routes;
mod database;
mod middlewares;
mod errors;
mod handlers;
mod services;

#[tokio::main]
async fn main() {

    dotenv().ok();
    
    app::run().await.unwrap();


}

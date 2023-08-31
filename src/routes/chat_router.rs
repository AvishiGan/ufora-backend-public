use std::sync::Arc;

use axum::{Extension, Router};
use surrealdb::{engine::remote::ws::Client, Surreal};

use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

pub fn get_chat_router() -> Router<Arc<Surreal<Client>>> {
    let web_socket_extension = Arc::new(crate::services::websocket::get_websocket_extension());

    tracing_subscriber_init();

    Router::new()
        // .route("/api/chat", post(chat_handler))
        // .route("/api/chat/:id", get(chat_handlers::chat_route))
        .layer(Extension(web_socket_extension))
}

// function to initialize tracing subscriber
pub fn tracing_subscriber_init() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_chat=trace".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use axum::{routing::get, Extension, Router};
use reqwest::StatusCode;
use serde_json::{json, Value};
use surrealdb::{engine::remote::ws::Client, Surreal};

use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use crate::handlers::chat::group_chat_handlers;
use crate::handlers::chat::personal_chat_handlers::{self, startpersonalchat};
use crate::models::chat::{People, PersonalChat};

pub fn get_chat_router() -> Router<Arc<Surreal<Client>>> {
    let personal_chat_web_socket_extension =
        Arc::new(crate::services::websocket::get_personal_chat_websocket_extension());
    let group_chat_web_socket_extension =
        Arc::new(crate::services::websocket::get_group_chat_websocket_extension());

    tracing_subscriber_init();

    Router::new()
        .route("/ws/group", get(group_chat_handlers::websocket_handler))
        .layer(Extension(group_chat_web_socket_extension))
        .route(
            "/ws/personal",
            get(personal_chat_handlers::websocket_handler),
        )
        .route("/api/startPersonalChat", get(startpersonalchat))
        .layer(Extension(personal_chat_web_socket_extension))
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

use std::sync::Arc;

use axum::{Extension, response::IntoResponse, extract::{WebSocketUpgrade, ws::WebSocket}};

use crate::services::websocket::WebsocketExtension;

pub async fn websocket_handler(
    ws: WebSocketUpgrade, 
    Extension(websocket_extension): Extension<Arc<WebsocketExtension>>
) -> impl IntoResponse {
    // Upgrade the connection to a websocket connection.
    ws.on_upgrade(|socket| websocket(socket, websocket_extension))
}

async fn websocket(
    socket: WebSocket,
    websocket_extension: Arc<WebsocketExtension>
) {
    // Create a new room for this connection.
}
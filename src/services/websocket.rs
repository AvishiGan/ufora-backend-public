use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use tokio::sync::broadcast;

// chat room struct
pub struct RoomState {
    pub users: Mutex<HashSet<String>>,
    pub tx: broadcast::Sender<String>,
}

impl RoomState {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashSet::new()),
            tx: broadcast::channel(69).0,
        }
    }
}

// Struct to be shared as the extension among chat handlers
pub struct WebsocketExtension {
    pub rooms: Mutex<HashMap<String, RoomState>>,
}

pub fn get_websocket_extension() -> WebsocketExtension {
    WebsocketExtension {
        rooms: Mutex::new(HashMap::new()),
    }
}

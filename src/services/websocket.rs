use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use tokio::sync::broadcast;

// chat room struct
#[derive(Debug)]
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
#[derive(Debug)]
pub struct PersonalChatWebsocketExtension {
    pub rooms: Mutex<HashMap<String, RoomState>>,
}

pub fn get_personal_chat_websocket_extension() -> PersonalChatWebsocketExtension {
    PersonalChatWebsocketExtension {
        rooms: Mutex::new(HashMap::new()),
    }
}

// Struct to be shared as the extension among chat handlers
#[derive(Debug)]
pub struct GroupChatWebsocketExtension {
    pub rooms: Mutex<HashMap<String, RoomState>>,
}

pub fn get_group_chat_websocket_extension() -> GroupChatWebsocketExtension {
    GroupChatWebsocketExtension {
        rooms: Mutex::new(HashMap::new()),
    }
}

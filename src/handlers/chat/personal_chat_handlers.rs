use std::sync::{Arc, Mutex};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    Extension, Json,
};
use futures::{SinkExt, StreamExt};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::{json, Value};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};
use tokio::sync::broadcast;

use crate::{
    models::chat::{ChatMessage, People, PersonalChat},
    services::websocket::{PersonalChatWebsocketExtension, RoomState},
};

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,

    Extension(websocket_extension): Extension<Arc<PersonalChatWebsocketExtension>>,
) -> impl IntoResponse {
    // Upgrade the connection to a websocket connection.
    ws.on_upgrade(|socket| websocket(socket, websocket_extension, claim))
}

async fn websocket(
    stream: WebSocket,
    websocket_extension: Arc<PersonalChatWebsocketExtension>,
    claim: crate::models::user_claim::Claim,
) {
    // By splitting we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // Username gets set in the receive loop, if it's valid.
    //
    // We have more state now that needs to be pulled out of the connect loop
    let mut tx = None::<broadcast::Sender<String>>;
    let  userid  = claim.get_surrealdb_thing().to_string();
    let mut channel = String::new(); 

    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            let mut connect: Value = match serde_json::from_str(&name) {
                Ok(connect) => connect,
                Err(error) => {
                    tracing::error!(%error);
                    let _ = sender
                        .send(Message::Text(String::from(
                            "Failed to parse connect message",
                        )))
                        .await;
                    break;
                }
            };

            connect
                .as_object_mut()
                .unwrap()
                .insert("userid".to_string(), userid.clone().into());

            println!("connect: {:?}", connect);

            // Scope to drop the mutex guard before the next await
            {
                // If userid that is sent by client is not taken, fill userid string.
                let mut rooms = websocket_extension.rooms.lock().unwrap();

                channel = connect.get("channel").unwrap().to_string();
                println!("channel: {:?}", channel);
                let room = rooms.entry(connect.get("channel").unwrap().to_string()).or_insert_with(RoomState::new);

                tx = Some(room.tx.clone());

                // if the user is not in the room, add the user to the room
                if !Mutex::get_mut(&mut room.users)
                    .unwrap()
                    .contains(&connect.get("userid").unwrap().to_string())
                {
                    Mutex::get_mut(&mut room.users)
                        .unwrap()
                        .insert(userid.clone()); 
                }
            }

            // If not empty we want to quit the loop else we want to quit function.
            if tx.is_some() {
                break;
            } else {
                // Only send our client that userid is taken.
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.")))
                    .await;
                println!("Username already taken.");
                return;
            }
        }
    }

    // We know if the loop exited `tx` is not `None`.
    let tx = tx.unwrap();
    // Subscribe before sending joined message.
    let mut rx = tx.subscribe();

    // Send joined message to all subscribers.
    let msg = format!("{} joined.", userid);
    tracing::debug!("{}", msg);
    let _ = tx.send(msg);

    // This task will receive broadcast messages and send text message to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // We need to access the `tx` variable directly again, so we can't shadow it here.
    // I moved the task spawning into a new block so the original `tx` is still visible later.
    let mut recv_task = {
        // Clone things we want to pass to the receiving task.
        let tx = tx.clone();
        let name = userid.clone();

        let datetime = chrono::Local::now().to_string();

        // This task will receive messages from client and send them to broadcast subscribers.
        tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = receiver.next().await {
                let messagebodywrapped: Result<Value, serde_json::Error> =
                    serde_json::from_str(&text);

                let messagebody = messagebodywrapped.unwrap();

                println!("messagebody: {:?}", messagebody);

                // if the message is typing, send the message to all in the room
                let message = match messagebody.get("istyping") {
                    Some(_) => {
                        json!(
                            {
                                "userid": name,
                                "istyping": true,
                            }
                        )
                    }
                    None => {
                        json!(
                            {
                                "userid": name, 
                                "datetime": datetime,
                                "text":  messagebody.get("message"),
                                "image": messagebody.get("image"),
                                "reply": messagebody.get("reply"),
                            }
                        )
                    }
                };

                println!("{:?}", message);

                println!("{:?}", message);

                // Add userid before message.
                let _ = tx.send(message.to_string());
            }
        })
    };

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // when user left, get the room of the user
    let mut rooms = websocket_extension.rooms.lock().unwrap();

    // Remove userid from map so new clients can take it.
    Mutex::get_mut(&mut rooms.get_mut(&channel).unwrap().users)
        .unwrap()
        .remove(&userid.to_string());

    // send left message to all in the room
    let msg = format!("{} left.", userid);
    tracing::debug!("{}", msg);
    println!("{:?}", msg);
    let _ = tx.send(msg);

    // Check if the room is empty now and remove the `RoomState` from the map.
    if Mutex::get_mut(&mut rooms.get_mut(&channel).unwrap().users)
        .unwrap()
        .is_empty()
    {
        rooms.remove(&channel);
        println!("room removed: {:?}", channel);
    }
    println!("rooms: {:?}", rooms);
}

pub async fn startpersonalchat(
    State(db): State<Arc<Surreal<Client>>>,
    claim: crate::models::user_claim::Claim,
    Json(people): Json<People>,
) -> (StatusCode, Json<Value>) {
    let chat = PersonalChat::new();

    let create_chat = chat.create_chat_query(claim, db.clone(), Json(people)).await;

    match create_chat {
        Ok(_) => (),
        Err(e) => {
            println!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "Chat could not be created"
                })),
            );
        }
    }

    println!("create_chat: {:?}", create_chat);

    let response = db.query(create_chat.unwrap()).await;

    println!("response: {:?}", response);

    match response {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "message": "Chat created successfully"
            })),
        ),
        Err(e) => {
            println!("{:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "Chat could not be created"
                })),
            )
        }
    }
}

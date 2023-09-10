use std::sync::{Arc, Mutex};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    Extension,
};
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::broadcast;

use crate::services::websocket::{RoomState, WebsocketExtension};

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Extension(websocket_extension): Extension<Arc<WebsocketExtension>>,
) -> impl IntoResponse {
    // Upgrade the connection to a websocket connection.
    ws.on_upgrade(|socket| websocket(socket, websocket_extension))
}

async fn websocket(stream: WebSocket, websocket_extension: Arc<WebsocketExtension>) {
    // By splitting we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // Username gets set in the receive loop, if it's valid.
    //
    // We have more state now that needs to be pulled out of the connect loop
    let mut tx = None::<broadcast::Sender<String>>;
    let mut username = String::new();
    let mut channel = String::new();

    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            #[derive(Deserialize, Debug)]
            struct Connect {
                username: String,
                channel: String,
            }

            let connect: Connect = match serde_json::from_str(&name) {
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

            println!("connect: {:?}", connect);

            // Scope to drop the mutex guard before the next await
            {
                // If username that is sent by client is not taken, fill username string.
                let mut rooms = websocket_extension.rooms.lock().unwrap();

                channel = connect.channel.clone();
                println!("channel: {:?}", channel);
                let room = rooms.entry(connect.channel).or_insert_with(RoomState::new);

                tx = Some(room.tx.clone());

                // if !room.users.contains(&connect.username) {
                //     room.users.insert(connect.username.to_owned());
                //     username = connect.username.clone();
                // }

                if !Mutex::get_mut(&mut room.users)
                    .unwrap()
                    .contains(&connect.username)
                {
                    Mutex::get_mut(&mut room.users)
                        .unwrap()
                        .insert(connect.username.clone());
                    username = connect.username.clone();
                }
            }

            // If not empty we want to quit the loop else we want to quit function.
            if tx.is_some() && !username.is_empty() {
                break;
            } else {
                // Only send our client that username is taken.
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
    let msg = format!("{} joined.", username);
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
        let name = username.clone();
        let datetime = chrono::Local::now().to_string();

        // This task will receive messages from client and send them to broadcast subscribers.
        tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = receiver.next().await {
                // Add username before message.
                let _ = tx.send(
                    json!(
                        {
                            "username": name,
                            "datetime": datetime,
                            "text": text
                        }
                    )
                    .to_string(),
                );
            }
        })
    };

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Send user left message.
    let msg = format!("{} left.", username);
    tracing::debug!("{}", msg);
    let _ = tx.send(msg);
    let mut rooms = websocket_extension.rooms.lock().unwrap();

    // Remove username from map so new clients can take it.

    // mutex remove users from room
    Mutex::get_mut(&mut rooms.get_mut(&channel).unwrap().users)
        .unwrap()
        .remove(&username);

    // TODO: Check if the room is empty now and remove the `RoomState` from the map.
}

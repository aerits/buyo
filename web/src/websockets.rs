use std::time::Duration;
use axum::{extract::{ws::{Utf8Bytes, WebSocket}, WebSocketUpgrade}, response::IntoResponse};
use futures::{FutureExt, StreamExt};
use tokio::time::sleep;
use crate::state;

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

/// Actual websocket statemachine (one will be spawned per connection)
pub async fn handle_socket(socket: WebSocket) {
    let id = state.counter.lock().await.clone();
    *state.counter.lock().await += 1;
    println!("{} connected", id);
    state.connections.lock().await.insert(id, socket);
    'progress: loop {
        let echo_to_all_connections = async {
            let mut mutex_guard = state.connections.lock().await;
            let (_sender, mut reciever) = mutex_guard.get_mut(&id).unwrap().split();
            if let Some(Ok(msg)) = reciever.next().await {
                drop(reciever);
                drop(_sender);
                let msg = axum::extract::ws::Message::Text(
                    Utf8Bytes::try_from(format!(
                        "{}:: {}",
                        id,
                        msg.into_text().unwrap().to_string()
                    ))
                    .unwrap(),
                );
                for socket in mutex_guard.iter_mut() {
                    if socket.1.send(msg.clone()).await.is_err() {
                        println!("unable to send to {}", id);
                        return false;
                    }
                }
            }
            return true;
        };
        let output = futures::select! {
            out = echo_to_all_connections.fuse() => {out}
            _ = sleep(Duration::from_millis(5)).fuse() => {true}
        };
        if !output {
            break 'progress;
        }
        sleep(Duration::from_millis(100)).await
    }
    state.connections.lock().await.remove(&id);
    println!("{} disconnected", id);
}

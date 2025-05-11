use axum::extract::Query;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::Html,
    routing::{get, post},
};
use std::env;
use std::fs;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::time::sleep;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use axum::{
    Router,
    body::Bytes,
    extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::any,
};
use axum_extra::{TypedHeader, headers};

use std::ops::ControlFlow;
use std::{net::SocketAddr, path::PathBuf};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;

//allows to split the websocket stream into separate TX and RX branches
use futures::{FutureExt, lock::Mutex, sink::SinkExt, stream::StreamExt};

use lazy_static::lazy_static;

use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

#[derive(Clone)]
struct AppState {
    connections: Arc<Mutex<HashMap<usize, WebSocket>>>,
    counter: Arc<Mutex<usize>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            connections: Arc::new(Mutex::new(HashMap::new())),
            counter: Arc::new(Mutex::new(0)),
        }
    }
}

lazy_static! {
    static ref state: AppState = AppState::new();
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/query", get(query))
        .route("/game", get(game))
        .route("/game/ws", get(url))
        .route("/ws", any(ws_handler))
        .route("/cookies", get(cooky))
        .nest_service("/static", ServeDir::new("client_app_output/static"))
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
    // let contents = fs::read_to_string("client_app_output/index.html").unwrap().to_string();
    let contents = "bruh".to_string();
    Html(contents)
}

async fn game() -> Html<String> {
    let contents = fs::read_to_string("client_app_output/index.html")
        .unwrap()
        .to_string();
    Html(contents)
}

async fn url() -> &'static str {
    "http://localhost:5000/ws"
}

async fn query(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let mut contents = String::new();
    for (key, value) in params {
        contents += &*(key + ": " + &*value + "\n");
    }
    Html(contents)
}

async fn cooky(cookies: Cookies) -> String {
    let visits = cookies
        .get("visits")
        .and_then(|c| c.value().parse().ok())
        .unwrap_or(0);
    cookies.add(Cookie::new("visits", (visits + 1).to_string()));
    format!("You've been here {} times before", visits)
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(socket: WebSocket) {
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

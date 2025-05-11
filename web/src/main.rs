use std::collections::HashMap;
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::Html,
    Json
};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use std::fs;
use std::env;
use axum::extract::Query;

use axum::{
    body::Bytes,
    extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::any,
    Router,
};
use axum_extra::{headers, TypedHeader};

use std::ops::ControlFlow;
use std::{net::SocketAddr, path::PathBuf};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;

//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/query", get(query))
        .route("/game", get(game))
        .route("/game/ws", get(url))
        .route("/ws", any(ws_handler))
        .nest_service("/static", ServeDir::new("client_app_output/static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
    // let contents = fs::read_to_string("client_app_output/index.html").unwrap().to_string();
    let contents = "bruh".to_string();
    Html(contents)
}

async fn game() -> Html<String> {
    let contents = fs::read_to_string("client_app_output/index.html").unwrap().to_string();
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

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    println!("websocket?");
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    socket.send(Message::text("Hello, world!")).await.unwrap();
}
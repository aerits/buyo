use std::{collections::HashMap, sync::Arc};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use axum::{
    Router,
    body::Bytes,
    extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    extract::Query,
    response::{IntoResponse, Html},
    routing::{any, get},
};
use futures::lock::Mutex;
use lazy_static::lazy_static;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
mod websockets;

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
    // invalidate cache
    static ref version: i128 = fastrand::Rng::new().i128(0..99999999);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/query", get(query))
        .route("/game", get(game))
        .route("/lobbies", get(lobbies))
        .route("/game/ws", get(url))
        .route("/ws", any(websockets::ws_handler))
        .route("/cookies", get(cooky))
        .nest_service("/static", ServeDir::new("client_app_output/static"))
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn generate_template(style: Option<&str>, body: &str) -> Html<String> {
    Html(format!("
<!DOCTYPE html>
<html>

<head>
    <meta content=\"text/html;charset=utf-8\" http-equiv=\"Content-Type\"/>
    <style>
{}
    </style>
</head>

<body>

{}

</body>
</html>
    ", style.unwrap_or(""), body))
}

fn generate_menu(links: Vec<(&str, &str)>) -> Html<String> {
    let header = "<h1><a href=\"/\">PPTE</a></h1>".to_string();
    let body = links.iter().fold(header, 
|acc, cur| acc + &format!("<h2><a href=\"{}\">{}</a></h2>", cur.0, cur.1) );
    generate_template(None, &body)
}

async fn root() -> Html<String> {
    let contents = "
    <h1>PPTE</h1>
    <h2><a href=\"lobbies\">multiplayer</a></h2>
    <h2><a href=\"solo\">solo</a></h2>
    <h2><a href=\"settings\">settings</h2>
    ";
    // generate_template(None, contents)
    generate_menu(vec![("lobbies", "multiplayer"), ("solo", "solo"), ("settings", "settings")])
}

async fn lobbies() -> Html<String> {
    generate_menu(vec![("game?m=quickplay", "quickplay"), ("rooms", "rooms")])
}

async fn game() -> Html<String> {
    let header = "<h1><a href=\"/\">PPTE</a></h1>".to_string();
    let contents = header + &format!("<canvas id=\"my_canvas\"></canvas>

<script type=\"module\" src=\"./static/client.js?v={}\"></script>

<script type=\"module\">
      import main from \"./static/client.js?v={}\";
      main();
</script>", version.to_string(), version.to_string());
    let style = "<style>
        body {
            margin: 0px;
            padding: 0px;
        }
        canvas#my_canvas {
			position: absolute;
            width: 95%;
            height: 80%;
        }
    </style>";
    generate_template(Some(style), &contents)
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


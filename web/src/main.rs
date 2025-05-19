use std::{collections::HashMap, hash::{DefaultHasher, Hash}, sync::Arc};
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

        .route("/login", get(login))
        .route("/login_action", get(login_action))
        .route("/sign_out", get(sign_out))

        .route("/game/ws", get(url))
        .route("/ws", any(websockets::ws_handler))

        .route("/cookies", get(cooky))
        .route("/fake_login_cookie", get(fake_login_cookie))
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
    <title>PPTE</title>
    <meta content=\"text/html;charset=utf-8\" http-equiv=\"Content-Type\"/>
{}
</head>

<body>

{}

</body>
</html>
    ", style.unwrap_or(""), body))
}

fn generate_header() -> String {
    "<h1><a href=\"/\">PPTE</a></h1>".to_string()
}

fn generate_menu(links: Vec<(&str, &str)>) -> Html<String> {
    let header = generate_header();
    let body = links.iter().fold(header, 
|acc, cur| acc + &format!("<h2><a href=\"{}\">{}</a></h2>", cur.0, cur.1) );
    generate_template(None, &body)
}

async fn root(cookies: Cookies) -> Html<String> {
    let logged_in = cookies
        .get("login_token")
        .is_some();
    let login_link = if logged_in {
        (format!("u/{}", cookies.get("login_token").unwrap().value()),
        format!("{}", cookies.get("login_token").unwrap().value())
    )
    } else {("login".to_owned(), "login".to_owned())};
    let login_link = (login_link.0.as_str(), login_link.1.as_str());
    let mut links = vec![("lobbies", "multiplayer"), ("solo", "solo"), ("leaderboard", "leaderboard"), ("settings", "settings"), login_link];
    if logged_in {links.push(("sign_out", "sign out"))}
    generate_menu(links)
}

async fn lobbies() -> Html<String> {
    generate_menu(vec![("game?m=quickplay", "quickplay"), ("rooms", "rooms")])
}

async fn solo() -> Html<String> {
    generate_menu(vec![("game?m=40lines", "blocks: 40 lines"), ("game?m=2min", "blocks: 2 minutes")])
}

async fn settings() -> Html<String> {
    todo!()
}

async fn login() -> Html<String> {
    let header = generate_header();
    let body = header + "
    <form action=\"/login_action\">
    <label for=\"username\">username: </label><br>
    <input type=\"text\" id=\"username\" name=\"username\"><br>
    <label for=\"password\">password: </label><br>
    <input type=\"password\" id=\"password\" name=\"password\"><br>
    <input type=\"submit\" value=\"login\">
    </form>
    ";
    generate_template(None, &body)
}

async fn login_action(Query(params): Query<HashMap<String, String>>, cookies: Cookies) -> Html<String> {
    let mut body = generate_header();
    let username = params.get("username").map(|x| x.clone()).unwrap_or("".to_owned());
    let password = params.get("password").map(|x| x.clone()).unwrap_or("".to_owned());
    cookies.add(Cookie::new("login_token", username));
    body += "Logged in";
    generate_template(None, &body)
}

async fn sign_out(cookies: Cookies) -> Html<String> {
    let head = r#"<meta http-equiv="Refresh" content="0; url='/'" />"#;
    let body = "";
    cookies.remove(Cookie::new("login_token", ""));
    generate_template(Some(head), body)
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
    "https://erm.0000727.xyz/ws"
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

async fn fake_login_cookie(Query(params): Query<HashMap<String, String>>, cookies: Cookies) -> String {
    let contents = match params.get("username") {
        Some(x) => {
            cookies.add(Cookie::new("login_token", x.clone()));
            x.clone()
        },
        None => {"no username provided".to_owned()},
    };
    contents
}


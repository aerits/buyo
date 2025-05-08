use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::Utf8Bytes;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<State>>;
struct State {
    clients: HashMap<SocketAddr, Tx>,
    client_room: HashMap<SocketAddr, String>,
    client_stats: HashMap<SocketAddr, ClientStats>,
    rooms: HashMap<String, Room>,
}

impl State {
    pub fn new() -> State {
        State {
            clients: HashMap::new(),
            client_room: HashMap::new(),
            client_stats: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

struct Room {
    name: String,
    password: Option<String>,
}

struct ClientStats {
    garbage_queue: u64,
}

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().clients.insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!(
            "Received a message from {}: {}",
            addr,
            msg.to_text().unwrap()
        );
        let peers = peer_map.lock().unwrap();

        // We want to broadcast the message to everyone except ourselves.
        let broadcast_recipients = peers
            .clients
            .iter()
            .filter(|(peer_addr, _)| {
                // (peer_addr != &&addr) &&
                    (peers.client_room.get(&peer_addr) == peers.client_room.get(&&addr))
            })
            .map(|(_, ws_sink)| ws_sink);
        let my_string = addr.to_string() + ":: " + &*msg.into_text().unwrap();
        let new_message = Message::Text(Utf8Bytes::try_from(my_string.into_bytes()).unwrap());
        for recp in broadcast_recipients {
            recp.unbounded_send(new_message.clone()).unwrap();
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().clients.remove(&addr);
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:7272".to_string());

    let state = PeerMap::new(Mutex::new(State::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }

    Ok(())
}

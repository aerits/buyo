use crate::enums::GameState;
use blockstackers_core::blockstacker::{color, BlockStacker};
use blockstackers_core::buyo_game::BType;
use blockstackers_core::vectors::BVec;
use futures::{SinkExt, StreamExt};
use pharos::{Events, Observable, ObserveConfig};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;
use ws_stream_wasm::{WsEvent, WsMessage, WsMeta, WsStream};

pub struct NetworkConnection {
    ws: WsMeta,
    wsio: WsStream,
    evts: Events<WsEvent>,
}
impl NetworkConnection {
    pub async fn new(ws_url: &str) -> Result<NetworkConnection, Box<dyn Error>> {
        let (mut ws, wsio) = WsMeta::connect( ws_url, None ).await?;
        let evts = ws.observe( ObserveConfig::default() ).await.unwrap();
        Ok(NetworkConnection { ws, wsio, evts })
    }
    pub async fn next(&mut self) -> Option<String> {
        match self.wsio.next().await {
            Some(x) => {match x {
                WsMessage::Text(x) => {Some(x)}
                WsMessage::Binary(x) => {Some(String::from_utf8(x).unwrap())}
            }}
            None => None,
        }
    }
    pub async fn send(&mut self, text: &str) {
        let _ = self.wsio.send(WsMessage::Text(text.to_string())).await;
    }

}

pub fn serialize_game<T: BlockStacker<F>, F: Display>(game: &GameState) -> String {
    let mut s = String::new();
    match game {
        GameState::Gaming(game) => {
            s.push_str("Gaming: ");
            for (v, c) in game.game.get_board() {
                let a = "(".to_owned() +  &v.to_string() +  "," + &c.to_string() + ")";
                s.push_str( &a );
            }
            for (v, c) in game.game.get_controlled_block() {
                let a = "(".to_owned() +  &v.to_string() +  "," + &c.to_string() + ")";
                s.push_str( &a );
            }
        }
        GameState::Menu() => {
            s.push_str("Menu: ");
        }
        GameState::LoadingAssets => {
            s.push_str("LoadingAssets: ");
        }
        _ => {
            panic!("Unsupported game state");
        }
    }

    s
}

pub fn deserialize_game(game: &str)  -> HashMap<BVec, BType> {
    let mut map = HashMap::new();
    if game.contains("Gaming") {
        let a = game.split("Gaming: ");
        let a = a.last().unwrap().to_string();
        // BlockStacker::<F>::from_string(&a);
        let re = Regex::new(r"\(\[(\d*), (\d*)\],([A-z]*)\)").unwrap();
        for i in re.captures_iter( &a ) {
            let x = u32::from_str( &i[1] ).unwrap();
            let y = u32::from_str( &i[2] ).unwrap();
            let btyp = BType::from_str( &i[3] ).unwrap();
            map.insert(BVec::new(x as i32, y as i32),  btyp);
        }
    } else if game.contains("Menu") {
        let a = game.split("Menu: ");
    } else if game.contains("LoadingAssets") {
        let a = game.split("LoadingAssets: ");
    }
    map
}
use crate::gamehandler::GameHandler;
use blockstackers_core::buyo_game::{BType, BuyoBuyo};
use blockstackers_core::vectors::BVec;
use blockstackers_core::Sprite;
use speedy2d::window::VirtualKeyCode;
use std::collections::HashMap;

pub enum GameState {
    // Gaming(GameHandler<BuyoBuyo, BType>),
    Gaming(GameHandler),
    Menu(),
    LoadingAssets,
    Error(String),
}
impl GameState {
    pub fn handle_inputs(
        &mut self,
        current_time: u64,
        pressed_down_keys: &mut HashMap<VirtualKeyCode, u64>,
        auto_repeating_keys: &mut HashMap<VirtualKeyCode, u64>,
    ) {
        match self {
            GameState::Gaming(game_handler) => {
                game_handler.handle_inputs(&current_time, pressed_down_keys, auto_repeating_keys);
            }
            _ => {}
        }
    }
    pub fn add_player(&mut self, addr: String, player: HashMap<BVec, Sprite>) {
        match self  {
            GameState::Gaming(game_handler) => {
                game_handler.other_players.insert(addr, player);
            }
            _ => {}
        }
    }
}

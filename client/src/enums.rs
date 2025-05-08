use std::collections::HashMap;
use std::fmt::Display;
use blockstackers_core::buyo_game::{BType, BuyoBuyo};
use speedy2d::window::VirtualKeyCode;
use blockstackers_core::blockstacker::BlockStacker;
use blockstackers_core::vectors::BVec;
use crate::gamehandler::GameHandler;

pub enum GameState {
    Gaming(GameHandler<BuyoBuyo, BType>),
    Menu,
    LoadingAssets,
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
            GameState::Menu => (),
            GameState::LoadingAssets => (),
        }
    }
    pub fn add_player(&mut self, addr: String, player: HashMap<BVec, BType>) {
        match self  {
            GameState::Gaming(game_handler) => {
                game_handler.other_players.insert(addr, player);
            }
            GameState::LoadingAssets => {},
            GameState::Menu => (),
        }
    }
}

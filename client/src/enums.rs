use std::collections::HashMap;

use blockstackers_core::buyo_game::{BType, BuyoBuyo};
use speedy2d::window::VirtualKeyCode;

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
                game_handler.handle_inputs(&current_time, pressed_down_keys, auto_repeating_keys)
            }
            GameState::Menu => (),
            GameState::LoadingAssets => (),
        }
    }
}

use blockstackers_core::buyo_game::{BType, BuyoBuyo};

use crate::gamehandler::GameHandler;

pub enum GameState {
    Gaming(GameHandler<BuyoBuyo, BType>),
    Menu,
    LoadingAssets,
}
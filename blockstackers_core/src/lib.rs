use std::fmt::Display;

pub mod blockstacker;
pub mod buyo_game;
pub mod randomizer;
pub mod vectors;
pub mod tet;
mod ring_buffer;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Sprite {
    Wall,
    
    BuyoRed,
    BuyoBlue,
    BuyoYellow,
    BuyoPurple,
    BuyoGreen,

    TetT,
    TetI,
    TetO,
    TetJ,
    TetL,
    TetS,
    TetZ,

    TetGhostT,
    TetGhostI,
    TetGhostO,
    TetGhostJ,
    TetGhostL,
    TetGhostS,
    TetGhostZ,
}
impl Display for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
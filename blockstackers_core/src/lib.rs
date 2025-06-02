use std::fmt::Display;

use strum_macros::EnumString;

pub mod blockstacker;
pub mod buyo_game;
pub mod randomizer;
pub mod vectors;
pub mod tet;
mod ring_buffer;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, EnumString)]
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
impl Sprite {
    pub fn from_str(s: &str) -> Option<Sprite> {
        return s.parse::<Sprite>().ok()
    }
}
impl Display for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
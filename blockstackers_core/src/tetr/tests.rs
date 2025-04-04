use std::collections::HashMap;

use crate::{blockstacker::BlockStacker, randomizer::Randomizer};

use super::{Tetr};
use crate::tetr::enums::Shapes;

#[test]
fn test_rotate_o() {
    let mut game = Tetr::new(10, 24, Randomizer::new(7, 727));
    game.spawn_c_mino(Shapes::O);
}
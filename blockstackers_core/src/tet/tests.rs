use std::collections::HashMap;

use crate::{blockstacker::BlockStacker, randomizer::Randomizer};

use super::{Tet};
use crate::tet::enums::Shapes;

#[test]
fn test_rotate_o() {
    let mut game = Tet::new(10, 24, Randomizer::new(7, 727));
    game.spawn_c_mino(Shapes::O);
    let init_pos = game.controlled_mino.take().unwrap().vec.clone();
    game.spawn_c_mino(Shapes::O);
    game.rotate_c_mino(1);
    assert_eq!(init_pos, game.controlled_mino.unwrap().vec);

}
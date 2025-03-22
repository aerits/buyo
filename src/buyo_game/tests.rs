use crate::{blockstacker::BlockStacker, randomizer};

use super::Game;
use super::BType;

struct test_init {
    bg: Game,
}

impl test_init {
    pub fn init() -> test_init {
        test_init {
            bg: Game::new(6, 12, randomizer::Randomizer::new(4)),
        }
    }
}

// comment out web stuff to run tests

#[test]
fn test_score_1chain() {
    let mut a = test_init::init();
    a.bg.chain_power = 1;
    a.bg.color_bonus.insert(BType::Blue);
    a.bg.puyos_cleared = 4;
    a.bg.group_bonus.push(0);
    assert_eq!(40, a.bg.score())
}

#[test]
fn test_score_2chain() {
    let mut a = test_init::init();
    a.bg.chain_power = 2;
    a.bg.color_bonus.insert(BType::Blue);
    // a.bg.color_bonus.insert(BType::Red);
    a.bg.puyos_cleared = 4;
    a.bg.group_bonus.push(0);
    // a.bg.group_bonus.push(0);
    assert_eq!(320, a.bg.score())
}

use std::time::Instant;

use crate::blockstacker::Tuning;
use crate::Sprite;
use crate::{blockstacker::BlockStacker, randomizer};

use super::BuyoBuyo;

struct TestInit {
    bg: BuyoBuyo,
}

impl TestInit {
    pub fn init() -> TestInit {
        TestInit {
            bg: BuyoBuyo::new(6, 12, randomizer::Randomizer::new(4, 69), Tuning::new()),
        }
    }
}

// comment out web stuff to run tests

#[test]
fn test_score_1chain() {
    let mut a = TestInit::init();
    a.bg.chain_power = 1;
    a.bg.color_bonus.insert(Sprite::BuyoBlue);
    a.bg.puyos_cleared = 4;
    a.bg.group_bonus.push(0);
    assert_eq!(40, a.bg.score())
}

#[test]
fn test_score_2chain() {
    let mut a = TestInit::init();
    a.bg.chain_power = 2;
    a.bg.color_bonus.insert(Sprite::BuyoBlue);
    // a.bg.color_bonus.insert(BType::Red);
    a.bg.puyos_cleared = 4;
    a.bg.group_bonus.push(0);
    // a.bg.group_bonus.push(0);
    assert_eq!(320, a.bg.score())
}

#[test]
fn test_randomizer() {
    let mut a = TestInit::init();
    a.bg.next_queue();
    a.bg.game_loop(0, 99999); 
    a.bg.hard_drop();
    a.bg.next_queue();
    a.bg.next_queue();
    a.bg.game_loop(0, 99999); 
    a.bg.hard_drop();
    a.bg.next_queue();
    a.bg.game_loop(0, 99999); 
    a.bg.hard_drop();
    a.bg.next_queue();
    a.bg.game_loop(0, 99999); 
    a.bg.hard_drop();
}

#[test]
fn test_offset() {
    let mut a = TestInit::init();
    let time_start = Instant::now();
    let mut time_last_update = Instant::now();
    let mut updates = 10;
    while updates > 0 {
        let b = a.bg.game_loop(time_last_update.duration_since(time_start).as_millis() as u64,
    Instant::now().duration_since(time_start).as_millis() as u64);
        if b {time_last_update = Instant::now(); a.bg.print_grid(); updates-=1;}
    }
    assert_eq!(0, 1);
}

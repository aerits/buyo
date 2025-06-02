use std::collections::HashMap;
use std::fmt::Display;
use crate::buyo_game::BuyoBuyo;
use crate::randomizer::{self, Randomizer};
use crate::tet::Tet;
use crate::vectors::BVec;
use crate::Sprite;

pub trait color {
    fn from_str(color: &str) -> Option<Self>
    where
        Self: Sized;
}

pub trait BlockStacker {
    // fn new(width: i32, height: i32, randomizer: Randomizer, tuning: Tuning) -> Box<>;
    fn get_board(&self) -> HashMap<BVec, Sprite>;
    fn next_queue(&self) -> HashMap<BVec, Sprite>;
    fn get_controlled_block(&self) -> Vec<(f32, f32, Sprite)>;
    fn input_left(&mut self) -> bool;
    fn input_right(&mut self) -> bool;
    fn input_rotation_right(&mut self);
    fn input_rotation_left(&mut self);
    fn input_180_rot(&mut self);
    fn hard_drop(&mut self);
    fn move_c_buyo_down(&mut self) -> bool;
    fn is_on_ground(&self) -> bool;
    fn score(&self) -> i32;
    fn total_score(&self) -> i32;
    fn game_loop(&mut self, last_update_time: u64, current_time: u64) -> bool;
    fn get_loop_state(&self) -> String;
    fn get_mut_tuning(&mut self) -> &mut Tuning;
}

impl dyn BlockStacker {
    pub fn new(type_: &str, width: i32, height: i32, randomizer: Randomizer, tuning: Tuning) -> Box<dyn BlockStacker> {
        match type_ {
            "buyo" => Box::new(BuyoBuyo::new(6, 12, randomizer, tuning)),
            "tet" => Box::new(Tet::new(10, 24, randomizer, tuning)),
            _ => panic!()
        }
    }
}

/// milliseconds for every thing that can be changed
pub struct Tuning {
    pub das: u64,
    pub arr: u64,
    pub lock_delay: u64,
    pub freeze_delay: u64,
    pub clear_delay: u64,
    pub spawn_delay: u64,
    /// ms to wait between moving the block down by fall skip
    pub fall_speed: u64,
    /// how many blocks the block moves down by every fall speed
    pub fall_skip: f32,
}

impl Tuning {
    pub fn new() -> Tuning {
        Tuning { das: 133, arr: 5, lock_delay: 20, freeze_delay: 1000, clear_delay: 500, spawn_delay: 0, fall_speed: 200, fall_skip: 0.5 }
    }
}
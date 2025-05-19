use std::collections::HashMap;
use std::fmt::Display;
use crate::randomizer::Randomizer;
use crate::vectors::BVec;
use crate::Sprite;

pub trait color {
    fn from_str(color: &str) -> Option<Self>
    where
        Self: Sized;
}

pub trait BlockStacker {
    fn new(width: i32, height: i32, randomizer: Randomizer, tuning: Tuning) -> Self;
    fn get_board(&self) -> HashMap<BVec, Sprite>;
    fn next_queue(&self) -> HashMap<BVec, Sprite>;
    fn get_controlled_block(&self) -> HashMap<BVec, Sprite>;
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
}

/// milliseconds for every thing that can be changed
pub struct Tuning {
    pub das: u64,
    pub arr: u64,
    pub lock_delay: u64,
    pub freeze_delay: u64,
    pub clear_delay: u64,
    pub spawn_delay: u64,
    pub fall_speed: u64,
}

impl Tuning {
    pub fn new() -> Tuning {
        Tuning { das: 133, arr: 5, lock_delay: 100, freeze_delay: 0, clear_delay: 500, spawn_delay: 0, fall_speed: 500 }
    }
}
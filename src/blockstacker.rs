use std::collections::HashMap;
use crate::Randomizer;
use crate::vectors::BVec;

pub trait BlockStacker<T> {
    fn new(width: i32, height: i32, randomizer: Randomizer) -> impl BlockStacker<T>;
    fn board(&self) -> HashMap<BVec, T>;
    fn input_left(&mut self);
    fn input_right(&mut self);
    fn input_rotation_right(&mut self);
    fn input_rotation_left(&mut self);
    fn input_180_rot(&mut self);
    fn hard_drop(&mut self);
    fn move_c_buyo_down(&mut self);
    fn game_loop(&mut self, time_to_freeze: bool) -> bool;
}
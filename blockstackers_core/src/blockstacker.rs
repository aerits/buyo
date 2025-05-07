use std::collections::HashMap;
use crate::randomizer::Randomizer;
use crate::vectors::BVec;

pub trait BlockStacker<T> {
    fn new(width: i32, height: i32, randomizer: Randomizer) -> Self;
    fn get_board(&self) -> HashMap<BVec, T>;
    fn next_queue(&self) -> HashMap<BVec, T>;
    fn convert_t_to_speedy2d_color(&self, t: T) -> speedy2d::color::Color;
    fn get_controlled_block(&self) -> HashMap<BVec, T>;
    fn input_left(&mut self);
    fn input_right(&mut self);
    fn input_rotation_right(&mut self);
    fn input_rotation_left(&mut self);
    fn input_180_rot(&mut self);
    fn hard_drop(&mut self);
    fn move_c_buyo_down(&mut self) -> bool;
    fn is_on_ground(&self) -> bool;
    fn score(&self) -> i32;
    fn total_score(&self) -> i32;
    fn game_loop(&mut self, time_to_freeze: bool) -> i32;
}
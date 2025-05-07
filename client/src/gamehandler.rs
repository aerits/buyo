use std::{collections::HashMap, marker::PhantomData};

use blockstackers_core::{blockstacker::BlockStacker, randomizer::Randomizer};
use speedy2d::{color::Color, font::{TextLayout, TextOptions}, window::VirtualKeyCode, Graphics2D};

use crate::jstime::get_current_time;
use crate::assets::Assets;

pub struct GameHandler<T: BlockStacker<F>, F> {
    game: T,
    phantom: std::marker::PhantomData<F>,
    last_update_time: u64,
    is_time_to_freeze: bool,               // set by game,
    freeze_time: u64,                      // set by game
    timestamp_when_on_ground: Option<u64>, // set by game
    das: u64,                              // set by user
    arr: u64,                              // set by user
    last_fall_time: u64,
    gravity: u64, // set by game
    block_offset: f32,
    block_offset_dy: f32,
    fps: i32,
}
impl<T: BlockStacker<F>, F> GameHandler<T, F> {
    pub fn new(width: i32, height: i32) -> GameHandler<T, F> {
        GameHandler {
            game: T::new(
                width,
                height,
                Randomizer::new(4, get_current_time() as u128),
            ),
            phantom: PhantomData,
            last_update_time: get_current_time(),
            is_time_to_freeze: false,
            freeze_time: 5000,
            timestamp_when_on_ground: None, // this says if its on the ground when did it land
            das: 133,                       // 133 ms
            arr: 1,                        // 10 ms
            last_fall_time: get_current_time(),
            gravity: 1,
            block_offset: 0.0, // this lets the buyos move down smoothly instead of moving a whole block down
            block_offset_dy: 0.5,
            fps: 0,
        }
    }
    pub fn handle_inputs(
        &mut self,
        current_time: &u64,
        pressed_down_keys: &mut HashMap<VirtualKeyCode, u64>,
        auto_repeating_keys: &mut HashMap<VirtualKeyCode, u64>,
    ) {
        for (key, time) in &auto_repeating_keys.clone() {
            if *current_time - *time > self.das && (*current_time - *time) % self.arr < 1 {
                match key {
                    VirtualKeyCode::Left => self.game.input_left(),
                    VirtualKeyCode::Right => self.game.input_right(),
                    _ => (),
                }
            }
        }
        for (key, time) in &pressed_down_keys.clone() {
            // log::info!("{:?}, {}", key, time);
            match key {
                VirtualKeyCode::Left => {
                    if !auto_repeating_keys.contains_key(&key) {
                        self.game.input_left();
                        auto_repeating_keys.insert(*key, *time);
                    }
                }
                VirtualKeyCode::Right => {
                    if !auto_repeating_keys.contains_key(&key) {
                        self.game.input_right();
                        auto_repeating_keys.insert(*key, *time);
                    }
                }
                VirtualKeyCode::Z => {
                    // if key_just_pressed(&current_time, time) {
                    self.game.input_rotation_right();
                    pressed_down_keys.remove(key);
                    // }
                }
                VirtualKeyCode::X => {
                    // if key_just_pressed(&current_time, time) {
                    self.game.input_rotation_left();
                    pressed_down_keys.remove(key);
                    // }
                }
                VirtualKeyCode::Space => {
                    // if key_just_pressed(&current_time, time) {
                    self.block_offset = 0.0;
                    self.game.hard_drop();
                    self.last_update_time = 0; // unix epoch time
                    pressed_down_keys.remove(key);
                    // }
                }
                _ => (),
            }
        }
    }
    pub fn update(&mut self, current_time: u64) {
        if current_time - self.last_update_time > 300 {
            // game_handler.game.print_grid();
            self.game.game_loop(self.is_time_to_freeze);
            self.last_update_time = current_time;
        }
        if current_time - self.last_fall_time > self.gravity {
            self.last_fall_time = current_time;
            self.block_offset += self.block_offset_dy;
            if self.block_offset >= 20.0 {
                // diameter of block
                self.game.move_c_buyo_down();
                self.block_offset -= 20.0;
            }
            // println!("{}", self.fps);
            // self.fps = 0;
        }
        // self.fps += 1;

        if self.game.is_on_ground() {
            self.block_offset = 0.0;
            match self.timestamp_when_on_ground {
                Some(timestamp) => {
                    if current_time - timestamp > self.freeze_time {
                        self.timestamp_when_on_ground = None;
                        self.is_time_to_freeze = true;
                        self.last_update_time = 0;
                    }
                }
                None => {
                    self.timestamp_when_on_ground = Some(get_current_time());
                }
            }
        } else {
            self.is_time_to_freeze = false;
            self.timestamp_when_on_ground = None;
        }
    }
    pub fn draw(
        &self,
        graphics: &mut Graphics2D,
        assets: &Assets,
    ) {
        graphics.clear_screen(Color::WHITE);
        // graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);
        for (v, c) in self.game.get_board() {
            graphics.draw_circle(
                (v.x as f32 * 20.0 + 20.0, v.y as f32 * 20.0 + 20.0),
                10.0,
                self.game.convert_t_to_speedy2d_color(c),
            );
        }
        for (v, c) in self.game.get_controlled_block() {
            graphics.draw_circle(
                (
                    v.x as f32 * 20.0 + 20.0,
                    v.y as f32 * 20.0 + 20.0 + self.block_offset,
                ),
                10.0,
                self.game.convert_t_to_speedy2d_color(c),
            );
        }
        // next queue
        // let (a, b) = self.game.next_buyo();
        for (v, c) in self.game.next_queue() {
            graphics.draw_circle(
                (200.0, 90.0 + (v.y * 20) as f32),
                10.0,
                self.game.convert_t_to_speedy2d_color(c),
            );
        }

        let score = assets.font.as_ref().unwrap().layout_text(
            &format!("{}", self.game.total_score()),
            50.0,
            TextOptions::new(),
        );
        let chainscore = assets.font.as_ref().unwrap().layout_text(
            &format!("{}", self.game.score()),
            50.0,
            TextOptions::new(),
        );
        graphics.draw_text((200.0, 130.0), Color::BLACK, &score);
        graphics.draw_text((200.0, 170.0), Color::BLACK, &chainscore);
        // log::info!("{}", self.game.score());

        // log::info!("{}", get_current_time());
    }
}
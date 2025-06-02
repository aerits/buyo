use std::{collections::HashMap, marker::PhantomData};
use std::fmt::Display;
use blockstackers_core::Sprite;
use blockstackers_core::{blockstacker::BlockStacker, randomizer::Randomizer};
use speedy2d::{color::Color, font::{TextLayout, TextOptions}, window::VirtualKeyCode, Graphics2D};
use blockstackers_core::vectors::BVec;
use crate::jstime::get_current_time;
use crate::assets::Assets;

pub struct GameHandler {
    pub game: Box<dyn BlockStacker>,
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
    pub other_players: HashMap<String, HashMap<BVec, Sprite>>,
    pub key_pressed: bool,
}
impl GameHandler {
    pub fn new(game: Box<dyn BlockStacker>) -> GameHandler {
        GameHandler {
            game,
            last_update_time: get_current_time(),
            is_time_to_freeze: false,
            freeze_time: 5000,
            timestamp_when_on_ground: None, // this says if its on the ground when did it land
            das: 133,                       // 133 ms
            arr: 1,                        // 10 ms
            last_fall_time: get_current_time(),
            gravity: 1,
            block_offset: 0.0, // this lets the buyos move down smoothly instead of moving a whole block down
            block_offset_dy: 1.0,
            fps: 0,
            other_players: HashMap::new(),
            key_pressed: false,
        }
    }
    pub fn handle_inputs(
        &mut self,
        current_time: &u64,
        pressed_down_keys: &mut HashMap<VirtualKeyCode, u64>,
        auto_repeating_keys: &mut HashMap<VirtualKeyCode, u64>,
    ) -> bool {
        let mut output = false;
        for (key, time) in &auto_repeating_keys.clone() {
            if *current_time - *time > self.das && (*current_time - *time) % self.arr < 1 {
                output = match key {
                    VirtualKeyCode::Left => self.game.input_left(),
                    VirtualKeyCode::Right => self.game.input_right(),
                    _ => false,
                };
            }
        }
        for (key, time) in &pressed_down_keys.clone() {
            // log::info!("{:?}, {}", key, time);
            match key {
                VirtualKeyCode::Left => {
                    if !auto_repeating_keys.contains_key(&key) {
                        self.game.input_left();
                        output = true;
                        auto_repeating_keys.insert(*key, *time);
                    }
                }
                VirtualKeyCode::Right => {
                    if !auto_repeating_keys.contains_key(&key) {
                        self.game.input_right();
                        output = true;
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
        if output {self.key_pressed = output};
        return output;
    }

    pub fn update(&mut self, current_time: u64) -> i32 {
        if self.game.game_loop(self.last_update_time, current_time) {
            self.last_update_time = current_time;
        }
        return 0;
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
                convert_sprite_to_speedy2d_color(&c),
            );
        }
        for (i, (_ip, player)) in self.other_players.iter().enumerate() {
            for (v, c) in player {
                let mut v = *v;
                v.add_i(10*(i+1) as i32, 0);
                graphics.draw_circle(
                    (v.x as f32 * 20.0 + 20.0, v.y as f32 * 20.0 + 20.0),
                    10.0,
                    convert_sprite_to_speedy2d_color(c),
                )
            }
        }
        for (x, y, c) in self.game.get_controlled_block() {
            graphics.draw_circle(
                (
                    x as f32 * 20.0 + 20.0,
                    y as f32 * 20.0 + 20.0 + self.block_offset,
                ),
                10.0,
                convert_sprite_to_speedy2d_color(&c),
            );
        }
        // next queue
        // let (a, b) = self.game.next_buyo();
        for (v, c) in self.game.next_queue() {
            graphics.draw_circle(
                (200.0, 90.0 + (v.y * 20) as f32),
                10.0,
                convert_sprite_to_speedy2d_color(&c),
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

fn convert_sprite_to_speedy2d_color(s: &Sprite) -> Color {
    match s {
        Sprite::Wall => Color::BLACK,
        Sprite::BuyoRed => Color::RED,
        Sprite::BuyoBlue => Color::BLUE,
        Sprite::BuyoYellow => Color::YELLOW,
        Sprite::BuyoPurple => Color::MAGENTA,
        Sprite::BuyoGreen => Color::GREEN,
        Sprite::TetT => Color::MAGENTA,
        Sprite::TetI => Color::CYAN,
        Sprite::TetO => Color::YELLOW,
        Sprite::TetJ => Color::BLUE,
        Sprite::TetL => Color::from_int_rgb(255, 255, 0), // orange
        Sprite::TetS => Color::GREEN,
        Sprite::TetZ => Color::RED,
        Sprite::TetGhostT => color_to_transparent(Color::MAGENTA),
        Sprite::TetGhostI => color_to_transparent(Color::CYAN),
        Sprite::TetGhostO => color_to_transparent(Color::YELLOW),
        Sprite::TetGhostJ => color_to_transparent(Color::BLUE),
        Sprite::TetGhostL => color_to_transparent(Color::from_int_rgb(255, 255, 0)), // orange
        Sprite::TetGhostS => color_to_transparent(Color::GREEN),
        Sprite::TetGhostZ => color_to_transparent(Color::RED),
    }
}
fn color_to_transparent(c: Color) -> Color {
    return Color::from_rgba(c.r(), c.g(), c.b(), 0.5)
}
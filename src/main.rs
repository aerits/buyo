use blockstacker::BlockStacker;
use buyo_game::{BType, Game};
use randomizer::Randomizer;
use speedy2d::color::Color;
use speedy2d::window::{VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use std::alloc::System;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::rc::Rc;
use std::time::Duration;
use std::{
    io,
    time::{self, SystemTime},
};

fn main() {
    let window = Window::new_centered("Title", (640, 480)).unwrap();
    window.run_loop(MyWindowHandler::new());
}

mod buyo_game;
mod randomizer;
mod vectors;
mod blockstacker;

enum GameState {
    Gaming(GameHandler<Game, BType>),
    Menu,
}

struct GameHandler<T: BlockStacker<F>, F> {
    game: T,
    phantom: std::marker::PhantomData<F>,
    last_update_time: SystemTime,
    time_to_freeze: bool, // set by game
    das: Duration, // set by user
    arr: Duration, // set by user
    last_fall_time: SystemTime,
    gravity: Duration, // set by game
    fps: i32,
}
impl <T: BlockStacker<F>, F> GameHandler<T, F> {
    pub fn new() -> GameHandler<T, F> {
        GameHandler {
            game: T::new(6, 12, Randomizer::new(4)),
            phantom: PhantomData,
            last_update_time: SystemTime::now(),
            time_to_freeze: false,
            das: Duration::from_millis(120),
            arr: Duration::from_millis(20),
            last_fall_time: SystemTime::now(),
            gravity: Duration::from_secs(1),
            fps: 0
        }
    }
}

struct MyWindowHandler {
    state: GameState,
    pressed_down_keys: HashMap<VirtualKeyCode, SystemTime>,
}

impl MyWindowHandler {
    pub fn new() -> MyWindowHandler {
        MyWindowHandler {
            state: GameState::Gaming(GameHandler::new()),
            pressed_down_keys: HashMap::new(),
        }
    }
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        match self.state {
            GameState::Gaming(ref mut game_handler) => {
                //////////////////////////////////////////////// HANDLE INPUTS
                for (key, time) in &self.pressed_down_keys {
                    match key {
                        VirtualKeyCode::Left => {
                            if SystemTime::now().duration_since(*time).unwrap() > game_handler.das
                                || key_just_pressed(time)
                            {
                                if SystemTime::now().duration_since(*time).unwrap().as_millis() % game_handler.arr.as_millis() == 0 {
                                    game_handler.game.input_left()
                                }
                                
                            }
                        }
                        VirtualKeyCode::Right => {
                            if SystemTime::now().duration_since(*time).unwrap() > game_handler.das
                                || key_just_pressed(time)
                            {
                                if SystemTime::now().duration_since(*time).unwrap().as_millis() % game_handler.arr.as_millis() == 0 {
                                    game_handler.game.input_right()
                                }
                            }
                        }
                        VirtualKeyCode::Z => {
                            if key_just_pressed(time) {
                                game_handler.game.input_rotation_right()
                            }
                        }
                        VirtualKeyCode::X => {
                            if key_just_pressed(time) {
                                game_handler.game.input_rotation_left()
                            }
                        }
                        VirtualKeyCode::Space => {
                            if key_just_pressed(time) {
                                game_handler.game.hard_drop();
                                game_handler.last_update_time = SystemTime::UNIX_EPOCH; 
                            }
                        }
                        _ => panic!(),
                    }
                }

                ///////////////////////////////////////////////// HANDLE GAME LOGIC
                if SystemTime::now()
                    .duration_since(game_handler.last_update_time)
                    .unwrap() > Duration::from_millis(500)
                {
                    // game_handler.game.print_grid();
                    let on_floor = !game_handler.game.game_loop(game_handler.time_to_freeze);
                    game_handler.last_update_time = SystemTime::now();
                    if on_floor {
                        game_handler.time_to_freeze = true;
                    } else {
                        game_handler.time_to_freeze = false;
                    }
                }
                if SystemTime::now().duration_since(game_handler.last_fall_time).unwrap() > game_handler.gravity {
                    game_handler.last_fall_time = SystemTime::now();
                    game_handler.game.move_c_buyo_down();
                    println!("{}", game_handler.fps);
                    game_handler.fps = 0;
                }
                game_handler.fps += 1;

                /////////////////////////////////////////// HANDLE DRAWING THE GAME
                graphics.clear_screen(Color::WHITE);
                // graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);
                for (v, c) in game_handler.game.board() {
                    graphics.draw_circle(
                        (v.x as f32 * 20.0 + 20.0, v.y as f32 * 20.0 + 20.0),
                        10.0,
                        btype_to_color(c),
                    );
                }
                // next queue
                let (a, b) = game_handler.game.next_buyo();
                graphics.draw_circle((500.0, 90.0), 10.0, btype_to_color(a));
                graphics.draw_circle((500.0, 110.0), 10.0, btype_to_color(b));
                helper.request_redraw();
            }
            GameState::Menu => todo!(),
        }
    }
    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
        scancode: speedy2d::window::KeyScancode,
    ) {
        match virtual_key_code {
            Some(x) => {
                if !self.pressed_down_keys.contains_key(&x) {
                    // if the key was just pressed, add it to the pressed down keys with a timestamp of when it was pressed
                    self.pressed_down_keys.insert(x, SystemTime::now());
                }
            }
            None => {}
        };
    }
    fn on_key_up(
        &mut self,
        helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: speedy2d::window::KeyScancode,
    ) {
        match virtual_key_code {
            Some(x) => {
                self.pressed_down_keys.remove(&x);
            }
            None => {}
        };
    }
}

fn btype_to_color(b: BType) -> Color {
    match b {
        BType::Red => Color::RED,
        BType::Blue => Color::BLUE,
        BType::Green => Color::GREEN,
        BType::Purple => Color::MAGENTA,
        BType::Wall => Color::BLACK,
    }
}

pub fn key_just_pressed(time: &SystemTime) -> bool {
    return SystemTime::now().duration_since(*time).unwrap() < Duration::from_millis(2);
}

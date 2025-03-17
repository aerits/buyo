use blockstacker::BlockStacker;
use buyo_game::{BType, Game};
use jstime::get_current_time;
use randomizer::Randomizer;
use reqwest::{Client, Response};
use speedy2d::color::Color;
use speedy2d::font::{Font, FormattedTextBlock, TextLayout, TextOptions};
use speedy2d::window::{VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, WebCanvas};
use std::collections::HashMap;
use std::marker::PhantomData;

fn main() {
    // let window = Window::new_centered("Title", (640, 480)).unwrap();
    // window.run_loop(MyWindowHandler::new());
    // wasm_logger::init(wasm_logger::Config::default());
    // std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // log::info!("Speedy2D WebGL sample");

    wasm_logger::init(wasm_logger::Config::default());
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut window = MyWindowHandler::new();
    log::info!("version 79");
    wasm_bindgen_futures::spawn_local(async move {
        let mut last_check = 0;
        while window.assets.font.is_none() {
            if get_current_time() - last_check > 1000 {
                window.assets.load().await;
                last_check = get_current_time();
            }
        }
        WebCanvas::new_for_id_with_user_events("my_canvas", window).unwrap();
    });
}

mod blockstacker;
mod buyo_game;
mod jstime;
mod randomizer;
mod vectors;

enum GameState {
    Gaming(GameHandler<Game, BType>),
    Menu,
    LoadingAssets,
}

struct GameHandler<T: BlockStacker<F>, F> {
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
            game: T::new(width, height, Randomizer::new(4)),
            phantom: PhantomData,
            last_update_time: get_current_time(),
            is_time_to_freeze: false,
            freeze_time: 5000,
            timestamp_when_on_ground: None, // this says if its on the ground when did it land
            das: 133,                       // 120 ms
            arr: 20,                        // 20 ms
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
            if *current_time - *time > self.das {
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
}

struct Assets {
    client: Client,
    font: Option<Font>,
    site_url: String,
}
impl Assets {
    pub fn new() -> Assets {
        let win = web_sys::window().unwrap();
        let url = win.document().unwrap().url().unwrap();
        Assets {
            client: Client::new(),
            font: None,
            site_url: url,
        }
    }
    pub async fn load(&mut self) {
        let resp = self.load_var("/static/assets/fonts/arial.ttf").await;
        match resp {
            Some(x) => {
                if x.status().is_success() {
                    match x.bytes().await {
                        Ok(x) => {
                            self.font = match Font::new(&x) {
                                Ok(x) => {Some(x)},
                                Err(e) => {log::info!("error: {}", e); None},
                            };
                        }
                        Err(x) => {
                            log::info!("error: {}", x)
                        }
                    };
                }
            }
            None => {
                log::info!("server didn't respond")
            }
        }
    }
    async fn load_var(&self, url: &str) -> Option<Response> {
        let client = Client::new();
        log::info!("working");
        let response = match client
            .get(self.site_url.clone() + url)
            .send()
            .await
        {
            Ok(x) => x,
            Err(e) => {
                log::info!("error: {}", e);
                return None;
            }
        };
        return Some(response);
    }
}

struct MyWindowHandler {
    state: GameState,
    pressed_down_keys: HashMap<VirtualKeyCode, u64>,
    auto_repeating_keys: HashMap<VirtualKeyCode, u64>,
    assets: Assets,
}

impl MyWindowHandler {
    pub fn new() -> MyWindowHandler {
        MyWindowHandler {
            state: GameState::LoadingAssets,
            pressed_down_keys: HashMap::new(),
            auto_repeating_keys: HashMap::new(),
            assets: Assets::new(),
        }
    }
}

impl WindowHandler for MyWindowHandler {
    fn on_start(
        &mut self,
        helper: &mut WindowHelper<()>,
        info: speedy2d::window::WindowStartupInfo,
    ) {
        // Create a Tokio runtime
        
    }
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        match self.state {
            GameState::Gaming(ref mut game_handler) => {
                // log::info!("version 75");
                //////////////////////////////////////////////// HANDLE INPUTS
                let current_time = get_current_time();
                game_handler.handle_inputs(
                    &current_time,
                    &mut self.pressed_down_keys,
                    &mut self.auto_repeating_keys,
                );

                ///////////////////////////////////////////////// HANDLE GAME LOGIC
                if current_time - game_handler.last_update_time > 500 {
                    // game_handler.game.print_grid();
                    game_handler.game.game_loop(game_handler.is_time_to_freeze);
                    game_handler.last_update_time = current_time;
                }
                if current_time - game_handler.last_fall_time > game_handler.gravity {
                    game_handler.last_fall_time = current_time;
                    game_handler.block_offset += game_handler.block_offset_dy;
                    if game_handler.block_offset >= 20.0 {
                        // diameter of block
                        game_handler.game.move_c_buyo_down();
                        game_handler.block_offset = 0.0;
                    }
                    // println!("{}", game_handler.fps);
                    // game_handler.fps = 0;
                }
                // game_handler.fps += 1;

                if game_handler.game.is_on_ground() {
                    game_handler.block_offset = 0.0;
                    match game_handler.timestamp_when_on_ground {
                        Some(timestamp) => {
                            if current_time - timestamp > game_handler.freeze_time {
                                game_handler.timestamp_when_on_ground = None;
                                game_handler.is_time_to_freeze = true;
                            }
                        }
                        None => {
                            game_handler.timestamp_when_on_ground = Some(get_current_time());
                        }
                    }
                } else {
                    game_handler.is_time_to_freeze = false;
                    game_handler.timestamp_when_on_ground = None;
                }

                /////////////////////////////////////////// HANDLE DRAWING THE GAME
                graphics.clear_screen(Color::WHITE);
                // graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);
                for (v, c) in game_handler.game.get_board() {
                    graphics.draw_circle(
                        (v.x as f32 * 20.0 + 20.0, v.y as f32 * 20.0 + 20.0),
                        10.0,
                        btype_to_color(c),
                    );
                }
                for (v, c) in game_handler.game.get_controlled_block() {
                    graphics.draw_circle(
                        (
                            v.x as f32 * 20.0 + 20.0,
                            v.y as f32 * 20.0 + 20.0 + game_handler.block_offset,
                        ),
                        10.0,
                        btype_to_color(c),
                    );
                }
                // next queue
                let (a, b) = game_handler.game.next_buyo();
                graphics.draw_circle((200.0, 90.0), 10.0, btype_to_color(a));
                graphics.draw_circle((200.0, 110.0), 10.0, btype_to_color(b));

                let score = self.assets.font.as_ref().unwrap().layout_text(
                    &format!("{}", game_handler.game.total_score()),
                    50.0,
                    TextOptions::new(),
                );
                let chainscore = self.assets.font.as_ref().unwrap().layout_text(
                    &format!("{}", game_handler.game.score()),
                    50.0,
                    TextOptions::new(),
                );
                graphics.draw_text((200.0, 130.0), Color::BLACK, &score);
                graphics.draw_text((200.0, 170.0), Color::BLACK, &chainscore);
                // log::info!("{}", game_handler.game.score());
                helper.request_redraw();
                // log::info!("{}", get_current_time());
            }
            GameState::Menu => todo!(),
            GameState::LoadingAssets => {
                if self.assets.font.is_some() {
                    log::info!("gaming");
                    self.state = GameState::Gaming(GameHandler::new(6, 12));
                    helper.request_redraw();
                }
                log::info!("bruh");
                
            }
        }
    }
    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper<()>,
        virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
        _scancode: speedy2d::window::KeyScancode,
    ) {
        // log::info!("{:?}", virtual_key_code);
        match virtual_key_code {
            Some(x) => {
                if !self.pressed_down_keys.contains_key(&x) {
                    // if the key was just pressed, add it to the pressed down keys with a timestamp of when it was pressed
                    self.pressed_down_keys.insert(x, get_current_time());
                }
            }
            None => {}
        };
    }
    fn on_key_up(
        &mut self,
        _helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: speedy2d::window::KeyScancode,
    ) {
        match virtual_key_code {
            Some(x) => {
                self.pressed_down_keys.remove(&x);
                self.auto_repeating_keys.remove(&x);
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

pub fn key_just_pressed(current: &u64, time: &u64) -> bool {
    return (current - time) <= 3;
}

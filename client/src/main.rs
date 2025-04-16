use blockstackers_core::{
    blockstacker::BlockStacker,
    buyo_game::{BType, BuyoBuyo},
    randomizer::Randomizer,
};
use futures::StreamExt;
use jstime::get_current_time;
// use reqwest::{Client, Response};
// use reqwest_websocket::{RequestBuilderExt, WebSocket};
use speedy2d::color::Color;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::window::{VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, WebCanvas};
use std::{collections::HashMap, ops::{Deref, DerefMut}, rc::Rc};
use std::marker::PhantomData;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    js_sys, wasm_bindgen::{JsCast, JsValue}, Request, RequestInit, RequestMode, Response
};

fn main() {
    // let window = Window::new_centered("Title", (640, 480)).unwrap();
    // window.run_loop(MyWindowHandler::new());
    // wasm_logger::init(wasm_logger::Config::default());
    // std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // log::info!("Speedy2D WebGL sample");

    wasm_logger::init(wasm_logger::Config::default());
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    log::info!("version 79");
    wasm_bindgen_futures::spawn_local(async move {
        // let mut nc = NetworkConnection::new().await.unwrap();
        let mut state = GameState::LoadingAssets;
        let mut window = MyWindowHandler::new(NetworkConnection {}, state);
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

mod jstime;

struct NetworkConnection {
    // ws: WebSocket,
    // recieved_messages: Vec<(String, u64)>,
}

impl NetworkConnection {
    
}

enum GameState {
    Gaming(GameHandler<BuyoBuyo, BType>),
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
    pub fn draw(
        &mut self,
        graphics: &mut Graphics2D,
        assets: &Assets,
        pressed_down_keys: &mut HashMap<VirtualKeyCode, u64>,
        auto_repeating_keys: &mut HashMap<VirtualKeyCode, u64>,
    ) {
        //////////////////////////////////////////////// HANDLE INPUTS
        let current_time = get_current_time();
        self.handle_inputs(&current_time, pressed_down_keys, auto_repeating_keys);

        ///////////////////////////////////////////////// HANDLE GAME LOGIC
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

        /////////////////////////////////////////// HANDLE DRAWING THE GAME
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

struct Assets {
    font: Option<Font>,
    site_url: String,
}
impl Assets {
    pub fn new() -> Assets {
        let win = web_sys::window().unwrap();
        let url = win.document().unwrap().url().unwrap();
        Assets {
            font: None,
            site_url: url,
        }
    }
    pub async fn load(&mut self) {
        let resp = self.load_var("/assets/fonts/arial.ttf").await;
        match resp {
            Some(x) => {
                self.font = match Font::new(&x) {
                    Ok(x) => Some(x),
                    Err(e) => {
                        log::info!("error: {}", e);
                        None
                    }
                };
            }
            None => {
                log::info!("server didn't respond")
            }
        }
    }
    async fn load_var(&self, url: &str) -> Option<Vec<u8>> {
        // Create a new RequestInit object
        let mut opts = RequestInit::new();
        opts.set_method("GET");

        // Create a new Request object
        let request = Request::new_with_str_and_init(url, &opts).unwrap();

        // Use the fetch API to make the request
        let window = web_sys::window().unwrap();
        let response_promise = window.fetch_with_request(&request);

        // Await the response using JsFuture
        let response_value = JsFuture::from(response_promise).await.unwrap();
        let response: Response = response_value.dyn_into().unwrap();

        // Check if the response is OK
        if response.ok() {
            // Get the response body as a Uint8Array
            let promise = response.array_buffer();
            let array_buffer = JsFuture::from(promise.unwrap()).await.unwrap();
            let bytes = js_sys::Uint8Array::new(&array_buffer);
            let mut vec = vec![0; bytes.length() as usize];
            bytes.copy_to(&mut vec[..]);
            Some(vec)
        } else {
            None
        }
    }
}

struct MyWindowHandler {
    state: GameState,
    pressed_down_keys: HashMap<VirtualKeyCode, u64>,
    auto_repeating_keys: HashMap<VirtualKeyCode, u64>,
    assets: Assets,
    net: NetworkConnection,
}

impl MyWindowHandler {
    pub fn new(net: NetworkConnection, state: GameState) -> MyWindowHandler {
        MyWindowHandler {
            state: state,
            pressed_down_keys: HashMap::new(),
            auto_repeating_keys: HashMap::new(),
            assets: Assets::new(),
            net,
        }
    }
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        match self.state {
            GameState::Gaming(ref mut game_handler) => {
                game_handler.draw(
                    graphics,
                    &self.assets,
                    &mut self.pressed_down_keys,
                    &mut self.auto_repeating_keys,
                );
                helper.request_redraw();
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

pub fn key_just_pressed(current: &u64, time: &u64) -> bool {
    return (current - time) <= 3;
}

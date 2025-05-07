use assets::Assets;
use blockstackers_core::{
    blockstacker::BlockStacker,
    buyo_game::{BType, BuyoBuyo},
    randomizer::Randomizer,
};
use enums::GameState;
use futures::{lock::Mutex, StreamExt};
use gamehandler::GameHandler;
use jstime::get_current_time;
// use reqwest::{Client, Response};
// use reqwest_websocket::{RequestBuilderExt, WebSocket};
use speedy2d::color::Color;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::window::{VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, WebCanvas};
use std::{collections::HashMap, ops::{Deref, DerefMut}, rc::Rc, sync::RwLock};
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
        let mut window = MyWindowHandler::new(NetworkConnection {}, &state);
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
mod enums;
mod gamehandler;
mod assets;
struct NetworkConnection {
    // ws: WebSocket,
    // recieved_messages: Vec<(String, u64)>,
}

impl NetworkConnection {
    fn new()
}

struct MyWindowHandler {
    state: Mutex<GameState>,
    pressed_down_keys: HashMap<VirtualKeyCode, u64>,
    auto_repeating_keys: HashMap<VirtualKeyCode, u64>,
    assets: Assets,
    net: NetworkConnection,
}

impl MyWindowHandler {
    pub fn new(net: NetworkConnection, state: Mutex<GameState>) -> MyWindowHandler {
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
        match *(match self.state.try_lock() {
            Some(x) => {x},
            None => {helper.request_redraw(); return;},
        }) {
            GameState::Gaming(ref game_handler) => {
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
                    self.state = Mutex::new(GameState::Gaming(GameHandler::new(6, 12)));
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

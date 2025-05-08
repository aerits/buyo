use assets::Assets;
use async_std::task::sleep;
use blockstackers_core::blockstacker::BlockStacker;
use blockstackers_core::buyo_game::{BType, BuyoBuyo};
use enums::GameState;
use futures::lock::{Mutex, MutexGuard};
use futures::FutureExt;
use gamehandler::GameHandler;
use jstime::get_current_time;
use network::NetworkConnection;
use speedy2d::window::{VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, WebCanvas};
use std::fmt::Display;
use std::{collections::HashMap, sync::Arc, time::Duration};

fn main() {
    // let window = Window::new_centered("Title", (640, 480)).unwrap();
    // window.run_loop(MyWindowHandler::new());
    // wasm_logger::init(wasm_logger::Config::default());
    // std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // log::info!("Speedy2D WebGL sample");

    wasm_logger::init(wasm_logger::Config::default());
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    log::info!("version 90");
    wasm_bindgen_futures::spawn_local(async move {
        let mut state = Arc::new(Mutex::new(GameState::LoadingAssets));
        let mut window = MyWindowHandler::new(state.clone());
        let mut last_check = 0;
        let url = window.assets.site_url.clone();
        let mut net = Arc::new(Mutex::new(NetworkConnection::new(&url).await));
        while window.assets.font.is_none() {
            if get_current_time() - last_check > 1000 {
                window.assets.load().await;
                last_check = get_current_time();
            }
        }
        *state.lock().await = GameState::Gaming(GameHandler::new(6, 12));

        // let network_loop =
        let draw_loop = async {
            WebCanvas::new_for_id_with_user_events("my_canvas", window).unwrap();
        };
        futures::join!(
            game_loop(net.clone(), state.clone()),
            draw_loop,
            network_loop(net.clone(), state.clone())
        );
    });
}

mod assets;
mod enums;
mod gamehandler;
mod jstime;
mod network;

async fn network_loop(net: Arc<Mutex<NetworkConnection>>, state: Arc<Mutex<GameState>>) {
    loop {
        loop {
            let a = match net.try_lock() {
                None => None,
                Some(mut x) => {
                    futures::select! {
                        a = x.next().fuse() => {a}
                        _ = sleep(Duration::from_millis(5)).fuse() => {None}
                    }
                }
            };
            match a {
                None => {
                    break;
                }
                Some(x) => {
                    let addr = x.split(":: ").next().unwrap().to_string();
                    log::info!("addr: {}", addr);
                    let game = network::deserialize_game(&x);
                    state.lock().await.add_player(addr, game);
                }
            };
        }

        // log::info!("{}", a.unwrap_or("nothing".to_owned()));
        sleep(Duration::from_millis(100)).await;
    }
}

async fn game_loop(net: Arc<Mutex<NetworkConnection>>, state: Arc<Mutex<GameState>>) {
    let mut failed_to_send = false;
    let mut last_send = 0;
    let mut key_pressed = false;
    loop {
        log::info!("updating");
        let mut changed = 0;
        match *state.lock().await {
            GameState::Gaming(ref mut game_handler) => {
                changed = game_handler.update(get_current_time());
                key_pressed = game_handler.key_pressed;
                if key_pressed {game_handler.key_pressed = false;}
            }
            GameState::Menu => (),
            GameState::LoadingAssets => (),
        }
        if changed == 1 || failed_to_send || get_current_time() - last_send > 1000 || key_pressed {
            key_pressed = false;
            match net.try_lock() {
                None => {
                    log::info!("couldn't send");
                    failed_to_send = true
                }
                Some(mut x) => {
                    last_send = get_current_time();
                    failed_to_send = false;
                    let game = &*state.lock().await;
                    x.send(&network::serialize_game::<BuyoBuyo, BType>(game))
                        .await;
                }
            }
        }
        sleep(Duration::from_millis(50)).await;
    }
}

struct MyWindowHandler {
    state: Arc<Mutex<GameState>>,
    pressed_down_keys: HashMap<VirtualKeyCode, u64>,
    auto_repeating_keys: HashMap<VirtualKeyCode, u64>,
    assets: Assets,
}

impl MyWindowHandler {
    pub fn new(state: Arc<Mutex<GameState>>) -> MyWindowHandler {
        MyWindowHandler {
            state,
            pressed_down_keys: HashMap::new(),
            auto_repeating_keys: HashMap::new(),
            assets: Assets::new(),
        }
    }
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        match *(match self.state.try_lock() {
            Some(mut x) => {
                x.handle_inputs(
                    get_current_time(),
                    &mut self.pressed_down_keys,
                    &mut self.auto_repeating_keys,
                );
                x
            }
            None => {
                helper.request_redraw();
                return;
            }
        }) {
            GameState::Gaming(ref game_handler) => {
                game_handler.draw(graphics, &self.assets);
                helper.request_redraw();
            }
            GameState::Menu => todo!(),
            GameState::LoadingAssets => {}
        }
    }
    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
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
    (current - time) <= 3
}

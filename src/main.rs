use blockstacker::BlockStacker;
use buyo_game::{BType, Game};
use jstime::get_current_time;
use randomizer::Randomizer;
use speedy2d::color::Color;
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

    WebCanvas::new_for_id_with_user_events("my_canvas", MyWindowHandler::new()).unwrap();
}

mod blockstacker;
mod buyo_game;
mod jstime;
mod randomizer;
mod vectors;

enum GameState {
    Gaming(GameHandler<Game, BType>),
    Menu,
}

struct GameHandler<T: BlockStacker<F>, F> {
    game: T,
    phantom: std::marker::PhantomData<F>,
    last_update_time: u64,
    is_time_to_freeze: bool, // set by game,
    freeze_time: u64, // set by game
    timestamp_when_on_ground: Option<u64>, // set by game
    das: u64,             // set by user
    arr: u64,             // set by user
    last_fall_time: u64,
    gravity: u64, // set by game
    fps: i32,
}
impl<T: BlockStacker<F>, F> GameHandler<T, F> {
    pub fn new() -> GameHandler<T, F> {
        GameHandler {
            game: T::new(6, 12, Randomizer::new(4)),
            phantom: PhantomData,
            last_update_time: get_current_time(),
            is_time_to_freeze: false,
            freeze_time: 5000,
            timestamp_when_on_ground: None, // this says if its on the ground when did it land
            das: 133, // 120 ms
            arr: 20,  // 20 ms
            last_fall_time: get_current_time(),
            gravity: 1000,
            fps: 0,
        }
    }
    pub fn handle_inputs(&mut self, current_time: &u64, pressed_down_keys: &HashMap<VirtualKeyCode, u64>) {
        for (key, time) in pressed_down_keys {
            // log::info!("{:?}, {}", key, time);
            match key {
                VirtualKeyCode::Left => {
                    if current_time - time > self.das
                        || key_just_pressed(&current_time, time)
                    {
                        if key_just_pressed(&current_time, time) || (current_time - time) % self.arr < 100 {
                            self.game.input_left()
                        }
                        // game_handler.game.input_left()
                    }
                }
                VirtualKeyCode::Right => {
                    if current_time - time > self.das
                        || key_just_pressed(&current_time, time)
                    {
                        if key_just_pressed(&current_time, time) ||  (current_time - time) % self.arr < 100 {
                            self.game.input_right()
                        }
                        // game_handler.game.input_right()
                    }
                }
                VirtualKeyCode::Z => {
                    if key_just_pressed(&current_time, time) {
                        self.game.input_rotation_right()
                    }
                }
                VirtualKeyCode::X => {
                    if key_just_pressed(&current_time, time) {
                        self.game.input_rotation_left()
                    }
                }
                VirtualKeyCode::Space => {
                    if key_just_pressed(&current_time, time) {
                        self.game.hard_drop();
                        self.last_update_time = 0; // unix epoch time
                    }
                }
                _ => (),
            }
        }
    }
}

struct MyWindowHandler {
    state: GameState,
    pressed_down_keys: HashMap<VirtualKeyCode, u64>,
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
    // fn on_start(
    //         &mut self,
    //         helper: &mut WindowHelper<()>,
    //         info: speedy2d::window::WindowStartupInfo
    //     ) {
    //     let mut last_update = get_current_time();
    //     loop {
    //         if get_current_time() - last_update > 100 {
    //             helper.request_redraw();
    //             last_update = get_current_time();
    //         }
    //     }
    // }
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        match self.state {
            GameState::Gaming(ref mut game_handler) => {
                //////////////////////////////////////////////// HANDLE INPUTS
                let current_time = get_current_time();
                game_handler.handle_inputs(&current_time, &self.pressed_down_keys);

                ///////////////////////////////////////////////// HANDLE GAME LOGIC
                if current_time - game_handler.last_update_time > 500 {
                    // game_handler.game.print_grid();
                    game_handler.game.game_loop(game_handler.is_time_to_freeze);
                    game_handler.last_update_time = current_time;
                    game_handler.is_time_to_freeze = false;
                }
                if current_time - game_handler.last_fall_time > game_handler.gravity {
                    game_handler.last_fall_time = current_time;
                    game_handler.game.move_c_buyo_down();
                    println!("{}", game_handler.fps);
                    game_handler.fps = 0;
                }
                game_handler.fps += 1;

                if game_handler.game.is_on_ground() {
                    match game_handler.timestamp_when_on_ground {
                        Some(timestamp) => {
                            if current_time - timestamp > game_handler.freeze_time {
                                game_handler.timestamp_when_on_ground = None;
                                game_handler.is_time_to_freeze = true;
                            }
                        },
                        None => {game_handler.timestamp_when_on_ground = Some(current_time);},
                    }
                }

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
                // log::info!("{}", get_current_time());
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

pub fn key_just_pressed(current: &u64, time: &u64) -> bool {
    return (current - time) < 6;
}

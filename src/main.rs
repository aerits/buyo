use buyo_game::{BType, Game};
use randomizer::Randomizer;
use speedy2d::color::Color;
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{VirtualKeyCode, WindowHandler, WindowHelper};
use std::{io, time::{self, SystemTime}};

fn main() {
    let window = Window::new_centered("Title", (640, 480)).unwrap();
    window.run_loop(MyWindowHandler::new());
}

mod buyo_game;
mod randomizer;
mod vectors;

struct MyWindowHandler {
    game: Game,
    last_update_time: SystemTime,
    time_to_freeze: bool,
}

impl MyWindowHandler {
    pub fn new() -> MyWindowHandler {
        MyWindowHandler { game: Game::new(6, 12, Randomizer::new(4)), last_update_time: SystemTime::now(), time_to_freeze: false }
    }
}

impl WindowHandler for MyWindowHandler
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        if SystemTime::now().duration_since(self.last_update_time).unwrap().as_secs() > 1 {
            // self.game.print_grid();
            let on_floor = !self.game.game_loop(self.time_to_freeze);
            self.last_update_time = SystemTime::now();
            if on_floor {
                self.time_to_freeze = true;
            } else {
                self.time_to_freeze = false;
            }
        }
        graphics.clear_screen(Color::WHITE);
        // graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);
        for (v, c) in self.game.board() {
            graphics.draw_circle((v.x as f32 * 20.0 + 20.0, v.y as f32 * 20.0 + 20.0), 10.0, btype_to_color(c));
        }
        // next queue
        let (a, b) = self.game.next_buyo();
        graphics.draw_circle((500.0, 90.0), 10.0, btype_to_color(a));
        graphics.draw_circle((500.0, 110.0), 10.0, btype_to_color(b));
        helper.request_redraw();
    }
    fn on_key_down(
            &mut self,
            helper: &mut WindowHelper<()>,
            virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
            scancode: speedy2d::window::KeyScancode
        ) {
        match virtual_key_code {
            Some(x) => {
                match x {
                    VirtualKeyCode::Left => self.game.input_left(),
                    VirtualKeyCode::Right => self.game.input_right(),
                    VirtualKeyCode::Z => self.game.input_rotation_right(),
                    VirtualKeyCode::X => self.game.input_rotation_left(),
                    VirtualKeyCode::Space => self.game.hard_drop(),
                    _ => (),
                }
            },
            None => (),
        }
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
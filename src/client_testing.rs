use buyo_game::Game;
use randomizer::Randomizer;
use std::{io, time::{self, SystemTime}};

mod buyo_game;
mod vectors;
mod randomizer;

fn client_testing() {
    let mut a = Game::new(6, 12, Randomizer::new(4));
    let mut now = SystemTime::now();
    let mut last_update = now;
    let mut successful_update = false;
    let mut time_on_floor = 0;
    let mut freeze = false;

    loop {
        now = SystemTime::now();

        // debugging stuff
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        match input.trim() {
            "l" => a.input_left(),
            "r" => a.input_right(),
            "z" => a.input_rotation_left(),
            "x" => a.input_rotation_right(),
            "h" => a.hard_drop(),
            _ => (),
        }
        
        let not_on_floor = a.game_loop(
            now.duration_since(last_update)
                .unwrap()
                .as_secs()
                .try_into()
                .unwrap(),
            // 1,
            freeze,
            &mut successful_update,
        );

        if successful_update {
            println!("{:?}", now);
            a.print_grid();
            last_update = now;
            successful_update = false;
            freeze = false;
            if !not_on_floor {
                time_on_floor += 1;
            }
            if time_on_floor == 2 {
                time_on_floor = 0;
                freeze = true;
            }
        }
    }
}

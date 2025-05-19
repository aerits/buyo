use core::time;
use std::collections::HashMap;

use enums::{LoopState, Mino, Rotation, Shapes};
use tables::Tables;

use crate::{blockstacker::{BlockStacker, Tuning}, randomizer::Randomizer, vectors::BVec, Sprite};

#[cfg(test)]
mod tests;

mod enums;
mod tables;

#[derive(Clone)]
struct C_Mino {
    vec: Vec<BVec>,
    color: Sprite,
    shape: Shapes,
    rot: Rotation,
}
impl C_Mino {
    pub fn new(vec: Vec<BVec>, color: Sprite, shape: Shapes) -> C_Mino {
        C_Mino {
            vec,
            color,
            shape,
            rot: Rotation::Up,
        }
    }
}

pub struct Tet {
    minos: HashMap<BVec, Sprite>,
    randomizer: Randomizer,
    controlled_mino: Option<C_Mino>,
    tables: Tables,
    loop_state: LoopState,
    tuning: Tuning,
}
impl Tet {
    fn spawn_c_mino(&mut self, shape: Shapes) {
        let o = vec![
            BVec::new(5, 0),
            BVec::new(6, 0),
            BVec::new(6, 1),
            BVec::new(5, 1),
        ];

        let i = vec![
            BVec::new(4, 1),
            BVec::new(5, 1),
            BVec::new(6, 1),
            BVec::new(7, 1),
        ];

        let l = vec![
            BVec::new(4, 1),
            BVec::new(4, 0),
            BVec::new(5, 1),
            BVec::new(6, 1),
        ];

        let j = vec![
            BVec::new(4, 1),
            BVec::new(5, 1),
            BVec::new(6, 1),
            BVec::new(6, 0),
        ];

        let s = vec![
            BVec::new(4, 1),
            BVec::new(5, 1),
            BVec::new(5, 0),
            BVec::new(6, 0),
        ];

        let z = vec![
            BVec::new(4, 0),
            BVec::new(5, 0),
            BVec::new(5, 1),
            BVec::new(6, 1),
        ];

        let t = vec![
            BVec::new(4, 1),
            BVec::new(5, 1),
            BVec::new(5, 0),
            BVec::new(6, 1),
        ];

        match shape {
            Shapes::O => self.controlled_mino = Some(C_Mino::new(o, Sprite::TetO, shape)),
            Shapes::L => self.controlled_mino = Some(C_Mino::new(l, Sprite::TetL, shape)),
            Shapes::J => self.controlled_mino = Some(C_Mino::new(j, Sprite::TetJ, shape)),
            Shapes::T => self.controlled_mino = Some(C_Mino::new(t, Sprite::TetT, shape)),
            Shapes::I => self.controlled_mino = Some(C_Mino::new(i, Sprite::TetI, shape)),
            Shapes::Z => self.controlled_mino = Some(C_Mino::new(z, Sprite::TetZ, shape)),
            Shapes::S => self.controlled_mino = Some(C_Mino::new(s, Sprite::TetS, shape)),
        }
    }
    fn rotate_c_mino(&mut self, rots: i32) {
        let c_mino = match &mut self.controlled_mino {
            Some(x) => x,
            None => return,
        };
        match c_mino.color {
            Sprite::TetO => return, // cannot rotate the O mino
            _ => (),
        }
        let temp = c_mino;
        let mut origin = temp.vec[0].clone();
        origin.mult_s(-1);
        for v in &mut temp.vec {
            v.add_v(origin);
        }
        for v in &mut temp.vec {
            for _ in 0..rots {
                let x_old = v.x;
                v.x = v.y;
                v.y = -x_old;
            }
        }
        // SRS system
        let rotation_final = match (temp.rot as i32 + rots) % 4 {
            0 => Rotation::Up,
            1 => Rotation::Right,
            2 => Rotation::Down,
            3 => Rotation::Left,
            _ => panic!("This is an impossible state"),
        };
        let mut kicks = self
            .tables
            .get_kicks(&temp.rot, &rotation_final, &temp.shape);
        // reverse y coordinate because the game's systems are 0 y is the top of the screeen
        for kick in &mut kicks {
            kick.y = kick.y * -1;
        }
        // test each kick until one works
        for kick in &kicks {
            let mut pos = temp.vec.clone();
            let mut collided = false;
            for v in &mut pos {
                v.add_v(*kick);
                if self.minos.contains_key(&v) {
                    collided = true;
                    break;
                }
            }
            if !collided {
                temp.vec = pos;
                return;
            }
        }
    }
    fn move_c_mino_if_no_collision(&mut self, v: BVec) -> bool {
        let mino = match &mut self.controlled_mino {
            Some(x) => x,
            None => return false,
        };
        let pos = mino.vec.clone();
        let new_pos: Vec<BVec> = pos.iter().map(|x| x + &v).collect();
        for vec in &new_pos {
            if self.minos.contains_key(vec) {
                return false;
            }
        }
        mino.vec = new_pos;
        return true;
    }
    fn print_board(&self) -> String {
        let max_y = self.get_board().iter().fold(
            0,
            |total, cur| if cur.0.y > total { cur.0.y } else { total },
        );
        let max_x = self.get_board().iter().fold(
            0,
            |total, cur| if cur.0.x > total { cur.0.x } else { total },
        );
        let mut grid: Vec<Vec<String>> = Vec::with_capacity(max_y as usize);
        for _i in 0..=max_y {
            grid.push(Vec::with_capacity(max_x as usize));
            for _j in 0..=max_x {
                grid.last_mut().unwrap().push(" ".to_owned());
            }
        }
        for (v, b) in self.get_board().iter() {
            let s = match b {
                Sprite::Wall => "#",
                Sprite::TetT => "T",
                Sprite::TetI => "I",
                Sprite::TetO => "O",
                Sprite::TetJ => "J",
                Sprite::TetL => "L",
                Sprite::TetS => "S",
                Sprite::TetZ => "Z",
                _ => panic!()
            }
            .to_owned();
            grid[v.y as usize][v.x as usize] = s;
        }
        let mut out = String::new();
        for row in grid.iter() {
            for col in row {
                print!("{}", col);
                out = out + col;
            }
            out = out + "\n";
            println!();
        }
        out
    }
    /// returns the rows to be cleared
    /// let the caller get rid of the lines
    fn clear_lines(&mut self) -> Vec<usize> {
        todo!()
    }
}

impl BlockStacker for Tet {
    fn new(width: i32, height: i32, randomizer: crate::randomizer::Randomizer, tuning: Tuning) -> Self {
        let mut minos: HashMap<BVec, Sprite> = HashMap::new();
        for i in 0..=width + 1 {
            for j in 0..=height + 1 {
                if i % (width + 1) == 0 || j == height + 1 {
                    minos.insert(BVec::new(i, j), Sprite::Wall);
                };
            }
        }

        Tet {
            minos,
            randomizer: randomizer,
            controlled_mino: None,
            tables: Tables::new(),
            loop_state: LoopState::Spawning,
            tuning,
        }
    }

    fn get_board(&self) -> std::collections::HashMap<crate::vectors::BVec, Sprite> {
        let vecs = match self.controlled_mino.clone() {
            Some(x) => x.vec,
            None => Vec::new(),
        };
        let mut a = self.minos.clone();
        for i in vecs {
            a.insert(i, self.controlled_mino.as_ref().unwrap().color);
        }
        return a;
    }

    fn next_queue(&self) -> std::collections::HashMap<crate::vectors::BVec, Sprite> {
        todo!()
    }

    // fn convert_t_to_speedy2d_color(&self, t: &Mino) -> speedy2d::color::Color {
    //     todo!()
    // }

    fn get_controlled_block(&self) -> Vec<(f32, f32, Sprite)> {
        // match &self.controlled_mino {
        //     None => {HashMap::new()}
        //     Some(c) => {c.vec.iter().fold(HashMap::new(), |mut acc, x| {
        //         acc.insert(*x, c.color);
        //         acc
        //     })}
        // }
        todo!()
    }

    fn input_left(&mut self) -> bool {
        self.move_c_mino_if_no_collision(BVec::new(-1, 0))
    }

    fn input_right(&mut self) -> bool {
        self.move_c_mino_if_no_collision(BVec::new(1, 0))
    }

    fn input_rotation_right(&mut self) {
        self.rotate_c_mino(1);
    }

    fn input_rotation_left(&mut self) {
        self.rotate_c_mino(3);
    }

    fn input_180_rot(&mut self) {
        self.rotate_c_mino(2);
    }

    fn hard_drop(&mut self) {
        while self.move_c_buyo_down() {}
    }

    fn move_c_buyo_down(&mut self) -> bool {
        self.move_c_mino_if_no_collision(BVec::new(0, 1))
    }

    fn is_on_ground(&self) -> bool {
        let vectors = match &self.controlled_mino {
            Some(x) => &x.vec,
            None => return false,
        };
        let mut output = true;
        for v in vectors {
            let u = v + &BVec::new(0, 1);
            if self.minos.contains_key(&u) {
                output = true;
            }
        }
        return output;
    }

    fn score(&self) -> i32 {
        todo!()
    }

    fn total_score(&self) -> i32 {
        todo!()
    }

    fn game_loop(&mut self, last_update: u64, current_time: u64) -> bool {
        let delta_time = current_time - last_update;
        match self.loop_state {
            LoopState::Falling => {
                if delta_time < self.tuning.fall_speed {
                    return false;
                }
                let on_ground = !self.move_c_buyo_down();
                if on_ground {
                    self.loop_state = LoopState::LockingOrClearing;
                }
            },
            LoopState::Spawning => {
                if delta_time < self.tuning.spawn_delay {
                    return false;
                }
                let shape = match self.randomizer.next() {
                    0 => Shapes::I,
                    1 => Shapes::J,
                    2 => Shapes::L,
                    3 => Shapes::O,
                    4 => Shapes::S,
                    5 => Shapes::T,
                    6 => Shapes::Z,
                    _ => panic!()
                };
                self.spawn_c_mino(shape);
                self.loop_state = LoopState::Falling;
            },
            LoopState::LockingOrClearing => {
                if delta_time < self.tuning.lock_delay {
                    return false;
                }
            },
            LoopState::OnFloor(time) => {

            }
        }
        true
    }
}

use std::collections::HashMap;

use enums::{Mino, Rotation, Shapes};
use tables::Tables;

use crate::{blockstacker::BlockStacker, randomizer::Randomizer, vectors::BVec};

#[cfg(test)]
mod tests;

mod enums;
mod tables;

struct C_Mino {
    vec: Vec<BVec>,
    color: Mino,
    shape: Shapes,
    rot: Rotation,
}
impl C_Mino {
    pub fn new(vec: Vec<BVec>, color: Mino, shape: Shapes) -> C_Mino {
        C_Mino {
            vec,
            color,
            shape,
            rot: Rotation::Up,
        }
    }
}

pub struct Tet {
    minos: HashMap<BVec, Mino>,
    randomizer: Randomizer,
    controlled_mino: Option<C_Mino>,
    tables: Tables,
}
impl Tet {
    fn spawn_c_mino(&mut self, shape: Shapes) {
        let o = vec![
            BVec::new(0, 0),
            BVec::new(1, 0),
            BVec::new(1, -1),
            BVec::new(0, -1),
        ];

        match shape {
            Shapes::O => self.controlled_mino = Some(C_Mino::new(o, Mino::Yellow, shape)),
            Shapes::L => todo!(),
            Shapes::J => todo!(),
            Shapes::T => todo!(),
            Shapes::I => todo!(),
            Shapes::Z => todo!(),
            Shapes::S => todo!(),
        }
    }
    fn rotate_c_mino(&mut self, rots: i32) {
        let c_mino = match &mut self.controlled_mino {
            Some(x) => x,
            None => return,
        };
        match c_mino.color {
            Mino::Yellow => return, // cannot rotate the O mino
            _ => (),
        }
        let mut temp = c_mino;
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
            _ => panic!("This is an impossible state")
        };
        let mut kicks = self.tables.get_kicks(&temp.rot, &rotation_final, &temp.shape);
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
}

impl BlockStacker<Mino> for Tet {
    fn new(width: i32, height: i32, randomizer: crate::randomizer::Randomizer) -> Self {
        let mut minos: HashMap<BVec, Mino> = HashMap::new();
        for i in 0..width + 2 {
            for j in 0..height + 2 {
                if i % width + 2 == 0 || j == height + 2 {
                    minos.insert(BVec { x: i, y: j }, Mino::Wall);
                };
            }
        }

        Tet {
            minos,
            randomizer: randomizer,
            controlled_mino: None,
            tables: Tables::new(),
        }
    }

    fn get_board(&self) -> std::collections::HashMap<crate::vectors::BVec, Mino> {
        todo!()
    }

    fn next_queue(&mut self) -> std::collections::HashMap<crate::vectors::BVec, Mino> {
        todo!()
    }

    fn convert_t_to_speedy2d_color(&self, t: Mino) -> speedy2d::color::Color {
        todo!()
    }

    fn get_controlled_block(&self) -> std::collections::HashMap<crate::vectors::BVec, Mino> {
        todo!()
    }

    fn input_left(&mut self) {
        todo!()
    }

    fn input_right(&mut self) {
        todo!()
    }

    fn input_rotation_right(&mut self) {
        todo!()
    }

    fn input_rotation_left(&mut self) {
        todo!()
    }

    fn input_180_rot(&mut self) {
        todo!()
    }

    fn hard_drop(&mut self) {
        todo!()
    }

    fn move_c_buyo_down(&mut self) -> bool {
        todo!()
    }

    fn is_on_ground(&self) -> bool {
        todo!()
    }

    fn score(&self) -> i32 {
        todo!()
    }

    fn total_score(&self) -> i32 {
        todo!()
    }

    fn game_loop(&mut self, time_to_freeze: bool) -> i32 {
        todo!()
    }
}

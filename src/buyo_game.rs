use std::collections::{HashMap, HashSet, VecDeque};

use crate::randomizer::Randomizer;
use crate::vectors::BVec;
use crate::blockstacker::BlockStacker;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum BType {
    Red,
    Blue,
    Green,
    Purple,
    Wall,
}

fn to_btype(i: i32) -> BType {
    match i {
        0 => BType::Red,
        1 => BType::Blue,
        2 => BType::Green,
        3 => BType::Purple,
        _ => panic!(),
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Buyo {
    p: BVec,
    t: BType,
}

pub struct Game {
    buyos: HashMap<BVec, BType>,
    controlled_buyo: Option<(Buyo, Buyo)>,
    randomizer: Randomizer,
}

impl BlockStacker<BType> for Game {
    // create a game board
    fn new(width: i32, height: i32, randomizer: Randomizer) -> Self {
        let mut buyos = HashMap::new();
        for x in 0..width + 2 {
            for y in 0..height + 2 {
                if x == 0 || x == width + 1 || y == 0 || y == height + 1 {
                    buyos.insert(BVec::new(x, y), BType::Wall);
                }
            }
        }
        Game {
            buyos,
            controlled_buyo: None,
            randomizer,
        }
    }
    fn board(&self) -> HashMap<BVec, BType> {
        let mut a = self.buyos.clone();
        match self.controlled_buyo {
            Some(x) => {
                a.insert(x.0.p, x.0.t);
                a.insert(x.1.p, x.1.t);
            }
            None => (),
        }
        return a;
    }
    fn input_left(&mut self) {
        self.move_c_buyo_if_no_collision(BVec { x: -1, y: 0 });
    }
    fn input_right(&mut self) {
        self.move_c_buyo_if_no_collision(BVec { x: 1, y: 0 });
    }
    fn input_rotation_right(&mut self) {
        self.rotate_c_buyo(1);
    }
    fn input_rotation_left(&mut self) {
        self.rotate_c_buyo(3);
    }
    fn input_180_rot(&mut self) {
        self.rotate_c_buyo(2);
    }
    fn hard_drop(&mut self) {
        while self.move_c_buyo_if_no_collision(BVec { x: 0, y: 1 }) {}
        self.freeze_c_buyo();
    }
    fn move_c_buyo_down(&mut self) {
        self.move_c_buyo_if_no_collision(BVec{x: 0, y: 1});
    }
    // place this in a loop that also does detection of inputs and whatnot
    // returns not on floor
    fn game_loop(
        &mut self,
        // dt: i32,
        time_to_freeze: bool,
        // successful_update: &mut bool,
    ) -> bool {
        // if dt < 1 {
        //     return true;
        // } // only update if change in time is 1
        // *successful_update = true;
        if self.controlled_buyo == None {
            let a = self.pop_buyos();
            if a {
                return true;
            }
            // no more buyos to pop
            let b1 = Buyo {
                p: BVec { x: 3, y: 2 },
                t: to_btype(self.randomizer.next()),
            };
            let b2 = Buyo {
                p: &b1.p + &BVec { x: 0, y: -1 },
                t: to_btype(self.randomizer.next()),
            };
            self.spawn_c_buyo((b1, b2));
            return true;
        }

        // let a = self.move_c_buyo_if_no_collision(BVec { x: 0, y: 1 }); // gravity on buyo
                                                                       // interpolate this on graphics
        if time_to_freeze {
            self.freeze_c_buyo();
            return true;
        }
        return true;
    }
}

impl Game {
    // set controlled buyo to the inputted buyo
    // if there already is a buyo return false
    fn spawn_c_buyo(&mut self, b: (Buyo, Buyo)) -> bool {
        if self.controlled_buyo.is_some() {
            return false;
        }
        self.controlled_buyo = Some(b);
        return true;
    }
    // rotate clockwise by r
    // if it can't be rotated, return false
    fn rotate_c_buyo(&mut self, r: i32) -> bool {
        let b = match &mut self.controlled_buyo {
            Some(x) => x,
            None => panic!(),
        };
        let v1_old = b.0.p.clone();
        let v2_old = b.1.p.clone();
        b.0.p.mult_s(-1);
        b.1.p.add_v(b.0.p); // set b.0 to be the origin
        b.0.p.clear();
        // mult by matrix [[cos -90 sin -90] [-sin -90 cos -90]] r amount of times
        for _ in 0..r {
            let x_old = b.1.p.x;
            b.1.p.x = b.1.p.y;
            b.1.p.y = -x_old;
        }

        // set pos of second vector to read later
        let pos = match b.1.p.x {
            1 => 0,  // right
            -1 => 1, // left
            0 => 2,  // down
            _ => panic!(),
        };

        // move vectors back to old positions
        b.0.p = v1_old;
        b.1.p.add_v(v1_old);

        // check if theres a vector already there on the grid
        if self.buyos.contains_key(&b.1.p) {
            // three cases, left right or down
            // if down: move the buyo up
            // if left: move right
            // if cant move right: don't do anything
            // if right: move left
            // if can't move left: don't do anything
            let should_undo: bool;
            match pos {
                2 => {
                    b.0.p.add_i(0, -1);
                    b.1.p.add_i(0, -1);
                    should_undo = false;
                }
                1 => {
                    b.0.p.add_i(1, 0); // move right
                    should_undo = self.buyos.contains_key(&b.0.p); // undo the rotation
                    b.1.p.add_i(1, 0);
                }
                0 => {
                    b.0.p.add_i(-1, 0);
                    should_undo = self.buyos.contains_key(&b.0.p); // undo the rotation
                    b.1.p.add_i(-1, 0);
                }
                _ => panic!(),
            }
            if should_undo {
                b.0.p = v1_old;
                b.1.p = v2_old;
            }
            return should_undo;
        }
        return true;
    }
    // place the controlled buyo into buyos
    // if controlled buyo is none, return false
    fn freeze_c_buyo(&mut self) -> bool {
        let x = match &self.controlled_buyo {
            Some(x) => x,
            None => return false,
        };
        self.buyos.insert(x.0.p, x.0.t);
        self.buyos.insert(x.1.p, x.1.t);
        self.controlled_buyo = None;
        true
    }
    // return false if there is no c buyo
    // return false if it collided and couldn't move
    fn move_c_buyo_if_no_collision(&mut self, v: BVec) -> bool {
        let x = match &mut self.controlled_buyo {
            Some(x) => x,
            None => return false,
        };
        x.0.p.add_v(v);
        x.1.p.add_v(v);
        if self.buyos.contains_key(&x.0.p) || self.buyos.contains_key(&x.1.p) {
            let mut v2 = v.clone();
            v2.mult_s(-1);
            x.0.p.add_v(v2);
            x.1.p.add_v(v2);
            return false;
        }
        true
    }
    pub fn next_buyo(&mut self) -> (BType, BType) {
        let crnt_ptr = self.randomizer.current_pointer();
        let type_a = to_btype(self.randomizer.get(crnt_ptr + 1));
        let type_b = to_btype(self.randomizer.get(crnt_ptr + 2));
        return (type_a, type_b);
    }
    // for every buyo in buyos, move them down as gravity would move them
    fn gravity(&mut self) -> bool {
        let mut moved = false;
        for (b, c) in self.buyos.clone() {
            if c != BType::Wall {
                let mut b1 = b.clone();
                b1.add_i(0, 1); // move buyo down and check if theres a collision
                while !self.buyos.contains_key(&b1) {
                    b1.add_i(0, 1); // while there aren't collisions keep moving down
                }
                b1.add_i(0, -1); // buyo is inside another buyo, so it needs to get moved up
                self.buyos.remove(&b);
                self.buyos.insert(b1, c);
                if b1 != b {
                    moved = true;
                }
            }
        }
        moved
    }
    // pop the buyos that are 4 or more of the same color connecting
    // wall color does not pop
    fn pop_buyos(&mut self) -> bool {
        let a = self.gravity();
        if a {
            while self.gravity() {}
            return true;
        }
        let mut has_popped: bool = false;
        for (b, c) in self.buyos.clone() {
            if &c == &BType::Wall {
                continue;
            }
            let mut count = 0;
            let mut q = VecDeque::new();
            let mut visited = HashSet::new();
            q.push_front(b);
            visited.insert(b);
            while !q.is_empty() {
                let current = q.pop_back().unwrap();
                count += 1;
                let adjacent_nodes = vec![
                    &current + &BVec { x: 0, y: 1 },
                    &current + &BVec { x: 0, y: -1 },
                    &current + &BVec { x: 1, y: 0 },
                    &current + &BVec { x: -1, y: 0 },
                ];
                for neighbor in adjacent_nodes {
                    if self.buyos.get(&neighbor) == Some(&c) && !visited.contains(&neighbor) {
                        q.push_front(neighbor);
                        visited.insert(neighbor);
                    }
                }
            }
            if count >= 4 {
                for b in visited {
                    self.buyos.remove(&b);
                }
                has_popped = true;
            }
        }
        has_popped
    }
    
    pub fn print_grid(&self) {
        // Determine the bounds of the grid
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for &bvec in self.buyos.keys() {
            if bvec.x < min_x {
                min_x = bvec.x;
            }
            if bvec.x > max_x {
                max_x = bvec.x;
            }
            if bvec.y < min_y {
                min_y = bvec.y;
            }
            if bvec.y > max_y {
                max_y = bvec.y;
            }
        }

        // Create a grid with the determined dimensions
        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;
        let mut grid = vec![vec![' '; width]; height];

        // Fill the grid with the corresponding characters for each BType
        for (bvec, btype) in &self.buyos {
            let grid_x = (bvec.x - min_x) as usize;
            let grid_y = (bvec.y - min_y) as usize;
            grid[grid_y][grid_x] = match btype {
                BType::Red => 'R',
                BType::Blue => 'B',
                BType::Green => 'G',
                BType::Purple => 'P',
                BType::Wall => '#',
            };
        }

        // If there is a controlled buyo, place it on the grid
        if let Some((b1, b2)) = &self.controlled_buyo {
            let grid_x1 = (b1.p.x - min_x) as usize;
            let grid_y1 = (b1.p.y - min_y) as usize;
            let grid_x2 = (b2.p.x - min_x) as usize;
            let grid_y2 = (b2.p.y - min_y) as usize;

            // Place the controlled buyos on the grid
            grid[grid_y1][grid_x1] = 'C'; // Representing the first controlled buyo
            grid[grid_y2][grid_x2] = 'C'; // Representing the second controlled buyo
        }

        // Print the grid without reversing
        for row in grid.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

// use speedy2d::color::Color;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;

use crate::blockstacker::{color, BlockStacker, Tuning};
use crate::randomizer::Randomizer;
use crate::vectors::BVec;
use crate::Sprite;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum BType {
    Red,
    Blue,
    Green,
    Purple,
    Wall,
}

#[derive(Debug)]
enum LoopState {
    SpawnNew,
    Falling,
    Locking,
    OnFloor(u64), // u64 is timestamp of landing on floor
    Chaining,
}

impl color for BType {
    fn from_str(color: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match color.to_lowercase().as_str() {
            "red" => Some(BType::Red),
            "blue" => Some(BType::Blue),
            "green" => Some(BType::Green),
            "purple" => Some(BType::Purple),
            "wall" => Some(BType::Wall),
            _ => None,
        }
    }
}

impl Display for BType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

fn to_sprite(i: i32) -> Sprite {
    match i {
        0 => Sprite::BuyoRed,
        1 => Sprite::BuyoBlue,
        2 => Sprite::BuyoGreen,
        3 => Sprite::BuyoPurple,
        _ => panic!(),
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Buyo {
    p: BVec,
    offset_x: i32,
    offset_y: i32,
    t: Sprite,
}

struct Tables {
    color_bonus_table: Vec<i32>,
    group_bonus_table: Vec<i32>,
    chain_power_table: Vec<i32>,
}

// tables are all hardcoded and will not change
impl Tables {
    pub fn new() -> Tables {
        Tables {
            color_bonus_table: vec![0, 0, 3, 6, 12, 24],
            group_bonus_table: vec![0, 2, 3, 4, 5, 6, 7, 10],
            chain_power_table: vec![
                0, 0, 8, 16, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448,
                480, 512, 544, 576, 608, 640, 672,
            ],
        }
    }
    fn get_item_in_table(&self, i: i32, table: &Vec<i32>) -> i32 {
        if i as usize > table.len() {
            *table.last().unwrap()
        } else {
            table[i as usize]
        }
    }
    pub fn get_cb(&self, i: i32) -> i32 {
        self.get_item_in_table(i, &self.color_bonus_table)
    }
    pub fn get_gb(&self, i: i32) -> i32 {
        self.get_item_in_table(i, &self.group_bonus_table)
    }
    pub fn get_cp(&self, i: i32) -> i32 {
        self.get_item_in_table(i, &self.chain_power_table)
    }
}

pub struct BuyoBuyo {
    buyos: HashMap<BVec, Sprite>,
    buyo_block_offsets: HashMap<BVec, f32>,
    controlled_buyo: Option<(Buyo, Buyo)>,
    randomizer: Randomizer,
    puyos_cleared: i32,           // -- for scoring --
    chain_power: i32,             // indice for table
    group_bonus: Vec<i32>,        // list of indices for table
    color_bonus: HashSet<Sprite>, // len is indice for table
    tables: Tables,
    total_score: i32,

    loop_state: LoopState, // -- for game loop
    tuning: Tuning,
}

impl BlockStacker for BuyoBuyo {
    // create a game board
    fn new(width: i32, height: i32, randomizer: Randomizer, tuning: Tuning) -> Self {
        let mut buyos = HashMap::new();
        for x in 0..width + 2 {
            for y in 0..height + 2 {
                if x == 0 || x == width + 1 || y == 0 || y == height + 1 {
                    buyos.insert(BVec::new(x, y), Sprite::Wall);
                }
            }
        }
        BuyoBuyo {
            buyos,
            buyo_block_offsets: HashMap::new(),
            controlled_buyo: None,
            randomizer,
            puyos_cleared: 0,
            chain_power: 0,
            group_bonus: Vec::new(),
            color_bonus: HashSet::new(),
            tables: Tables::new(),
            total_score: 0,
            loop_state: LoopState::SpawnNew,
            tuning,
        }
    }
    fn get_board(&self) -> HashMap<BVec, Sprite> {
        let a = self.buyos.clone();
        return a;
    }
    fn next_queue(&self) -> HashMap<BVec, Sprite> {
        let a = self.next_buyo();
        let mut map = HashMap::new();
        map.insert(BVec::new(0, 0), a.0.clone());
        map.insert(BVec::new(0, 1), a.1.clone());
        return map;
    }
    fn get_controlled_block(&self) -> HashMap<BVec, Sprite> {
        let mut a = HashMap::new();
        match self.controlled_buyo {
            Some(x) => {
                a.insert(x.0.p, x.0.t);
                a.insert(x.1.p, x.1.t);
            }
            None => (),
        }
        return a;
    }
    fn input_left(&mut self) -> bool {
        self.move_c_buyo_if_no_collision(BVec::new(-1, 0))
    }
    fn input_right(&mut self) -> bool {
        self.move_c_buyo_if_no_collision(BVec::new(1, 0))
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
        while self.move_c_buyo_if_no_collision(BVec::new(0, 1)) {}
        self.freeze_c_buyo();
    }
    fn move_c_buyo_down(&mut self) -> bool {
        self.move_c_buyo_if_no_collision(BVec::new(0, 1))
    }
    fn is_on_ground(&self) -> bool {
        match self.controlled_buyo {
            Some(x) => {
                let b1onfloor = self.buyos.contains_key(&(&x.0.p + &BVec::new(0, 1)));
                let b2onfloor = self.buyos.contains_key(&(&x.1.p + &BVec::new(0, 1)));
                return b1onfloor || b2onfloor;
            }
            None => false,
        }
    }
    fn score(&self) -> i32 {
        let mut bonus = self.tables.get_cp(self.chain_power)
            + self.tables.get_cb(self.color_bonus.len() as i32)
            + self.group_bonus.iter().sum::<i32>();
        if bonus < 1 {
            bonus = 1;
        } else if bonus > 999 {
            bonus = 999;
        }
        return (10 * self.puyos_cleared) * bonus;
    }
    fn total_score(&self) -> i32 {
        return self.total_score;
    }
    /// returns true if something changed
    fn game_loop(&mut self, last_update_time: u64, current_time: u64) -> bool {
        let delta_time = current_time - last_update_time;
        println!("time: {delta_time}, state: {:?}", self.loop_state);
        match self.loop_state {
            LoopState::SpawnNew => {
                if delta_time < self.tuning.spawn_delay {
                    return false;
                }
                self.reset_chain();
                let b1 = Buyo {
                    p: BVec::new(3, 2),
                    t: to_sprite(self.randomizer.next()),
                    offset_x: 0,
                    offset_y: 0
                };
                let b2 = Buyo {
                    p: &b1.p + &BVec::new(0, -1),
                    t: to_sprite(self.randomizer.next()),
                    offset_x: 0,
                    offset_y: 0
                };
                self.spawn_c_buyo((b1, b2));
                self.loop_state = LoopState::Falling;
            }
            LoopState::Falling => {
                if self.controlled_buyo.is_none() {self.loop_state = LoopState::Locking; return false;}
                if delta_time < self.tuning.fall_speed {
                    return false;
                }
                let b = match &mut self.controlled_buyo {
                    Some(b) => b,
                    None => return false,
                };
                b.0.offset_y += (0.5 * 100.0) as i32;
                b.1.offset_y += (0.5 * 100.0) as i32;
                println!("offset: {:?}", b.0.offset_y);
                if b.0.offset_y as f32 / 100.0 > 1f32 {
                    // has moved into the next square
                    b.0.offset_y = 0;
                    b.1.offset_y = 0;
                    let on_ground = !self.move_c_buyo_down();
                    if on_ground {
                        self.loop_state = LoopState::OnFloor(current_time);
                    }
                }
            }
            LoopState::Locking => {
                // todo let you leave lock state back to falling state
                if self.tuning.lock_delay < delta_time {
                    self.loop_state = LoopState::Chaining;
                } else {
                    return false;
                }
            }
            LoopState::OnFloor(time) => {
                if current_time - time < self.tuning.freeze_delay {
                    return false;
                }
                self.freeze_c_buyo();
                self.loop_state = LoopState::Locking;
            }
            LoopState::Chaining => {
                if delta_time < self.tuning.clear_delay {
                    return false;
                }
                let b = self.pop_buyos();
                if b.0 == false {
                    self.loop_state = LoopState::SpawnNew;
                }
            }
        }
        return true;
    }
}

impl BuyoBuyo {
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
            None => return false,
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
    pub fn next_buyo(&self) -> (Sprite, Sprite) {
        let crnt_ptr = self.randomizer.current_pointer();
        let type_a = to_sprite(self.randomizer.get(crnt_ptr + 1));
        let type_b = to_sprite(self.randomizer.get(crnt_ptr + 2));
        if crnt_ptr == 0 {
            let type_a = to_sprite(self.randomizer.get(crnt_ptr + 3));
            let type_b = to_sprite(self.randomizer.get(crnt_ptr + 4));
            return (type_b, type_a);
        }
        return (type_b, type_a);
    }
    // for every buyo in buyos, move them down as gravity would move them
    fn gravity(&mut self) -> bool {
        let mut moved = false;
        for (b, c) in self.buyos.clone() {
            if c != Sprite::Wall {
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
    // pop the buyos that are 4 or more of the same Color connecting
    // wall Color does not pop
    fn pop_buyos(&mut self) -> (bool, i32) {
        let a = self.gravity();
        if a {
            while self.gravity() {}
            return (true, 1);
        }
        self.color_bonus.clear();
        self.group_bonus.clear();
        self.puyos_cleared = 0;
        let mut change_in_score = self.score();
        let mut has_popped: bool = false;
        for (b, c) in self.buyos.clone() {
            if &c == &Sprite::Wall {
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
                    &current + &BVec::new(0, 1),
                    &current + &BVec::new(0, -1),
                    &current + &BVec::new(1, 0),
                    &current + &BVec::new(-1, 0),
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
                self.puyos_cleared += count;
                self.color_bonus.insert(c);
                self.group_bonus.push(self.tables.get_gb(count - 4));
                has_popped = true;
            }
        }

        if has_popped {
            self.chain_power += 1;
        }
        change_in_score = self.score() - change_in_score;
        self.total_score += change_in_score;
        (has_popped, 0)
    }

    fn reset_chain(&mut self) {
        self.puyos_cleared = 0;
        self.color_bonus.clear();
        self.group_bonus.clear();
        self.chain_power = 0;
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
                Sprite::BuyoRed => 'R',
                Sprite::BuyoBlue => 'B',
                Sprite::BuyoGreen => 'G',
                Sprite::BuyoPurple => 'P',
                Sprite::BuyoYellow => 'Y',
                Sprite::Wall => '#',
                _ => panic!(),
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

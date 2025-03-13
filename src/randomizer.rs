use rand::{rngs::ThreadRng, Rng};

pub struct Randomizer {
    queue: Vec<i32>,
    max: i32,
    current: i32,
    rng: ThreadRng
}

impl Randomizer {
    pub fn new(max: i32) -> Randomizer {
        let rng = rand::rng();
        Randomizer { queue: Vec::new(), max, current: 0, rng }
    }
    pub fn get(&mut self, i: i32) -> i32 {
        while (self.queue.len() as i32) < i+1 {
            let new_num = self.rng.random_range(0..self.max);
            self.queue.push(new_num);
        }
        return *self.queue.get(i as usize).unwrap();
    }
    pub fn current_pointer(&self) -> i32 {
        return *&self.current
    }
    pub fn next(&mut self) -> i32 {
        let a = self.get(self.current);
        self.current+=1;
        return a;
    }
}
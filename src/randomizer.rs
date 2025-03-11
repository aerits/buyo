use std::thread::current;

pub struct Randomizer {
    queue: Vec<i32>,
    max: i32,
    current: i32
}

impl Randomizer {
    pub fn new(max: i32) -> Randomizer {
        Randomizer { queue: Vec::new(), max, current: 0 }
    }
    pub fn get(&mut self, i: i32) -> i32 {
        if (self.queue.len() as i32) < i {
            // TODO
        }
        return 0;
    }
    pub fn next(&mut self) -> i32 {
        let a = self.get(self.current);
        self.current+=1;
        return a;
    }
}
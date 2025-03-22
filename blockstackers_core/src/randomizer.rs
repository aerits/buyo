use oorandom::Rand64;

pub struct Randomizer {
    queue: Vec<i32>,
    max: i32,
    current: i32,
    rng: Rand64
}

impl Randomizer {
    pub fn new(max: i32, seed: u128) -> Randomizer {
        let rng = Rand64::new(seed);
        Randomizer { queue: Vec::new(), max, current: 0, rng }
    }
    pub fn get(&mut self, i: i32) -> i32 {
        while (self.queue.len() as i32) < i+1 {
            let new_num = self.rng.rand_range(0..(self.max as u64));
            self.queue.push(new_num as i32);
        }
        return *self.queue.get(i as usize).unwrap();
    }
    pub fn current_pointer(&self) -> i32 {
        return self.current.clone() - 1
    }
    pub fn next(&mut self) -> i32 {
        let a = self.get(self.current);
        self.current+=1;
        return a;
    }
}
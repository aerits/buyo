use std::cell::RefCell;

use oorandom::Rand64;
use crate::ring_buffer::RingBufferVec;

pub struct Randomizer {
    queue: RefCell<RingBufferVec<i32>>,
    max: i32,
    current: i32,
    rng: RefCell<Rand64>
}

impl Randomizer {
    pub fn new(max: i32, seed: u128) -> Randomizer {
        let rng = Rand64::new(seed);
        Randomizer { queue: RefCell::new(RingBufferVec::new(0, 100)), max, current: 0, rng: RefCell::new(rng) }
    }
    pub fn get(&self, i: i32) -> i32 {
        while (self.queue.borrow().len() as i32) < i+1 {
            let new_num = self.rng.borrow_mut().rand_range(0..(self.max as u64));
            self.queue.borrow_mut().push(new_num as i32);
        }
        return *self.queue.borrow().get(i as usize).unwrap();
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
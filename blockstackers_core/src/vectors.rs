use std::ops::Add;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct BVec {
    pub x: i32,
    pub y: i32,
}
impl BVec {
    pub fn new(x: i32, y: i32) -> BVec {
        BVec { x, y }
    }
    pub fn add_v(&mut self, rhs: BVec) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
    pub fn add_i(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
    pub fn mult_s(&mut self, n: i32) {
        self.x *= n;
        self.y *= n;
    }
    pub fn clear(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}
impl Add for &BVec {
    type Output = BVec;

    fn add(self, rhs: Self) -> Self::Output {
        BVec::new(self.x + rhs.x, self.y + rhs.y)
    }
}
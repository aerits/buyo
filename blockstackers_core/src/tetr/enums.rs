pub enum Mino {
    Red,
    Blue,
    Yellow,
    Orange,
    Wall,
}

pub enum Shapes {
    O,
    L,
    J,
    T,
    I,
    Z,
    S,
}
#[derive(Eq, Hash, PartialEq)]
pub enum Rotation {
    Up,
    Right,
    Down,
    Left,
}
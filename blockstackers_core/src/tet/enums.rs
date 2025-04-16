pub enum Mino {
    Red,
    Blue,
    LightBlue,
    Yellow,
    Orange,
    Green,
    Purple,
    Wall,
}
#[derive(Clone, Copy)]
pub enum Shapes {
    O,
    L,
    J,
    T,
    I,
    Z,
    S,
}
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum Rotation {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}
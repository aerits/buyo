use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
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

impl Display for Mino {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
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

#[derive(Debug)]
pub enum LoopState {
    Falling,
    Spawning,
    OnFloor(u64),
    LockingOrClearing,
}
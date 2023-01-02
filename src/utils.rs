use enumflags2::{bitflags, BitFlags};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[bitflags]
#[repr(u8)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}
impl Direction{
pub fn reverse(&self) -> Direction {
    match self {
        Direction::Down => Direction::Up,
        Direction::Up => Direction::Down,
        Direction::Right => Direction::Left,
        Direction::Left => Direction::Right
    }
}
}
pub type DirectionFlags = BitFlags<Direction>;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

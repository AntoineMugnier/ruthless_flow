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

pub type DirectionFlags = BitFlags<Direction>;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

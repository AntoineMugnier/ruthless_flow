use enumflags2::{bitflags, BitFlags};

#[derive(Copy, Clone)]
#[bitflags]
#[repr(u8)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left
}

pub type DirectionFlags = BitFlags<Direction>;

#[derive(Copy, Clone)]
pub struct Coordinates{
    pub x: usize,
    pub y : usize
}


#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    None
}
#[derive(Copy, Clone)]
pub struct Coordinates{
    pub x: usize,
    pub y : usize
}

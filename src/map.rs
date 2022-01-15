use std::{result::Iter, iter::Cycle,  slice};

use crate::utils::{Direction, Coordinates};

pub enum Tile{
    Occupied,
    Marked{head_direction: Direction, tail_direction : (Direction, Direction, Direction)},
    Free,
    Separator,
    Walled
}

pub struct Map{  
    _sto : Vec<[Tile; 50]>,
    _heads : Vec<Coordinates>,
    x_size : u32,
    
}

impl Map {
pub fn new()-> Self{
    Map{_sto: }
}

pub fn get_heads_iter_mut(&self) ->  slice::IterMut<Coordinates> {
    self._heads.iter_mut()    
}

pub fn set_tile(&mut self, tile : Tile, coordinates : Coordinates){
    self._sto[coordinates.x][coordinates.y] = tile;
}

pub fn get_neighbour_tile (& self, coordinates : Coordinates, direction : Direction) -> (Tile, Coordinates){
    match direction {
        Direction::Up => todo!(),
        Direction::Down => todo!(),
        Direction::Right => todo!(),
        Direction::Left => todo!(),
        Direction::None => todo!(),
    }
    self._sto[coordinates.x][coordinates.y] +
    (self._sto[2][2], Coordinates{x: 2,y: 2})
}

}
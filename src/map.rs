use std::{result::Iter, iter::Cycle,  slice, collections::VecDeque};

use crate::utils::{Direction, Coordinates};

#[derive(Copy, Clone)]
pub enum TileType{
    Marked ,
    Free,
    Separator,
    Wall
}

pub struct Map {  
    pub sto : VecDeque<Vec<TileType>>  
}

impl Map {
pub fn new()-> Self{
    Map{
        sto: VecDeque::from(
            [
                vec![TileType::Free, TileType::Free, TileType::Free, TileType::Free, TileType::Free],
                vec![TileType::Free, TileType::Free, TileType::Free, TileType::Free, TileType::Free],
                vec![TileType::Free, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Free],
                vec![TileType::Free, TileType::Free, TileType::Free, TileType::Free, TileType::Free]
            ]
        )
    }
}

pub fn set_tile(&mut self, position : Coordinates,  tile_type : TileType){
    self.sto[position.x][position.y] = tile_type;
}

pub fn get_tile (&mut self, position : Coordinates, direction :Direction) -> Option<(TileType, Coordinates)>{

    let mut x = position.x as isize;
    let mut y :isize =  position.y as isize;

    match direction {
        Direction::Up => {
            y +=1;
        },
        Direction::Down => {
            y -= 1;
        },
        Direction::Right =>{
            x += 1;
        },
        Direction::Left =>{
            x -= 1;
        },
        Direction::None => {
        },
        };

        if (x +1) <= self.get_length() as isize 
        && (y +1) <= self.get_height() as isize
        && x >= 0
        && y >= 0{
            let tile_type = self.sto[y as usize][x as usize];
            let position =  Coordinates { x: x as usize, y: y as usize};
            
            Some((tile_type, position))
        }
        else{
            None
        } 
}

pub fn get_length(&self) -> usize{
    self.sto.len()
}

pub fn get_height(&self) -> usize{
    self.sto[0].len()
}
}

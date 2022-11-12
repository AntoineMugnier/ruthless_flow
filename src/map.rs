use std::{result::Iter, iter::Cycle,  slice, collections::VecDeque};
use mockall::automock;

use crate::utils::{Direction, Coordinates};

#[derive(Copy, Clone, PartialEq)]
pub enum TileType{
    Marked ,
    Free,
    Separator,
    Wall
}

#[cfg_attr(test, mockall::automock)]
pub trait Map{
    fn new()-> Self;
    fn set_tile(&mut self, position : Coordinates,  tile_type : TileType);
    fn get_tile (&mut self, position : Coordinates) -> TileType;
    fn get_neighbour_tile (&mut self, position : Coordinates, direction :Direction) -> Option<(TileType, Coordinates)>;
    fn get_length(&self) -> usize;
    fn get_height(&self) -> usize;
}

pub struct SimpleMap {  
    pub sto : VecDeque<Vec<TileType>>  
}
impl SimpleMap{
}
impl Map for SimpleMap {

fn new()-> Self{
    SimpleMap{
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

fn set_tile(&mut self, position : Coordinates,  tile_type : TileType){
    self.sto[position.y][position.x] = tile_type;
}

fn get_tile (&mut self, position : Coordinates) -> TileType{
    self.sto[position.y][position.x]
}
fn get_neighbour_tile (&mut self, position : Coordinates, direction :Direction) -> Option<(TileType, Coordinates)>{

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
        }
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

fn get_length(&self) -> usize{
    self.sto.len()
}

fn get_height(&self) -> usize{
    self.sto[0].len()
}
}

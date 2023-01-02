use std::{collections::VecDeque};
use crate::utils::{Coordinates, Direction};
use crate::mpsc::Sender;
use crate::frontend;
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
    Marked,
    Free,
    Separator,
    Wall,
}

#[cfg_attr(test, mockall::automock)]
pub trait MapTrait {
    fn new(frontend_sender: Sender<frontend::Event>, sto: VecDeque<Vec<TileType>>) -> Self;
    fn set_tile(&mut self, position: Coordinates, tile_type: TileType);
    fn get_tile(&mut self, position: Coordinates) -> TileType;
    fn get_neighbour_tile(
        &mut self,
        position: Coordinates,
        direction: Direction,
    ) -> Option<(TileType, Coordinates)>;
    fn get_length(&self) -> usize;
    fn get_height(&self) -> usize;
    fn slide(&mut self);
}

pub struct Map {
    pub sto: VecDeque<Vec<TileType>>,
    frontend_sender: Sender<frontend::Event>
}
impl Map {}
impl MapTrait for Map {
    fn new(
        frontend_sender: Sender<frontend::Event>, 
        sto: VecDeque<Vec<TileType>>
    ) -> Self {
        Map {
            frontend_sender,
            sto
        }
    }

    fn slide(&mut self){

    }

    fn set_tile(&mut self, position: Coordinates, tile_type: TileType) {
        self.sto[position.y][position.x] = tile_type;
    }

    fn get_tile(&mut self, position: Coordinates) -> TileType {
        self.sto[position.y][position.x]
    }
    fn get_neighbour_tile(
        &mut self,
        position: Coordinates,
        direction: Direction,
    ) -> Option<(TileType, Coordinates)> {
        let mut x = position.x as isize;
        let mut y: isize = position.y as isize;

        match direction {
            Direction::Up => {
                y += 1;
            }
            Direction::Down => {
                y -= 1;
            }
            Direction::Right => {
                x += 1;
            }
            Direction::Left => {
                x -= 1;
            }
        };

        if (x + 1) <= self.get_length() as isize
            && (y + 1) <= self.get_height() as isize
            && x >= 0
            && y >= 0
        {
            let tile_type = self.sto[y as usize][x as usize];
            let position = Coordinates {
                x: x as usize,
                y: y as usize,
            };

            Some((tile_type, position))
        } else {
            None
        }
    }

    fn get_length(&self) -> usize {
        self.sto.len()
    }

    fn get_height(&self) -> usize {
        self.sto[0].len()
    }
}

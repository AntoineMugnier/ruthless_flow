use std::{collections::VecDeque};
use crate::utils::{Coordinates, Direction};
use crate::mpsc::Sender;
use crate::frontend;

use super::heads::Id;
#[derive(Copy, Clone, Debug, PartialEq)]

// Each tile of the map can be or become one of these variants
pub enum TileType {
    Head{id : Id},
    Marked{id : Id},
    Free,
    Separator,
    Wall,
}

#[cfg_attr(test, mockall::automock)]
// Defines interface for reading, editiing tiles, and sliding the map
pub trait MapTrait {
    fn new(frontend_sender: Sender<frontend::Event>, sto: VecDeque<Vec<TileType>>, usable_map_lines : usize) -> Self;
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
    fn is_on_arrival_line(&self, position: Coordinates) -> bool;
    fn will_head_pop_out_during_next_sliding(&self) -> bool;
}

// Stores all the tiles of the game and manage their editions and reading through the map `MapTrait` trait.
pub struct Map {
    pub sto: VecDeque<Vec<TileType>>,
    frontend_sender: Sender<frontend::Event>,
    map_nb_visible_lines : usize,
    y_offset : usize
}
impl Map {}

impl MapTrait for Map {
    fn new(
        frontend_sender: Sender<frontend::Event>, 
        sto: VecDeque<Vec<TileType>>,
        map_nb_visible_lines : usize
    ) -> Self {
        
        Map {
            frontend_sender,
            sto,
            map_nb_visible_lines,
            y_offset : 0
        }
    }

    // Return true if a head tile is on the arrival line, else false
    fn is_on_arrival_line(&self, position: Coordinates) -> bool{

        let arrival_line_y = self.get_height()  - self.map_nb_visible_lines;
        (position.y - self.y_offset) == arrival_line_y 
    }
    
    // Return true if a head tile is in on the first line of the map
    fn will_head_pop_out_during_next_sliding(&self) -> bool{
        let next_line_to_pop_out = &self.sto[0]; // Bottom line of the map

        for tile in next_line_to_pop_out{
            match tile {
                TileType::Head {..} =>{
                    return true
                }
                _ =>{}
            }
        }
        false
    }

    // Remove the first line of the map but **does not** affect coordinates referential 
    fn slide(&mut self){
        self.sto.pop_front().unwrap(); // Remove bottom line of the map
        self.y_offset+=1;
        let evt = frontend::Event::NewMapLine;
        self.frontend_sender.send(evt).unwrap();

    }

    fn set_tile(&mut self, position: Coordinates, tile_type: TileType) {

        let position = Coordinates{ x: position.x, y:  position.y - self.y_offset};

        self.sto[position.y][position.x] = tile_type;

        let event = frontend::Event::SetTile{position ,tile_type};
        self.frontend_sender.send(event).unwrap();
    }

    fn get_tile(&mut self, position: Coordinates) -> TileType {
        self.sto[position.y - self.y_offset][position.x]
    }

    // Get the tile type and the coordinates of a tile adjacent to a certain position
    fn get_neighbour_tile(
        &mut self,
        position: Coordinates,
        direction: Direction,
    ) -> Option<(TileType, Coordinates)> {
        let mut x = position.x as isize;
        let mut y: isize = position.y as isize - self.y_offset as isize;

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
                y: y as usize + self.y_offset,
            };

            Some((tile_type, position))
        } else {
            None
        }
    }

    fn get_length(&self) -> usize {
        self.sto[0].len()
    }

    fn get_height(&self) -> usize {
        self.sto.len()
    }
}

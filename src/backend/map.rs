use std::{collections::VecDeque};
use crate::utils::{Coordinates, Direction};
use crate::mpsc::Sender;
use crate::frontend;

use super::board::EndGameReason;
use super::heads::Id;
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
    Head{id : Id},
    Marked,
    Free,
    Separator,
    Wall,
}

#[cfg_attr(test, mockall::automock)]
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
    fn send_end_game_evt(&self, end_game_reason : EndGameReason);

}

pub struct Map {
    pub sto: VecDeque<Vec<TileType>>,
    frontend_sender: Sender<frontend::Event>,
    usable_map_lines : usize,
    y_offset : usize
}
impl Map {}
impl MapTrait for Map {
    fn new(
        frontend_sender: Sender<frontend::Event>, 
        sto: VecDeque<Vec<TileType>>,
        usable_map_lines : usize
    ) -> Self {

        // Send fist visible lines to frontend
        for i in 0..usable_map_lines{
            let new_line = sto[i].clone(); 
            let evt = frontend::Event::NewMapLine{line: new_line};
            frontend_sender.send(evt).unwrap();            
        }

        Map {
            frontend_sender,
            sto,
            usable_map_lines,
            y_offset : 0
        }
    }

    fn send_end_game_evt(&self, end_game_reason : EndGameReason){
        let event = frontend::Event::EndGame{game_end_reason: end_game_reason};
        self.frontend_sender.send(event).unwrap();
    }

    fn slide(&mut self){

        let last_line = self.sto.pop_front().unwrap(); // Remove bottom line of the map

        // If a head tile has been popped out, stop game
        for tile in last_line{
            match tile {
                TileType::Head {..} =>{
                    self.send_end_game_evt(EndGameReason::HeadPoppedOutByRisingEdge)
                }
                _ =>{}
            }
        }

        self.y_offset+=1;

        // Post new line to frontend
        let new_line = self.sto[self.usable_map_lines - 1].clone(); 

        let evt = frontend::Event::NewMapLine{line: new_line};
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

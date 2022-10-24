use crate::utils::{Direction, Coordinates};
use crate::map::TileType;

pub struct Head{
    position: Coordinates,
    coming_from : Direction
}

impl Head {
    pub fn new(position: Coordinates, coming_from: Direction) -> Head {
        Head{position, coming_from}
    }

    pub fn set_position(&mut self, position: Coordinates){
        self.position = position;
    }

    pub fn set_provenance(&mut self, coming_from: Direction){
        self.coming_from = coming_from;
    }


    pub fn get_position(&self) -> Coordinates{
        self.position 
    }


    pub fn get_provenance(&self) -> Direction{
        self.coming_from 
    }
}

use std::collections::VecDeque;
use crate::{utils::Coordinates, backend::{map::TileType}};

pub struct GfxMap{
    sto: VecDeque<Vec<TileType>>
}

impl GfxMap{
    pub fn new(sto : VecDeque<Vec<TileType>>) -> GfxMap{
        GfxMap{sto}
    }

    pub fn add_line(&mut self, line:Vec<TileType>){
        self.sto.push_front(line)
    }

    pub fn set_tile(&mut self, position: Coordinates, tile_type: TileType) {
        self.sto[position.y][position.x] = tile_type;
    }

    
}
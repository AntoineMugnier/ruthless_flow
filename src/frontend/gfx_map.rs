use std::collections::VecDeque;
use piston_window::{Context, G2d, clear, rectangle};

use crate::{utils::Coordinates, backend::{map::TileType}};

pub struct GfxMap{
    sto: VecDeque<Vec<TileType>>
}

impl GfxMap{
    pub fn new(sto : VecDeque<Vec<TileType>>) -> GfxMap{
        GfxMap{sto}
    }
    pub fn render(&mut self, c: &Context, g: &mut G2d){
        clear([1.0 ,1.0, 1.0 , 1.0], g);
        rectangle([1.0, 0.0, 0.0, 1.0], // red
                  [0.0, 0.0, 100.0, 100.0],
                  c.transform, g);
    }

    pub fn add_line(&mut self, line:Vec<TileType>){
        self.sto.push_front(line)
    }

    pub fn set_tile(&mut self, position: Coordinates, tile_type: TileType) {
        self.sto[position.y][position.x] = tile_type;
    }

    
}
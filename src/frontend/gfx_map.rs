use std::collections::VecDeque;
use piston_window::{Context,color, G2d, clear, rectangle, line, line_from_to, Position, rectangle_from_to, };
use crate::{backend::{map::TileType}, utils::Coordinates};
use super::config;

pub struct GfxMap{
    sto: VecDeque<Vec<TileType>>
}

impl GfxMap{
    pub fn new(sto : VecDeque<Vec<TileType>>) -> GfxMap{
        //Reverse Y axis
        let sto = sto.into_iter().rev().collect(); 
        GfxMap{sto}
    }

    fn get_length(&self) -> usize {
        self.sto[0].len()
    }

    fn get_height(&self) -> usize {
        self.sto.len()
    }

    pub fn set_tile(&mut self, position: Coordinates, tile_type: TileType) {
        let y_max = self.get_height() - 1;
        
        // Set the tile according to the new referential 
        self.sto[y_max - position.y ][position.x] = tile_type;
    }

    fn render_tiles(&mut self, c: &Context, g: &mut G2d){

        let tile_height = (config::map::END_Y - config::map::ORIGIN_Y)/ (self.get_height() as f64);
        let tile_length = (config::map::END_X - config::map::ORIGIN_X)/ (self.get_length() as f64);

        for (y_tile, line_of_tiles) in self.sto.iter().enumerate(){
            for  (x_tile, tile) in line_of_tiles.iter().enumerate(){
                let tile_color;
                match tile {
                    TileType::Marked => tile_color = color::RED,
                    TileType::Free => tile_color = color::WHITE,
                    TileType::Separator => tile_color = color::BLUE,
                    TileType::Wall => tile_color = color::BLACK,
                }
            

            let x_origin = config::map::ORIGIN_X + (x_tile as f64) * tile_length;
            let x_end = x_origin + tile_length;

            let y_origin =config::map::ORIGIN_Y + (y_tile as f64) * tile_height;
            let y_end = y_origin + tile_height;

            rectangle_from_to(tile_color, 
                    [x_origin, y_origin],[x_end, y_end],
                    c.transform, g);
            }
        }
    }
    
    fn render_grid(&mut self, c: &Context, g: &mut G2d){

        // Draw borders
        line(color::BLACK, config::map::grid::BAR_WIDTH,  [config::map::ORIGIN_X, config::map::ORIGIN_Y, config::map::ORIGIN_X,  config::map::END_Y], c.transform, g);
        line(color::BLACK, config::map::grid::BAR_WIDTH,  [config::map::ORIGIN_X, config::map::END_Y, config::map::END_X,  config::map::END_Y], c.transform, g);
        line(color::BLACK, config::map::grid::BAR_WIDTH,  [config::map::END_X, config::map::END_Y, config::map::END_X,  config::map::ORIGIN_Y], c.transform, g);
        line(color::BLACK, config::map::grid::BAR_WIDTH,  [config::map::END_X, config::map::ORIGIN_Y, config::map::ORIGIN_X,  config::map::ORIGIN_Y], c.transform, g);

        //Draw mesh
        //Horizontal lines
        for mesh_line_index in 1..self.get_height(){
            let mesh_line_origin_y = (mesh_line_index as f64/self.get_height() as f64) * (config::map::END_Y - config::map::ORIGIN_Y) + config::map::ORIGIN_Y;
            line(color::BLACK, config::map::grid::BAR_WIDTH,  [config::map::ORIGIN_X, mesh_line_origin_y, config::map::END_X,  mesh_line_origin_y], c.transform, g);
        }
        
        //Vertical lines
        for mesh_col_index in 1..self.get_length(){
            let mesh_col_origin_x = (mesh_col_index as f64/self.get_length() as f64) * (config::map::END_X - config::map::ORIGIN_X) + config::map::ORIGIN_X;
            line(color::BLACK, config::map::grid::BAR_WIDTH,  [mesh_col_origin_x, config::map::ORIGIN_Y, mesh_col_origin_x, config::map::END_Y], c.transform, g);
        }
    }



    pub fn render(&mut self, c: &Context, g: &mut G2d){

        self.render_tiles(c, g);

        self.render_grid(c, g);
    }

    pub fn add_line(&mut self, line:Vec<TileType>){
        self.sto.push_front(line)
    }



    
}
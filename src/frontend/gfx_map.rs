use std::{collections::VecDeque, time::SystemTime};
use piston_window::{Context,color, G2d, clear, rectangle, line, line_from_to, Position, rectangle_from_to, };
use crate::{backend::{map::TileType}, utils::Coordinates};
use super::config;

pub struct GfxMap{
    sto: VecDeque<Vec<TileType>>,
    time_at_last_line_received : SystemTime,
    map_nb_visible_lines : usize
}

impl GfxMap{
    pub fn new(map_nb_visible_lines : usize) -> GfxMap{
        //Reverse Y axis
        let sto = VecDeque::new(); 
        let time_at_last_line_received = SystemTime::now();

        GfxMap{sto, time_at_last_line_received, map_nb_visible_lines}
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
    if self.sto.len() >= self.map_nb_visible_lines{
        let tile_height = (config::map::END_Y - config::map::ORIGIN_Y)/ (self.get_height() as f64 - 1.0);
        let tile_length = (config::map::END_X - config::map::ORIGIN_X)/ (self.get_length() as f64);

        let time_elapsed_since_newer_line = self.time_at_last_line_received.elapsed().unwrap();
        const TIME_ELAPSED_BETWEEN_TWO_NEW_LINES_MS :f64= 1.0/(crate::backend::config::MAP_SLIDE_FRQUENCY as f64) * 1000.0;
        let time_ratio = (time_elapsed_since_newer_line.as_millis() as f64)/(TIME_ELAPSED_BETWEEN_TWO_NEW_LINES_MS as f64);
        let first_tile_line_height = time_ratio * tile_height;
        let last_tile_line_height = (1.0 -time_ratio) * tile_height;

        let mut draw_tile = |x_origin, y_origin, new_tile_height, tile_type|{            
            let x_end = x_origin + tile_length;
            let y_end = y_origin + new_tile_height;

            let tile_color;
            match tile_type {
                TileType::Marked => tile_color = config::map::tiles::HEAD_MARK_COLOR,
                TileType::Free => tile_color = config::map::tiles::FREE_COLOR,
                TileType::Separator => tile_color = config::map::tiles::SEPARATOR_COLOR,
                TileType::Wall => tile_color = config::map::tiles::WALL_COLOR,
                TileType::Head{..} => tile_color = config::map::tiles::HEAD_COLOR
            }

            rectangle_from_to(tile_color, 
                    [x_origin, y_origin],[x_end, y_end],
                    c.transform, g);
        };

        let mut y_origin = config::map::ORIGIN_Y;

        for (line_index, line_of_tiles) in self.sto.iter().enumerate(){
            let mut x_origin = config::map::ORIGIN_X;

            // First line
            if line_index == 0 {
                for  tile_type in line_of_tiles.iter(){
                    draw_tile(x_origin, y_origin, first_tile_line_height, *tile_type);
                    x_origin+=tile_length;
                }
                y_origin += first_tile_line_height;
            }
            // Last line
            else if  line_index == (self.get_height() - 1){
                for  tile_type in line_of_tiles.iter(){
                    draw_tile(x_origin, y_origin, last_tile_line_height, *tile_type);
                    x_origin+=tile_length;
                }
            }
            // Others
            else{
                for  tile_type in line_of_tiles.iter(){
                    draw_tile(x_origin, y_origin, tile_height, *tile_type);
                    x_origin+=tile_length;
                }
                y_origin += tile_height;
            }
        }

    }
    }
    
    fn render_grid(&mut self, c: &Context, g: &mut G2d){

        // Draw borders
        line(color::BLACK, config::map::grid::BAR_WIDTH,  [config::map::ORIGIN_X, config::map::ORIGIN_Y, config::map::ORIGIN_X,  config::map::END_Y], c.transform, g);
        line(color::BLACK, config::map::grid::BAR_WIDTH,  [config::map::ORIGIN_X, config::map::END_Y, config::map::END_X,  config::map::END_Y], c.transform, g);
        line(color::BLACK, config::map::grid::BAR_WIDTH,  [config::map::END_X, config::map::END_Y, config::map::END_X,  config::map::ORIGIN_Y], c.transform, g);
        line(color::BLACK, config::map::grid::BAR_WIDTH,  [config::map::END_X, config::map::ORIGIN_Y, config::map::ORIGIN_X,  config::map::ORIGIN_Y], c.transform, g);
        /*
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
        */
    }



    pub fn render(&mut self, c: &Context, g: &mut G2d){

        self.render_tiles(c, g);

        self.render_grid(c, g);
    }

    pub fn add_line(&mut self, line:Vec<TileType>){
        self.sto.push_front(line);
        if self.sto.len() > self.map_nb_visible_lines{
            self.sto.pop_back();
        }
        self.time_at_last_line_received = SystemTime::now();
    }



    
}
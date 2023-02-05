use std::{collections::VecDeque, time::{SystemTime, Duration}};
use piston_window::{Context,color, G2d, clear, rectangle, line, line_from_to, Position, rectangle_from_to, };
use crate::{backend::{map::TileType}, utils::Coordinates};
use super::config;

pub struct GfxMap{
    sto: VecDeque<Vec<TileType>>,
    map_nb_visible_lines : usize,
    sliding_state : SlidingState
}
enum SlidingState{
    Enabled{time_at_first_line_received:SystemTime},
    Disabled
}
impl GfxMap{
    pub fn new(map_nb_visible_lines : usize) -> GfxMap{
        //Reverse Y axis
        let sto = VecDeque::new(); 
        let time_at_first_line_received = SystemTime::now();

        GfxMap{sto,  map_nb_visible_lines, sliding_state : SlidingState::Disabled}
    }

    fn get_length(&self) -> usize {
        self.sto[0].len()
    }

    fn get_height(&self) -> usize {
        self.sto.len()
    }

    pub fn set_tile(&mut self, position: Coordinates, tile_type: TileType) {
        
        // Set the tile according to the new referential 
        self.sto[position.y ][position.x] = tile_type;
    }

    fn render_tiles(&mut self, c: &Context, g: &mut G2d){
        if self.sto.len() >= self.map_nb_visible_lines{

            const TIME_ELAPSED_BETWEEN_TWO_NEW_LINES_MS :f64= 1.0/(crate::backend::config::MAP_SLIDE_FRQUENCY as f64) * 1000.0;
            
            let sliding_ratio;

            match self.sliding_state{
                SlidingState::Enabled { time_at_first_line_received } => {
                    let time_at_first_line_received_ms = time_at_first_line_received.elapsed().unwrap().as_millis();
                    let time_elapsed_since_newer_line_ms = time_at_first_line_received_ms as f64 % TIME_ELAPSED_BETWEEN_TWO_NEW_LINES_MS;

                    sliding_ratio = (time_elapsed_since_newer_line_ms as f64)/(TIME_ELAPSED_BETWEEN_TWO_NEW_LINES_MS as f64);
                    println!("{}", sliding_ratio);
                    
                },
                SlidingState::Disabled => sliding_ratio = 0.0, 
            }

            self.render_sliding_map(c, g, sliding_ratio);
        }
    }

    fn render_sliding_map(&mut self, c: &Context, g: &mut G2d, sliding_ratio: f64) {

        let tile_height = (config::map::END_Y - config::map::ORIGIN_Y)/ (self.get_height() as f64 - 1.0);
        let tile_length = (config::map::END_X - config::map::ORIGIN_X)/ (self.get_length() as f64);

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
        let first_tile_line_height = sliding_ratio * tile_height;
        let last_tile_line_height = (1.0 -sliding_ratio) * tile_height;

        for (line_index, line_of_tiles) in self.sto.iter().rev().enumerate(){
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
    
    fn render_frame(&mut self, c: &Context, g: &mut G2d){

        // Draw borders
        line(config::map::frame::BAR_COLOR, config::map::frame::BAR_WIDTH,  [config::map::ORIGIN_X, config::map::ORIGIN_Y, config::map::ORIGIN_X,  config::map::END_Y], c.transform, g);
        line(config::map::frame::BAR_COLOR, config::map::frame::BAR_WIDTH,  [config::map::ORIGIN_X, config::map::END_Y, config::map::END_X,  config::map::END_Y], c.transform, g);
        line(config::map::frame::BAR_COLOR, config::map::frame::BAR_WIDTH,  [config::map::END_X, config::map::END_Y, config::map::END_X,  config::map::ORIGIN_Y], c.transform, g);
        line(config::map::frame::BAR_COLOR, config::map::frame::BAR_WIDTH,  [config::map::END_X, config::map::ORIGIN_Y, config::map::ORIGIN_X,  config::map::ORIGIN_Y], c.transform, g);
    }

    pub fn render(&mut self, c: &Context, g: &mut G2d){

        self.render_tiles(c, g);

        self.render_frame(c, g);
    }

    pub fn start_sliding(&mut self){
            self.sliding_state = SlidingState::Enabled{time_at_first_line_received: SystemTime::now()};

    }

    pub fn add_line(&mut self, line:Vec<TileType>){
        self.sto.push_back(line);
        
        if self.sto.len() > self.map_nb_visible_lines{
            self.sto.pop_front();
        }

    }



    
}
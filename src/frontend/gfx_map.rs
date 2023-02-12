use std::{collections::VecDeque, time::{SystemTime, Duration}};
use piston_window::{Context,color, G2d, clear, rectangle, line, line_from_to, Position, rectangle_from_to, };
use crate::{backend::{heads::Id,map::TileType}, utils::Coordinates};
use super::config;

pub struct GfxMap{
    sto: VecDeque<Vec<TileType>>,
    map_nb_visible_lines : usize,
    sliding_state : SlidingState
}
enum SlidingState{
    Enabled{time_since_last_slide:SystemTime},
    Disabled
}
impl GfxMap{
    pub fn new(map_nb_visible_lines : usize, sto :  VecDeque<Vec<TileType>>) -> GfxMap{
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

        const TIME_ELAPSED_BETWEEN_TWO_NEW_LINES_MS :f64= 1.0/(crate::backend::config::MAP_SLIDE_FRQUENCY as f64) * 1000.0;
        
        let sliding_ratio;

        match self.sliding_state{
            SlidingState::Enabled { time_since_last_slide } => {
                let mut time_since_last_slide_ms = time_since_last_slide.elapsed().unwrap().as_millis() as f64;

                if time_since_last_slide_ms >= TIME_ELAPSED_BETWEEN_TWO_NEW_LINES_MS {
                    time_since_last_slide_ms = TIME_ELAPSED_BETWEEN_TWO_NEW_LINES_MS;
                }

                sliding_ratio = (time_since_last_slide_ms as f64)/(TIME_ELAPSED_BETWEEN_TWO_NEW_LINES_MS as f64);
                
            },
            SlidingState::Disabled => sliding_ratio = 0.0, 
        }

        self.render_sliding_map(c, g, sliding_ratio);
    }
    
    fn get_head_mark_color_with_id(id : crate::backend::heads::Id) -> [f32;4]{
        match id {
            0 => config::map::tiles::HEAD_MARK_COLOR_0,
            1 => config::map::tiles::HEAD_MARK_COLOR_1,
            2 => config::map::tiles::HEAD_MARK_COLOR_2,
            _ => config::map::tiles::HEAD_MARK_COLOR_3 
        }
    }

    fn get_head_color_with_id(id : crate::backend::heads::Id) -> [f32;4]{
        match id {
            0 => config::map::tiles::HEAD_COLOR_0,
            1 => config::map::tiles::HEAD_COLOR_1,
            2 => config::map::tiles::HEAD_COLOR_2,
            _ => config::map::tiles::HEAD_COLOR_3 
        }
    }

    fn render_sliding_map(&mut self, c: &Context, g: &mut G2d, sliding_ratio: f64) {

        let tile_height = (config::map::END_Y - config::map::ORIGIN_Y)/  (self.map_nb_visible_lines as f64);
        let tile_length = (config::map::END_X - config::map::ORIGIN_X)/ (self.get_length() as f64);

        let mut draw_tile = |x_origin, y_origin, new_tile_height, tile_type,  c: &Context, g: &mut G2d|{
            let x_end = x_origin + tile_length;
            let y_end = y_origin + new_tile_height;

            let tile_color;
            match tile_type {
                TileType::Free => tile_color = config::map::tiles::FREE_COLOR,
                TileType::Separator => tile_color = config::map::tiles::SEPARATOR_COLOR,
                TileType::Wall => tile_color = config::map::tiles::WALL_COLOR,
                TileType::Marked{id} => tile_color = Self::get_head_mark_color_with_id(id),
                TileType::Head{id} => tile_color = Self::get_head_color_with_id(id)
            }

            rectangle_from_to(tile_color, 
                    [x_origin, y_origin],[x_end, y_end],
                    c.transform, g);
        };

        let mut y_origin = config::map::ORIGIN_Y;
        let first_tile_line_height = sliding_ratio * tile_height;
        let last_tile_line_height = (1.0 -sliding_ratio) * tile_height;

        for line_index in (0..=self.map_nb_visible_lines).rev() {
            let line_of_tiles = &self.sto[line_index];
            let mut x_origin = config::map::ORIGIN_X;

            // First line
            if line_index == self.map_nb_visible_lines {

                for  tile_type in line_of_tiles.iter(){
                    draw_tile(x_origin, y_origin, first_tile_line_height, *tile_type, c, g);
                    x_origin+=tile_length;
                }
                self.render_arrival_line(line_index, first_tile_line_height, y_origin, c, g);
                y_origin += first_tile_line_height;
            }
            // Last line
            else if  line_index == 0{

                for  tile_type in line_of_tiles.iter(){
                    draw_tile(x_origin, y_origin, last_tile_line_height, *tile_type, c, g);
                    x_origin+=tile_length;
                }
                self.render_arrival_line(line_index, last_tile_line_height, y_origin, c, g);

            }
            // Others
            else{

                for  tile_type in line_of_tiles.iter(){
                    draw_tile(x_origin, y_origin, tile_height, *tile_type, c, g);
                    x_origin+=tile_length;
                }
                self.render_arrival_line(line_index, tile_height, y_origin, c, g);
                y_origin += tile_height;

            }


        }
    }

    pub fn render_arrival_line(&mut self, line_index: usize, tile_height : f64, y_origin : f64,  c: &Context, g: &mut G2d){

        let y_line  = self.get_height() - self.map_nb_visible_lines;

        if y_line <= self.map_nb_visible_lines {
            if y_line == line_index {
                rectangle_from_to(config::map::arrival_line::COLOR, 
                    [config::map::ORIGIN_X, y_origin],[config::map::END_X, y_origin + tile_height],
                    c.transform, g);
            }

        }
    }

    pub fn render(&mut self, c: &Context, g: &mut G2d){

        self.render_tiles(c, g);

        super::utils::draw_frame([config::map::ORIGIN_X, config::map::ORIGIN_Y, config::map::END_X,  config::map::END_Y], config::map::frame::BAR_COLOR, config::map::frame::BAR_WIDTH, c, g );
    }

    pub fn start_sliding(&mut self){
        self.sliding_state = SlidingState::Enabled{time_since_last_slide: SystemTime::now()};

    }

    pub fn slide(&mut self){
        self.sto.pop_front();
        self.sliding_state = SlidingState::Enabled{time_since_last_slide: SystemTime::now()};

    }



    
}
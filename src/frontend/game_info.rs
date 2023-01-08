use piston_window::{Context, G2d,line, color};

use crate::utils::Direction;
use super::config;

pub struct GameInfoGfx{
    direction : Option<Direction>
}

pub enum Event{
    UpdateCurrentUserDir
    //UPDATE_NB_HEADS,
    //UPDATE_TIME_ELAPSED
}

impl GameInfoGfx{

    pub fn new() -> GameInfoGfx{
        GameInfoGfx{direction: None}
    }

    fn render_frame(&mut self, c: &Context, g: &mut G2d){
         // Draw borders
         line(color::BLACK, config::game_info::BAR_WIDTH,  [config::game_info::ORIGIN_X, config::game_info::ORIGIN_Y, config::game_info::ORIGIN_X,  config::game_info::END_Y], c.transform, g);
         line(color::BLACK, config::game_info::BAR_WIDTH,  [config::game_info::ORIGIN_X, config::game_info::END_Y, config::game_info::END_X,  config::game_info::END_Y], c.transform, g);
         line(color::BLACK, config::game_info::BAR_WIDTH,  [config::game_info::END_X, config::game_info::END_Y, config::game_info::END_X,  config::game_info::ORIGIN_Y], c.transform, g);
         line(color::BLACK, config::game_info::BAR_WIDTH,  [config::game_info::END_X, config::game_info::ORIGIN_Y, config::game_info::ORIGIN_X,  config::game_info::ORIGIN_Y], c.transform, g);
    }
    pub fn render(&mut self, c: &Context, g: &mut G2d){
        self.render_frame(c, g);
    }

    pub fn set_user_direction(&mut self, direction: Option<Direction>){
        self.direction = direction;}

}
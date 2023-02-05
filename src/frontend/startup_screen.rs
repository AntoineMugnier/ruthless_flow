use piston_window::{Glyphs, Context, G2d, color::WHITE, rectangle_from_to};
use crate::frontend::config;

pub struct StartupScreen{

}

impl StartupScreen{
    pub fn new() -> StartupScreen{
        StartupScreen{}
    }

    pub fn render(&mut self, glyph_cache : &mut Glyphs, c: &Context, g: &mut G2d){
        rectangle_from_to(config::startup_screen::BACKGROUND_COLOR, 
            [config::startup_screen::ORIGIN_X, config::startup_screen::ORIGIN_Y],[config::startup_screen::END_X, config::startup_screen::END_Y],
            c.transform, g);
    }

}
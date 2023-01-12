use piston_window::{Context, G2d,line, color, text, DrawState, Transformed, glyph_cache::rusttype::GlyphCache, TextureSettings, Glyphs, Flip, Texture, G2dTexture, G2dTextureContext, image, rectangle::square, Image};

use crate::utils::Direction;
use super::config;

pub struct GameInfoGfx{
    direction : Direction,
}

pub enum Event{
    UpdateCurrentUserDir
    //UPDATE_NB_HEADS,
    //UPDATE_TIME_ELAPSED
}

impl GameInfoGfx{

    pub fn new() -> GameInfoGfx{

        GameInfoGfx{direction: Direction::Up}
    }

    fn render_frame(&mut self, c: &Context, g: &mut G2d){
         // Draw borders
         line(color::BLACK, config::game_info::frame::BAR_WIDTH,  [config::game_info::ORIGIN_X, config::game_info::ORIGIN_Y, config::game_info::ORIGIN_X,  config::game_info::END_Y], c.transform, g);
         line(color::BLACK, config::game_info::frame::BAR_WIDTH,  [config::game_info::ORIGIN_X, config::game_info::END_Y, config::game_info::END_X,  config::game_info::END_Y], c.transform, g);
         line(color::BLACK, config::game_info::frame::BAR_WIDTH,  [config::game_info::END_X, config::game_info::END_Y, config::game_info::END_X,  config::game_info::ORIGIN_Y], c.transform, g);
         line(color::BLACK, config::game_info::frame::BAR_WIDTH,  [config::game_info::END_X, config::game_info::ORIGIN_Y, config::game_info::ORIGIN_X,  config::game_info::ORIGIN_Y], c.transform, g);
    }

    fn draw_arrow(x: f64, y : f64, direction: Direction,  glyph_cache: &mut Glyphs, c: &Context, g: &mut G2d){
        let transform = c.transform.trans(x, y);
        
        let arrow_char : char;

        //Choose the proper unicode arrow character depending on selected dir
        match direction{
            Direction::Up =>arrow_char = '\u{2191}',
            Direction::Down =>arrow_char = '\u{2193}',
            Direction::Right =>arrow_char ='\u{2192}',
            Direction::Left => arrow_char ='\u{2190}',
        }
    
        text::Text::new_color(color::BLACK, config::game_info::dir::arrow::FONT_SIZE).draw(&arrow_char.to_string(),
        glyph_cache,
        &c.draw_state,
        transform,
        g).unwrap();
    }

    fn render_user_direction(&mut self, glyph_cache : &mut Glyphs,    c: &Context, g: &mut G2d){
        
        let mut draw_str = |str, x, y|{
        let transform = c.transform.trans(x, y);
        
        text::Text::new_color(color::BLACK, config::game_info::dir::text::FONT_SIZE).draw(str,
        glyph_cache,
        &c.draw_state,
        transform,
        g).unwrap();
        };
        
        let direction_str = "DIR:";
        draw_str(direction_str, config::game_info::dir::text::ORIGIN_X , config::game_info::dir::text::ORIGIN_Y);

        Self::draw_arrow(config::game_info::dir::arrow::ORIGIN_X , config::game_info::dir::arrow::ORIGIN_Y, self.direction,  glyph_cache, c, g);
    }

    pub fn render(&mut self, glyph_cache : &mut Glyphs, c: &Context, g: &mut G2d){
        self.render_frame(c, g);
        self.render_user_direction(glyph_cache,c, g);
    }

    pub fn set_user_direction(&mut self, direction:Direction){
        self.direction = direction;}

}
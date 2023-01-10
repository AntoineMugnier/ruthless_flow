use piston_window::{Context, G2d,line, color, text, DrawState, Transformed, glyph_cache::rusttype::GlyphCache, TextureSettings, Glyphs, Flip, Texture, G2dTexture, G2dTextureContext, image, rectangle::square, Image};

use crate::utils::Direction;
use super::config;

pub struct GameInfoGfx{
    direction : Direction,
    arrow_logo : G2dTexture
}

pub enum Event{
    UpdateCurrentUserDir
    //UPDATE_NB_HEADS,
    //UPDATE_TIME_ELAPSED
}

impl GameInfoGfx{

    pub fn new(texture_context : &mut G2dTextureContext) -> GameInfoGfx{

    // Create the arrow texture
    let arrow_logo: G2dTexture = Texture::from_path(
        texture_context,
        config::assets::UP_ARROW_PATH,
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

        GameInfoGfx{direction: Direction::Up, arrow_logo}
    }

    fn render_frame(&mut self, c: &Context, g: &mut G2d){
         // Draw borders
         line(color::BLACK, config::game_info::frame::BAR_WIDTH,  [config::game_info::ORIGIN_X, config::game_info::ORIGIN_Y, config::game_info::ORIGIN_X,  config::game_info::END_Y], c.transform, g);
         line(color::BLACK, config::game_info::frame::BAR_WIDTH,  [config::game_info::ORIGIN_X, config::game_info::END_Y, config::game_info::END_X,  config::game_info::END_Y], c.transform, g);
         line(color::BLACK, config::game_info::frame::BAR_WIDTH,  [config::game_info::END_X, config::game_info::END_Y, config::game_info::END_X,  config::game_info::ORIGIN_Y], c.transform, g);
         line(color::BLACK, config::game_info::frame::BAR_WIDTH,  [config::game_info::END_X, config::game_info::ORIGIN_Y, config::game_info::ORIGIN_X,  config::game_info::ORIGIN_Y], c.transform, g);
    }

    fn render_user_direction(&mut self, glyph_cache : &mut Glyphs,    c: &Context, g: &mut G2d){
        
        let mut draw_str = |str, x, y|{
        let transform = c.transform.trans(x, y);
        text::Text::new_color(color::BLACK, 12).draw(str,
        glyph_cache,
        &c.draw_state,
        transform,
        g).unwrap();
        };
        
        draw_str("DIR: ", config::game_info::dir::TEXT_ORIGIN_X , config::game_info::dir::TEXT_ORIGIN_Y);



        // Create the image object and attach a square Rectangle object inside.
        let image= Image::new().rect(square(0.0, 0.0, 200.0));
        
        // A texture to use with the image
        //Draw the image with the texture
 		image.draw(&self.arrow_logo, &c.draw_state, c.transform, g);


    }

    pub fn render(&mut self, glyph_cache : &mut Glyphs, c: &Context, g: &mut G2d){
        self.render_frame(c, g);
        self.render_user_direction(glyph_cache,c, g);
    }

    pub fn set_user_direction(&mut self, direction:Direction){
        self.direction = direction;}

}
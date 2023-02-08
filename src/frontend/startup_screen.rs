use piston_window::{Glyphs, Context, G2d, text::Text,  rectangle_from_to, Transformed};
use crate::frontend::config;

pub struct StartupScreen{

}
const RULES_TITLE : &str = "The Rules";
const RULES_DESC : [&str; 14] = [
"In this game, you control the path of a liquid flowing unstoppably in a network of pipes. Your view",
"on the map of the pipe network is constantly sliding downward . Your goal as the player is to make",
"all of your flow heads reach the end of the map while avoiding one of your head of being caught",
"by the rising edge. Your heads may encounter \"separators\" when naviguing into the pipes.",
"These blue tiles will split your flowinto two child flows, each with its own head. You may avoid ",
"having more than one head by merging one head into another one flow. You cannot merge a head in",
"its own flow",
" ",
"You can control the directions that can simultaneously be taken by all of your flows using your",
"keyboard directional arrows.",
" ",
"Good luck!",
" ",
"                                                                         PRESS ENTER TO CONTINUE"];


impl StartupScreen{
    pub fn new() -> StartupScreen{
        StartupScreen{}
    }

    pub fn render_title(&mut self, glyph_cache : &mut Glyphs, c: &Context, g: &mut G2d){
        let transform = c.transform.trans(config::startup_screen::title::ORIGIN_X, config::startup_screen::title::ORIGIN_Y);
        Text::new_color(config::startup_screen::title::FONT_COLOR, config::startup_screen::title::FONT_SIZE).draw(RULES_TITLE,
        glyph_cache,
        &c.draw_state,
        transform,
        g).unwrap();
    }

    pub fn render_description(&mut self, glyph_cache : &mut Glyphs, c: &Context, g: &mut G2d){
        let draw_text =|text, origin_x, origin_y, glyph_cache : &mut Glyphs,  c : &Context, g: &mut G2d | {
            let transform = c.transform.trans(origin_x, origin_y);
            Text::new_color(config::startup_screen::description::FONT_COLOR, config::startup_screen::description::FONT_SIZE).draw(text,
            glyph_cache,
            &c.draw_state,
            transform,
            g).unwrap();
        };

        let mut origin_y = config::startup_screen::description::ORIGIN_Y;

        for line in RULES_DESC.iter(){
            draw_text(line, config::startup_screen::description::ORIGIN_X, origin_y, glyph_cache, c , g);
            origin_y +=  config::startup_screen::description::FONT_SIZE as f64 *2.0;
        }
    }


    pub fn render(&mut self, glyph_cache : &mut Glyphs, c: &Context, g: &mut G2d){
        rectangle_from_to(config::startup_screen::BACKGROUND_COLOR, 
            [config::startup_screen::ORIGIN_X, config::startup_screen::ORIGIN_Y],[config::startup_screen::END_X, config::startup_screen::END_Y],
            c.transform, g);

        super::utils::draw_frame([config::startup_screen::ORIGIN_X, config::startup_screen::ORIGIN_Y, config::startup_screen::END_X,  config::startup_screen::END_Y], config::startup_screen::frame::BAR_COLOR, config::startup_screen::frame::BAR_WIDTH, c, g );

        self.render_description(glyph_cache, c, g);
        
        self.render_title(glyph_cache, c, g);
    }

}
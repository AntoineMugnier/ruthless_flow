use piston_window::{Glyphs, Context, G2d, rectangle_from_to, Transformed, Text};

use crate::{frontend::config, backend::board::EndGameReason};

pub struct EndGameBox{
end_game_reason : Option<EndGameReason>,
end_game_title : Option<String>,
end_game_description : Option<String>
}

impl EndGameBox{
    pub fn new() -> EndGameBox{
        EndGameBox{end_game_reason : None, end_game_title : None, end_game_description : None}
    }

    pub fn update_end_game_reason(&mut self, end_game_reason : EndGameReason){
        self.end_game_reason = Some(end_game_reason);
        match end_game_reason{
            EndGameReason::Victory => {
                self.end_game_title = Some(String::from("VICTORY !"));
                self.end_game_description = Some(String::from("One of your heads has reached the arrival line"));
            },
            //May not be relevant
            EndGameReason::NoRemainingHeads => {
                self.end_game_title = Some(String::from("DEFEAT !"));
                self.end_game_description = Some(String::from("Your have no more heads"));

            }
            EndGameReason::HeadPoppedOutByRisingEdge =>{
                self.end_game_title = Some(String::from("DEFEAT !"));
                self.end_game_description = Some(String::from("The rising edge has killed one of your heads"));

            },
        }
    }
    
    pub fn render_title(&mut self, glyph_cache : &mut Glyphs, c: &Context, g: &mut G2d){
        let draw_text =|text, origin_x, origin_y, glyph_cache : &mut Glyphs,  c : &Context, g: &mut G2d | {
            let transform = c.transform.trans(origin_x, origin_y);
            Text::new_color(config::end_game_box::title::FONT_COLOR, config::end_game_box::title::FONT_SIZE).draw(text,
            glyph_cache,
            &c.draw_state,
            transform,
            g).unwrap();
        };
        draw_text(self.end_game_title.as_ref().unwrap().as_str(), config::end_game_box::title::ORIGIN_X, config::end_game_box::title::ORIGIN_Y, glyph_cache, c , g);
    }
    
    pub fn render_description(&mut self, glyph_cache : &mut Glyphs, c: &Context, g: &mut G2d){
        let draw_text =|text, origin_x, origin_y, glyph_cache : &mut Glyphs,  c : &Context, g: &mut G2d | {
            let transform = c.transform.trans(origin_x, origin_y);
            Text::new_color(config::end_game_box::description::FONT_COLOR, config::end_game_box::description::FONT_SIZE).draw(text,
            glyph_cache,
            &c.draw_state,
            transform,
            g).unwrap();
        };
        
        draw_text(self.end_game_description.as_ref().unwrap().as_str(), config::end_game_box::description::ORIGIN_X, config::end_game_box::description::ORIGIN_Y, glyph_cache, c , g);
    }

    pub fn render(&mut self, glyph_cache : &mut Glyphs, c: &Context, g: &mut G2d){

        rectangle_from_to(config::end_game_box::BACKGROUND_COLOR, 
            [config::end_game_box::ORIGIN_X, config::end_game_box::ORIGIN_Y],[config::end_game_box::END_X, config::end_game_box::END_Y],
            c.transform, g);

        super::utils::draw_frame([config::end_game_box::ORIGIN_X, config::end_game_box::ORIGIN_Y, config::end_game_box::END_X,  config::end_game_box::END_Y], config::end_game_box::frame::BAR_COLOR, config::end_game_box::frame::BAR_WIDTH, c, g );
                
        self.render_title(glyph_cache, c, g);
        self.render_description(glyph_cache, c, g);


    }
}
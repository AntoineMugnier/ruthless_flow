use piston_window::{Context, G2d, text, Transformed, Glyphs, rectangle_from_to};
use std::time::{SystemTime, Duration};

use crate::utils::Direction;
use super::config;

pub struct GameInfoGfx{
    direction : Direction,
    timer_state : TimerState,
    nb_heads : usize
}
pub enum TimerState{
    Enabled{init_time : SystemTime},
    Frozen{time_elapsed_when_frozen : Duration},
    Disabled
}
impl GameInfoGfx{

    pub fn new() -> GameInfoGfx{

        GameInfoGfx{direction: Direction::Up, timer_state : TimerState::Disabled, nb_heads: 0}
    }

    pub fn start_timer(&mut self){
        self.timer_state = TimerState::Enabled{init_time: SystemTime::now()};
    }

    pub fn freeze_timer(&mut self) {
        if let TimerState::Enabled {init_time} = self.timer_state{
            self.timer_state = TimerState::Frozen { time_elapsed_when_frozen: init_time.elapsed().unwrap() };
        }
    }

    fn render_time(&mut self,  glyph_cache : &mut Glyphs, c: &Context, g: &mut G2d){

        let minutes;
        let seconds;
        let centiseconds;

        match self.timer_state{
            TimerState::Enabled { init_time } => {
                let time_elapsed = init_time.elapsed().unwrap().as_millis();
                minutes = (time_elapsed/(1000*60))%60;
                seconds = (time_elapsed/1000) % 60;
                centiseconds = (time_elapsed/10) %100;
            },
            TimerState::Frozen { time_elapsed_when_frozen } =>{
                let time_elapsed_when_frozen = time_elapsed_when_frozen.as_millis();
                minutes = (time_elapsed_when_frozen/(1000*60))%60;
                seconds = (time_elapsed_when_frozen/1000) % 60;
                centiseconds = (time_elapsed_when_frozen/10) %100;
            },
            TimerState::Disabled => {
                minutes = 0;
                seconds = 0;
                centiseconds = 0;
            },
        }
        
        let transform = c.transform.trans(config::game_info::time::ORIGIN_X, config::game_info::time::ORIGIN_Y);
        let direction_str = format!("{:02}:{:02}:{:02}", minutes , seconds, centiseconds);

        text::Text::new_color(config::game_info::FONT_COLOR, config::game_info::time::FONT_SIZE).draw(&direction_str,
        glyph_cache,
        &c.draw_state,
        transform,
        g).unwrap();
    }

    
    fn render_nb_heads(&mut self,  glyph_cache : &mut Glyphs, c: &Context, g: &mut G2d){

        let transform = c.transform.trans(config::game_info::nb_heads::ORIGIN_X, config::game_info::nb_heads::ORIGIN_Y);
        let heads_str = format!("Heads: {}", self.nb_heads);

        text::Text::new_color(config::game_info::FONT_COLOR, config::game_info::nb_heads::FONT_SIZE).draw(&heads_str,
        glyph_cache,
        &c.draw_state,
        transform,
        g).unwrap();
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
    
        text::Text::new_color(config::game_info::FONT_COLOR, config::game_info::dir::arrow::FONT_SIZE).draw(&arrow_char.to_string(),
        glyph_cache,
        &c.draw_state,
        transform,
        g).unwrap();
    }

    fn render_user_direction(&mut self, glyph_cache : &mut Glyphs, c: &Context, g: &mut G2d){
        
        let mut draw_str = |str, x, y|{
        let transform = c.transform.trans(x, y);
        
        text::Text::new_color(config::game_info::FONT_COLOR, config::game_info::dir::text::FONT_SIZE).draw(str,
        glyph_cache,
        &c.draw_state,
        transform,
        g).unwrap();
        };
        
        let direction_str = "Dir:";
        draw_str(direction_str, config::game_info::dir::text::ORIGIN_X , config::game_info::dir::text::ORIGIN_Y);

        Self::draw_arrow(config::game_info::dir::arrow::ORIGIN_X , config::game_info::dir::arrow::ORIGIN_Y, self.direction,  glyph_cache, c, g);
    }

    pub fn update_nb_heads(&mut self, nb_heads : usize){
        self.nb_heads = nb_heads;
    }

    pub fn render(&mut self, glyph_cache : &mut Glyphs, c: &Context, g: &mut G2d){
        rectangle_from_to(config::game_info::BACKGROUND_COLOR, 
            [config::game_info::ORIGIN_X, config::game_info::ORIGIN_Y],[config::game_info::END_X, config::game_info::END_Y],
            c.transform, g);

        super::utils::draw_frame([config::game_info::ORIGIN_X, config::game_info::ORIGIN_Y, config::game_info::END_X,  config::game_info::END_Y], config::game_info::frame::BAR_COLOR, config::game_info::frame::BAR_WIDTH, c, g );
        self.render_user_direction(glyph_cache,c, g);
        self.render_time(glyph_cache, c, g);
        self.render_nb_heads(glyph_cache, c, g);
    }

    pub fn set_user_direction(&mut self, direction:Direction){
        self.direction = direction;
    }

}
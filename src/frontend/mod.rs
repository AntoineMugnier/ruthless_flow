extern crate piston_window;
pub mod gfx_map;
mod utils;
pub mod game_info;
mod startup_screen;
pub mod config;
mod end_game_box;
use crate::{utils::{Coordinates, Direction}, backend::{self, board::EndGameReason}};
use piston_window::{*, glyph_cache::rusttype::GlyphCache};

use crate::{mpsc::{Sender, Receiver}, backend::{board, map::TileType}};

use self::{gfx_map::GfxMap, game_info::GameInfoGfx, startup_screen::StartupScreen, end_game_box::EndGameBox};

enum FrontendState{
    Startup,
    Playing,
    Ending
}

#[derive(Debug)]
pub enum Event {
    NewMapLine,
    SetTile{position: Coordinates, tile_type: TileType},
    UserDirSet{direction : Direction},
    UpdateNbHeads{nb_heads: usize},
    EndGame{game_end_reason: board::EndGameReason}
}
pub struct Frontend{
    window: PistonWindow,
    glyphs: Glyphs,
    gfx_map : GfxMap,
    game_info_gfx : GameInfoGfx,
    startup_screen : StartupScreen,
    end_game_box : EndGameBox,
    backend_event_sender: Sender<board::Event>,
    frontend_event_receiver: Receiver<Event>,
    current_game_stage : FrontendState
}
impl Frontend{

    pub fn new(backend_event_sender: Sender<board::Event>, frontend_event_receiver: Receiver<Event>, gfx_map : GfxMap) -> Frontend{
        
        let mut window: PistonWindow = 
            WindowSettings::new("Ruthless Flow", config::SCREEN_SIZE)
            .exit_on_esc(true)
            .graphics_api(OpenGL::V3_2)
            .build().unwrap();
        let glyphs = window.load_font(config::assets::FONTS_PATH).unwrap();
        let game_info_gfx = GameInfoGfx::new();
        let end_game_box = EndGameBox::new();
        let current_game_stage = FrontendState::Startup;
        let startup_screen = StartupScreen::new();
        Frontend {window, glyphs, gfx_map, game_info_gfx, startup_screen,end_game_box, backend_event_sender, frontend_event_receiver, current_game_stage}
    }

    pub fn render_title( glyph_cache : &mut Glyphs, c: &Context, g: &mut G2d){
        let transform = c.transform.trans(config::title::ORIGIN_X, config::title::ORIGIN_Y);
        let title = "Ruthless Flow";
        text::Text::new_color(config::title::FONT_COLOR, config::title::FONT_SIZE).draw(&title,
        glyph_cache,
        &c.draw_state,
        transform,
        g).unwrap();

    }
    
    pub fn trigger_game_ending_screen(&mut self){
        self.current_game_stage = FrontendState::Ending;
    }
    
    pub fn startup_state_handler(&mut self, e: impl GenericEvent ) {
            
            if let Some(args) = e.render_args() {
                self.window.draw_2d(&e, |c, g, device| {

                Self::draw_game_board(&mut self.gfx_map, &mut self.game_info_gfx ,&mut self.glyphs, &c, g);

                self.startup_screen.render(&mut self.glyphs, &c, g);

                self.glyphs.factory.encoder.flush(device);
            });
        }
    
        
        if let Some(args) = e.press_args() {
            match args {
                Button::Keyboard(key) => {
                    match key {
                        Key::Return =>{
                            self.start_game();
                            return
                        },
                        _ => ()
                    }
                }
                _ => ()
        }
    }
}

    fn start_game(&mut self) {
        self.gfx_map.start_sliding();

        // Inform backend that a game has started
        let event = backend::board::Event::StartGame;
        self.backend_event_sender.send(event).unwrap();

        // Start the playing time chrono printed on the game info side pannel
        self.game_info_gfx.start_timer();
        
        self.current_game_stage = FrontendState::Playing;
    }

    pub fn playing_state_handler(&mut self, e : impl GenericEvent ) {
            
            if let Some(args) = e.render_args() {
                self.window.draw_2d(&e, |c, g, device| {
                
                // The board is always drawn
                Self::draw_game_board(&mut self.gfx_map, &mut self.game_info_gfx ,&mut self.glyphs, &c, g);

                self.glyphs.factory.encoder.flush(device);
                });
            }
            
    
            if let Some(args) = e.press_args() {
                match args {
                    Button::Keyboard(key) => {
                        match key {
                            Key::Left =>{
                                self.send_next_direction(Direction::Left);
                            },
                            Key::Right =>{
                                self.send_next_direction(Direction::Right);
                            },
                            Key::Up =>{
                                self.send_next_direction(Direction::Up);
                            },
                            Key::Down =>{
                                self.send_next_direction(Direction::Down);
                            }
                            _ => ()
                        }
                    },
                    _ => ()
                }
            }
    }

    pub fn ending_state_handler(&mut self, e: impl GenericEvent ) {
        if let Some(args) = e.render_args() {
            self.window.draw_2d(&e, |c, g, device| {
            
                // The board is always drawn
                Self::draw_game_board(&mut self.gfx_map, &mut self.game_info_gfx ,&mut self.glyphs, &c, g);
                self.end_game_box.render(&mut self.glyphs,  &c, g);
                self.glyphs.factory.encoder.flush(device);

            });
        }
    }

    fn draw_game_board(gfx_map: &mut GfxMap, game_info_gfx : &mut GameInfoGfx, glyphs: &mut Glyphs, c: &Context, g: &mut G2d) {
        clear(config::BACKGROUND_COLOR, g);
        Self::render_title(glyphs,  &c, g);
        gfx_map.render(&c, g);
        game_info_gfx.render(glyphs,  &c, g);
    }

    fn update_model(&mut self, e: &impl GenericEvent){
        if let Some(args) = e.update_args() {
            while let Ok(evt) = self.frontend_event_receiver.try_recv() {
                match evt {
                    Event::NewMapLine{} =>{
                        self.gfx_map.slide();
                    },
                    Event::SetTile { position, tile_type } =>{
                        self.gfx_map.set_tile(position, tile_type);
                    }
                    Event::UserDirSet { direction } => {
                        self.game_info_gfx.set_user_direction(direction);
                    }, 
                    Event::UpdateNbHeads { nb_heads } => {
                        self.game_info_gfx.update_nb_heads(nb_heads);
                    },
                    Event::EndGame { game_end_reason } => {
                        self.end_game_box.update_end_game_reason(game_end_reason);
                        // Freeze time on the game info side pannel
                        self.game_info_gfx.freeze_timer();
                        self.trigger_game_ending_screen(); return }, 
                }
            }
        }
    }

    pub fn run(&mut self) {
        while let Some(e) = self.window.next() {
            
            self.update_model(&e);

            match self.current_game_stage{
                FrontendState::Startup => self.startup_state_handler(e),
                FrontendState::Playing => self.playing_state_handler(e),
                FrontendState::Ending => self.ending_state_handler(e),
            }
    }
}


fn send_next_direction(&mut self, direction : Direction){

    let event = backend::board::Event::SetNextHeadDir { direction};
    self.backend_event_sender.send(event).unwrap();
    
}
}
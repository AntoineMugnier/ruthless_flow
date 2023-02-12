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

enum GameStage{
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
    texture_context : G2dTextureContext,
    gfx_map : GfxMap,
    game_info_gfx : GameInfoGfx,
    startup_screen : StartupScreen,
    end_game_box : EndGameBox,
    backend_event_sender: Sender<board::Event>,
    frontend_event_receiver: Receiver<Event>,
    current_game_stage : GameStage
}
impl Frontend{

    pub fn new(backend_event_sender: Sender<board::Event>, frontend_event_receiver: Receiver<Event>, gfx_map : GfxMap) -> Frontend{
        
        let mut window: PistonWindow = 
            WindowSettings::new("Ruthless Flow", config::SCREEN_SIZE)
            .exit_on_esc(true)
            .graphics_api(OpenGL::V3_2)
            .build().unwrap();
        let glyphs = window.load_font(config::assets::FONTS_PATH).unwrap();
        let texture_context = window.create_texture_context();
        let game_info_gfx = GameInfoGfx::new();
        let end_game_box = EndGameBox::new();
        let current_game_stage = GameStage::Startup;
        let startup_screen = StartupScreen::new();
        Frontend {window, glyphs, texture_context, gfx_map, game_info_gfx, startup_screen,end_game_box, backend_event_sender, frontend_event_receiver, current_game_stage}
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
    
    pub fn trigger_game_ending_screen(&mut self, game_end_reason: EndGameReason ){
        println!("{:?}", game_end_reason);
        self.current_game_stage = GameStage::Ending;
    }
    
    pub fn handle_startup(&mut self, e: impl GenericEvent ) {
            
            if let Some(args) = e.render_args() {
                self.window.draw_2d(&e, |c, g, device| {
                
                // The board is always drawn
                clear(config::BACKGROUND_COLOR, g);
                Self::render_title(&mut self.glyphs,  &c, g);
                self.gfx_map.render(&c, g);
                self.startup_screen.render(&mut self.glyphs, &c, g);
                self.glyphs.factory.encoder.flush(device);
            });
        }
        
            
            if let Some(args) = e.update_args() {
                while let Ok(evt) = self.frontend_event_receiver.try_recv() {
                    match evt {
                        Event::SetTile { position, tile_type } =>{
                            self.gfx_map.set_tile(position, tile_type);
                        },
                        _ =>{}
                }
            }
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

        // Inform backend that a game is started
        let event = backend::board::Event::StartGame;
        self.backend_event_sender.send(event).unwrap();

        self.current_game_stage = GameStage::Playing;
    }

    pub fn handle_running_game(&mut self, e : impl GenericEvent ) {
        
            
            if let Some(args) = e.render_args() {
                self.window.draw_2d(&e, |c, g, device| {
                
                // The board is always drawn
                clear(config::BACKGROUND_COLOR, g);
                Self::render_title(&mut self.glyphs,  &c, g);
                self.gfx_map.render(&c, g);
                self.game_info_gfx.render(&mut self.glyphs,  &c, g);
                self.glyphs.factory.encoder.flush(device);
                });
            }
            
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
                            self.trigger_game_ending_screen(game_end_reason); return }, 
                    }
                }
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

    pub fn handle_ending_game(&mut self, e: impl GenericEvent ) {
        if let Some(args) = e.render_args() {
            self.window.draw_2d(&e, |c, g, device| {
            
            // The board is always drawn
            clear(config::BACKGROUND_COLOR, g);
            Self::render_title(&mut self.glyphs,  &c, g);
            self.gfx_map.render(&c, g);
            self.game_info_gfx.render(&mut self.glyphs,  &c, g);
            self.end_game_box.render(&mut self.glyphs,  &c, g);
            self.glyphs.factory.encoder.flush(device);
            });
        }
    }

    pub fn run(&mut self) {
        while let Some(e) = self.window.next() {

        match self.current_game_stage{
            GameStage::Startup => self.handle_startup(e),
            GameStage::Playing => self.handle_running_game(e),
            GameStage::Ending => self.handle_ending_game(e),
        }
    }
        

}

fn send_next_direction(&mut self, direction : Direction){
    let event = backend::board::Event::SetNextHeadDir { direction};
    self.backend_event_sender.send(event).unwrap();
    
}
}
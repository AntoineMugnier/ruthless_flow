extern crate piston_window;
pub mod gfx_map;
pub mod game_info;
pub mod config;
use crate::{utils::{Coordinates, Direction}, backend::{self, board::EndGameReason}};
use piston_window::{*, glyph_cache::rusttype::GlyphCache};

use crate::{mpsc::{Sender, Receiver}, backend::{board, map::TileType}};

use self::{gfx_map::GfxMap, game_info::GameInfoGfx};

#[derive(Debug)]
pub enum Event {
    NewMapLine{line : Vec<TileType>},
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
    backend_event_sender: Sender<board::Event>,
    frontend_event_receiver: Receiver<Event>,

}
impl Frontend{

    pub fn new(backend_event_sender: Sender<board::Event>, frontend_event_receiver: Receiver<Event>, map_nb_visible_lines : usize) -> Frontend{
        
        let mut window: PistonWindow = 
            WindowSettings::new("Ruthless Flow", config::SCREEN_SIZE)
            .exit_on_esc(true)
            .graphics_api(OpenGL::V3_2)
            .build().unwrap();
        let glyphs = window.load_font(config::assets::FONTS_PATH).unwrap();
        let mut texture_context = window.create_texture_context();
        let game_info_gfx = GameInfoGfx::new();
        let gfx_map = GfxMap::new(map_nb_visible_lines);
        Frontend {window, glyphs, texture_context, gfx_map, game_info_gfx, backend_event_sender, frontend_event_receiver,}
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
    
    pub fn end_game(&mut self, game_end_reason: EndGameReason ){
        println!("{:?}", game_end_reason);
    }

    pub fn run(&mut self) {
        
        while let Some(e) = self.window.next() {
            
            if let Some(args) = e.render_args() {
                
                self.window.draw_2d(&e, |c, g, device| {
                    clear(config::BACKGROUND_COLOR, g);
                    self.gfx_map.render(&c, g);
                    self.game_info_gfx.render(&mut self.glyphs,  &c, g);
                    Self::render_title(&mut self.glyphs,  &c, g);
                    self.glyphs.factory.encoder.flush(device);

                });
            }
            
            if let Some(args) = e.update_args() {
                while let Ok(evt) = self.frontend_event_receiver.try_recv() {
                    match evt {
                        Event::NewMapLine{line} =>{
                            self.gfx_map.add_line(line);
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
                        Event::EndGame { game_end_reason } => {self.end_game(game_end_reason); return }, 
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

}

fn send_next_direction(&mut self, direction : Direction){
    let event = backend::board::Event::SetNextHeadDir { direction};
    self.backend_event_sender.send(event).unwrap();
    
}
}
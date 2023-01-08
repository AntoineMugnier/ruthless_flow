extern crate piston_window;
pub mod gfx_map;
mod config;
use crate::{utils::{Coordinates, Direction}, backend};
use piston_window::*;

use crate::{mpsc::{Sender, Receiver}, backend::{board, map::TileType}};

use self::gfx_map::GfxMap;

pub enum Event {
    NewMapLine{line : Vec<TileType>},
    SetTile{position: Coordinates, tile_type: TileType}
}
pub struct Frontend{
    window: PistonWindow,
    gfx_map : GfxMap,
    backend_event_sender: Sender<board::Events>,
    frontend_event_receiver: Receiver<Event>,

}
impl Frontend{

    pub fn new(gfx_map : GfxMap, backend_event_sender: Sender<board::Events>, frontend_event_receiver: Receiver<Event>
    ) -> Frontend{
        
        let mut window: PistonWindow = 
            WindowSettings::new("Ruthless Flow", config::SCREEN_SIZE)
            .exit_on_esc(true).build().unwrap();

        Frontend {window, gfx_map, backend_event_sender, frontend_event_receiver}
    }

    pub fn run(&mut self) {
        
        while let Some(e) = self.window.next() {
            
            if let Some(ref args) = e.render_args() {
                self.window.draw_2d(&e, |c, g, _device| {
                    self.gfx_map.render(&c, g);
                });
            }
            
            if let Some(ref args) = e.update_args() {
                while let Ok(evt) = self.frontend_event_receiver.try_recv() {
                    match evt {
                        Event::NewMapLine{line} =>{
                            self.gfx_map.add_line(line);
                        },
                        Event::SetTile { position, tile_type } =>{
                            self.gfx_map.set_tile(position, tile_type);
                        } 
                    }
                }
            }
    
            if let Some(ref args) = e.press_args() {
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
    let event = backend::board::Events::SetNextHeadDir { direction};
    self.backend_event_sender.send(event).unwrap();
}
}
extern crate piston_window;
pub mod gfx_map;
use std::collections::VecDeque;

use piston_window::*;

use crate::{mpsc::{Sender, Receiver}, backend::{board, map::TileType}, utils::Coordinates};

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
            WindowSettings::new("Ruthless Flow", [640, 480])
            .exit_on_esc(true).build().unwrap();

        Frontend {window, gfx_map, backend_event_sender, frontend_event_receiver}
    }

    pub fn run(&mut self) {
        
        while let Some(e) = self.window.next() {
            
            if let Some(ref args) = e.render_args() {
                self.window.draw_2d(&e, |c, g, _device| {
                    clear([1.0; 4], g);
                    rectangle([1.0, 0.0, 0.0, 1.0], // red
                              [0.0, 0.0, 100.0, 100.0],
                              c.transform, g);
                });
            }
            
            if let Some(ref args) = e.update_args() {
                while let Ok(evt) = self.frontend_event_receiver.recv() {
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
            }

            
        }
}

}
extern crate piston_window;

use piston_window::*;

use crate::{mpsc::{Sender, Receiver}, backend::board::BoardEvevents};

pub enum FrontendEvents {
    
}
pub struct Frontend{
    window: PistonWindow,
    backend_sender: Sender<BoardEvevents>,
    frontend_receiver: Receiver<FrontendEvents>,

}

impl Frontend{

    pub fn new(backend_sender: Sender<BoardEvevents>, frontend_receiver: Receiver<FrontendEvents>
    ) -> Frontend{
        
        let mut window: PistonWindow = 
            WindowSettings::new("Ruthless Flow", [640, 480])
            .exit_on_esc(true).build().unwrap();

            
        Frontend {window, backend_sender, frontend_receiver}
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
            }
    
            if let Some(ref args) = e.press_args() {
            }

            
        }
}

}
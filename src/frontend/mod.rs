extern crate piston_window;

use piston_window::*;

use crate::{mpsc::{Sender, Receiver}, backend::board};

pub enum Events {
    
}
pub struct Frontend{
    window: PistonWindow,
    backend_event_sender: Sender<board::Events>,
    frontend_event_receiver: Receiver<Events>
}
impl Frontend{

    pub fn new(backend_event_sender: Sender<board::Events>, frontend_event_receiver: Receiver<Events>
    ) -> Frontend{
        
        let mut window: PistonWindow = 
            WindowSettings::new("Ruthless Flow", [640, 480])
            .exit_on_esc(true).build().unwrap();

            
        Frontend {window, backend_event_sender, frontend_event_receiver}
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

                    }
                }
            }
    
            if let Some(ref args) = e.press_args() {
            }

            
        }
}

}
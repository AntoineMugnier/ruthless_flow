mod backend;
mod frontend;
use crate::backend::board::Board;
use backend::map::*;
use std::thread;
use frontend::Frontend;
mod mpsc;

#[cfg(not(test))]
use crate::mpsc::{channel};

#[cfg(not(test))]
fn main() {

    let (backend_sender, backend_receiver) = channel();
    let (frontend_sender, frontend_receiver) = channel();

    let backend_sender_clone = backend_sender.clone();
    thread::spawn(move || {
            let mut board: Board<Map>  = Board::new(frontend_sender, backend_sender_clone, backend_receiver);
            board.run();
        });

    //let mut board: Board<Map>  = Board::new(frontend_sender.clone(), frontend_receiver);
    //board.run();

    let mut frontend = Frontend::new(backend_sender, frontend_receiver);
    frontend.run()

        
}

mod backend;
mod frontend;
mod file_parsing;
mod utils;
use crate::backend::board::Board;
use backend::map::*;
use std::thread;
use frontend::Frontend;
use std::collections::VecDeque;
use file_parsing::read_map;
use frontend::gfx_map::GfxMap;
mod mpsc;

#[cfg(not(test))]
use crate::mpsc::{channel};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_ne!(args.len(), 1, "Missing map path argument");

    let map_path = &args[1];


    let (backend_event_sender, backend_receiver) = channel();
    let (frontend_event_sender, frontend_event_receiver) = channel();


    let sto = read_map(map_path);

    let map_nb_visible_lines : usize = ((crate::frontend::config::map::LENGTH_Y/crate::frontend::config::map::LENGTH_X) * (sto[0].len() as f64)) as usize;
    println!("{}", map_nb_visible_lines);
    let backend_event_sender_clone = backend_event_sender.clone(); // For the Board to post events to itself
    let frontend_event_sender_clone = frontend_event_sender.clone();

    let  map = Map::new(frontend_event_sender,  sto.clone(),  map_nb_visible_lines);
    
    thread::spawn(move || {
            let mut board: Board<Map>  = Board::new(map, backend_event_sender_clone, backend_receiver, frontend_event_sender_clone);
            board.run();
        });

    let mut frontend = Frontend::new(backend_event_sender, frontend_event_receiver, map_nb_visible_lines);
    frontend.run();
       
}

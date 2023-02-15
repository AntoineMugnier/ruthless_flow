mod backend;
mod frontend;
mod file_parsing;
mod utils;
use backend::map::*;
use std::thread;
use frontend::Frontend;
use file_parsing::read_map;
use frontend::gfx_map::GfxMap;
mod mpsc;

use crate::backend::Backend;
#[cfg(not(test))]
use crate::mpsc::{channel};

#[cfg(not(test))]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_ne!(args.len(), 1, "Missing map path argument");

    let map_path = &args[1];


    let (backend_event_sender, backend_receiver) = channel();
    let (frontend_event_sender, frontend_event_receiver) = channel();


    let sto = read_map(map_path);

    let map_nb_visible_lines : usize = ((crate::frontend::config::map::LENGTH_Y/crate::frontend::config::map::LENGTH_X) * (sto[0].len() as f64)) as usize;
    let backend_event_sender_clone = backend_event_sender.clone(); // For the Backend to post events to itself
    let frontend_event_sender_clone = frontend_event_sender.clone();

    let  map = Map::new(frontend_event_sender,  sto.clone(),  map_nb_visible_lines);
    let gfx_map = GfxMap::new(map_nb_visible_lines, sto);

    thread::spawn(move || {
            let mut board: Backend<Map>  = Backend::new(map, backend_event_sender_clone, backend_receiver, frontend_event_sender_clone);
            board.run();
        });

    let mut frontend = Frontend::new(backend_event_sender, frontend_event_receiver, gfx_map);
    frontend.run();
       
}

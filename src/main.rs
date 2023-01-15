mod backend;
mod frontend;
mod utils;
use crate::backend::board::Board;
use backend::map::*;
use std::thread;
use frontend::Frontend;
mod mpsc;

#[cfg(not(test))]
use crate::mpsc::{channel};

#[cfg(not(test))]
fn main() {
    use std::collections::VecDeque;

    use frontend::gfx_map::GfxMap;


    let (backend_event_sender, backend_receiver) = channel();
    let (frontend_event_sender, frontend_event_receiver) = channel();

    let sto =  VecDeque::from([
        vec![TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,],
        vec![TileType::Free,TileType::Free,TileType::Separator,TileType::Free,TileType::Free, TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,],
        vec![TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,],
        vec![TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,],
        vec![TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Wall,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,],
        vec![TileType::Free,TileType::Free,TileType::Wall,TileType::Free,TileType::Wall,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,],
        vec![TileType::Free,TileType::Free,TileType::Wall,TileType::Free,TileType::Wall,TileType::Free,TileType::Separator,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,],
        vec![TileType::Free,TileType::Free,TileType::Wall,TileType::Free,TileType::Wall,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,],
        vec![TileType::Free,TileType::Free,TileType::Wall,TileType::Free,TileType::Wall,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,],
        vec![TileType::Free,TileType::Free,TileType::Wall,TileType::Free,TileType::Wall,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,],
        vec![TileType::Free,TileType::Free,TileType::Wall,TileType::Free,TileType::Wall,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,],
        vec![TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,],
        vec![TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,TileType::Free,]]
    );
    let map_nb_visible_lines = 14;


    let backend_event_sender_clone = backend_event_sender.clone(); // For the Board to post events to itself
    let frontend_event_sender_clone = frontend_event_sender.clone();

    let  map = Map::new(frontend_event_sender,  sto.clone());

    let gfx_map = GfxMap::new(sto);
    
    thread::spawn(move || {
            let mut board: Board<Map>  = Board::new(map, backend_event_sender_clone, backend_receiver, frontend_event_sender_clone);
            board.run();
        });

    let mut frontend = Frontend::new(gfx_map, backend_event_sender, frontend_event_receiver);
    frontend.run();
       
}

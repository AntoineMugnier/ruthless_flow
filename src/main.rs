use std::sync::mpsc::channel;

use board::Board;
use map::SimpleMap;

mod board;
mod heads;
mod map;
mod state_machine;
mod utils;
mod direction_picker;
mod head_list;
mod mpsc;
fn main() {
//    let (sender, receiver) = channel();
//    let mut board: SimpleBoard<SimpleMap>  = board::SimpleBoard::new(sender.clone(), receiver);
//    board.run();
}

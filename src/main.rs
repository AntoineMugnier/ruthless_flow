mod backend;
use crate::backend::board::Board;
pub use backend::map::*;

mod mpsc;

#[cfg(not(test))]
use crate::mpsc::{channel};

#[cfg(not(test))]
fn main() {
    let (sender, receiver) = channel();
    let mut board: Board<Map>  = Board::new(sender.clone(), receiver);
    board.run();
}

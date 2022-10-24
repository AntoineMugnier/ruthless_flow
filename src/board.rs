use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::heads::{Head, self};
use crate::utils::{Direction, Coordinates};
use crate::map::{Map, TileType};
use std::sync::mpsc::{channel, Receiver};

//extern crate timer;

pub enum BoardEvevents {
    SLIDE_FRAME_TICK,
    MOVE_HEADS_TICK,
    SET_NEXT_HEAD_DIRECTION{direction : Direction}
    
}
type HeadsCell = Rc<RefCell<Head>>;

type HeadsVec = Vec<HeadsCell>;

pub struct Board{
    map: Map,
    heads :  HeadsVec,
    events_receiver : Receiver<BoardEvevents>,
    next_direction : Direction
}

impl Board{

pub fn new(events_receiver : Receiver<BoardEvevents>) -> Self{
    
    let mut map = Map::new();

    // Create first head
    let first_head_position = Coordinates{x:map.get_length()/2, y: 0};
    let first_head =Head::new(first_head_position, Direction::None);
    let heads = vec![Rc::new(RefCell::new(first_head))];

    Self { map, heads, events_receiver, next_direction : Direction::None}
}

fn move_and_mark_tile(head : &mut HeadsCell, position: Coordinates, direction : Direction, map : &mut Map) -> Option<HeadsCell>{
    let mut head = head.clone();
    head.borrow_mut().set_provenance(direction);
    map.set_tile(position, TileType::Marked);
    head.borrow_mut().set_position(position);
    Some(head.clone())
}

fn explore_direction(direction : Direction, head : &mut HeadsCell, map : &mut Map) -> Option<HeadsCell>{
    let position = head.borrow().get_position();
    if let Some((tile_type, position)) = map.get_tile(position, direction){
        match tile_type {
            TileType::Free =>{
                Board::move_and_mark_tile(head, position,direction, map)
            }
            TileType::Marked => {
                None
            },
            TileType::Separator => {
                Board::move_and_mark_tile(head, position,direction, map)
            },
            TileType::Wall => {
                Some(head.clone())
            },
        }
    }
    // TileType is out of range
    else{
        Some(head.clone())
    }
}
fn split_heads_on_separator(&mut self){
    for head in self.heads.iter(){
        //self.heads.push(*head);
    }
}
fn move_heads_handler(&mut self, direction : Direction){
    let map = &mut self.map;
    let heads = self.heads.iter_mut().filter_map(|head| Board::explore_direction (direction, head, map)
).collect::<HeadsVec>();

}

pub fn start(&mut self){
    
    while let Ok(evt) = self.events_receiver.recv() {
    match evt {
        BoardEvevents::SLIDE_FRAME_TICK => {()}
        BoardEvevents::MOVE_HEADS_TICK =>  self.move_heads_handler(self.next_direction),
        BoardEvevents::SET_NEXT_HEAD_DIRECTION { direction } => self.next_direction = direction
    }
    }


}
}

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::heads::{Head, self, HeadAction, HeadState};
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
    next_direction : Option<Direction>
}

impl Board{

pub fn new(events_receiver : Receiver<BoardEvevents>) -> Self{
    
    let mut map = Map::new();

    // Create first head
    let first_head_position = Coordinates{x:map.get_length()/2, y: 0};
    let first_head =Head::new(first_head_position, None);
    let heads = vec![Rc::new(RefCell::new(first_head))];

    Self { map, heads, events_receiver, next_direction : None}
}



fn move_heads_handler(&mut self, direction : Direction){
    let map = &mut self.map;

    for head in self.heads.iter(){
        // match head.borrow_mut()
    }

    self.heads = self.heads.iter().filter_map(|head| 
        {
           match head.borrow_mut().try_moving_to_direction(direction,map) {
                 HeadState::TO_KEEP => return Some(head.clone()),
                 HeadState::TO_KILL => return None,
           }
        }
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

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
    SET_NEXT_HEAD_DIRECTION{direction : Option<Direction>}
    
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
    let first_head =Head::new(first_head_position, Direction::Down);
    let heads = vec![Rc::new(RefCell::new(first_head))];

    Self { map, heads, events_receiver, next_direction : None}
}



fn move_heads_handler(&mut self, direction : Option<Direction>){
    let map = &mut self.map;

    // Separator
    let  mut new_head_vec = Vec::new();
    for head in self.heads.iter(){
        let mut head =  head.borrow_mut();
        if let Some(newborn_head) = head.split_heads_if_on_separator(map){
            new_head_vec.push(Rc::new(RefCell::new(newborn_head)))
        }
    }
    self.heads = new_head_vec;

    let new_head_vec  = self.heads.iter().filter_map(|head| 
        {
           match head.borrow_mut().try_moving_to_direction(direction,map) {
                 HeadState::TO_KEEP => return Some(head.clone()),
                 HeadState::TO_KILL => return None,
           }
        }
    ).collect::<HeadsVec>();
    self.heads = new_head_vec;

}

pub fn run(&mut self){
    
    while let Ok(evt) = self.events_receiver.recv() {
    match evt {
        BoardEvevents::SLIDE_FRAME_TICK => {()}
        BoardEvevents::MOVE_HEADS_TICK =>  self.move_heads_handler(self.next_direction),
        BoardEvevents::SET_NEXT_HEAD_DIRECTION { direction } => self.next_direction = direction
    }
    }


}
}

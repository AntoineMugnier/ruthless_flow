use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Deref;
use std::rc::Rc;
use crate::heads::{SimpleHead, self, HeadAction, HeadEvents, Head};
use crate::utils::{Direction, Coordinates};
use crate::map::{TileType, Map};
use std::sync::mpsc::{ Receiver, Sender};

//extern crate timer;


type HeadsCell = Box<SimpleHead>;

type HeadsVec = Vec<HeadsCell>;

pub enum BoardEvevents {
    SLIDE_FRAME_TICK,
    KILL_HEAD{id : heads::Id},
    ADD_HEAD { position: Coordinates, coming_from : Direction, parent_direction : Direction },
    MOVE_HEADS_TICK,
    SET_NEXT_HEAD_DIRECTION{direction : Option<Direction>}
    
}

pub struct Board<MapType : Map>{
    map: MapType,
    heads :  HeadsVec,
    events_receiver : Receiver<BoardEvevents>,
    events_sender : Sender<BoardEvevents>,
    next_direction : Option<Direction>
}

impl <MapType: Map>Board<MapType>{

pub fn new(events_sender : Sender<BoardEvevents>, events_receiver : Receiver<BoardEvevents>) -> Self{
    
    let mut map = MapType::new();

    // Create first head
    let first_head_position = Coordinates{x:map.get_length()/2, y: 0};
    let first_head =SimpleHead::new( 0, first_head_position, Direction::Down, events_sender.clone());
    let heads = vec![Box::new(first_head)];

    Self { map, heads, events_sender, events_receiver, next_direction : None}
}



fn move_heads_handler(&mut self, direction : Option<Direction>){
    let map = &mut self.map;

    for head in self.heads.iter_mut()
        {
            let move_head_event = HeadEvents::MOVE_HEAD { direction, map};
           head.dispatch(move_head_event);
        }
    
}

pub fn run(&mut self){
    
    while let Ok(evt) = self.events_receiver.recv() {
    match evt {
        BoardEvevents::SLIDE_FRAME_TICK => {()}
        BoardEvevents::MOVE_HEADS_TICK =>  self.move_heads_handler(self.next_direction),
        BoardEvevents::SET_NEXT_HEAD_DIRECTION { direction } => self.next_direction = direction,
        BoardEvevents::KILL_HEAD { id } => {},
        BoardEvevents::ADD_HEAD { position, coming_from, parent_direction } => {},

    
    }
    }


}
}

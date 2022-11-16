use crate::heads::{self, Head, HeadAction, HeadEvents, SimpleHead};
use crate::map::{Map, TileType};
use crate::utils::{Coordinates, Direction, DirectionFlags};
use std::sync::mpsc::{Receiver, Sender};

//extern crate timer;

type HeadsCell = Box<SimpleHead>;

type HeadsVec = Vec<HeadsCell>;

pub enum BoardEvevents {
    SLIDE_FRAME_TICK,
    KILL_HEAD {
        id: heads::Id,
    },
    ADD_HEAD {
        position: Coordinates,
        coming_from: Direction,
        parent_direction: Direction,
    },
    MOVE_HEADS_TICK,
    SET_NEXT_HEAD_DIRECTION {
        direction: Option<Direction>,
    },
}

pub struct SimpleBoard<MapType: Map> {
    map: MapType,
    heads: HeadsVec,
    events_receiver: Receiver<BoardEvevents>,
    events_sender: Sender<BoardEvevents>,
    next_direction: Option<Direction>,
}

mod private {
    use super::*;

    pub trait Sealed {
        fn move_heads_handler(&mut self, direction: Option<Direction>);
    }
}
pub trait Board: private::Sealed {
    fn new(events_sender: Sender<BoardEvevents>, events_receiver: Receiver<BoardEvevents>);
    fn run(&mut self);
    fn move_heads_handler(&mut self, direction: Option<Direction>);
}

impl<MapType: Map> private::Sealed for SimpleBoard<MapType> {
    fn move_heads_handler(&mut self, direction: Option<Direction>) {
        let map = &mut self.map;

        for head in self.heads.iter_mut() {
            let prohibited_directions = DirectionFlags::empty();
            let move_head_event = HeadEvents::MOVE_HEAD { direction, prohibited_directions, map };
            head.dispatch(move_head_event);
        }
    }
}

impl<MapType: Map> SimpleBoard<MapType> {
    pub fn new(
        events_sender: Sender<BoardEvevents>,
        events_receiver: Receiver<BoardEvevents>,
    ) -> Self {
        let mut map = MapType::new();

        // Create first head
        let first_head_position = Coordinates {
            x: map.get_length() / 2,
            y: 0,
        };

        let first_head = SimpleHead::new(
            0,
            first_head_position,
            Direction::Down,
            events_sender.clone(),
        );

        let heads = vec![Box::new(first_head)];

        Self {
            map,
            heads,
            events_sender,
            events_receiver,
            next_direction: None,
        }
    }

    pub fn run(&mut self) {
        while let Ok(evt) = self.events_receiver.recv() {
            match evt {
                BoardEvevents::SLIDE_FRAME_TICK => (),
                BoardEvevents::MOVE_HEADS_TICK => {
                    private::Sealed::move_heads_handler(self, self.next_direction)
                }
                BoardEvevents::SET_NEXT_HEAD_DIRECTION { direction } => {
                    self.next_direction = direction
                }
                BoardEvevents::KILL_HEAD { id } => {}
                BoardEvevents::ADD_HEAD {
                    position,
                    coming_from,
                    parent_direction,
                } => {}
            }
        }
    }
}

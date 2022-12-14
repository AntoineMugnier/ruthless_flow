use crate::head_list::HeadList;
use crate::heads::{self, Head, HeadAction, HeadEvents, SimpleHead};
use crate::map::{Map, TileType};
use crate::utils::{Coordinates, Direction, DirectionFlags};
use std::sync::mpsc::{Receiver};
use crate::mpsc::Sender;

#[derive(Debug, Clone)]
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
    heads: HeadList<SimpleHead>,
    events_receiver: Receiver<BoardEvevents>,
    events_sender: Sender<BoardEvevents>,
    next_direction: Option<Direction>,
}

mod private {
    use super::*;

    pub trait Sealed {
        fn move_heads_handler(&mut self, direction: Option<Direction>);
        fn kill_head_handler(&mut self, id: heads::Id);
    }
}
pub trait Board: private::Sealed {
    fn new(events_sender: Sender<BoardEvevents>, events_receiver: Receiver<BoardEvevents>);
    fn run(&mut self);
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

    fn kill_head_handler(&mut self, id: heads::Id) {
        self.heads.remove(id);
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

        let mut heads = HeadList::new();
        heads.add_head(first_head_position,Direction::Down,events_sender.clone(), &mut map);
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
                BoardEvevents::KILL_HEAD { id } => {
                    private::Sealed::kill_head_handler(self, id)
                }
                BoardEvevents::ADD_HEAD {
                    position,
                    coming_from,
                    parent_direction,
                } => {
                    let head = self.heads.add_head(position, coming_from, self.events_sender.clone(), &mut self.map);
                    let event = HeadEvents::MOVE_HEAD { direction: self.next_direction , prohibited_directions: DirectionFlags::from(parent_direction), map: &mut self.map};
                    head.dispatch(event)
                }
            }
        }
    }
}

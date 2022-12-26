use super::head_list::HeadList;
use super::heads::{self, Head, HeadAction, HeadEvents, SimpleHead};
use super::map::{MapTrait, TileType};
use super::utils::{Coordinates, Direction, DirectionFlags};
use crate::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;


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

pub struct Board<MapType: MapTrait> {
    map: MapType,
    heads: HeadList<SimpleHead>,
    events_receiver: Receiver<BoardEvevents>,
    events_sender: Sender<BoardEvevents>,
    next_direction: Option<Direction>,
    move_heads_timer_h: std::thread::JoinHandle<()>
}

impl <MapType: MapTrait> Board<MapType>{
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

    fn set_next_head_direction(&mut self, direction: Option<Direction>) {
        self.next_direction = direction
    }
    
    fn add_head_handler(&mut self, position: Coordinates, coming_from: Direction, parent_direction: Direction) {
        let head = self.heads.add_head(position, coming_from, self.events_sender.clone());
        let event = HeadEvents::MOVE_HEAD { direction: self.next_direction , prohibited_directions: DirectionFlags::from(parent_direction), map: &mut self.map};
        head.dispatch(event);
    }

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
        heads.add_head(first_head_position,Direction::Down, events_sender.clone());

        // Spawn the thread that will trigger MOVE_HEADS_TICK every second
        let event_sender_clone = events_sender.clone();
        let move_heads_timer_h = thread::spawn(move || {
        loop {
            let event = BoardEvevents::MOVE_HEADS_TICK{};
            event_sender_clone.send(event).unwrap();
            thread::sleep(Duration::from_secs(1));
            }
        });

        Self {
            map,
            heads,
            events_sender,
            events_receiver,
            next_direction: None,
            move_heads_timer_h
        }
    }


    pub fn run(&mut self) {
        while let Ok(evt) = self.events_receiver.recv() {
            match evt {
                BoardEvevents::SLIDE_FRAME_TICK => (),
                BoardEvevents::MOVE_HEADS_TICK => {
                    self.move_heads_handler(self.next_direction);
                }
                BoardEvevents::SET_NEXT_HEAD_DIRECTION { direction } => {
                    self.set_next_head_direction(direction);
                }
                BoardEvevents::KILL_HEAD { id } => {
                    self.kill_head_handler(id);
                }
                BoardEvevents::ADD_HEAD {
                    position,
                    coming_from,
                    parent_direction,
                } => {
                    self.add_head_handler(position, coming_from, parent_direction);
                }
            }
        }
    }
}

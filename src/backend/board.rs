use super::head_list::HeadList;
use super::heads::{self, Head, HeadEvents, SimpleHead};
use super::map::{MapTrait};
use super::utils::{Coordinates, Direction, DirectionFlags};
use crate::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use crate::frontend::FrontendEvents;

#[derive(Debug, Clone)]
pub enum BoardEvevents {
    SlideFrameTick,
    KillHead {
        id: heads::Id,
    },
    AddHead {
        position: Coordinates,
        coming_from: Direction,
        parent_direction: Direction,
    },
    MoveHeadsTick,
    SetNextHeadDir {
        direction: Option<Direction>,
    },
}

pub struct Board<MapType: MapTrait> {
    map: MapType,
    heads: HeadList<SimpleHead>,
    frontend_sender : Sender<FrontendEvents>, 
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
            let move_head_event = HeadEvents::MoveHead { direction, prohibited_directions, map };
            head.dispatch(move_head_event);
        }
    }

    fn kill_head_handler(&mut self, id: heads::Id) {
        self.heads.remove(id);
    }

    fn set_next_head_dir(&mut self, direction: Option<Direction>) {
        self.next_direction = direction
    }
    
    fn add_head_handler(&mut self, position: Coordinates, coming_from: Direction, parent_direction: Direction) {
        let head = self.heads.add_head(position, coming_from, self.events_sender.clone());
        let event = HeadEvents::MoveHead { direction: self.next_direction , prohibited_directions: DirectionFlags::from(parent_direction), map: &mut self.map};
        head.dispatch(event);
    }

    pub fn new(
        frontend_sender : Sender<FrontendEvents>, 
        events_sender: Sender<BoardEvevents>,
        events_receiver: Receiver<BoardEvevents>,
    ) -> Self {
        let  map = MapType::new();

        // Create first head
        let first_head_position = Coordinates {
            x: map.get_length() / 2,
            y: 0,
        };

        let mut heads = HeadList::new();
        heads.add_head(first_head_position,Direction::Down, events_sender.clone());

        // Spawn the thread that will trigger MoveHeadsTick every second
        let event_sender_clone = events_sender.clone();
        let move_heads_timer_h = thread::spawn(move || {
        loop {
            let event = BoardEvevents::MoveHeadsTick{};
            event_sender_clone.send(event).unwrap();
            thread::sleep(Duration::from_secs(1));
            }
        });

        Self {
            map,
            heads,
            frontend_sender,
            events_sender,
            events_receiver,
            next_direction: None,
            move_heads_timer_h
        }
    }


    pub fn run(&mut self) {
        while let Ok(evt) = self.events_receiver.recv() {
            match evt {
                BoardEvevents::SlideFrameTick => (),
                BoardEvevents::MoveHeadsTick => {
                    self.move_heads_handler(self.next_direction);
                }
                BoardEvevents::SetNextHeadDir { direction } => {
                    self.set_next_head_dir(direction);
                }
                BoardEvevents::KillHead { id } => {
                    self.kill_head_handler(id);
                }
                BoardEvevents::AddHead {
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

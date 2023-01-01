use super::head_list::HeadList;
use super::heads::{self, Head, SimpleHead};
use super::map::{MapTrait};
use super::utils::{Coordinates, Direction, DirectionFlags};
use crate::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Events {
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
    board_events_receiver: Receiver<Events>,
    board_events_sender: Sender<Events>,
    next_direction: Option<Direction>,
    move_heads_timer_h: std::thread::JoinHandle<()>
}

impl <MapType: MapTrait> Board<MapType>{
    fn move_heads_handler(&mut self, direction: Option<Direction>) {
        let map = &mut self.map;

        for head in self.heads.iter_mut() {
            let prohibited_directions = DirectionFlags::empty();
            let move_head_event = heads::Events::MoveHead { direction, prohibited_directions, map };
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
        let head = self.heads.add_head(position, coming_from, self.board_events_sender.clone());
        let event = heads::Events::MoveHead { direction: self.next_direction , prohibited_directions: DirectionFlags::from(parent_direction), map: &mut self.map};
        head.dispatch(event);
    }

    pub fn new(
        map : MapType,
        board_events_sender: Sender<Events>,
        board_events_receiver: Receiver<Events>,
    ) -> Self {

        // Create first head
        let first_head_position = Coordinates {
            x: map.get_length() / 2,
            y: 0,
        };

        let mut heads = HeadList::new();
        heads.add_head(first_head_position,Direction::Down, board_events_sender.clone());

        // Spawn the thread that will trigger MoveHeadsTick every second
        let event_sender_clone = board_events_sender.clone();
        let move_heads_timer_h = thread::spawn(move || {
        loop {
            let event = Events::MoveHeadsTick{};
            event_sender_clone.send(event).unwrap();
            thread::sleep(Duration::from_secs(1));
            }
        });

        Self {
            map,
            heads,
            board_events_sender,
            board_events_receiver,
            next_direction: None,
            move_heads_timer_h
        }
    }


    pub fn run(&mut self) {
        while let Ok(evt) = self.board_events_receiver.recv() {
            match evt {
                Events::SlideFrameTick => (),
                Events::MoveHeadsTick => {
                    self.move_heads_handler(self.next_direction);
                }
                Events::SetNextHeadDir { direction } => {
                    self.set_next_head_dir(direction);
                }
                Events::KillHead { id } => {
                    self.kill_head_handler(id);
                }
                Events::AddHead {
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

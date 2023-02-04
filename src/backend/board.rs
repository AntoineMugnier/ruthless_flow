use super::config;
use super::head_list::HeadList;
use super::heads::{self, Head, SimpleHead};
use super::map::{MapTrait};
use crate::utils::{Coordinates, Direction, DirectionFlags};
use crate::mpsc::{Receiver, Sender};
use crate::frontend;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Event {
    SlideMapTick,
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
        direction: Direction,
    },
    EndGame{}
}

#[derive(Debug, Clone)]
pub enum EndGameReason{
    NoRemainingHeads,
    HeadPoppedOutByRisingEdge
}


pub struct Board<MapType: MapTrait> {
    map: MapType,
    heads: HeadList<SimpleHead>,
    board_events_receiver: Receiver<Event>,
    board_events_sender: Sender<Event>,
    frontend_events_sender: Sender<frontend::Event>,
    next_direction: Direction,
    move_heads_timer_h: std::thread::JoinHandle<()>,
    slide_map_timer_h:  std::thread::JoinHandle<()>,
}

impl <MapType: MapTrait> Board<MapType>{
    fn move_heads_handler(&mut self, direction: Direction) {
        let map = &mut self.map;

        for head in self.heads.iter_mut() {
            let prohibited_directions = DirectionFlags::empty();
            let move_head_event = heads::Event::MoveHead { direction, prohibited_directions, map };
            head.dispatch(move_head_event);
        }
        
    }

    fn send_end_game_evt(&self, end_game_reason : EndGameReason){
        let event = frontend::Event::EndGame{game_end_reason: end_game_reason};
        self.frontend_events_sender.send(event).unwrap();
    }

    fn send_current_nb_heads(&self){
        let event = frontend::Event::UpdateNbHeads{nb_heads: self.heads.get_nb_heads()};
        self.frontend_events_sender.send(event);
    }
    
    fn kill_head_handler(&mut self, id: heads::Id)   {
        self.heads.remove(id);

        self.send_current_nb_heads();
        if(self.heads.get_nb_heads() ==0){
            self.send_end_game_evt(EndGameReason::NoRemainingHeads);
            
        }
    }

    fn set_next_head_dir(&mut self, direction: Direction) {
        if direction != self.next_direction{
        self.next_direction = direction;
        self.frontend_events_sender.send(frontend::Event::UserDirSet{direction});
        }
        
    }
    
    fn add_head_handler(&mut self, position: Coordinates, coming_from: Direction, parent_direction: Direction)  {
        let head = self.heads.add_head(position, coming_from, self.board_events_sender.clone());
        let event = heads::Event::MoveHead { direction: self.next_direction , prohibited_directions: DirectionFlags::from(parent_direction), map: &mut self.map};
        head.dispatch(event);

        self.send_current_nb_heads();
        
    }

    pub fn new(
        map : MapType,
        board_events_sender: Sender<Event>,
        board_events_receiver: Receiver<Event>,
        frontend_events_sender: Sender<frontend::Event>
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
            let slide_map_timer_h = thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(1000/config::MAP_SLIDE_FRQUENCY));
                let event = Event::SlideMapTick{};
                event_sender_clone.send(event).unwrap();
                }
            });

        // Spawn the thread that will trigger MoveHeadsTick every second
        let event_sender_clone = board_events_sender.clone();
        let move_heads_timer_h = thread::spawn(move || {
        loop {
            let event = Event::MoveHeadsTick{};
            event_sender_clone.send(event).unwrap();
            thread::sleep(Duration::from_millis(1000/config::HEADS_MOVE_FREQUENCY));
            }
        });

        let board = Self {
            map,
            heads,
            board_events_sender,
            board_events_receiver,
            frontend_events_sender,
            next_direction: Direction::Up,
            move_heads_timer_h,
            slide_map_timer_h
        };

        board.send_current_nb_heads();

        board
    }

    fn slide_map_handler(&mut self) {
        self.map.slide();
        
    }
    
    pub fn run(&mut self) {
        while let Ok(evt) = self.board_events_receiver.recv() {
            match evt {
                Event::SlideMapTick => self.slide_map_handler(),
                Event::MoveHeadsTick => {
                    self.move_heads_handler(self.next_direction)
                }
                Event::SetNextHeadDir { direction } => {
                    self.set_next_head_dir(direction)
                }
                Event::KillHead { id } => {
                    self.kill_head_handler(id)
                }
                Event::AddHead {
                    position,
                    coming_from,
                    parent_direction,
                } => {
                    self.add_head_handler(position, coming_from, parent_direction)
                },
                Event::EndGame{} => {}
            }
        }
    }
}

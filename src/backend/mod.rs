pub mod map;
pub mod heads;
mod direction_picker;
mod head_list;
pub mod config;

use crate::utils::{Coordinates, Direction, DirectionFlags};
use crate::mpsc::{Receiver, Sender};
use crate::frontend;
use std::thread;
use std::time::Duration;

use self::head_list::HeadList;
use self::heads::{SimpleHead, Head};
use self::map::{MapTrait, TileType};

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
    StartGame,
    EndGame{end_game_reason : EndGameReason}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EndGameReason{
    Victory,
    NoRemainingHeads,
    HeadPoppedOutByRisingEdge
}

enum BoardState{
    Startup,
    Playing,
    Ending
}

pub struct Backend<MapType: MapTrait> {
    map: MapType,
    heads: HeadList<SimpleHead>,
    board_events_receiver: Receiver<Event>,
    board_events_sender: Sender<Event>,
    frontend_events_sender: Sender<frontend::Event>,
    next_direction: Direction,
    board_state: BoardState,
    move_heads_timer_h: Option<std::thread::JoinHandle<()>>,
    slide_map_timer_h:  Option<std::thread::JoinHandle<()>>,
}

impl <MapType: MapTrait> Backend<MapType>{
    fn move_heads_handler(&mut self, direction: Direction) {
        let map = &mut self.map;

        for head in self.heads.iter_mut() {
            let prohibited_directions = DirectionFlags::empty();
            let move_head_event = heads::Event::MoveHead { direction, prohibited_directions, map };
            head.dispatch(move_head_event);
        }
        
    }

    fn end_game(&mut self, end_game_reason : EndGameReason){
        let event = frontend::Event::EndGame{game_end_reason: end_game_reason};
        self.frontend_events_sender.send(event).unwrap();
        self.board_state = BoardState::Ending;
    }

    fn send_current_nb_heads(&self){
        let event = frontend::Event::UpdateNbHeads{nb_heads: self.heads.get_nb_heads()};
        self.frontend_events_sender.send(event).unwrap();
    }
    
    fn kill_head_handler(&mut self, id: heads::Id)   {
        self.heads.remove(id);

        self.send_current_nb_heads();
        if self.heads.get_nb_heads() ==0{
            self.end_game(EndGameReason::NoRemainingHeads);
            
        }
    }

    fn set_next_head_dir(&mut self, direction: Direction) {
        if direction != self.next_direction{
            self.next_direction = direction;
            self.frontend_events_sender.send(frontend::Event::UserDirSet{direction}).unwrap();
        }
        
    }
    
    fn add_head_handler(&mut self, position: Coordinates, coming_from: Direction, parent_direction: Direction)  {
        let head = self.heads.add_head(position, coming_from, self.board_events_sender.clone());
        let event = heads::Event::MoveHead { direction: self.next_direction , prohibited_directions: DirectionFlags::from(parent_direction), map: &mut self.map};
        head.dispatch(event);

        self.send_current_nb_heads();
        
    }
    pub fn start_timers(&mut self){
        // Spawn the thread that will trigger MoveHeadsTick every second
        let event_sender_clone = self.board_events_sender.clone();
        self.move_heads_timer_h = Some(thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(1000/config::HEADS_MOVE_FREQUENCY));
            let event = Event::MoveHeadsTick{};
            event_sender_clone.send(event).unwrap();
            }
        }));

        // Spawn the thread that will trigger MoveHeadsTick every second
        let event_sender_clone = self.board_events_sender.clone();
        self.slide_map_timer_h = Some(thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(1000/config::MAP_SLIDE_FRQUENCY));
            let event = Event::SlideMapTick{};
            event_sender_clone.send(event).unwrap();
            }
        }));


    }
    
    pub fn new(
        mut map : MapType,
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
        
        //Set tile of the first head
        map.set_tile(first_head_position, TileType::Head{id: 0});

        let board = Self {
            map,
            heads,
            board_events_sender,
            board_events_receiver,
            frontend_events_sender,
            next_direction: Direction::Up,
            board_state: BoardState::Startup,
            move_heads_timer_h: None,
            slide_map_timer_h: None
        };

        board.send_current_nb_heads();
        
        board
    }


    fn slide_map_handler(&mut self) {
        if self.map.will_head_pop_out_during_next_sliding(){
            self.end_game(EndGameReason::HeadPoppedOutByRisingEdge);
        }
        else{
            self.map.slide();
        }
    }
    
    pub fn startup_state_handler(&mut self, evt : Event){
            match evt {
                Event::StartGame => {
                    self.start_timers(); // Enable timers that pace the application
                    self.board_state = BoardState::Playing;
                    return
                }, 
                _ =>{}
            }
    }

    pub fn playing_state_handler(&mut self, evt : Event){
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
            Event::EndGame{end_game_reason} => {
                self.end_game(end_game_reason);
            }
            _ => {}
        }
    }

    pub fn ending_state_handler(&mut self, _evt : Event){
    }

    pub fn run(&mut self) {

        while let Ok(evt) = self.board_events_receiver.recv() {
            match self.board_state{
                BoardState::Startup => self.startup_state_handler(evt),
                BoardState::Playing => self.playing_state_handler(evt),
                BoardState::Ending => self.ending_state_handler(evt),
            }
        }
    }
}

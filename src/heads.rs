use enumflags2::{make_bitflags, BitFlag};
use std::cell::{RefCell, UnsafeCell};
use std::u8;

use crate::board::BoardEvevents;
use crate::direction_picker::DirectionPicker;
use crate::mpsc::Sender;
use crate::map::{Map, TileType};
use crate::utils::{Coordinates, Direction, DirectionFlags};
use rand::{thread_rng, Rng};


pub enum HeadEvents<'a, MapType: Map> {
    MOVE_HEAD {
        direction: Option<Direction>,
        prohibited_directions : DirectionFlags, // bitfield to hold already explored or forbidden directions
        map: &'a mut MapType,
    },
}
pub type Id = u32;

#[derive(PartialEq)]
pub enum HeadAction {
    HAS_NOT_MOVED,
    HAS_MOVED(TileType),
}

pub struct SimpleHead {
    id: Id,
    position: Coordinates,
    coming_from: Direction,
    events_sender: Sender<BoardEvevents>,
    head_split : bool

}

pub trait Head: private::Sealed {
    fn new(
        id: Id,
        position: Coordinates,
        coming_from: Direction,
        events_sender: Sender<BoardEvevents>,
    ) -> SimpleHead;
    fn dispatch(&mut self, event: HeadEvents<impl Map>);
}
mod private {
    use super::*;

    pub trait Sealed {
        fn set_position(&mut self, position: Coordinates);
        fn set_provenance(&mut self, coming_from: Direction);
        fn get_position(&self) -> Coordinates;
        fn get_provenance(&self) -> Direction;
        fn move_head_handler(&mut self, direction: Option<Direction>, prohibited_directions : DirectionFlags, map: &mut impl Map);
        fn explore_direction(
            position: Coordinates,
            chosen_direction: Direction,
            prohibited_directions: &mut DirectionFlags,
            map: &mut impl Map,
        ) -> (Direction, TileType, Coordinates);
        fn move_and_mark_tile(&mut self, map: &mut impl Map, target_position: Coordinates, chosen_direction: Direction);
    }
}

impl private::Sealed for SimpleHead {
    fn set_position(&mut self, position: Coordinates) {
        self.position = position;
    }

    fn set_provenance(&mut self, coming_from: Direction) {
        self.coming_from = coming_from;
    }

    fn get_position(&self) -> Coordinates {
        self.position
    }

    fn get_provenance(&self) -> Direction {
        self.coming_from
    }

    fn move_and_mark_tile(&mut self, map: &mut impl Map, target_position: Coordinates, chosen_direction: Direction) {
        map.set_tile(target_position, TileType::Marked);
        self.set_position(target_position);
        self.set_provenance(chosen_direction.reverse());
    }

    fn move_head_handler(&mut self, direction: Option<Direction>, mut prohibited_directions : DirectionFlags, map: &mut impl Map) {

        // Prevent head from going back to its previous path
        prohibited_directions.insert(self.coming_from); 

        // Select a random direction if no one has been set
        let proposed_direction;
        if let Some(direction) = direction {
            if direction != self.coming_from{
                proposed_direction = direction;
                prohibited_directions.insert(proposed_direction);
            }
            else{ 
                proposed_direction = DirectionPicker::pick(&mut prohibited_directions);
            }
        } else {
            proposed_direction = DirectionPicker::pick(&mut prohibited_directions);
        }

        // Try to explore explore the `proposed_direction`. If the move is impossible, explore all the other authorized directions around the head.
        let (chosen_direction, target_tile, target_position) = Self::explore_direction(self.get_position(), proposed_direction, &mut prohibited_directions, map);

        // Order the board to create a new head if the tile on which we are on a separator
        if self.head_split {
            let add_head_event = BoardEvevents::ADD_HEAD {
                position: self.get_position(),
                coming_from: self.get_provenance(),
                parent_direction: chosen_direction,
            };
            self.events_sender.send(add_head_event).unwrap();
            self.head_split = false;
        }

        //Special action depending on the type of tile we reach
        match target_tile{
            // Order the board to kill self
            TileType::Marked => {
                let remove_head_event = BoardEvevents::KILL_HEAD { id: self.id };
                self.events_sender.send(remove_head_event).unwrap();
            }
            // Move the head to the location and mark the tile
            TileType::Free =>  {
                self.move_and_mark_tile(map, target_position, chosen_direction);
            },
            TileType::Separator =>  {
                self.move_and_mark_tile(map, target_position, chosen_direction);
                self.head_split = true;
            },
            TileType::Wall => assert!(true, "Cannot move to a wall"),
        }
        
    }

    fn explore_direction(
        original_position: Coordinates,
        chosen_direction: Direction,
        prohibited_directions: &mut DirectionFlags,
        map: &mut impl Map,
    ) -> (Direction, TileType, Coordinates) {

        if let Some((tile_type, target_position)) = map.get_neighbour_tile(original_position, chosen_direction) {

            match tile_type {
                TileType::Free | TileType::Separator | TileType::Marked => return (chosen_direction, tile_type, target_position),
                TileType::Wall => {
                    let chosen_direction = DirectionPicker::pick(prohibited_directions);
                    Self::explore_direction(original_position, chosen_direction, prohibited_directions, map)
                },
            }
        }
        // We are targetting an edge of the map
        else {
            let chosen_direction = DirectionPicker::pick(prohibited_directions);
            Self::explore_direction(original_position, chosen_direction, prohibited_directions, map)
        }
        
    }

}

// TODO
// - Separator
// - random choice of dir
//
impl Head for SimpleHead {
    fn new(
        id: Id,
        position: Coordinates,
        coming_from: Direction,
        events_sender: Sender<BoardEvevents>,
    ) -> SimpleHead { // TODO, initialize with map
        SimpleHead {
            id,
            position,
            coming_from,
            events_sender,
            head_split : false
        }
    }

    fn dispatch(&mut self, event: HeadEvents<impl Map>) {
        match event {
            HeadEvents::MOVE_HEAD { direction, prohibited_directions, map } => {
                private::Sealed::move_head_handler(self, direction,prohibited_directions, map)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use mockall::{predicate, Sequence, mock};
    use crate::{map::{MockMap}, mpsc::MockSender};
    use crate::mpsc::SendError;

    use super::*;

mod TestConditions{
use crate::direction_picker::PickerCtx;

use super::*;

pub struct General<'a>{
    pub previous_way : Way,
    pub first_stage: FirstStage<'a>,
    pub separator: Option<Separator>,
    pub to_wall: Option<ToWall<'a>>
}

#[derive(Copy, Clone)]
pub struct Separator{}

pub enum FirstStage<'a>{
    ValidDir{way: Way},
    InvalidDir{way: Way, picker_ctx : &'a PickerCtx},
}

#[derive(Copy, Clone)]
pub struct Way{pub alt_direction : Direction, pub alt_target_position : Coordinates, pub alt_target_tile : TileType}

pub struct ToWall<'a>{pub ways: Vec<Way> ,pub picker_ctx : &'a PickerCtx}


}

fn test_move(seq : & mut Sequence, map: & mut  MockMap, event_sender : &mut MockSender<BoardEvevents>, tc: &TestConditions::General){
    let original_position = tc.previous_way.alt_target_position;
    let mut direction_taken;
    let mut target_position;
    let mut target_tile;

    match tc.first_stage{
        TestConditions::FirstStage::InvalidDir { way, picker_ctx } => {
        let alt_direction = way.alt_direction;
        picker_ctx.expect().once().in_sequence(seq).returning(move |_| alt_direction);
        direction_taken = way.alt_direction;
        target_tile =  way.alt_target_tile;
        target_position =  way.alt_target_position;
    },
        TestConditions::FirstStage::ValidDir {way} =>{
        direction_taken = way.alt_direction;
        target_position = way.alt_target_position;
        target_tile = way.alt_target_tile;
    }}
    
    map.expect_get_neighbour_tile().once().in_sequence(seq)
    .withf(move |position, direction| {*position == original_position && *direction == direction_taken} ).
    return_const(Some((target_tile, target_position)));
    
    if let Some(to_wall)= &tc.to_wall{
        for way in to_wall.ways.iter(){
            direction_taken = way.alt_direction;
            target_tile =  way.alt_target_tile;
            target_position =  way.alt_target_position;
            to_wall.picker_ctx.expect().once().in_sequence(seq).returning(move |_| direction_taken);

            map.expect_get_neighbour_tile().once().in_sequence(seq)
            .withf(move |position, direction| {print!("{:?}, {:?}",position,direction); *position == original_position && *direction == direction_taken} ).
            return_const(Some((target_tile, target_position)));
        }
    }

    if let Some(_) = tc.separator{
        event_sender.expect_send().once().in_sequence(seq).withf(move |board_event| {
            match board_event{
            BoardEvevents::ADD_HEAD { position, coming_from, parent_direction} => return true,
            _ => return false
        }
        }).return_const(Result::<(), SendError<BoardEvevents>>::Ok(()));
    }

    map.expect_set_tile().once().in_sequence(seq)
    .withf(move |position, tile_type| {*position == target_position && *tile_type == TileType::Marked})
    .return_const(());
    
    
}


#[test]
fn test_basic_moves(){
    //Test variants:
    //   - Move to Free Tile
    //   - Try Move to Wall
    //   - Try Move to map edge
    //   - Move to separator and diverge
    //   - Move to Marked and kill
    // - inject prohibited directions

        
    let mut seq = Sequence::new();
    let mut map = MockMap::default();
    let mut event_sender = Sender::default();
    let picker_ctx  = DirectionPicker::pick_context();

    // Test 0, Starting on Free Tile, normal move to free tile with chosen direction accepted

    let previous_way_0 = TestConditions::Way{alt_direction: Direction::Up, alt_target_position :Coordinates{x :10, y:10}, alt_target_tile : TileType::Marked};
    let target_way_0 = TestConditions::Way{alt_direction: Direction::Right, alt_target_position : Coordinates{x :11, y:10}, alt_target_tile : TileType::Free};

    let tc0 = TestConditions::General{
        previous_way: previous_way_0,
        first_stage: TestConditions::FirstStage::ValidDir{way: target_way_0},
        separator: None,
        to_wall: None
    };

    // Test 1: Normal move to free tile with chosen direction accepted 
    test_move(&mut seq, &mut map, &mut event_sender, &tc0);

    let previous_way_1 = target_way_0;
    let target_way_1 = TestConditions::Way{alt_direction: Direction::Up, alt_target_position : Coordinates{x :11, y:11}, alt_target_tile : TileType::Free};

    let tc1 = TestConditions::General{
        previous_way : previous_way_1,
        first_stage: TestConditions::FirstStage::ValidDir{
            way: target_way_1},  
        separator: None,
        to_wall : None
    };

    // Test 2: Chosen direction refused because it's backward leads to move to free tile
    test_move(&mut seq, &mut map, &mut event_sender, &tc1);

    let previous_way_2 = target_way_1;
    let backward_way_2 = target_way_1.alt_direction.reverse();
    let target_way_2 = TestConditions::Way{alt_direction: Direction::Left, alt_target_position : Coordinates{x :10, y:11}, alt_target_tile : TileType::Free};
    
    let tc2 = TestConditions::General{
        previous_way : previous_way_2,
        first_stage: TestConditions::FirstStage::InvalidDir{way: target_way_2 , picker_ctx: &picker_ctx},
        separator: None,
        to_wall: None
    };

    test_move(&mut seq, &mut map, &mut event_sender, &tc2);

    // Test 3: Chosen direction is refused because of a wall 
    let previous_way_3 = target_way_2;
    let failed_target_way_3 = TestConditions::Way{alt_direction: Direction::Left, alt_target_position : Coordinates{x :9, y:11}, alt_target_tile : TileType::Wall}; 
    let target_way_3 = TestConditions::Way{alt_direction: Direction::Up, alt_target_position : Coordinates{x :10, y:12}, alt_target_tile : TileType::Free};

    let tc3 = TestConditions::General{
        previous_way : previous_way_3,
        first_stage: TestConditions::FirstStage::ValidDir{way: failed_target_way_3},
        separator: None,
        to_wall: Some(TestConditions::ToWall{ways: vec![target_way_3], picker_ctx: &picker_ctx})
    };
    test_move(&mut seq, &mut map, &mut event_sender, &tc3);
    

    // Test 4: Chosen direction refused because it's empty leads to move to free tile
    let previous_way_4 = target_way_3;
    let target_way_4 = TestConditions::Way{alt_direction: Direction::Left, alt_target_position : Coordinates{x :8, y:11}, alt_target_tile : TileType::Free};
    
    let tc4 = TestConditions::General{
        previous_way : previous_way_4,
        first_stage: TestConditions::FirstStage::InvalidDir{way: target_way_4 , picker_ctx: &picker_ctx},
        separator: None,
        to_wall: None
    };
    test_move(&mut seq, &mut map, &mut event_sender, &tc4);


    let mut simple_head = SimpleHead::new(0, previous_way_0.alt_target_position, previous_way_0.alt_direction,  event_sender);

    dispatch_head_evt(Some(target_way_0.alt_direction), &mut map, &mut simple_head);
    dispatch_head_evt(Some(target_way_1.alt_direction), &mut map, &mut simple_head);
    dispatch_head_evt(Some(backward_way_2), &mut map, &mut simple_head);
    dispatch_head_evt(Some(failed_target_way_3.alt_direction), &mut map, &mut simple_head);
    dispatch_head_evt(None, &mut map, &mut simple_head);

}

fn dispatch_head_evt(head_going_to: Option<Direction>, map: &mut MockMap, simple_head: &mut SimpleHead) {
    let event = HeadEvents::MOVE_HEAD { direction: head_going_to, prohibited_directions : DirectionFlags::empty(),  map: map};
    simple_head.dispatch(event);
}



        // Check prohibited input_dir
        // Chek move dir to wall 
        // Move to Wall

        // Move to map edge

        // Move to Marked tile

        // Move to Free tile

        // Move to Separator
    
}

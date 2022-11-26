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
        fn reverse_dir(going_to: Direction) -> Direction;
        fn move_and_mark_tile(&mut self, map: &mut impl Map, target_position: Coordinates, chosen_direction: Direction);
    }
}

impl private::Sealed for SimpleHead {
    fn set_position(&mut self, position: Coordinates) {
        self.position = position;
    }

    fn reverse_dir(going_to: Direction) -> Direction {
        match going_to {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right
        }
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
        self.set_provenance(Self::reverse_dir(chosen_direction));
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
        position: Coordinates,
        chosen_direction: Direction,
        prohibited_directions: &mut DirectionFlags,
        map: &mut impl Map,
    ) -> (Direction, TileType, Coordinates) {

        if let Some((tile_type, position)) = map.get_neighbour_tile(position, chosen_direction) {

            match tile_type {
                TileType::Free | TileType::Separator | TileType::Marked => return (chosen_direction, tile_type, position),
                TileType::Wall => {
                    let chosen_direction = DirectionPicker::pick(prohibited_directions);
                    Self::explore_direction(position, chosen_direction, prohibited_directions, map)
                },
            }
        }
        // We are targetting an edge of the map
        else {
            let chosen_direction = DirectionPicker::pick(prohibited_directions);
            Self::explore_direction(position, chosen_direction, prohibited_directions, map)
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

    /*
        //let picker_ctx = DirectionPicker::pick_context();
        //picker_ctx.expect().times(1).in_sequence(&mut seq).returning(|_| Direction::Up);
*/
mod TestConditions{
use crate::direction_picker::PickerCtx;

use super::*;

#[derive(Copy, Clone)]
pub struct General<'a>{
    pub original_tile : TileType,
    pub head_provenance : Direction,
    pub original_position: Coordinates,
    pub target_tile : TileType,
    pub head_going_to : Direction,
    pub target_position: Coordinates,
    pub separator: Option<Separator>,
    pub backward : Option<Backward<'a>>
}

#[derive(Copy, Clone)]
pub struct Separator{}
#[derive(Copy, Clone)]
pub struct Backward<'a>{pub alt_direction : Direction, pub alt_target_position : Coordinates, pub alt_target_tile : TileType, pub picker_ctx : &'a PickerCtx}

}

fn test_move(seq : & mut Sequence, map: & mut  MockMap, event_sender : &mut MockSender<BoardEvevents>, tc: TestConditions::General){
    let mut head_going_to;
    let mut target_tile;
    let mut target_position;

    if let Some(backward)= tc.backward{
        backward.picker_ctx.expect().times(1).in_sequence(seq).returning(move |_| backward.alt_direction);
        head_going_to = backward.alt_direction;
        target_tile = backward.alt_target_tile;
        target_position = backward.alt_target_position;
    }
    else{
        head_going_to = tc.head_going_to;
        target_tile = tc.target_tile;
        target_position = tc.target_position;
    }

    map.expect_get_neighbour_tile().times(1).in_sequence(seq)
    .withf(move |position, direction| {*position == tc.original_position && *direction==head_going_to} ).
    return_const(Some((target_tile, target_position)));
    
    if let Some(_) = tc.separator{
        event_sender.expect_send().times(1).in_sequence(seq).withf(move |board_event| {
            match board_event{
            BoardEvevents::ADD_HEAD { position, coming_from, parent_direction} => return true,
            _ => return false
        }
        }).return_const(Result::<(), SendError<BoardEvevents>>::Ok(()));
    }

    map.expect_set_tile().times(1).in_sequence(seq)
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
    let tc0 = TestConditions::General{
        original_tile : TileType::Marked,
        head_provenance : Direction::Up,
        original_position : Coordinates{x :10, y:10}, 
        target_tile  : TileType::Free,
        head_going_to : Direction::Right,
        target_position : Coordinates{x :11, y:10},
        separator: None,
        backward: None
    };

    // Test 1: Normal move to free tile with chosen direction accepted 
    test_move(&mut seq, &mut map, &mut event_sender, tc0);

    let tc1 = TestConditions::General{
        original_tile : tc0.target_tile,
        head_provenance : tc0.head_going_to,
        original_position : tc0.target_position, 
        target_tile  : TileType::Free,
        head_going_to : Direction::Up,
        target_position : Coordinates{x :11, y:11},
        separator: None,
        backward: None
    };

    // Test 2: Chosen direction refused because it's backward leads to move to free tile
    test_move(&mut seq, &mut map, &mut event_sender, tc1);

    let tc2 = TestConditions::General{
        original_tile : tc1.target_tile,
        head_provenance : tc1.head_going_to,
        original_position : tc1.target_position, 
        target_tile  : TileType::Free,
        head_going_to : Direction::Down,
        target_position : Coordinates{x :11, y:11},
        separator: None,
        backward: Some(TestConditions::Backward{alt_direction: Direction::Left, alt_target_position : Coordinates{x :10, y:11}, alt_target_tile : TileType::Free, picker_ctx: &picker_ctx})
    };

    test_move(&mut seq, &mut map, &mut event_sender, tc2);


    let mut simple_head = SimpleHead::new(0, tc0.original_position, tc0.head_provenance,  event_sender);

    dispatch_head_evt(tc0, &mut map, &mut simple_head);
    dispatch_head_evt(tc1, &mut map, &mut simple_head);
    dispatch_head_evt(tc2, &mut map, &mut simple_head);


}

fn dispatch_head_evt(tc: TestConditions::General, map: &mut MockMap, simple_head: &mut SimpleHead) {
    let event = HeadEvents::MOVE_HEAD { direction: Some(tc.head_going_to), prohibited_directions : DirectionFlags::empty(),  map: map};
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

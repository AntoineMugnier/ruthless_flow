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


    fn move_head_handler(&mut self, direction: Option<Direction>, mut prohibited_directions : DirectionFlags, map: &mut impl Map) {

        // Prevent head from going back to its previous path
        prohibited_directions.insert(self.coming_from);

        // Select a random direction if no one has been set
        let proposed_direction;
        if let Some(direction) = direction {
            proposed_direction = direction;
            prohibited_directions.insert(proposed_direction);
        } else {
            proposed_direction = DirectionPicker::pick(&mut prohibited_directions);
        }

        // Try to explore explore the `proposed_direction`. If the move is impossible, explore all the other authorized directions around the head.
        let (chosen_direction, target_tile, target_position) = Self::explore_direction(self.get_position(), proposed_direction, &mut prohibited_directions, map);

        // Order the board to create a new head if the tile on which we are on a separator
        if map.get_tile(self.position) == TileType::Separator {
            let add_head_event = BoardEvevents::ADD_HEAD {
                position: self.get_position(),
                coming_from: self.get_provenance(),
                parent_direction: chosen_direction,
            };
            self.events_sender.send(add_head_event).unwrap();
        }

        //Special action depending on the type of tile we reach
        match target_tile{
            // Order the board to kill self
            TileType::Marked => {
                let remove_head_event = BoardEvevents::KILL_HEAD { id: self.id };
                self.events_sender.send(remove_head_event).unwrap();
            }
            // Move the head to the location and mark the tile
            TileType::Free | TileType::Separator =>  {
                map.set_tile(target_position, TileType::Marked);
                self.set_position(target_position); 
                self.set_provenance(chosen_direction);
            }
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
    ) -> SimpleHead {
        SimpleHead {
            id,
            position,
            coming_from,
            events_sender
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
    use crate::{map::{MockMap}, direction_picker::DirectionPicker};
    use crate::mpsc::SendError;
    use super::*;

    /*
        //let picker_ctx = DirectionPicker::pick_context();
        //picker_ctx.expect().times(1).in_sequence(&mut seq).returning(|_| Direction::Up);
*/


#[test]
fn test_move_head_to_free_tile_from_free_tile() {
    // Move from a free tile to a free tile, in one single shot
    {
        let mut seq = Sequence::new();

        //Initialize mocks
        let mut map = MockMap::default();
        let mut event_sender = Sender::default();

        //Test constants
        const ORIGINAL_TILE : TileType = TileType::Free;
        const HEAD_COMING_FROM : Direction = Direction::Up;
        const HEAD_ORIGINAL_POSITION: Coordinates = Coordinates{x :10, y:10}; 
        const TARGET_TILE : TileType = TileType::Free;
        const HEAD_GOING_TO : Direction = Direction::Right;
        const HEAD_TARGET_POSITION : Coordinates = Coordinates{x :11, y:10};

        // Test flow
        map.expect_get_neighbour_tile().times(1).in_sequence(&mut seq).return_const(Some((TARGET_TILE, HEAD_TARGET_POSITION)));
        map.expect_get_tile().times(1).in_sequence(&mut seq).withf(|position| {*position == HEAD_ORIGINAL_POSITION} ).return_const(ORIGINAL_TILE);
        map.expect_set_tile().times(1).in_sequence(&mut seq).withf(|position, tile_type| {*position == HEAD_TARGET_POSITION && *tile_type == TileType::Marked}).return_const(());
        
        let event = HeadEvents::MOVE_HEAD { direction: Some(HEAD_GOING_TO), prohibited_directions : DirectionFlags::empty(),  map: &mut map};
        let mut simple_head = SimpleHead::new(0, HEAD_ORIGINAL_POSITION, HEAD_COMING_FROM, event_sender);
        
        simple_head.dispatch(event);
    }
}
    #[test]
    fn test_move_head_to_free_tile_from_separator() {
        // Move from a separator tile to a free tile, in one single shot
        {
            let mut seq = Sequence::new();

            //Initialize mocks
            let mut map = MockMap::default();
            let mut event_sender = Sender::default();

            //Test constants
            const ORIGINAL_TILE : TileType = TileType::Separator;
            const HEAD_COMING_FROM : Direction = Direction::Up;
            const HEAD_ORIGINAL_POSITION: Coordinates = Coordinates{x :10, y:10};
            const TARGET_TILE : TileType = TileType::Free;
            const HEAD_GOING_TO : Direction = Direction::Up;
            const HEAD_TARGET_POSITION : Coordinates = Coordinates{x :10, y:11};

            // Test flow
            map.expect_get_neighbour_tile().times(1).in_sequence(&mut seq).return_const(Some((TARGET_TILE, HEAD_TARGET_POSITION)));
            map.expect_get_tile().times(1).in_sequence(&mut seq).withf(|position| {*position == HEAD_ORIGINAL_POSITION} ).return_const(ORIGINAL_TILE);
            event_sender.expect_send().times(1).in_sequence(&mut seq).withf(|board_event| {
                match board_event{
                BoardEvevents::ADD_HEAD { position:HEAD_ORIGINAL_POSITION, coming_from: HEAD_COMING_FROM, parent_direction: HEAD_GOING_TO} => return true,
                _ => return false
            }
                }).return_const(Result::<(), SendError<BoardEvevents>>::Ok(()));
            map.expect_set_tile().times(1).in_sequence(&mut seq).withf(|position, tile_type| {*position == HEAD_TARGET_POSITION && *tile_type == TileType::Marked}).return_const(());
            
            let event = HeadEvents::MOVE_HEAD { direction: Some(HEAD_GOING_TO), prohibited_directions : DirectionFlags::empty(),  map: &mut map};
            let mut simple_head = SimpleHead::new(0, HEAD_ORIGINAL_POSITION, HEAD_COMING_FROM, event_sender);
            
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
}

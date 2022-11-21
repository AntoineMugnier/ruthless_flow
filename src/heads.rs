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

        let original_tile = map.get_tile(self.position);
        let original_position = self.get_position();
        let original_provenance = self.get_provenance();

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
                position: original_position,
                coming_from: original_provenance,
                parent_direction: chosen_direction,
            };
            self.events_sender.send(add_head_event).unwrap();
        }

        //Special action depending on the type of tile we reach
        match target_tile{
            // Order the board to kil self
            TileType::Marked => {
                let remove_head_event = BoardEvevents::KILL_HEAD { id: self.id };
                self.events_sender.send(remove_head_event).unwrap();
        }
            // Move the head to the location and mark the tile
            TileType::Free | TileType::Separator =>  {
                map.set_tile(self.position, TileType::Marked);
                self.set_position(target_position); 
                self.coming_from = chosen_direction;
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


    
    #[test]
    fn test_move_heads() {

                //let picker_ctx = DirectionPicker::pick_context();
/*
        //Create mock map
        {
            let event_sender = Sender::default();
            let mut seq = Sequence::new();
            let mut map = MockMap::default();

            //Direct move to free tile
            let mut simple_head = SimpleHead::new(0, Coordinates{x:0, y:0}, Direction::Down, event_sender);
            map.expect_get_neighbour_tile().times(1).in_sequence(&mut seq).returning(|position, direction| (Some((TileType::Free, Coordinates{x:1, y:0}))));
            map.expect_set_tile().times(1).in_sequence(&mut seq).withf(|position, tile_type| {position.x == 1 && position.y == 0 && *tile_type == TileType::Marked}).return_const(());
            let event = HeadEvents::MOVE_HEAD { direction: Some(Direction::Down), prohibited_directions : DirectionFlags::empty(),  map: &mut map};
            simple_head.dispatch(event);
        }
        //let picker_ctx = DirectionPicker::pick_context();
*/
        {
            let mut seq = Sequence::new();
            let mut map = MockMap::default();
            let mut event_sender = Sender::default();

            map.expect_get_tile().times(1).in_sequence(&mut seq).return_const(TileType::Separator);
            map.expect_get_neighbour_tile().times(1).in_sequence(&mut seq).returning(|position, direction| (Some((TileType::Separator, Coordinates{x:1, y:0}))));
            event_sender.expect_send().times(1).in_sequence(&mut seq).withf(|board_event| {
                match board_event{
                BoardEvevents::ADD_HEAD { position: Coordinates{x:1, y:0}, coming_from: Direction::Up, parent_direction: Direction::Left} => return true,
                _ => return false
            }
                }).return_const(Result::<(), SendError<BoardEvevents>>::Ok(()));
            map.expect_set_tile().times(1).in_sequence(&mut seq).withf(|position, tile_type| {position.x == 1 && position.y == 0 && *tile_type == TileType::Marked}).return_const(());
            let event = HeadEvents::MOVE_HEAD { direction: Some(Direction::Down), prohibited_directions : DirectionFlags::empty(),  map: &mut map};
            let mut simple_head = SimpleHead::new(0, Coordinates{x:0, y:0}, Direction::Down, event_sender);
            simple_head.dispatch(event);
        }

        //picker_ctx.expect().times(1).in_sequence(&mut seq).returning(|_| Direction::Up);

        // Check prohibited input_dir
        // Chek move dir to wall 
        // Move to Wall

        // Move to map edge

        // Move to Marked tile

        // Move to Free tile

        // Move to Separator
    }
}

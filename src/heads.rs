use enumflags2::{make_bitflags, BitFlag};
use std::cell::{RefCell, UnsafeCell};
use std::sync::mpsc::{channel, Sender};
use std::u8;

use crate::board::BoardEvevents;
use crate::map::{Map, TileType};
use crate::utils::{Coordinates, Direction, DirectionFlags};
use rand::{thread_rng, Rng};

#[cfg(not(test))]
type DirectionPicker = crate::direction_picker::DirectionPicker;

#[cfg(test)]
type DirectionPicker = crate::direction_picker::MockDirectionPicker;

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
    HAS_MOVED,
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
        fn move_and_mark_tile(
            &mut self,
            position: Coordinates,
            direction: Direction,
            map: &mut impl Map,
        ) -> HeadAction;
        fn move_head_handler(&mut self, direction: Option<Direction>, prohibited_directions : DirectionFlags, map: &mut impl Map);
        fn try_explore_direction(
            &mut self,
            choosen_direction: Direction,
            prohibited_directions: &mut DirectionFlags,
            map: &mut impl Map,
        ) -> Direction;
        fn explore_direction(&mut self, direction: Direction, map: &mut impl Map) -> HeadAction;
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

    fn move_and_mark_tile(
        &mut self,
        position: Coordinates,
        direction: Direction,
        map: &mut impl Map,
    ) -> HeadAction {
        self.coming_from = direction;
        self.position = position;

        map.set_tile(position, TileType::Marked);
        HeadAction::HAS_MOVED
    }


    fn move_head_handler(&mut self, direction: Option<Direction>, mut prohibited_directions : DirectionFlags, map: &mut impl Map) {
        let head_original_position = self.get_position();
        let head_original_provenance = self.get_provenance();

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

        let chosen_direction = self.try_explore_direction(proposed_direction, &mut prohibited_directions, map);

        let new_tile_type = map.get_tile(self.get_position());

        match new_tile_type {
            TileType::Separator => {
                let add_head_event = BoardEvevents::ADD_HEAD {
                    position: head_original_position,
                    coming_from: head_original_provenance,
                    parent_direction: chosen_direction,
                };
                self.events_sender.send(add_head_event).unwrap();
            }
            TileType::Marked => {
                let remove_head_event = BoardEvevents::KILL_HEAD { id: self.id };
                self.events_sender.send(remove_head_event).unwrap();
            }
            _ => {}
        }
    }

    fn try_explore_direction(
        &mut self,
        choosen_direction: Direction,
        prohibited_directions: &mut DirectionFlags,
        map: &mut impl Map,
    ) -> Direction {
        let status = self.explore_direction(choosen_direction, map);

        match status {
            HeadAction::HAS_NOT_MOVED => {
                let choosen_direction = DirectionPicker::pick(prohibited_directions);
                self.try_explore_direction(choosen_direction, prohibited_directions, map)
            }
            HeadAction::HAS_MOVED => return choosen_direction,
        }
    }

    fn explore_direction(&mut self, direction: Direction, map: &mut impl Map) -> HeadAction {
        let position = self.get_position();
        if let Some((tile_type, position)) = map.get_neighbour_tile(position, direction) {
            match tile_type {
                TileType::Free | TileType::Separator | TileType::Marked => {
                    self.move_and_mark_tile(position, direction, map)
                }
                TileType::Wall => HeadAction::HAS_NOT_MOVED,
            }
        }
        // We are targetting an edge of the map
        else {
            HeadAction::HAS_NOT_MOVED
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
            events_sender,
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
    use mockall::predicate;

    use crate::map::{SimpleMap, MockMap};
    use super::*;

    #[test]
    fn test_move_heads() {

        //Head construction
        let (event_sender, event_receiver) = channel();
        let mut simple_head = SimpleHead::new(0, Coordinates{x:0, y:0}, Direction::Down, event_sender);
        
        //Create mock map
        let mut map = MockMap::new();
        map.expect_get_tile();
        map.expect_get_neighbour_tile();
        map.expect_set_tile();

        // let event = HeadEvents::MOVE_HEAD { direction: Some(Direction::Down), map: &mut map};
        // simple_head.dispatch(event);

        // Move to Wall

        // Move to map edge

        // Move to Marked tile

        // Move to Free tile

        // Move to Separator
    }
}

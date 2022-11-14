use enumflags2::{make_bitflags, BitFlag};
use std::cell::{RefCell, UnsafeCell};
use std::sync::mpsc::{channel, Sender};
use std::u8;

use crate::board::BoardEvevents;
use crate::map::{Map, TileType};
use crate::utils::{Coordinates, Direction, DirectionFlags};
use rand::{thread_rng, Rng};
pub enum HeadEvents<'a, MapType: Map> {
    MOVE_HEAD {
        direction: Option<Direction>,
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
    use super::HeadAction;
    use crate::{
        map::Map,
        utils::{Coordinates, Direction, DirectionFlags},
    };

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
        fn pick_one_of_directions_left(unavailable_dirs: &mut DirectionFlags) -> Direction;
        fn move_head_handler(&mut self, direction: Option<Direction>, map: &mut impl Map);
        fn try_explore_direction(
            &mut self,
            choosen_direction: Direction,
            dir_flags: &mut DirectionFlags,
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

    fn pick_one_of_directions_left(unavailable_dirs: &mut DirectionFlags) -> Direction {
        // Full bitfield means that all dirs have already been explored, which should not be possible. If it is the case the map is ill-formed
        assert!(
            unavailable_dirs.contains(!DirectionFlags::all()),
            "No more available dirs to pick"
        );

        // Generate a vector containing all available directions
        let mut dir_vec = Vec::<Direction>::new();
        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if !unavailable_dirs.contains(dir) {
                dir_vec.push(dir)
            }
        }

        // Select a radom direction among available ones
        let mut rng = thread_rng();
        let random_index = rng.gen_range(0..dir_vec.len());
        let picked_direction = dir_vec[random_index];

        // Make the direction unavailable in dir_flags
        unavailable_dirs.insert(picked_direction);

        picked_direction
    }

    fn move_head_handler(&mut self, direction: Option<Direction>, map: &mut impl Map) {
        let head_original_position = self.get_position();
        let head_original_provenance = self.get_provenance();

        // Create the bitfield to hold already explored or forbidden directions
        let mut dir_flags = DirectionFlags::empty();

        // Prevent head from going back to its previous path
        dir_flags.insert(self.coming_from);

        // Select a random direction if no one has been set
        let proposed_direction;
        if let Some(direction) = direction {
            proposed_direction = direction;
            dir_flags.insert(proposed_direction);
        } else {
            proposed_direction = Self::pick_one_of_directions_left(&mut dir_flags);
        }

        let chosen_direction = self.try_explore_direction(proposed_direction, &mut dir_flags, map);

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
        dir_flags: &mut DirectionFlags,
        map: &mut impl Map,
    ) -> Direction {
        let status = self.explore_direction(choosen_direction, map);

        match status {
            HeadAction::HAS_NOT_MOVED => {
                let choosen_direction = Self::pick_one_of_directions_left(dir_flags);
                self.try_explore_direction(choosen_direction, dir_flags, map)
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
            HeadEvents::MOVE_HEAD { direction, map } => {
                private::Sealed::move_head_handler(self, direction, map)
            }
        };
    }
}

#[cfg(test)]
mod tests {}

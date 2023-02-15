
use super::direction_picker::DirectionPicker;
use crate::mpsc::Sender;
use super::map::{MapTrait, TileType};
use crate::utils::{Coordinates, Direction, DirectionFlags};
use crate:: backend::{self};

pub enum Event<'a, MapType: MapTrait> {
    MoveHead {
        direction: Direction,
        prohibited_directions : DirectionFlags, // bitfield to hold already explored or forbidden directions
        map: &'a mut MapType,
    },
}
pub type Id = u32;

pub struct SimpleHead {
    id: Id,
    position: Coordinates,
    coming_from: Direction,
    board_events_sender: Sender<backend::Event>,
    head_split : bool

}

pub trait Head: private::Sealed {
    fn new(
        id: Id,
        position: Coordinates,
        coming_from: Direction,
        board_events_sender: Sender<backend::Event>,
    ) -> Self;
    fn dispatch(&mut self, event: Event<impl MapTrait>);
    fn get_id(&mut self) -> Id;

}
mod private {
    use super::*;

    pub trait Sealed {
        fn set_position(&mut self, position: Coordinates);
        fn set_provenance(&mut self, coming_from: Direction);
        fn get_position(&self) -> Coordinates;
        fn get_provenance(&self) -> Direction;
        fn move_head_handler(&mut self, direction: Direction, prohibited_directions : DirectionFlags, map: &mut impl MapTrait);
        fn explore_direction(
            &self,
            position: Coordinates,
            chosen_direction: Direction,
            prohibited_directions: &mut DirectionFlags,
            map: &mut impl MapTrait,
        ) -> Option<(Direction, TileType, Coordinates)>;
        fn move_and_mark_tile(&mut self, map: &mut impl MapTrait, target_position: Coordinates, chosen_direction: Direction);
        fn continue_exploring_if_any_direction_is_available(&self,
            original_position: Coordinates,
              prohibited_directions: &mut DirectionFlags,
              map: &mut impl MapTrait) -> Option<(Direction, TileType, Coordinates)>;
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

    fn move_and_mark_tile(&mut self, map: &mut impl MapTrait, target_position: Coordinates, chosen_direction: Direction) {
        map.set_tile(target_position, TileType::Head { id: self.id });
        self.set_position(target_position);
        self.set_provenance(chosen_direction.reverse());
    }

    fn move_head_handler(&mut self, direction: Direction, mut prohibited_directions : DirectionFlags, map: &mut impl MapTrait) {

        // Prevent head from going back to its previous path
        prohibited_directions.insert(self.coming_from); 

        // Check if the direction sent to the function is not prohibited
        let proposed_direction;
            if prohibited_directions.contains(direction){
                if let Some(picked_dir) = DirectionPicker::pick(&mut prohibited_directions){
                    proposed_direction = picked_dir;
                }
                //If we run out of possible moves, the head does not move this turn
                else{
                    return
                }
            }
            else{
                proposed_direction = direction;
                prohibited_directions.insert(proposed_direction);
            }

        // Try to explore explore the `proposed_direction`. If the move is impossible, explore all the other authorized directions around the head.
        if let Some((chosen_direction, target_tile, target_position)) = self.explore_direction(self.get_position(), proposed_direction, &mut prohibited_directions, map){

            // Order the backend to create a new head if the tile on which we are on a separator
            if self.head_split {
                let add_head_event = backend::Event::AddHead {
                    position: self.get_position(),
                    coming_from: self.get_provenance(),
                    parent_direction: chosen_direction,
                };
                self.board_events_sender.send(add_head_event).unwrap();
                self.head_split = false;
            }

            //Set he current tile we leave as marked
            map.set_tile(self.position, TileType::Marked{id: self.id});

            //Special action depending on the type of tile we reach
            match target_tile{
                // Order the backend to kill self
                TileType::Marked{..} | TileType::Head{..} => {
                    let remove_head_event = backend::Event::KillHead { id: self.id };
                    self.board_events_sender.send(remove_head_event).unwrap();
                }
                // Move the head to the location and mark the tile
                TileType::Free =>  {
                    self.move_and_mark_tile(map, target_position, chosen_direction);
                },
                TileType::Separator =>  {
                    self.move_and_mark_tile(map, target_position, chosen_direction);
                    self.head_split = true;
                },
                TileType::Wall => panic!("Cannot move to a wall"),
            }

            // Check if the head has win the game
             if map.is_on_arrival_line(self.position){
                let event = backend::Event::EndGame{end_game_reason : backend::EndGameReason::Victory};
                self.board_events_sender.send(event).unwrap();
             }
        }
        
    }
    fn continue_exploring_if_any_direction_is_available(&self,
        original_position: Coordinates,
          prohibited_directions: &mut DirectionFlags,
          map: &mut impl MapTrait) -> Option<(Direction, TileType, Coordinates)>{
        if let Some(picked_dir) = DirectionPicker::pick(prohibited_directions){
            return self.explore_direction(original_position, picked_dir, prohibited_directions, map)
        }
        else{
            return None;
        }
    }

    
    fn explore_direction(&self,
        original_position: Coordinates,
        chosen_direction: Direction,
        prohibited_directions: &mut DirectionFlags,
        map: &mut impl MapTrait,
    ) -> Option<(Direction, TileType, Coordinates)> {

        if let Some((tile_type, target_position)) = map.get_neighbour_tile(original_position, chosen_direction) {
            match tile_type {
                TileType::Free | TileType::Separator => return Some((chosen_direction, tile_type, target_position)),
                TileType::Marked{id} =>{ 
                    if id == self.id{
                       self.continue_exploring_if_any_direction_is_available(original_position, prohibited_directions, map)
                    }
                    else{
                        return Some((chosen_direction, tile_type, target_position))
                    }
            },
                TileType::Wall | TileType::Head{..}=> {
                    self.continue_exploring_if_any_direction_is_available(original_position, prohibited_directions, map)
                },
            }
        }
        // We are targetting an edge of the map
        else {
            self.continue_exploring_if_any_direction_is_available(original_position, prohibited_directions, map)
        }
    }
}

impl Head for SimpleHead {
    fn new(
        id: Id,
        position: Coordinates,
        coming_from: Direction,
        board_events_sender: Sender<backend::Event>,
    ) -> SimpleHead { // TODO, initialize with map
        SimpleHead {
            id,
            position,
            coming_from,
            board_events_sender,
            head_split : false
        }
    }

    fn get_id(&mut self) -> Id{
        self.id
    }

    fn dispatch(&mut self, event: Event<impl MapTrait>) {
        match event {
            Event::MoveHead { direction, prohibited_directions, map } => {
                private::Sealed::move_head_handler(self, direction,prohibited_directions, map)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use mockall::{Sequence};
    use super::super::map::MockMapTrait;
    use crate::{mpsc::{MockSender, SendError}, backend::{self}};

    use super::*;

mod test_conditions{
use super::super::super::direction_picker::PickerCtx;

use super::*;

pub struct General<'a>{
    pub head_id : Id,
    pub previous_way : Way,
    pub first_stage: FirstStage<'a>,
    pub to_wall: Option<ToWall<'a>>,
    pub on_separator: Option<OnSeparator>,
    pub last_stage :LastStage,
    pub finish_on_arrival_line :Option<FinishOnArrivalLine>
}


pub enum FirstStage<'a>{
    ValidDir{way: Way},
    InvalidDir{way: Way, picker_ctx : &'a PickerCtx},
}

pub struct OnSeparator{}
#[derive(PartialEq)]
pub struct FinishOnArrivalLine{}

pub enum LastStage{
    ToMarked{head_id : Id},
    ToFree{head_id: Id},
    ToSeparator{head_id: Id},

}

#[derive(Copy, Clone)]
pub struct Way{pub alt_direction : Direction, pub alt_target_position : Coordinates, pub alt_target_tile : TileType}

pub struct ToWall<'a>{pub ways: Vec<Way> ,pub picker_ctx : &'a PickerCtx}


}

fn test_move(seq : & mut Sequence, map: & mut  MockMapTrait, backend_events_sender : &mut MockSender<backend::Event>, tc: &test_conditions::General){
    let original_position = tc.previous_way.alt_target_position;
    let original_direction = tc.previous_way.alt_direction;

    let mut target_direction;
    let mut target_position;
    let mut target_tile;
    let head_id = tc.head_id;

    match tc.first_stage{
        test_conditions::FirstStage::InvalidDir { way, picker_ctx } => {
        let alt_direction = way.alt_direction;
        picker_ctx.expect().once().in_sequence(seq).returning(move |_| Some(alt_direction));
        target_direction = way.alt_direction;
        target_tile =  way.alt_target_tile;
        target_position =  way.alt_target_position;
    },
        test_conditions::FirstStage::ValidDir {way} =>{
        target_direction = way.alt_direction;
        target_position = way.alt_target_position;
        target_tile = way.alt_target_tile;
    }}
    
    map.expect_get_neighbour_tile().once().in_sequence(seq)
    .withf(move |position, direction| {*position == original_position && *direction == target_direction} ).
    return_const(Some((target_tile, target_position)));
    
    if let Some(to_wall)= &tc.to_wall{
        for way in to_wall.ways.iter(){
            target_direction = way.alt_direction;
            target_tile =  way.alt_target_tile;
            target_position =  way.alt_target_position;
            to_wall.picker_ctx.expect().once().in_sequence(seq).returning(move |_| Some(target_direction));

            map.expect_get_neighbour_tile().once().in_sequence(seq)
            .withf(move |position, direction| { *position == original_position && *direction == target_direction} ).
            return_const(Some((target_tile, target_position)));
        }
    }

    if let Some(_) = tc.on_separator{
        backend_events_sender.expect_send().once().in_sequence(seq).withf(move |board_event| {
            match board_event{
            backend::Event::AddHead { position, coming_from, parent_direction} => *position==original_position && *coming_from == original_direction.reverse() && *parent_direction==target_direction ,
            _ => return false
        }
        }).return_const(Result::<(), SendError<backend::Event>>::Ok(()));
    }

    map.expect_set_tile().once().in_sequence(seq)
    .withf(move |position, tile_type| {*position == original_position && *tile_type == TileType::Marked{id: head_id}})
    .return_const(());

    match tc.last_stage {
        test_conditions::LastStage::ToMarked{head_id:expected_id} => {
            backend_events_sender.expect_send().once().in_sequence(seq).withf(move |board_event| {
                match board_event{
                backend::Event::KillHead {id} =>  *id == expected_id,
                _ => return false
            }}
            ).return_const(Result::<(), SendError<backend::Event>>::Ok(()));
        },
        test_conditions::LastStage::ToFree{head_id} | test_conditions::LastStage::ToSeparator{head_id} => {
            map.expect_set_tile().once().in_sequence(seq)
            .withf(move |position, tile_type| {*position == target_position && *tile_type == TileType::Head { id : head_id}})
            .return_const(());
        }
    };
    
    map.expect_is_on_arrival_line().once().in_sequence(seq)
    .return_const( tc.finish_on_arrival_line == Some(test_conditions::FinishOnArrivalLine{}));

    match tc.finish_on_arrival_line{
        Some(_) => {
            backend_events_sender.expect_send().once().in_sequence(seq).withf(move |board_event| {
                match board_event{
                backend::Event::EndGame {end_game_reason : backend::EndGameReason::Victory} =>  return true,
                _ => return false
            }}
            ).return_const(Result::<(), SendError<backend::Event>>::Ok(()));
        },
        None => {},
    }
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
    let mut map = MockMapTrait::default();
    let mut backend_events_sender = Sender::default();
    let picker_ctx  = DirectionPicker::pick_context();
    let head_id = 524;
    let foreign_head_id = 326;

    // Test 0, Starting on Free Tile, normal move to free tile with chosen direction accepted
    let previous_way_0 = test_conditions::Way{alt_direction: Direction::Up, alt_target_position :Coordinates{x :10, y:10}, alt_target_tile : TileType::Free};
    let target_way_0 = test_conditions::Way{alt_direction: Direction::Right, alt_target_position : Coordinates{x :11, y:10}, alt_target_tile : TileType::Free};

    let tc0 = test_conditions::General{
        head_id, 
        previous_way: previous_way_0,
        first_stage: test_conditions::FirstStage::ValidDir{way: target_way_0},
        to_wall: None,
        on_separator: None,
        last_stage : test_conditions::LastStage::ToFree{head_id},
        finish_on_arrival_line: None
    };

    test_move(&mut seq, &mut map, &mut backend_events_sender, &tc0);
    
    // Test 1: Normal move to free tile with chosen direction accepted 
    let previous_way_1 = target_way_0;
    let target_way_1 = test_conditions::Way{alt_direction: Direction::Up, alt_target_position : Coordinates{x :11, y:11}, alt_target_tile : TileType::Free};

    let tc1 = test_conditions::General{
        head_id, 
        previous_way : previous_way_1,
        first_stage: test_conditions::FirstStage::ValidDir{
            way: target_way_1},  
        to_wall : None,
        on_separator: None,
        last_stage : test_conditions::LastStage::ToFree{head_id},
        finish_on_arrival_line: None
    };

    test_move(&mut seq, &mut map, &mut backend_events_sender, &tc1);

    // Test 2: Chosen direction refused because it's backward leads to move to free tile
    let previous_way_2 = target_way_1;
    let backward_way_2 = target_way_1.alt_direction.reverse();
    let target_way_2 = test_conditions::Way{alt_direction: Direction::Left, alt_target_position : Coordinates{x :10, y:11}, alt_target_tile : TileType::Free};
    
    let tc2 = test_conditions::General{
        head_id, 
        previous_way : previous_way_2,
        first_stage: test_conditions::FirstStage::InvalidDir{way: target_way_2 , picker_ctx: &picker_ctx},
        to_wall: None,
        on_separator: None,
        last_stage : test_conditions::LastStage::ToFree{head_id},
        finish_on_arrival_line: None
    };

    test_move(&mut seq, &mut map, &mut backend_events_sender, &tc2);

    // Test 3: Chosen direction is refused because of a wall 
    let previous_way_3 = target_way_2;
    let failed_target_way_3 = test_conditions::Way{alt_direction: Direction::Left, alt_target_position : Coordinates{x :9, y:11}, alt_target_tile : TileType::Wall}; 
    let target_way_3 = test_conditions::Way{alt_direction: Direction::Up, alt_target_position : Coordinates{x :10, y:12}, alt_target_tile : TileType::Separator};

    let tc3 = test_conditions::General{
        head_id, 
        previous_way : previous_way_3,
        first_stage: test_conditions::FirstStage::ValidDir{way: failed_target_way_3},
        to_wall: Some(test_conditions::ToWall{ways: vec![target_way_3], picker_ctx: &picker_ctx}),
        on_separator: None,
        last_stage : test_conditions::LastStage::ToSeparator{head_id},
        finish_on_arrival_line: None

    };
    test_move(&mut seq, &mut map, &mut backend_events_sender, &tc3);
    

    // Test 4: Chosen direction leads to free tile
    let previous_way_4 = target_way_3;
    let target_way_4 = test_conditions::Way{alt_direction: Direction::Right, alt_target_position : Coordinates{x :8, y:10}, alt_target_tile : TileType::Free};
    
    let tc4 = test_conditions::General{
        head_id, 
        previous_way : previous_way_4,
        first_stage: test_conditions::FirstStage::ValidDir { way: target_way_4},
        to_wall: None,
        on_separator: Some(test_conditions::OnSeparator{}),
        last_stage : test_conditions::LastStage::ToFree{head_id},
        finish_on_arrival_line: None
    };
    test_move(&mut seq, &mut map, &mut backend_events_sender, &tc4);

    // Test 5: Chosen direction leads to marked tile and then to merge
    let previous_way_5 = target_way_4;
    let target_way_5 = test_conditions::Way{alt_direction: Direction::Down, alt_target_position : Coordinates{x :8, y:9}, alt_target_tile : TileType::Marked{id : foreign_head_id}};
    let tc5 = test_conditions::General{
        head_id, 
        previous_way : previous_way_5,
        first_stage: test_conditions::FirstStage::ValidDir { way: target_way_5},
        on_separator: None,
        to_wall: None,
        last_stage : test_conditions::LastStage::ToMarked{head_id},
        finish_on_arrival_line: None
    };
    test_move(&mut seq, &mut map, &mut backend_events_sender, &tc5);

    let mut simple_head = SimpleHead::new(head_id, previous_way_0.alt_target_position, previous_way_0.alt_direction,  backend_events_sender);

    dispatch_head_evt(target_way_0.alt_direction, &mut map, &mut simple_head);
    dispatch_head_evt(target_way_1.alt_direction, &mut map, &mut simple_head);
    dispatch_head_evt(backward_way_2, &mut map, &mut simple_head);
    dispatch_head_evt(failed_target_way_3.alt_direction, &mut map, &mut simple_head);
    dispatch_head_evt(target_way_4.alt_direction, &mut map, &mut simple_head);
    dispatch_head_evt(target_way_5.alt_direction, &mut map, &mut simple_head);
}

#[test]
fn test_reaching_arrival_line(){
    let mut seq = Sequence::new();
    let mut map = MockMapTrait::default();
    let mut backend_events_sender = Sender::default();
    let picker_ctx  = DirectionPicker::pick_context();
    let head_id = 524;

    // Test 6: Chosen direction leads to marked tile and then to merge
    let previous_way_6 = test_conditions::Way{alt_direction: Direction::Up, alt_target_position :Coordinates{x :10, y:10}, alt_target_tile : TileType::Free};
    let wrong_target_way_6 = test_conditions::Way{alt_direction: Direction::Left, alt_target_position : Coordinates{x :4, y:6}, alt_target_tile : TileType::Marked { id: head_id }};
    let target_way_6 = test_conditions::Way{alt_direction: Direction::Right, alt_target_position : Coordinates{x :6, y:5}, alt_target_tile : TileType::Free};

    let tc6 = test_conditions::General{
        head_id, 
        previous_way : previous_way_6,
        first_stage: test_conditions::FirstStage:: ValidDir { way: wrong_target_way_6 },
        on_separator: None,
        to_wall: Some(test_conditions::ToWall{ways: vec![target_way_6], picker_ctx: &picker_ctx}),
        last_stage : test_conditions::LastStage::ToFree{head_id},
        finish_on_arrival_line: Some(test_conditions::FinishOnArrivalLine {})
    };

    test_move(&mut seq, &mut map, &mut backend_events_sender, &tc6);

    let mut simple_head = SimpleHead::new(head_id, previous_way_6.alt_target_position, previous_way_6.alt_direction,  backend_events_sender);
    dispatch_head_evt(wrong_target_way_6.alt_direction, &mut map, &mut simple_head);
}

fn dispatch_head_evt(head_going_to: Direction, map: &mut MockMapTrait, simple_head: &mut SimpleHead) {
    let event = Event::MoveHead { direction: head_going_to, prohibited_directions : DirectionFlags::empty(),  map: map};
    simple_head.dispatch(event);
}
    
}

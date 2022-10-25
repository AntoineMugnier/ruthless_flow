use crate::utils::{Direction, Coordinates};
use crate::map::{TileType, Map};

pub struct Head{
    position: Coordinates,
    coming_from : Direction
}

pub enum HeadAction {
    HAS_NOT_MOVED,
    HAS_MOVED,
    MERGED,
}

pub enum HeadState {
    TO_KILL,
    TO_KEEP,
}

impl Head {
    pub fn new(position: Coordinates, coming_from: Direction) -> Head {
        Head{position, coming_from}
    }

    pub fn set_position(&mut self, position: Coordinates){
        self.position = position;
    }

    pub fn set_provenance(&mut self, coming_from: Direction){
        self.coming_from = coming_from;
    }


    pub fn get_position(&self) -> Coordinates{
        self.position 
    }


    pub fn get_provenance(&self) -> Direction{
        self.coming_from 
    }

    fn move_and_mark_tile(&mut self, position: Coordinates, direction : Direction, map : &mut Map) -> HeadAction{

        self.coming_from = direction;
        self.position = position;

        map.set_tile(position, TileType::Marked);
        HeadAction::HAS_MOVED
    }
/* 
    fn split_heads_on_separator(&mut self){
        for head in self.heads.iter(){
            //self.heads.push(*head);
        }
    }
*/
    pub fn try_moving_to_direction(&mut self, direction : Direction, map : &mut Map) -> HeadState {
        loop{
            let status = self.explore_direction(direction, map);
                match status{
                    HeadAction::HAS_NOT_MOVED => {
                        self.explore_direction(direction, map);
                        return HeadState::TO_KEEP
                    },
                    HeadAction::HAS_MOVED => return HeadState::TO_KEEP,
                    HeadAction::MERGED => return HeadState::TO_KILL,
                }
            }
    }
    
    fn explore_direction(&mut self, direction : Direction, map : &mut Map) -> HeadAction{
        let position = self.get_position();
        if let Some((tile_type, position)) = map.get_tile(position, direction){
            match tile_type {
                TileType::Free =>{
                    self.move_and_mark_tile(position, direction, map)
                }
                TileType::Marked => {
                    HeadAction::MERGED
                },
                TileType::Separator => {
                    self.move_and_mark_tile(position, direction, map)
                },
                TileType::Wall => {
                    HeadAction::HAS_NOT_MOVED
                },
            }
        }
        // TileType is out of range
        else{
            HeadAction::HAS_NOT_MOVED
        }
    }
}

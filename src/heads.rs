use enumflags2::{make_bitflags, BitFlag};

use crate::utils::{Direction,DirectionFlags, Coordinates};
use crate::map::{TileType, Map};
use rand::{thread_rng, Rng};

#[derive(Clone)]
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


// TODO
// - Separator
// - random choice of dir
// 
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
 
    pub fn split_heads_if_on_separator(&mut self, map: &mut Map) -> Option<Head>{
        if map.get_tile(self.get_position()) == TileType::Separator{
            Some(self.clone())
        }
        else{
            None
        }
    }

    pub fn pick_one_of_directions_left(unavailable_dirs: &mut DirectionFlags) -> Direction{ 

        // Full bitfield means that all dirs have already been explored, which should not be possible. If it is the case the map is ill-formed
        assert!(unavailable_dirs.contains(!DirectionFlags::all()), "No more available dirs to pick"); 

        // Generate a vector containing all available directions
        let mut dir_vec = Vec::<Direction>::new();
        for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right]{
            if !unavailable_dirs.contains(dir){
                dir_vec.push(dir)
            }
        }

        // Select a radom direction among available ones
        let mut rng = thread_rng();
        let random_index = rng.gen_range(0..dir_vec.len());
        let picked_direction  =  dir_vec[random_index];

        // Make the direction unavailable in dir_flags
        unavailable_dirs.insert(picked_direction);

        picked_direction
    }

    pub fn try_moving_to_direction(&mut self, direction : Option<Direction>,  map : &mut Map) -> HeadState {

        // Create the bitfield to hold already explored or forbidden directions
        let mut dir_flags = DirectionFlags::empty();
        
        // Prevent head from going back to its previous path
        dir_flags.insert(self.coming_from); 

        // Select a random direction if no one has been set
        let choosen_direction;
        if let Some(direction) = direction{
            choosen_direction = direction;
            dir_flags.insert(choosen_direction);

        }
        else{
            choosen_direction = Self::pick_one_of_directions_left(&mut dir_flags);
        }
        
        let status = self.explore_direction(choosen_direction, map);

        match status{
            HeadAction::HAS_NOT_MOVED => {

                loop{
                    let choosen_direction = Self::pick_one_of_directions_left(&mut dir_flags);
                    if let HeadAction::HAS_NOT_MOVED = self.explore_direction(choosen_direction, map){
                        return HeadState::TO_KEEP
                    }

                }
            },
            HeadAction::HAS_MOVED => return HeadState::TO_KEEP,
            HeadAction::MERGED => return HeadState::TO_KILL,
        }
    }
    
    fn explore_direction(&mut self, direction : Direction, map : &mut Map) -> HeadAction{
        let position = self.get_position();
        if let Some((tile_type, position)) = map.get_neighbour_tile(position, direction){
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


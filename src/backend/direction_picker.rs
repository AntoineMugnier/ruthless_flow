
#[cfg(not(test))]
pub type DirectionPicker = private::DirectionPicker;
#[cfg(test)]
pub type DirectionPicker = private::MockDirectionPicker;
#[cfg(test)]
pub type PickerCtx = private::__mock_MockDirectionPicker::__pick::Context;

mod private{
    use rand::{thread_rng, Rng};
    use super::super::utils::{DirectionFlags, Direction};
    

    pub struct DirectionPicker{}

    #[cfg_attr(test, mockall::automock)]
    impl DirectionPicker{
    pub fn pick(prohibited_directions: &mut DirectionFlags) -> Direction {
        // Full bitfield means that all dirs have already been explored, which should not be possible. If it is the case the map is ill-formed
        assert!(
            prohibited_directions.contains(!DirectionFlags::all()),
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
            if !prohibited_directions.contains(dir) {
                dir_vec.push(dir)
            }
        }

        // Select a random direction among available ones
        let mut rng = thread_rng();
        let random_index = rng.gen_range(0..dir_vec.len());
        let picked_direction = dir_vec[random_index];

        // Make the direction unavailable in prohibited_directions
        prohibited_directions.insert(picked_direction);

        picked_direction
    }
}
}
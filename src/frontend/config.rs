

pub const WINDOW_LENGTH : u32 = 1920;
pub const WINDOW_HEIGHT : u32 = 1080;
pub const SCREEN_SIZE : [u32;2] = [WINDOW_LENGTH, WINDOW_HEIGHT];

pub mod map{
    pub const ORIGIN_X : f64 = 100.0;
    pub const ORIGIN_Y : f64 = 100.0;

    pub const END_X : f64 = 1000.0;
    pub const END_Y : f64 = 600.0;

    pub mod grid{
        pub const BAR_WIDTH : f64 = 1.0;
    }
}
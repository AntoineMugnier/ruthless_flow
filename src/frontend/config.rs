

pub const WINDOW_LENGTH : u32 = 1280;
pub const WINDOW_HEIGHT : u32 = 720;
pub const SCREEN_SIZE : [u32;2] = [WINDOW_LENGTH, WINDOW_HEIGHT];

pub mod map{
    pub const ORIGIN_X : f64 = 50.0;
    pub const ORIGIN_Y : f64 = 150.0;

    pub const END_X : f64 = 1050.0;
    pub const END_Y : f64 = 650.0;

    pub mod grid{
        pub const BAR_WIDTH : f64 = 1.0;
    }
}
pub mod game_info{
    pub const ORIGIN_X : f64 = 1100.0;
    pub const ORIGIN_Y : f64 = 150.0;

    pub const END_X : f64 = 1230.0;
    pub const END_Y : f64 = 325.0;
    
    pub mod frame{
        pub const BAR_WIDTH : f64 = 1.0;
    }

    pub mod dir{
        pub const TEXT_ORIGIN_X : f64 = super::ORIGIN_X + 20.0;
        pub const TEXT_ORIGIN_Y: f64 = super::ORIGIN_Y + 20.0;

    }
}

pub mod assets{
    pub const FONTS_PATH : &str = "assets/fonts/04B_30__.TTF";
    pub const UP_ARROW_PATH : &str = "assets/images/rust.png";

}

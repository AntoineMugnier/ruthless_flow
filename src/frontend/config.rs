

pub const WINDOW_LENGTH : u32 = 1280;
pub const WINDOW_HEIGHT : u32 = 720;
pub const SCREEN_SIZE : [u32;2] = [WINDOW_LENGTH, WINDOW_HEIGHT];

pub const BACKGROUND_COLOR: [f32; 4] = [0.12, 0.16, 0.21, 1.0];

pub mod map{
    pub const ORIGIN_X : f64 = 50.0;
    pub const ORIGIN_Y : f64 = 150.0;

    pub const END_X : f64 = 1050.0;
    pub const END_Y : f64 = 650.0;

    pub mod frame{
        pub const BAR_WIDTH : f64 = 0.5;
        pub const BAR_COLOR: [f32; 4] = [0.08, 0.12, 0.16, 1.0];

    }
    pub mod tiles{
        pub const WALL_COLOR: [f32; 4] = [0.16, 0.21, 0.29, 1.0];
        pub const SEPARATOR_COLOR: [f32; 4] = [0.0, 0.0, 255.0, 1.0];
        pub const HEAD_COLOR: [f32; 4] = [0.70, 0.11, 0.11, 1.0];
        pub const HEAD_MARK_COLOR: [f32; 4] = [0.42, 0.15, 0.15, 1.0];
        pub const FREE_COLOR: [f32; 4] = [255.0, 255.0, 255.0, 1.0];
    }
}
pub mod game_info{
    pub const ORIGIN_X : f64 = 1100.0;
    pub const ORIGIN_Y : f64 = super::map::ORIGIN_Y;

    pub const END_X : f64 = 1230.0;
    pub const END_Y : f64 = 400.0;

    pub const BACKGROUND_COLOR: [f32; 4] = [0.16, 0.21, 0.29, 1.0];
    pub const FONT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];


    pub mod frame{
        pub const BAR_WIDTH : f64 = 0.5;
        pub const BAR_COLOR: [f32; 4] = [0.08, 0.12, 0.16, 1.0];
    }

    pub mod time{
        pub const FONT_SIZE: u32 = 18;
        pub const ORIGIN_X : f64 = super::ORIGIN_X + 20.0;
        pub const ORIGIN_Y: f64 = super::ORIGIN_Y  + 40.0;
    }

    pub mod dir{
        pub const ORIGIN_X : f64 = super::ORIGIN_X + 22.0;
        pub const ORIGIN_Y: f64 = super::time::ORIGIN_Y + ((super::END_Y - super::ORIGIN_Y) as f64/3.0) ;

        pub mod arrow{
            pub const FONT_SIZE: u32 = 32;
            pub const ORIGIN_X : f64 = super::ORIGIN_X + 60.0;
            pub const ORIGIN_Y : f64 = super::ORIGIN_Y + 5.0;
        }
        pub mod text{
            pub const FONT_SIZE: u32 = 18;
            pub const ORIGIN_X : f64 = super::ORIGIN_X;
            pub const ORIGIN_Y: f64 = super::ORIGIN_Y;
        }
    }

    pub mod nb_heads{
        pub const FONT_SIZE: u32 = 18;
        pub const ORIGIN_X : f64 = super::ORIGIN_X + 20.0;
        pub const ORIGIN_Y: f64 = super::dir::ORIGIN_Y + ((super::END_Y - super::ORIGIN_Y) as f64/3.0) ;
    }

}

pub mod assets{
    pub const FONTS_PATH : &str = "assets/fonts/Cambria Bold 700.ttf";
}



pub const WINDOW_LENGTH : u32 = 1920;
pub const WINDOW_HEIGHT : u32 = 1080;
pub const SCREEN_SIZE : [u32;2] = [WINDOW_LENGTH, WINDOW_HEIGHT];

pub const BACKGROUND_COLOR: [f32; 4] = [0.12, 0.16, 0.21, 1.0];

pub mod map{
    pub const ORIGIN_X : f64 = 50.0;
    pub const ORIGIN_Y : f64 = 150.0;

    pub const LENGTH_X : f64 = 1650.0;
    pub const LENGTH_Y : f64 = 800.0;

    pub const END_X : f64 = ORIGIN_X + LENGTH_X;
    pub const END_Y : f64 = ORIGIN_Y + LENGTH_Y;

    pub mod frame{
        pub const BAR_WIDTH : f64 = 0.5;
        pub const BAR_COLOR: [f32; 4] = [0.08, 0.12, 0.16, 1.0];
    }
    pub mod tiles{
        pub const WALL_COLOR: [f32; 4] = [0.16, 0.21, 0.29, 1.0];
        pub const SEPARATOR_COLOR: [f32; 4] = [0.0, 0.0, 255.0, 1.0];
        pub const FREE_COLOR: [f32; 4] = [255.0, 255.0, 255.0, 1.0];

        pub const HEAD_COLOR_0: [f32; 4] = [0.70, 0.11, 0.11, 1.0];
        pub const HEAD_MARK_COLOR_0: [f32; 4] = [0.42, 0.15, 0.15, 1.0];
        pub const HEAD_COLOR_1: [f32; 4] = [0.11, 0.70, 0.11, 1.0];
        pub const HEAD_MARK_COLOR_1: [f32; 4] = [0.15, 0.42, 0.15, 1.0];
        pub const HEAD_COLOR_2: [f32; 4] = [0.70, 0.70, 0.11, 1.0];
        pub const HEAD_MARK_COLOR_2: [f32; 4] = [0.42, 0.42, 0.15, 1.0];
        pub const HEAD_COLOR_3: [f32; 4] = [0.11,  0.70,  0.70, 1.0];
        pub const HEAD_MARK_COLOR_3: [f32; 4] = [0.15, 0.42, 0.42, 1.0];
    }
    pub mod arrival_line{
        pub const COLOR: [f32; 4] = [1.0, 0.0, 0.0, 0.5];
    }
}

pub mod startup_screen{
    pub const ORIGIN_X : f64 = 360.0;
    pub const ORIGIN_Y : f64 = 210.0;

    pub const LENGTH_X : f64 = 1020.0;
    pub const LENGTH_Y : f64 = 700.0;

    pub const END_X : f64 = ORIGIN_X + LENGTH_X;
    pub const END_Y : f64 = ORIGIN_Y + LENGTH_Y;
    pub mod frame{
        pub const BAR_WIDTH : f64 = 0.5;
        pub const BAR_COLOR: [f32; 4] = [0.08, 0.12, 0.16, 1.0];
    }

    pub mod title{
        pub const ORIGIN_X : f64 = super::ORIGIN_X + 425.0;
        pub const ORIGIN_Y : f64 = super::ORIGIN_Y + 80.0;
        pub const FONT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        pub const FONT_SIZE: u32 = 36;
    }

    pub mod description{
        pub const ORIGIN_X : f64 = super::ORIGIN_X + 50.0;
        pub const ORIGIN_Y : f64 = super::ORIGIN_Y + 150.0;
        pub const FONT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        pub const FONT_SIZE: u32 = 18;
    }
    pub const BACKGROUND_COLOR: [f32; 4] = [0.4,0.4,0.4, 1.0];

}
pub mod title{
    pub const ORIGIN_X : f64 = 50.0;
    pub const ORIGIN_Y : f64 = 100.0;
    pub const FONT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    pub const FONT_SIZE: u32 = 48;

}
pub mod game_info{
    pub const ORIGIN_X : f64 = 1750.0;
    pub const ORIGIN_Y : f64 = super::map::ORIGIN_Y;

    pub const END_X : f64 = 1875.0;
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

pub mod end_game_box{
        pub const ORIGIN_X : f64 = super::map::ORIGIN_X + super::map::LENGTH_X/2.0 - LENGTH_X/2.0;
        pub const ORIGIN_Y : f64 = super::map::ORIGIN_Y + super::map::LENGTH_Y/2.0 - LENGTH_Y/2.0;
    
        pub const LENGTH_X : f64 = 550.0;
        pub const LENGTH_Y : f64 = 200.0;
    
        pub const END_X : f64 = ORIGIN_X + LENGTH_X;
        pub const END_Y : f64 = ORIGIN_Y + LENGTH_Y;

        pub mod frame{
            pub const BAR_WIDTH : f64 = 0.5;
            pub const BAR_COLOR: [f32; 4] = [0.08, 0.12, 0.16, 1.0];
        }
    
        pub mod title{
            pub const ORIGIN_X : f64 = super::ORIGIN_X + 195.0;
            pub const ORIGIN_Y : f64 = super::ORIGIN_Y + 60.0;
            pub const FONT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            pub const FONT_SIZE: u32 = 36;
        }
    
        pub mod description{
            pub const ORIGIN_X : f64 = super::ORIGIN_X + 60.0;
            pub const ORIGIN_Y : f64 = super::ORIGIN_Y + 150.0;
            pub const FONT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            pub const FONT_SIZE: u32 = 18;
        }
        pub const BACKGROUND_COLOR: [f32; 4] = [0.4,0.4,0.4, 1.0];

}
pub mod assets{
    pub const FONTS_PATH : &str = "assets/fonts/Cambria Bold 700.ttf";
}

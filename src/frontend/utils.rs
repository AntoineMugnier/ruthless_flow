use piston_window::{Context, G2d, line};

pub fn draw_frame(coordinates: [f64;4] ,color :[f32; 4], width : f64, c: &Context, g: &mut G2d){
    line(color, width,  [coordinates[0], coordinates[1], coordinates[0],  coordinates[3]], c.transform, g);
    line(color, width,  [coordinates[0], coordinates[3], coordinates[2],  coordinates[3]], c.transform, g);
    line(color, width,  [coordinates[2], coordinates[3], coordinates[2],  coordinates[1]], c.transform, g);
    line(color, width,  [coordinates[2], coordinates[1], coordinates[0],  coordinates[1]], c.transform, g);
}
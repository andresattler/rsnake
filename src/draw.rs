use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block(x: i32, y: i32, color: Color, con: &Context, g: &mut G2d) {
    rectangle(
        color,
        [to_coord(x), to_coord(y), BLOCK_SIZE - 1.0, BLOCK_SIZE - 1.0],
        con.transform,
        g,
    );
}

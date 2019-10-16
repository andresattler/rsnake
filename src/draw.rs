use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn draw_block(x: i32, y: i32, color: Color, con: &Context, g: &mut G2d) {
    rectangle(
        color,
        [to_coord(x), to_coord(y) , BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g
        );
}


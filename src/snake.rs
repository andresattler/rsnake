use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub y: i32,
    pub x: i32,
    pub dir: Direction,
}

const SNAKE_COLOR: Color = [0.0,0.4,0.4,1.0];

impl Snake  {
    pub fn new(x: i32, y: i32) -> Snake {
        Snake {
            x,
            y,
            dir: Direction::Left,
        }
    }
    pub fn update(&mut self) {
        let (dx, dy) = match self.dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        self.x += dx;
        self.y += dy;
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_block(self.x, self.y, SNAKE_COLOR, &con, g);
    }
}

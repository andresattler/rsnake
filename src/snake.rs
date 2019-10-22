use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::draw_block;
use std::ops::Neg;

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Direction {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

pub struct Block {
    pub x: i32,
    pub y: i32,
}

pub struct Snake {
    body: LinkedList<Block>,
    pub dir: Direction,
    growing: bool,
}

const SNAKE_COLOR: Color = [0.0, 0.4, 0.4, 1.0];

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });
        Snake {
            body,
            dir: Direction::Left,
            growing: false,
        }
    }
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }
    pub fn eat(&mut self) {
        self.growing = true;
    }
    pub fn update(&mut self) {
        let (dx, dy) = match self.dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let (head_x, head_y) = self.head_position();
        self.body.push_front(Block {
            x: head_x + dx,
            y: head_y + dy,
        });
        if self.growing == true {
            self.growing = false;
        } else {
            self.body.pop_back();
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(block.x, block.y, SNAKE_COLOR, &con, g);
        }
    }
}

use piston_window::*;

use crate::snake::{Direction};

pub struct Game {
    pub snake_y: i32,
    pub snake_x: i32,
    pub snake_dir: Direction,
    waiting_time: f64,
}

const STEP_PERIOD: f64 = 0.1;

impl Game {
    pub fn new(x: i32, y: i32) -> Game {
        Game {
            snake_y: y,
            snake_x: x,
            snake_dir: Direction::Up,
            waiting_time: 0.0,
        }
    }
    pub fn key_pressed(&mut self, key: Key) {
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None
        };
        self.snake_dir = dir.unwrap();
    }
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.waiting_time > STEP_PERIOD {
            let (x, y) = match self.snake_dir {
                Direction::Up => (0, -1),
                Direction::Down => (0, 1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };
            self.snake_x += x;
            self.snake_y += y;
            self.waiting_time = 0.0;
        }
    }
}

use piston_window::types::Color;
use piston_window::*;

use crate::draw::draw_block;
use crate::snake::{Direction, Snake};

pub struct Game {
    waiting_time: f64,
    pub snake: Snake,
}

const STEP_PERIOD: f64 = 0.1;
const FOOD_COLOR: Color = [0.9, 0.0, 0.0, 1.0];

impl Game {
    pub fn new(x: i32, y: i32) -> Game {
        Game {
            waiting_time: 0.0,
            snake: Snake::new(x, y),
        }
    }
    pub fn key_pressed(&mut self, key: Key) {
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };
        if let Some(new_dir) = dir {
            self.snake.dir = new_dir;
        }
    }
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.waiting_time > STEP_PERIOD {
            self.snake.update();
            self.waiting_time = 0.0;
        }
    }
    pub fn draw_food(con: &Context, g: &mut G2d) {
        draw_block(5, 5, FOOD_COLOR, &con, g);
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(&con, g);
        Game::draw_food(&con, g);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_game() -> Game {
        Game::new(4, 4)
    }

    #[test]
    // Make sure this does not panic
    fn test_key_pressed() {
        dummy_game().key_pressed(Key::A)
    }
}

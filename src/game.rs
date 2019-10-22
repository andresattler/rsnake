use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};

use crate::draw::draw_block;
use crate::snake::{Block, Direction, Snake};

pub struct Game {
    waiting_time: f64,
    snake: Snake,
    food: Block,
    width: i32,
    height: i32,
}

const STEP_PERIOD: f64 = 0.1;
const FOOD_COLOR: Color = [0.9, 0.0, 0.0, 1.0];

impl Game {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Game {
        Game {
            waiting_time: 0.0,
            snake: Snake::new(x, y),
            food: Block { x: 5, y: 5 },
            width,
            height,
        }
    }

    /// Checks whether or not the given Block collides with the
    /// given Snake.
    fn collides_with(block: Block, snake: &Snake) -> bool {
        snake.body.iter().any(|&b| b == block)
    }

    fn place_food(&mut self) {
        let mut rng = thread_rng();
        let mut random = |max| rng.gen_range(1, max - 1);
        while {
            self.food.x = random(self.width);
            self.food.y = random(self.height);
            Self::collides_with(self.food, &self.snake)
        } {}
    }
    /// Takes a pressed key (one of the arrow keys) and turns the snake around.
    /// The snake is not allowed to go in the opposite direction as it would bite itself.
    pub fn key_pressed(&mut self, key: Key) {
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };
        if let Some(new_dir) = dir {
            if -new_dir != self.snake.dir {
                self.snake.dir = new_dir;
            }
        }
    }
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.waiting_time > STEP_PERIOD {
            self.snake.update();
            let (snake_head_x, snake_head_y) = self.snake.head_position();
            if snake_head_x == self.food.x && snake_head_y == self.food.y {
                self.snake.eat();
                self.place_food();
            }
            self.waiting_time = 0.0;
        }
    }
    pub fn draw_food(&self, con: &Context, g: &mut G2d) {
        draw_block(self.food.x, self.food.y, FOOD_COLOR, &con, g);
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(&con, g);
        self.draw_food(&con, g);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_game() -> Game {
        Game::new(4, 4, 20, 20)
    }

    #[test]
    // Make sure this does not panic
    fn test_key_pressed() {
        dummy_game().key_pressed(Key::A)
    }
}

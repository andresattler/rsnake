extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use draw::draw_block;
use game::Game;

const BACKGROUND_COLOR: Color = [0.7, 0.7, 0.7, 1.0];

fn main() {
    let (width, height) = (750, 750);

    let mut window: PistonWindow = WindowSettings::new("Snake", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut game = Game::new(4,4);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g| {
            clear(BACKGROUND_COLOR, g);
            draw_block(game.snake_x, game.snake_y,[0.0,0.4,0.4,1.0], &c, g);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}

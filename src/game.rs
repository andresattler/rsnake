pub struct Game {
    pub snake_y: i32,
    pub snake_x: i32,
    waiting_time: f64,
}

const STEP_PERIOD: f64 = 0.1;

impl Game {
    pub fn new(x: i32, y: i32) -> Game {
        Game {
            snake_y: y,
            snake_x: x,
            waiting_time: 0.0,
        }
    }
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.waiting_time > STEP_PERIOD {
            self.snake_x += 1;
            self.waiting_time = 0.0;
        }
    }
}

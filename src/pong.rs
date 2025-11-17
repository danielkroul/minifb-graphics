use crate::{Buffer, Color};

enum GameState {
    Alive,
    Dead,
}

struct Ball {
    position: (i32, i32),
    velocity: (i32, i32),
    radius: i32,
    color: Color,
}

impl Ball {
    pub fn new(bounds: (i32, i32)) -> Ball {
        Ball { 
            position: (bounds.0 / 2, bounds.1 / 2),
            velocity: (2, 1),
            radius: 20,
            color: Color::WHITE,
        }
    }

    fn bounce(&mut self, bounds: (i32, i32)) {
        // Horizontal bounce
        match self.position.0 {
            x if x - self.radius <= 0 => {
                // Hit the left wall
                self.position.0 = self.radius; // prevent sticking into wall
                self.velocity.0 = self.velocity.0.abs(); // bounce right
            }
            x if x + self.radius >= bounds.0 => {
                // Hit the right wall
                self.position.0 = bounds.0 - self.radius;
                self.velocity.0 = -self.velocity.0; // bounce left
            }
            _ => {}
        }

        // Vertical bounce
        match self.position.1 {
            y if y - self.radius <= 0 => {
                // Hit the top wall
                self.position.1 = self.radius;
                self.velocity.1 = self.velocity.1.abs(); // bounce down
            }
            y if y + self.radius >= bounds.1 => {
                // Hit the bottom wall
                self.position.1 = bounds.1 - self.radius;
                self.velocity.1 = -self.velocity.1; // bounce up
            }
            _ => {}
        }
    }

    fn apply_velocity(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }

    fn draw(&self, buffer: &mut Buffer) {
        buffer.draw_rect(
            self.position.0 - self.radius,
            self.position.1 - self.radius,
            self.radius * 2,
            self.radius * 2,
            self.color,
        );
    }

    pub fn update(&mut self, bounds: (i32, i32), buffer: &mut Buffer) {
        self.bounce(bounds);
        self.apply_velocity();
        self.draw(buffer);
    }
}

struct Paddle {
    position: (i32, i32),
    size: (i32, i32),
    speed: i32,
}

pub struct PongGame {
    state: GameState,
    bounds: (i32, i32),
    paddles: Vec<Paddle>,
    balls: Vec<Ball>,
}

impl PongGame {
    pub fn new(bounds: (i32, i32)) -> PongGame {
        PongGame { 
            state: GameState::Dead,
            bounds: bounds,
            paddles: vec![],
            balls: vec![Ball::new(bounds)],
        }
    }

    pub fn reset(&mut self) {
        self.state = GameState::Alive;
    }

    pub fn update(&mut self, buffer: &mut Buffer) {
        match self.state {
            GameState::Alive => {
                self.balls
                    .iter_mut()
                    .for_each(|ball| ball.update(self.bounds, buffer));
            },
            GameState::Dead => self.reset(),
        }
    }
}
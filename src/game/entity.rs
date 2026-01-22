use anathema::{default_widgets::Canvas, widgets::Style};

use crate::game::vector::Vector;

#[derive(Debug)]
pub struct Entity {
    position: Vector,
    size: Vector,
    character: char,
    pub velocity: Vector,
    pub is_alive: bool,
}

impl Entity {
    pub fn new(position: Vector, size: Vector, character: char) -> Self {
        Self {
            position,
            size,
            character,
            velocity: Vector::zero(),
            is_alive: true,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for row in 0..self.size.x as i32 {
            for col in 0..self.size.y as i32 {
                canvas.put(
                    self.character,
                    Style::new(),
                    (row + self.position.x as i32, col + self.position.y as i32),
                );
            }
        }
    }

    pub fn apply_force(&mut self, force: Vector) {
        self.velocity += force;
    }

    pub fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }

    pub fn update(&mut self, game_size: Vector) {
        self.apply_velocity();
        self.bounce_off_walls(&game_size);
        self.handle_off_screen(&game_size);
    }

    pub fn bounce_off_walls(&mut self, game_size: &Vector) {
        if self.position.x <= 0.0 {
            self.position.x = 0.0;
            self.velocity.x *= -1.0;
        } else if self.position.x >= game_size.x - 1.0 {
            self.position.x = game_size.x - 1.0;
            self.velocity.x *= -1.0;
        }

        if self.position.y <= 0.0 {
            self.position.y = 0.0;
            self.velocity.y *= -1.0;
        }
    }

    pub fn handle_off_screen(&mut self, game_size: &Vector) {
        if self.position.y > game_size.y {
            self.is_alive = false;
        }
    }

    pub fn is_colliding_with(&self, other: &Self) -> bool {
        self.position.x < other.position.x + other.size.x
            && self.position.x + self.size.x > other.position.x
            && self.position.y < other.position.y + other.size.y
            && self.position.y + self.size.y > other.position.y
    }
}

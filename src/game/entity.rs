use anathema::{default_widgets::Canvas, widgets::Style};

use crate::game::vector::Vector;

pub struct Entity {
    position: Vector,
    width: u16,
    height: u16,
    character: char,
    velocity: Vector,
}

impl Entity {
    pub fn new(position: Vector, width: u16, height: u16, character: char) -> Self {
        Self {
            position,
            width,
            height,
            character,
            velocity: Vector::zero(),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.put(self.character, Style::new(), self.position.coords_as_i32());
    }

    pub fn apply_force(&mut self, force: Vector) {
        self.velocity += force;
    }

    pub fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }

    pub fn update(&mut self, game_size: Vector) {
        self.apply_velocity();
        self.bounce_off_walls(game_size);
    }

    pub fn bounce_off_walls(&mut self, game_size: Vector) {
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
        } else if self.position.y >= game_size.y - 1.0 {
            self.position.y = game_size.y - 1.0;
            self.velocity.y *= -1.0;
        }
    }
}

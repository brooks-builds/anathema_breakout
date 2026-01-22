use anathema::{default_widgets::Canvas, state::Color, widgets::Style};

use crate::game::vector::Vector;

#[derive(Debug)]
pub struct Entity {
    pub position: Vector,
    pub size: Vector,
    character: char,
    pub velocity: Vector,
    pub is_alive: bool,
    pub bg_color: Color,
    pub health: u8,
}

impl Entity {
    pub fn new(
        position: Vector,
        size: Vector,
        character: char,
        bg_color: Color,
        health: u8,
    ) -> Self {
        Self {
            position,
            size,
            character,
            velocity: Vector::zero(),
            is_alive: true,
            bg_color,
            health,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        let mut style = Style::new();

        style.set_bg(self.bg_color);

        for row in 0..self.size.x as i32 {
            for col in 0..self.size.y as i32 {
                canvas.put(
                    self.character,
                    style,
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

    pub fn is_colliding_with(&self, other: &Self) -> bool {
        self.position.x < other.position.x + other.size.x
            && self.position.x + self.size.x > other.position.x
            && self.position.y < other.position.y + other.size.y
            && self.position.y + self.size.y > other.position.y
    }
}

enum Side {
    Top,
    Left,
    Right,
    Bottom,
}

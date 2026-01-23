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

        for row in 0..self.size.x {
            for col in 0..self.size.y {
                canvas.put(
                    self.character,
                    style,
                    (row + self.position.x, col + self.position.y),
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
        if self.position.x <= 0 {
            self.position.x = 0;
            self.velocity.x *= -1;
        } else if self.position.x >= game_size.x - 1 {
            self.position.x = game_size.x - 1;
            self.velocity.x *= -1;
        }

        if self.position.y <= 0 {
            self.position.y = 0;
            self.velocity.y *= -1;
        }
    }

    pub fn is_point_inside(&self, point: &Vector) -> bool {
        point.x >= self.position.x
            && point.x < self.position.x + self.size.x
            && point.y >= self.position.y
            && point.y < self.position.y + self.size.y
    }

    pub fn previous_location(&self) -> Vector {
        self.position - self.velocity
    }
}

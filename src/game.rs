mod entity;
mod vector;

use std::f32::consts::PI;

use crate::game::{entity::Entity, vector::Vector};
use anathema::{
    component::Component,
    default_widgets::Canvas,
    state::{Color, State, Value},
};
use bb_anathema_components::BBAppComponent;
use rand::{rngs::ThreadRng, seq::IndexedRandom, thread_rng};

#[derive(Debug, Default)]
pub struct Game(GameEntities);

impl BBAppComponent for Game {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "game",
            "templates/game.aml",
            Self::default(),
            GameState::default(),
        )?;

        Ok(())
    }
}

#[derive(State, Debug, Default)]
pub struct GameState {
    game_width: Value<i64>,
    game_height: Value<i64>,
    playing: Value<bool>,
}

impl Component for Game {
    type State = GameState;

    type Message = ();

    fn on_tick(
        &mut self,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
        _dt: std::time::Duration,
    ) {
        let game_width = *state.game_width.to_ref();
        let game_height = *state.game_height.to_ref();
        let game_size = Vector::from((game_width, game_height));

        children.elements().by_tag("canvas").first(|el, _| {
            let canvas = el.to::<Canvas>();

            canvas.clear();

            let Some(ball) = &mut self.0.ball else { return };
            let Some(paddle) = &mut self.0.paddle else {
                return;
            };

            ball.update(game_size);
            paddle.update(game_size);

            if ball.is_colliding_with(paddle) {
                ball.position.y = paddle.position.y - 1.0;

                let relative_intersect_x =
                    (paddle.position.x + (paddle.size.x / 2.0)) - ball.position.x;
                let normalized_relative_intersect_x = relative_intersect_x / (paddle.size.x / 2.0);
                let angle = normalized_relative_intersect_x * (2.0 * PI / 12.0);
                let new_velocity =
                    Vector::new(self.0.speed * angle.cos(), self.0.speed * -angle.sin());
                ball.velocity = new_velocity;

                self.0.speed += 0.01;
                self.0.speed = self.0.speed.clamp(0.1, 2.0);
            }

            for brick in self.0.bricks.iter_mut() {
                if ball.is_colliding_with(brick) {
                    brick.health -= 1;

                    if ball.position.y < brick.position.y + brick.size.y
                        && ball.position.y > brick.position.y
                    {
                        ball.velocity.y *= -1.0;
                    } else {
                        ball.velocity.x *= -1.0;
                    }
                }
            }

            if ball.position.y > game_size.y {
                ball.is_alive = false;
            }

            ball.draw(canvas);
            paddle.draw(canvas);

            for brick in self.0.bricks.iter() {
                brick.draw(canvas);
            }

            if !ball.is_alive {
                self.0.ball = None;
                context.publish("lost_life", ());
                state.playing.set(false);
            }

            self.0.bricks.retain(|brick| brick.health > 0);
        });
    }

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let width = context
            .attribute("width")
            .expect("don't have a width")
            .to_int()
            .expect("width is not a number");
        let height = context
            .attribute("height")
            .expect("don't have a height")
            .to_int()
            .expect("height isn't a number");

        state.game_width.set(width);
        state.game_height.set(height);
    }

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if event.name() == "begin" {
            let game_width = *state.game_width.to_ref() as f32;
            let game_height = *state.game_height.to_ref() as f32;

            self.0.speed = 0.5;

            let ball_position = Vector::new(game_width / 2.0, game_height / 3.0);
            let ball_velocity = Vector::new(0.0, self.0.speed);
            let ball_size = Vector::new(1.0, 1.0);
            let mut ball = Entity::new(
                ball_position,
                ball_size,
                '*',
                anathema::state::Color::Reset,
                1,
            );
            ball.apply_force(ball_velocity);
            self.0.ball = Some(ball);

            let paddle_size = Vector::new(7.0, 2.0);
            let paddle_position = Vector::new(
                game_width / 2.0 - paddle_size.x / 2.0,
                game_height - paddle_size.y,
            );
            let paddle = Entity::new(
                paddle_position,
                paddle_size,
                '=',
                anathema::state::Color::Reset,
                1,
            );
            self.0.paddle = Some(paddle);

            let bricks_per_row = 12.0;
            let brick_size = Vector::new(game_width / bricks_per_row, 1.0);
            let brick_character = ' ';
            for count in 0..bricks_per_row as i32 {
                let count = count as f32;
                let position = Vector::new(count * brick_size.x, 0.0);
                let (health, color) = (1, Color::Green);
                let brick = Entity::new(position, brick_size, brick_character, color, health);
                self.0.bricks.push(brick);
            }

            for count in 0..bricks_per_row as i32 {
                let count = count as f32;
                let position = Vector::new(count * brick_size.x, brick_size.y);
                let (health, color) = (1, Color::Red);
                let brick = Entity::new(position, brick_size, brick_character, color, health);
                self.0.bricks.push(brick);
            }

            for count in 0..bricks_per_row as i32 {
                let count = count as f32;
                let position = Vector::new(count * brick_size.x, brick_size.y + 1.0);
                let (health, color) = (1, Color::Blue);
                let brick = Entity::new(position, brick_size, brick_character, color, health);
                self.0.bricks.push(brick);
            }

            state.playing.set(true);
        }
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let Some(paddle) = &mut self.0.paddle else {
            return;
        };
        let move_speed = 1.0;

        if matches!(key.code, anathema::component::KeyCode::Left) {
            let force = Vector::new(-move_speed, 0.0);
            paddle.apply_force(force);
        } else if matches!(key.code, anathema::component::KeyCode::Right) {
            let force = Vector::new(move_speed, 0.0);
            paddle.apply_force(force);
        }
    }

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let Some(paddle) = &mut self.0.paddle else {
            return;
        };
        let mouse_position = mouse.pos();

        paddle.velocity.x = 0.0;
        paddle.position.x = mouse_position.x as f32 - paddle.size.x / 2.0;
    }
}

#[derive(Debug, Default)]
pub struct GameEntities {
    ball: Option<Entity>,
    paddle: Option<Entity>,
    speed: f32,
    bricks: Vec<Entity>,
}

fn random_brick() -> (u8, Color) {
    let mut rng = rand::rng();
    let possible_colors = [
        (1, Color::Red),
        (2, Color::Green),
        (3, Color::Yellow),
        (4, Color::Blue),
        (5, Color::Magenta),
        (6, Color::Cyan),
    ];

    possible_colors
        .choose(&mut rng)
        .copied()
        .expect("we have a color")
}

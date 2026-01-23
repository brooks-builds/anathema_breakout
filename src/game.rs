mod entity;
mod vector;

use crate::game::{entity::Entity, vector::Vector};
use anathema::{
    component::Component,
    default_widgets::Canvas,
    state::{Color, State, Value},
};
use bb_anathema_components::BBAppComponent;
use rand::seq::IndexedRandom;

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
    game_width: Value<i32>,
    game_height: Value<i32>,
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
        let game_size = Vector::new(game_width, game_height);

        children.elements().by_tag("canvas").first(|el, _| {
            let canvas = el.to::<Canvas>();

            canvas.clear();

            let Some(ball) = &mut self.0.ball else { return };
            let Some(paddle) = &mut self.0.paddle else {
                return;
            };

            ball.update(game_size);
            paddle.update(game_size);

            if paddle.is_point_inside(&ball.position) {
                ball.position.y = paddle.position.y - 1;
                ball.velocity.y *= -1;
                // am I on the left, center, or right sides
                let shifted_ball = ball.position.x - (paddle.position.x + (paddle.size.x / 2));
                ball.velocity.x = shifted_ball;
            }

            for brick in self.0.bricks.iter_mut() {
                if brick.is_point_inside(&ball.position) {
                    let previous_ball_position = ball.previous_location();

                    brick.health -= 1;

                    if brick.health == 0 {
                        context.publish("scored", brick.value);
                    }

                    if previous_ball_position.x < brick.position.x {
                        ball.velocity.x *= -1;
                        ball.position.x = brick.position.x - 1;
                    } else if previous_ball_position.x > brick.position.x + brick.size.x - 1 {
                        ball.velocity.x *= -1;
                        ball.position.x = brick.position.x + brick.size.x
                    }

                    if previous_ball_position.y < brick.position.y {
                        ball.position.y = brick.position.y - 1;
                    } else if previous_ball_position.y > brick.position.y + brick.size.y - 1 {
                        ball.position.y = brick.position.y + brick.size.y;
                    }

                    ball.velocity.y *= -1;
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

            self.0.bricks.retain(|brick| brick.health > 0);

            if self.0.bricks.is_empty() && ball.is_alive {
                state.playing.set(false);
            } else if !ball.is_alive {
                self.0.ball = None;
                context.publish("lost_life", ());
                state.playing.set(false);
            }
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

        state.game_width.set(width as i32);
        state.game_height.set(height as i32);
    }

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if event.name() == "begin" {
            let game_width = *state.game_width.to_ref();
            let game_height = *state.game_height.to_ref();
            let ball_position = Vector::new(game_width / 2, game_height / 2);
            let ball_velocity = Vector::new(0, 1);
            let ball_size = Vector::new(1, 1);
            let mut ball = Entity::new(
                ball_position,
                ball_size,
                '*',
                anathema::state::Color::Reset,
                1,
            );
            ball.apply_force(ball_velocity);
            self.0.ball = Some(ball);

            let paddle_size = Vector::new(7, 2);
            let paddle_position = Vector::new(
                game_width / 2 - paddle_size.x / 2,
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

            if self.0.bricks.is_empty() {
                let bricks_per_row = 12;
                let brick_size = Vector::new(game_width / bricks_per_row, 1);
                let brick_character = ' ';
                for count in 0..bricks_per_row {
                    let position = Vector::new(count * brick_size.x, 0);
                    let (health, color) = random_brick();
                    let brick = Entity::new(position, brick_size, brick_character, color, health);
                    self.0.bricks.push(brick);
                }

                for count in 0..bricks_per_row {
                    let position = Vector::new(count * brick_size.x, brick_size.y);
                    let (health, color) = random_brick();
                    let brick = Entity::new(position, brick_size, brick_character, color, health);
                    self.0.bricks.push(brick);
                }

                for count in 0..bricks_per_row {
                    let position = Vector::new(count * brick_size.x, brick_size.y + 1);
                    let (health, color) = random_brick();
                    let brick = Entity::new(position, brick_size, brick_character, color, health);
                    self.0.bricks.push(brick);
                }
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
        let move_speed = 1;

        if matches!(key.code, anathema::component::KeyCode::Left) {
            let force = Vector::new(-move_speed, 0);
            paddle.apply_force(force);
        } else if matches!(key.code, anathema::component::KeyCode::Right) {
            let force = Vector::new(move_speed, 0);
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

        paddle.velocity.x = 0;
        paddle.position.x = mouse_position.x - paddle.size.x / 2;
    }
}

#[derive(Debug, Default)]
pub struct GameEntities {
    ball: Option<Entity>,
    paddle: Option<Entity>,
    bricks: Vec<Entity>,
}

fn random_brick() -> (u32, Color) {
    let mut rng = rand::rng();
    let possible_colors = [
        (1, Color::Red),
        // (2, Color::Green),
        // (3, Color::Yellow),
        // (4, Color::Blue),
        // (5, Color::Magenta),
        // (6, Color::Cyan),
    ];

    possible_colors
        .choose(&mut rng)
        .copied()
        .expect("we have a color")
}

mod entity;
mod vector;

use crate::game::{entity::Entity, vector::Vector};
use anathema::{
    component::Component,
    default_widgets::Canvas,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

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
                ball.velocity.y *= -1.0;
            }

            ball.draw(canvas);
            paddle.draw(canvas);

            if !ball.is_alive {
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

            let ball_position = Vector::new(game_width / 2.0, game_height / 3.0);
            let ball_velocity = Vector::new(0.0, 0.1);
            let ball_size = Vector::new(1.0, 1.0);
            let mut ball = Entity::new(ball_position, ball_size, '*');
            ball.apply_force(ball_velocity);
            self.0.ball = Some(ball);

            let paddle_size = Vector::new(8.0, 2.0);
            let paddle_position = Vector::new(
                game_width / 2.0 - paddle_size.x / 2.0,
                game_height - paddle_size.y,
            );
            let paddle = Entity::new(paddle_position, paddle_size, '=');
            self.0.paddle = Some(paddle);

            state.playing.set(true);
        }
    }

    fn accept_focus(&self) -> bool {
        false
    }
}

#[derive(Debug, Default)]
pub struct GameEntities {
    ball: Option<Entity>,
    paddle: Option<Entity>,
}

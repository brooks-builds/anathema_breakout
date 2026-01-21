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

            if let Some(ball) = &mut self.0.ball {
                ball.update(game_size);
                ball.draw(canvas);

                if !ball.is_alive {
                    self.0.ball = None;
                    context.publish("lost_life", ());
                    state.playing.set(false);
                }
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
            let ball_velocity = Vector::new(0.0, 4.1);
            let game_width = *state.game_width.to_ref();
            let game_height = *state.game_height.to_ref();
            let position = Vector::from((game_width / 2, game_height / 3));
            let mut ball = Entity::new(position, 1, 1, '*');

            ball.apply_force(ball_velocity);
            state.playing.set(true);
            self.0.ball = Some(ball);
        }
    }

    fn accept_focus(&self) -> bool {
        false
    }
}

#[derive(Debug, Default)]
pub struct GameEntities {
    ball: Option<Entity>,
}

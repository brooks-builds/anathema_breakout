mod entity;
mod vector;

use anathema::{
    component::Component,
    default_widgets::Canvas,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;
use color_eyre::owo_colors::OwoColorize;

use crate::game::{entity::Entity, vector::Vector};

pub struct Game(Vec<Entity>);

impl BBAppComponent for Game {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "game",
            "templates/game.aml",
            Self(vec![]),
            GameState::default(),
        )?;

        Ok(())
    }
}

#[derive(State, Debug, Default)]
pub struct GameState {
    game_width: Value<i64>,
    game_height: Value<i64>,
}

impl Component for Game {
    type State = GameState;

    type Message = ();

    fn on_tick(
        &mut self,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
        dt: std::time::Duration,
    ) {
        let game_width = *state.game_width.to_ref();
        let game_height = *state.game_height.to_ref();
        let game_size = Vector::from((game_width, game_height));

        children.elements().by_tag("canvas").first(|el, _| {
            let mut canvas = el.to::<Canvas>();

            canvas.clear();
            for entity in self.0.iter_mut() {
                entity.update(game_size);
                entity.draw(&mut canvas);
            }
        });
    }

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
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
        let ball_position = Vector::from((width / 2, height / 2));
        let mut ball = Entity::new(ball_position, 1, 1, '*');
        let initial_ball_force = Vector::new(0.3, 0.2);

        ball.apply_force(initial_ball_force);

        self.0.push(ball);

        state.game_width.set(width);
        state.game_height.set(height);
    }
}

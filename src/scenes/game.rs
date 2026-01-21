use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct GameScene;

impl BBAppComponent for GameScene {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "game_scene",
            "templates/scenes/game.aml",
            Self,
            GameSceneState::default(),
        )?;

        Ok(())
    }
}

impl Component for GameScene {
    type State = GameSceneState;

    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }
}

#[derive(Debug, State)]
pub struct GameSceneState {
    lives: Value<u8>,
}

impl Default for GameSceneState {
    fn default() -> Self {
        Self {
            lives: Value::new(3),
        }
    }
}

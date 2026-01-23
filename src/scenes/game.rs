use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

use crate::app::CurrentGameScene;

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

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if event.name() == "lost_life" {
            let lives = *state.lives.to_ref();
            if lives == 0 {
                context.publish("change_scene", CurrentGameScene::End);
            } else {
                state.lives.set(lives - 1);
            }
        } else if event.name() == "scored" {
            event.stop_propagation();
            let score = event.data_checked::<u32>().copied().unwrap_or_default();
            context.publish("scored", score);
        } else if event.name() == "automation_mode_toggle" {
            event.stop_propagation();
            context.publish("automation_mode_toggle", ());
        }
    }

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        state.lives.set(3);
    }
}

#[derive(Debug, State, Default)]
pub struct GameSceneState {
    lives: Value<u8>,
}

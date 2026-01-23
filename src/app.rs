use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct App;

impl BBAppComponent for App {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component("App", "templates/app.aml", Self, AppState::default())?;

        Ok(())
    }
}

impl Component for App {
    type State = AppState;

    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if event.name() == "change_scene" {
            let Some(game_scene) = event.data_checked::<CurrentGameScene>() else {
                return;
            };

            state.scene.set(game_scene.into());
        } else if event.name() == "scored" {
            let points = event.data_checked::<u32>().copied().unwrap_or_default();
            let score = *state.score.to_ref();

            state.score.set(score + points);
        }
    }

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let viewport = context.viewport.size();
        let width = viewport.width;
        let height = viewport.height;

        state.width.set(width);
        state.height.set(height);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum CurrentGameScene {
    #[default]
    Splash,
    Game,
    End,
}

impl From<&CurrentGameScene> for String {
    fn from(value: &CurrentGameScene) -> Self {
        match value {
            CurrentGameScene::Splash => "splash",
            CurrentGameScene::Game => "game",
            CurrentGameScene::End => "end",
        }
        .to_owned()
    }
}

impl From<CurrentGameScene> for String {
    fn from(value: CurrentGameScene) -> Self {
        (&value).into()
    }
}

#[derive(Debug, State)]
pub struct AppState {
    scene: Value<String>,
    score: Value<u32>,
    high_score: Value<u32>,
    level: Value<u8>,
    width: Value<u16>,
    height: Value<u16>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            scene: Value::new(CurrentGameScene::Splash.into()),
            score: Value::default(),
            high_score: Value::default(),
            level: Value::default(),
            width: Value::default(),
            height: Value::default(),
        }
    }
}

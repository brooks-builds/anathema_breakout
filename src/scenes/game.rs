use anathema::component::Component;
use bb_anathema_components::BBAppComponent;

pub struct GameScene;

impl BBAppComponent for GameScene {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component("game_scene", "templates/scenes/game.aml", Self, ())?;

        Ok(())
    }
}

impl Component for GameScene {
    type State = ();

    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }
}

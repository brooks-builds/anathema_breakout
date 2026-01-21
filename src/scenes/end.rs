use anathema::component::Component;
use bb_anathema_components::BBAppComponent;

use crate::app::CurrentGameScene;

pub struct EndScene;

impl BBAppComponent for EndScene {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component("end_scene", "templates/scenes/end.aml", Self, ())?;

        Ok(())
    }
}

impl Component for EndScene {
    type State = ();

    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if event.name() == "back_to_start" {
            context.publish("change_scene", CurrentGameScene::Splash);
        }
    }
}

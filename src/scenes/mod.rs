mod end;
mod game;
pub mod splash;

use anathema::runtime::Builder;
use bb_anathema_components::BBAppComponent;
use eyre::Result;

use crate::scenes::{end::EndScene, game::GameScene, splash::SplashScene};

pub fn register_scenes(builder: &mut Builder<()>) -> Result<()> {
    SplashScene::register_to(builder)?;
    GameScene::register_to(builder)?;
    EndScene::register_to(builder)?;

    Ok(())
}

mod app;
mod scenes;

use crate::app::App;
use anathema::{
    prelude::{Backend, Document, TuiBackend},
    runtime::Runtime,
};
use bb_anathema_components::BBAppComponent;

pub fn run() -> eyre::Result<()> {
    let doc = Document::new("@App");
    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_mouse()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    backend.finalize();

    let mut builder = Runtime::builder(doc, &backend);

    bb_anathema_components::register_all(&mut builder)?;
    App::register_to(&mut builder)?;
    scenes::register_scenes(&mut builder)?;

    builder
        .finish(&mut backend, |runtime, backend| runtime.run(backend))
        .unwrap();

    Ok(())
}

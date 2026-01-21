use anathema_breakout::run;
use eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    run()?;

    Ok(())
}

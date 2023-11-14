use commerce_commander::ui::init::{run, shutdown, startup, Err, Result};

fn main() -> Result<()> {
    startup()?;
    run()?;
    shutdown()?;
    Ok(())
}

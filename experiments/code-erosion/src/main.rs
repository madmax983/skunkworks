mod app;
mod erosion;
mod terrain;

use anyhow::Result;
use app::App;
use tui_shared::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new()?;

    app.run(&mut tui.terminal)?;

    Ok(())
}

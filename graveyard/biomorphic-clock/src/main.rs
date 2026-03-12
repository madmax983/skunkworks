mod simulation;
mod tui;

use anyhow::Result;
use tui_shared::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let app = tui::App::new()?;
    tui::run_app(&mut tui.terminal, app)?;
    Ok(())
}

mod tui;

use anyhow::Result;

fn main() -> Result<()> {
    let app = tui::App::new()?;
    tui::run_app(app)?;
    Ok(())
}

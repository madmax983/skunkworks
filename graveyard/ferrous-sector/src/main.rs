mod decay;
mod platter;
mod ui;

use crate::ui::App;
use anyhow::Result;
use std::env;
use tui_shared::Tui;

fn main() -> Result<()> {
    // Determine path to load. Default to "src" if available, else "."
    let default_path = if std::path::Path::new("src").exists() {
        "src"
    } else {
        "."
    };
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| default_path.to_string());

    let mut tui = Tui::init()?;
    let mut app = App::new(&path);
    app.run(&mut tui.terminal)?;
    Ok(())
}

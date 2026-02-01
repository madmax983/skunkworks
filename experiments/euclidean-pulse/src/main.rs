mod algo;
mod audio;
mod monitor;
mod tui;

use anyhow::Result;
use audio::init_audio;
use monitor::SystemMonitor;
use tui::App;

fn main() -> Result<()> {
    // 1. Initialize Monitor
    let monitor = SystemMonitor::new();

    // 2. Initialize Audio (BPM = 120)
    let audio = init_audio(120.0)?;

    // 3. Initialize Terminal
    let terminal = ratatui::init();

    // 4. Run App
    let app = App::new(monitor, audio);
    let res = app.run(terminal);

    // 5. Restore Terminal
    ratatui::restore();

    res
}

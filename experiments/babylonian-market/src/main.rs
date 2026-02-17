pub mod babylonian;
pub mod mayan;
pub mod market_wrapper;
pub mod tui;

use anyhow::Result;

fn main() -> Result<()> {
    tui::run_app()
}

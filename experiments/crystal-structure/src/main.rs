mod app;
mod lattice;
mod math;

use anyhow::Result;
use app::App;
use lattice::Lattice;
use std::env;

fn main() -> Result<()> {
    // Determine path to walk. Default to current dir.
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    let lattice = Lattice::from_dir(path)?;

    let terminal = ratatui::init();
    let app = App::new(lattice);
    let res = app.run(terminal);
    ratatui::restore();

    res
}

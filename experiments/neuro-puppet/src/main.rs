pub mod neuron;
pub mod network;
pub mod body;
pub mod app;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut app = App::new();
    app.run()
}

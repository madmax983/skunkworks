pub mod app;
pub mod body;
pub mod network;
pub mod neuron;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut app = App::new();
    app.run()
}

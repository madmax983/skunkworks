git checkout experiments/synaptic-pachinko/src/main.rs
sed -i '10s/use ratatui::style::{Color, Style, Stylize};/use ratatui::style::{Color, Style};/' experiments/synaptic-pachinko/src/main.rs
git checkout experiments/lattice-brain/src/audio.rs
sed -i 's/use crossbeam_channel::{unbounded, Receiver, Sender};/use crossbeam_channel::{unbounded, Sender};/' experiments/lattice-brain/src/audio.rs
sed -i 's/let mut connections: Vec<Vec<(usize, f32)>> = Vec::new();/let mut _connections: Vec<Vec<(usize, f32)>> = Vec::new();/' experiments/lattice-brain/src/audio.rs
sed -i 's/connections = c;/_connections = c; let _ = _connections;/' experiments/lattice-brain/src/audio.rs
sed -i 's/pub mean_field: f32,/\/\/pub mean_field: f32,/' experiments/lattice-brain/src/audio.rs
sed -i '33i \#[allow(clippy::type_complexity)]' experiments/lattice-brain/src/audio.rs
git checkout experiments/lattice-brain/src/main.rs
sed -i '136s/pub z: f64,/\/\/pub z: f64,/' experiments/lattice-brain/src/main.rs
sed -i '144s/pub z: f64,/\/\/pub z: f64,/' experiments/lattice-brain/src/main.rs

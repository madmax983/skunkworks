use std::env;

fn window_conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "Origami Lattice: Codebase Morphogenesis".to_owned(),
        ..Default::default()
    }
}

// I can remove `#[macroquad::main]` and write my own main

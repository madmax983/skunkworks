use macroquad::prelude::*;

fn get_headless_mode() -> bool {
    std::env::var("HEADLESS").unwrap_or_else(|_| "false".to_string()) == "true"
}

#[macroquad::main("Arthropod-Origami-Lattice")]
async fn main() {
    let is_headless = get_headless_mode();
    let mut frame_count = 0;

    loop {
        clear_background(BLACK);

        if is_headless {
            frame_count += 1;
            if frame_count > 10 {
                break;
            }
        }

        next_frame().await;
    }
}

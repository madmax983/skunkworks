use macroquad::prelude::*;

mod substrate;
mod fungus;

use substrate::StegoSubstrate;
use fungus::HyphaeNetwork;

const GRID_WIDTH: usize = 300;
const GRID_HEIGHT: usize = 200;

#[macroquad::main("Stego Mycelium")]
async fn main() {
    let mut substrate = StegoSubstrate::new(GRID_WIDTH, GRID_HEIGHT);
    let message = "HIDDEN MESSAGE: The Splice Surgeon was here. This hybrid inherits the steganographic genes of stego-cartridge and the chaotic growth of chaotic-mycelium. The fungus eats the entropy.";
    substrate.embed_message(message);

    // Start near (10, height/2)
    let start_pos = IVec2::new(10, (GRID_HEIGHT/2) as i32);
    let mut fungus = HyphaeNetwork::new(GRID_WIDTH, GRID_HEIGHT, start_pos);

    loop {
        if is_key_pressed(KeyCode::R) {
            substrate = StegoSubstrate::new(GRID_WIDTH, GRID_HEIGHT);
            substrate.embed_message(message);
            fungus = HyphaeNetwork::new(GRID_WIDTH, GRID_HEIGHT, start_pos);
        }

        // Update
        fungus.update(&substrate, 100);

        // Draw
        clear_background(BLACK);

        let cell_w = screen_width() / GRID_WIDTH as f32;
        let cell_h = screen_height() / GRID_HEIGHT as f32;

        substrate.draw();
        fungus.draw(cell_w, cell_h);

        // Decode and Draw Text
        let decoded = fungus.get_message(&substrate);

        draw_rectangle(0., 0., screen_width(), 100., Color::new(0., 0., 0., 0.8));
        draw_text("Stego Mycelium", 10., 20., 30., WHITE);

        // Wrap text
        let mut y = 45.;
        let chars_per_line = (screen_width() / 12.0) as usize;
        let chars = decoded.chars().collect::<Vec<char>>();
        for chunk in chars.chunks(chars_per_line) {
             let line: String = chunk.iter().collect();
             draw_text(&format!("> {}", line), 10., y, 20., YELLOW);
             y += 20.;
        }

        // Visualize "tips" count
        draw_text(&format!("Tips: {}", fungus.active_tips.len()), screen_width() - 100., 20., 20., GRAY);

        next_frame().await
    }
}

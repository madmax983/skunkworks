
mod heap;
mod synesthesia;
mod audio;

use macroquad::prelude::*;
use heap::Heap;
use audio::AudioManager;
use synesthesia::draw_heap;
use ::rand::Rng;

#[macroquad::main("MemCanvas")]
async fn main() {
    let mut heap = Heap::new(2048); // Size in "units" (blocks)
    let audio = AudioManager::new().await;

    let mut rng = ::rand::thread_rng();
    let mut paused = false;
    let mut auto_mode = true;

    // Track allocated pointers to free them later
    let mut pointers: Vec<usize> = Vec::new();

    loop {
        clear_background(BLACK);

        // Input
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            heap = Heap::new(2048);
            pointers.clear();
        }
        if is_key_pressed(KeyCode::M) {
            auto_mode = !auto_mode;
        }

        // Simulation
        if !paused && auto_mode {
            // Alloc
            if rng.gen_bool(0.1) {
                let size = rng.gen_range(10..100);
                if let Some(ptr) = heap.malloc(size) {
                    pointers.push(ptr);
                    audio.play_alloc(size);
                }
            }

            // Free
            if !pointers.is_empty() && rng.gen_bool(0.05) {
                let idx = rng.gen_range(0..pointers.len());
                let ptr = pointers.remove(idx);

                // Find block size for audio?
                // Heap doesn't expose size by ptr easily without search.
                // We'll search in heap.free or just play a generic sound.
                // Or look it up first.
                // Let's just play sound.
                audio.play_free(0);
                heap.free(ptr);
            }
        }

        // Update visual decay
        heap.update_lifetimes(0.02);

        // Draw
        let screen_w = screen_width();
        let screen_h = screen_height();
        draw_heap(&heap, screen_w, screen_h);

        // UI
        draw_text("Space: Pause | R: Reset | M: Auto Mode", 10.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Heap Size: {}/{}", heap.blocks.iter().filter(|b| !b.is_free).map(|b| b.size).sum::<usize>(), heap.total_size), 10.0, 40.0, 20.0, WHITE);
        draw_text(&format!("Blocks: {}", heap.blocks.len()), 10.0, 60.0, 20.0, WHITE);

        next_frame().await;
    }
}

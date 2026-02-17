use macroquad::prelude::*;
use ::rand::Rng;

mod heap;
use heap::*;

enum Mode {
    WebServer,
    Database,
    Leak,
}

#[macroquad::main("Malloc Expressionism")]
async fn main() {
    let mut heap = Heap::new(1000); // 1000 units of memory
    let mut allocations: Vec<(usize, f32)> = Vec::new(); // (addr, time_to_live)
    let mut mode = Mode::WebServer;

    // Canvas setup
    let grid_width = 50; // 50 blocks per row
    let block_size = 20.0;

    loop {
        clear_background(Color::new(0.95, 0.94, 0.92, 1.0)); // Canvas paper color

        // Input
        if is_key_pressed(KeyCode::Key1) {
            mode = Mode::WebServer;
            heap = Heap::new(1000); allocations.clear();
        }
        if is_key_pressed(KeyCode::Key2) {
            mode = Mode::Database;
            heap = Heap::new(1000); allocations.clear();
        }
        if is_key_pressed(KeyCode::Key3) {
            mode = Mode::Leak;
            heap = Heap::new(1000); allocations.clear();
        }
        if is_key_pressed(KeyCode::Space) {
            heap = Heap::new(1000);
            allocations.clear();
        }

        // Simulation
        let mut rng = ::rand::thread_rng();

        match mode {
            Mode::WebServer => {
                // High churn: many small allocs, short life
                if rng.gen_bool(0.15) {
                    let size = rng.gen_range(1..5);
                    // Random bright colors (Impressionist palette)
                    let color = Color::new(
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.2..0.8),
                        rng.gen_range(0.2..0.8),
                        1.0
                    );
                    if let Some(addr) = heap.malloc(size, color) {
                        allocations.push((addr, rng.gen_range(1.0..3.0)));
                    }
                }
            },
            Mode::Database => {
                // Low churn: large allocs, long life (Color Field)
                if rng.gen_bool(0.02) {
                    let size = rng.gen_range(10..100);
                    // Deep, saturated colors (Rothko)
                    let base = rng.gen_range(0.0..0.3);
                    let color = Color::new(
                        base,
                        base + rng.gen_range(0.0..0.2),
                        base + rng.gen_range(0.4..0.7),
                        1.0
                    );
                    if let Some(addr) = heap.malloc(size, color) {
                        allocations.push((addr, rng.gen_range(5.0..15.0)));
                    }
                }
            },
            Mode::Leak => {
                // Allocs, no free (Accumulation)
                if rng.gen_bool(0.1) {
                    let size = rng.gen_range(1..8);
                    // Muddy, chaotic colors
                    let color = Color::new(
                        rng.gen_range(0.2..0.8),
                        rng.gen_range(0.1..0.6),
                        rng.gen_range(0.0..0.2),
                        1.0
                    );
                    if let Some(_addr) = heap.malloc(size, color) {
                        // Infinite TTL
                    }
                }
            }
        }

        // Handle frees
        let dt = get_frame_time();
        let mut to_free = Vec::new();
        for (i, (_addr, ttl)) in allocations.iter_mut().enumerate() {
            *ttl -= dt;
            if *ttl <= 0.0 {
                to_free.push(i);
            }
        }

        to_free.sort_by(|a, b| b.cmp(a));
        for idx in to_free {
            if idx < allocations.len() {
                let (addr, _) = allocations.remove(idx);
                heap.free(addr);
            }
        }

        // Draw
        let start_x = 40.0;
        let start_y = 80.0;
        let margin = 2.0;

        for block in &heap.blocks {
            let mut drawn_size = 0;
            let mut current_addr = block.start;

            while drawn_size < block.size {
                let row = current_addr / grid_width;
                let col = current_addr % grid_width;

                let remaining_in_row = grid_width - col;
                let chunk_size = std::cmp::min(block.size - drawn_size, remaining_in_row);

                let x = start_x + col as f32 * block_size + margin;
                let y = start_y + row as f32 * block_size + margin;
                let w = chunk_size as f32 * block_size - margin * 2.0;
                let h = block_size - margin * 2.0;

                // Draw
                let color = if block.free {
                     Color::new(0.92, 0.92, 0.9, 1.0) // Faint texture
                } else {
                    block.color
                };

                draw_rectangle(x, y, w, h, color);

                // Draw border for allocs
                if !block.free {
                    draw_rectangle_lines(x, y, w, h, 1.0, Color::new(0.0, 0.0, 0.0, 0.3));
                }

                drawn_size += chunk_size;
                current_addr += chunk_size;
            }
        }

        // UI
        draw_text("Malloc Expressionism", 20.0, 40.0, 40.0, BLACK);
        draw_text(&format!("Movement: {}", mode_name(&mode)), 20.0, 70.0, 20.0, DARKGRAY);

        let stats_y = screen_height() - 40.0;
        draw_text(&format!("Allocations: {}", allocations.len()), 20.0, stats_y, 20.0, DARKGRAY);
        draw_text("1: Impressionism | 2: Color Field | 3: Accumulation | Space: Reset", 20.0, screen_height() - 15.0, 20.0, BLACK);

        next_frame().await
    }
}

fn mode_name(mode: &Mode) -> &str {
    match mode {
        Mode::WebServer => "Web Server Impressionism",
        Mode::Database => "Database Color Field",
        Mode::Leak => "Memory Leak Accumulation",
    }
}

//! # Arthropod Quipu 🐜🧶
//!
//! **Concept:** Interactive Knotted Ledger.
//!
//! This hybrid visualizer explores what happens when we cross the immediate-mode UI library of `arthropod` with the ancient knotted cord storage system of `quipu`.

use arthropod::Button;
use macroquad::prelude::*;
use quipu::{Cord, Knot, Quipu};

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Quipu".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

// Bypass macroquad::main to support headless execution
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running in headless mode. Bypassing macroquad initialization.");
        return;
    }

    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let mut quipu = Quipu::new();
    // Add a single cord to start visualizing
    quipu.add_cord(Cord::from(0u64));

    let btn_add_1 =
        Button::new("Add 1", 20.0, 20.0, 100.0, 40.0).with_colors(RED, ORANGE, DARKGRAY);
    let btn_add_10 =
        Button::new("Add 10", 20.0, 70.0, 100.0, 40.0).with_colors(GREEN, LIME, DARKGREEN);
    let btn_add_100 =
        Button::new("Add 100", 20.0, 120.0, 100.0, 40.0).with_colors(BLUE, SKYBLUE, DARKBLUE);
    let btn_clear =
        Button::new("Clear", 20.0, 170.0, 100.0, 40.0).with_colors(GRAY, LIGHTGRAY, BLACK);

    loop {
        clear_background(color_u8!(30, 25, 20, 255)); // dark brown background

        let mut value_to_add = 0;

        if btn_add_1.draw() {
            value_to_add = 1;
        }

        if btn_add_10.draw() {
            value_to_add = 10;
        }

        if btn_add_100.draw() {
            value_to_add = 100;
        }

        if btn_clear.draw() {
            quipu = Quipu::new();
            quipu.add_cord(Cord::from(0u64));
        }

        if value_to_add > 0 {
            // Get current value, add, and replace cord
            let current_val = quipu.cords[0].value();
            quipu.cords[0] = Cord::from(current_val + value_to_add);
        }

        // Render the Quipu Cord
        // Draw the main horizontal string
        draw_line(150.0, 50.0, 750.0, 50.0, 4.0, DARKBROWN);

        // Draw the pendant cord
        let cord_x = 450.0;
        draw_line(cord_x, 50.0, cord_x, 550.0, 3.0, DARKBROWN);

        let cord = &quipu.cords[0];
        let mut y_offset = 100.0;

        // Render clusters (hundreds, tens, units)
        for cluster in cord.clusters.iter().rev() {
            if cluster.is_empty() {
                y_offset += 60.0;
                continue;
            }

            for knot in cluster {
                match knot {
                    Knot::Simple => {
                        draw_circle(cord_x, y_offset, 10.0, BEIGE);
                        draw_circle_lines(cord_x, y_offset, 10.0, 2.0, BLACK);
                        y_offset += 25.0;
                    }
                    Knot::Long(n) => {
                        draw_rectangle(
                            cord_x - 15.0,
                            y_offset - 10.0,
                            30.0,
                            20.0 + (*n as f32 * 5.0),
                            BEIGE,
                        );
                        draw_rectangle_lines(
                            cord_x - 15.0,
                            y_offset - 10.0,
                            30.0,
                            20.0 + (*n as f32 * 5.0),
                            2.0,
                            BLACK,
                        );
                        // Draw individual turns
                        for i in 0..*n {
                            draw_line(
                                cord_x - 15.0,
                                y_offset + (i as f32 * 5.0),
                                cord_x + 15.0,
                                y_offset + (i as f32 * 5.0),
                                2.0,
                                BLACK,
                            );
                        }
                        y_offset += 30.0 + (*n as f32 * 5.0);
                    }
                    Knot::FigureEight => {
                        draw_circle(cord_x, y_offset, 12.0, GOLD);
                        draw_circle_lines(cord_x, y_offset, 12.0, 2.0, BLACK);
                        // inner symbol
                        draw_line(
                            cord_x - 5.0,
                            y_offset - 5.0,
                            cord_x + 5.0,
                            y_offset + 5.0,
                            2.0,
                            BLACK,
                        );
                        draw_line(
                            cord_x + 5.0,
                            y_offset - 5.0,
                            cord_x - 5.0,
                            y_offset + 5.0,
                            2.0,
                            BLACK,
                        );
                        y_offset += 30.0;
                    }
                }
            }
            y_offset += 40.0; // Space between clusters
        }

        draw_text(
            format!("Cord Value: {}", cord.value()).as_str(),
            20.0,
            250.0,
            30.0,
            WHITE,
        );

        next_frame().await;
    }
}

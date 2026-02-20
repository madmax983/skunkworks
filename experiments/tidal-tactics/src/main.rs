mod fluid;
mod game;
mod map;

use game::{GameState, Team, TurnPhase, UnitType};
use macroquad::prelude::*;
use map::{HEIGHT, WIDTH};

#[macroquad::main("Tidal Tactics")]
async fn main() {
    let mut game = GameState::new();

    // UI Setup
    let mut flow_timer = 0.0;

    loop {
        let dt = get_frame_time();

        let sw = screen_width();
        let sh = screen_height();

        // Reserve space for UI on right
        let ui_width = 200.0;
        let map_w = sw - ui_width;
        let cell_w = map_w / WIDTH as f32;
        let cell_h = sh / HEIGHT as f32;
        // Keep aspect ratio square based on min
        let cell_size = cell_w.min(cell_h);

        let map_offset_x = 0.0;
        let map_offset_y = 0.0;

        // Input
        match game.phase {
            TurnPhase::PlayerInput => {
                if is_key_pressed(KeyCode::Space) {
                    game.end_turn();
                    flow_timer = 5.0;
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mx, my) = mouse_position();
                    if mx < map_w {
                        let gx = (mx / cell_size) as usize;
                        let gy = (my / cell_size) as usize;

                        if gx < WIDTH && gy < HEIGHT {
                            // Try to select unit first
                            if let Some(idx) =
                                game.units.iter().position(|u| u.x == gx && u.y == gy)
                            {
                                // Select if it's player's unit (or just view info)
                                game.selected_unit = Some(idx);
                            } else {
                                // Try move selected unit
                                if let Some(sel_idx) = game.selected_unit {
                                    // Copy team to avoid borrow conflict
                                    let is_player = game.units[sel_idx].team == Team::Player;

                                    if is_player {
                                        game.try_move_selected(gx, gy);
                                    }
                                }
                            }
                        }
                    } else {
                        // Clicked on UI?
                        // End Turn button? (Rect is at 80, 30 height)
                        if mx > map_w + 10.0 && mx < map_w + 110.0 && my > 80.0 && my < 110.0 {
                            game.end_turn();
                            flow_timer = 5.0; // 5 seconds of flow
                        }
                    }
                }

                // Right click to modify terrain/water (Debug/God Mode or actual gameplay?)
                // Let's keep the God Mode controls for fun
                if is_mouse_button_down(MouseButton::Right) {
                    let (mx, my) = mouse_position();
                    if mx < map_w {
                        let gx = (mx / cell_size) as usize;
                        let gy = (my / cell_size) as usize;
                        if gx < WIDTH && gy < HEIGHT {
                            let idx = gy * WIDTH + gx;
                            game.map.water[idx] += 10.0 * dt;
                        }
                    }
                }
            }
            TurnPhase::FlowSimulation => {
                // Update Fluid
                let sim_dt = dt.min(0.05);

                // Tide Mechanic: Sea at x=0
                let time = get_time();
                let tide_level = 5.0 + 3.0 * (time as f32 / 5.0).sin(); // Oscillate between 2.0 and 8.0

                for y in 0..HEIGHT {
                    let idx = y * WIDTH; // x=0
                    let terrain = game.map.terrain[idx];
                    let target_water = (tide_level - terrain).max(0.0);
                    // Smoothly approach target water level
                    let current_water = game.map.water[idx];
                    game.map.water[idx] =
                        current_water + (target_water - current_water) * 5.0 * sim_dt;
                }

                fluid::step(&mut game.map, sim_dt);

                flow_timer -= dt;
                if flow_timer <= 0.0 {
                    game.phase = TurnPhase::PlayerInput;
                    game.turn += 1;
                    // Reset moves
                    for unit in &mut game.units {
                        if unit.team == Team::Player {
                            unit.moves_left = 2;
                        }
                    }
                }
            }
        }

        clear_background(BLACK);

        // Render Map
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = y * WIDTH + x;
                let tx = map_offset_x + x as f32 * cell_size;
                let ty = map_offset_y + y as f32 * cell_size;

                // Terrain
                let h = game.map.terrain[idx];
                let color = if h < 2.0 {
                    Color::new(0.9, 0.8, 0.4, 1.0)
                } else if h < 6.0 {
                    Color::new(0.2, 0.7, 0.2, 1.0)
                } else {
                    Color::new(0.5, 0.5, 0.5, 1.0)
                };
                let shade = 0.5 + (h / 10.0).clamp(0.0, 0.5);
                draw_rectangle(
                    tx,
                    ty,
                    cell_size,
                    cell_size,
                    Color::new(color.r * shade, color.g * shade, color.b * shade, 1.0),
                );

                // Water
                let w = game.map.water[idx];
                if w > 0.01 {
                    let depth_factor = (w / 10.0).clamp(0.0, 1.0);
                    let alpha = 0.4 + 0.5 * depth_factor;
                    draw_rectangle(
                        tx,
                        ty,
                        cell_size,
                        cell_size,
                        Color::new(0.0, 0.2, 1.0, alpha),
                    );
                }

                // Selection Highlight
                if let Some(sel_idx) = game.selected_unit {
                    let u = &game.units[sel_idx];
                    if u.x == x && u.y == y {
                        draw_rectangle_lines(tx, ty, cell_size, cell_size, 2.0, YELLOW);
                    }
                }
            }
        }

        // Render Units
        for unit in &game.units {
            let tx = map_offset_x + unit.x as f32 * cell_size;
            let ty = map_offset_y + unit.y as f32 * cell_size;
            let color = match unit.team {
                Team::Player => BLUE,
                Team::Enemy => RED,
            };
            // Draw circle for unit
            draw_circle(
                tx + cell_size / 2.0,
                ty + cell_size / 2.0,
                cell_size / 3.0,
                color,
            );

            // Draw Type icon (simple letter)
            let label = match unit.unit_type {
                UnitType::Tank => "T",
                UnitType::Hovercraft => "H",
                UnitType::Engineer => "E",
            };
            // Center text? approximation
            draw_text(
                label,
                tx + cell_size / 3.0,
                ty + cell_size / 1.5,
                cell_size / 2.0,
                WHITE,
            );
        }

        // UI Panel
        draw_rectangle(map_w, 0.0, ui_width, sh, GRAY);
        draw_text("Tidal Tactics", map_w + 10.0, 30.0, 30.0, WHITE);

        let phase_text = match game.phase {
            TurnPhase::PlayerInput => "Player Turn",
            TurnPhase::FlowSimulation => "Flowing...",
        };
        draw_text(phase_text, map_w + 10.0, 60.0, 20.0, WHITE);
        draw_text(
            &format!("Turn: {}", game.turn),
            map_w + 10.0,
            45.0,
            20.0,
            WHITE,
        );

        if game.phase == TurnPhase::PlayerInput {
            draw_rectangle(map_w + 10.0, 80.0, 100.0, 30.0, DARKGRAY);
            draw_text("End Turn", map_w + 20.0, 100.0, 20.0, WHITE);
            draw_text("(Space)", map_w + 20.0, 125.0, 15.0, GRAY);
        }

        if let Some(idx) = game.selected_unit {
            let u = &game.units[idx];
            draw_text(
                &format!("Unit: {:?}", u.unit_type),
                map_w + 10.0,
                150.0,
                20.0,
                WHITE,
            );
            draw_text(
                &format!("Moves: {}", u.moves_left),
                map_w + 10.0,
                170.0,
                20.0,
                WHITE,
            );

            // Check water depth at unit pos
            let idx = u.y * WIDTH + u.x;
            let depth = game.map.water[idx];
            draw_text(
                &format!("Depth: {:.1}", depth),
                map_w + 10.0,
                190.0,
                20.0,
                WHITE,
            );

            if depth > 0.5 && u.unit_type == UnitType::Tank {
                draw_text("WARNING: DROWNING", map_w + 10.0, 210.0, 20.0, RED);
            }
        }

        // Debug Interaction Text
        draw_text("R-Click: Add Water", map_w + 10.0, sh - 20.0, 15.0, WHITE);

        next_frame().await
    }
}

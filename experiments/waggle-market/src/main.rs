mod agent;
use agent::{Bee, BeeState, Source, SourceType};
use macroquad::prelude::*;
use market_sim::{Grid, Particle};

#[macroquad::main("Waggle Market")]
async fn main() {
    let mut grid = Grid::new(40, 100); // 40 columns (time/width), 100 rows (price)
    let mut bees = Vec::new();
    let mut sources = Vec::new();

    // Initial Population
    for i in 0..200 {
        bees.push(Bee::new(vec2(screen_width() * 0.25, screen_height() * 0.5), i));
    }

    // Initial Sources
    sources.push(Source {
        position: vec2(100.0, 100.0),
        radius: 30.0,
        source_type: SourceType::Supply, // Cheap goods (Red/Asks)
        value: 1.0,
    });
    sources.push(Source {
        position: vec2(100.0, screen_height() - 100.0),
        radius: 30.0,
        source_type: SourceType::Demand, // Buyers (Green/Bids)
        value: 1.0,
    });

    let hive_pos = vec2(screen_width() * 0.75, screen_height() * 0.5); // Center of Market View

    loop {
        let dt = get_frame_time();

        // --- Input ---
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            if mx < screen_width() / 2.0 {
                // Add new source in Field
                sources.push(Source {
                    position: vec2(mx, my),
                    radius: 30.0,
                    source_type: if macroquad::rand::gen_range(0.0, 1.0) < 0.5 { SourceType::Supply } else { SourceType::Demand },
                    value: macroquad::rand::gen_range(0.5, 2.0),
                });
            }
        }

        if is_key_pressed(KeyCode::R) {
            grid = Grid::new(40, 100);
            bees.clear();
            sources.clear();
            for i in 0..200 {
                bees.push(Bee::new(vec2(screen_width() * 0.25, screen_height() * 0.5), i));
            }
        }

        // --- Update Bees ---
        let field_rect = Rect::new(0.0, 0.0, screen_width() / 2.0, screen_height());

        // Update individual bee logic
        for bee in &mut bees {
            bee.update(&sources, &mut grid, hive_pos, field_rect, dt);
        }

        // Recruitment Logic
        // 1. Identify active dances (Dancer Pos, Target Source Pos, Type, Value)
         let active_dances: Vec<(Vec2, Vec2, SourceType, f32)> = bees.iter()
            .filter_map(|b| {
                if let BeeState::Dancing { target_pos, source_type, value, .. } = b.state {
                    Some((b.position, target_pos, source_type, value))
                } else {
                    None
                }
            })
            .collect();

        // 2. Recruit nearby observers
        for bee in &mut bees {
             if matches!(bee.state, BeeState::Scouting) {
                for (dancer_pos, target_source_pos, source_type, value) in &active_dances {
                    if bee.position.distance(*dancer_pos) < 50.0 {
                         if macroquad::rand::gen_range(0.0, 1.0) < (value * 0.02) { // 2% chance per frame if close
                            bee.state = BeeState::Recruited {
                                target_pos: *target_source_pos,
                                source_type: *source_type,
                            };
                            break; // Recruited by one dance
                         }
                    }
                }
             }
        }


        // --- Update Market ---
        let _trades = grid.update();

        // --- Draw ---
        clear_background(BLACK);

        // Draw Split Line
        draw_line(screen_width() / 2.0, 0.0, screen_width() / 2.0, screen_height(), 2.0, DARKGRAY);

        // Draw Field (Left)
        for source in &sources {
            let color = match source.source_type {
                SourceType::Supply => RED,
                SourceType::Demand => GREEN,
            };
            draw_circle(source.position.x, source.position.y, source.radius, color);
            draw_circle_lines(source.position.x, source.position.y, source.radius + 5.0 * (get_time() as f32).sin(), 1.0, WHITE);
        }

        // Draw Bees
        for bee in &bees {
            let color = match bee.state {
                BeeState::Scouting => WHITE,
                BeeState::Returning { .. } => BLUE,
                BeeState::Dancing { .. } => YELLOW,
                BeeState::Recruited { .. } => PURPLE,
                BeeState::Foraging { .. } => ORANGE,
            };
            draw_circle(bee.position.x, bee.position.y, 3.0, color);

            // Draw dance line
            if let BeeState::Dancing { .. } = bee.state {
                draw_circle_lines(bee.position.x, bee.position.y, 10.0, 1.0, YELLOW);
            }
        }

        // Draw Hive Location (Right Side Context)
        draw_circle(hive_pos.x, hive_pos.y, 15.0, GOLD);
        draw_text("HIVE / ORDER BOOK", hive_pos.x - 50.0, hive_pos.y - 20.0, 20.0, GOLD);

        // Draw Market Grid (Right)
        // Map grid to screen rect
        let market_x = screen_width() / 2.0 + 50.0;
        let market_y = 50.0;
        let market_w = screen_width() / 2.0 - 100.0;
        let market_h = screen_height() - 100.0;

        let cell_w = market_w / grid.width as f32;
        let cell_h = market_h / grid.height as f32;

        draw_rectangle_lines(market_x, market_y, market_w, market_h, 2.0, GRAY);

        for y in 0..grid.height {
            for x in 0..grid.width {
                let p = grid.get(x, y);
                if !matches!(p, Particle::Empty) {
                    let px = market_x + x as f32 * cell_w;
                    let py = market_y + y as f32 * cell_h;

                    let color = match p {
                        Particle::Bid(_) => GREEN,
                        Particle::Ask(_) => RED,
                        Particle::Trade { age } => {
                            let alpha = age as f32 / 5.0;
                            Color::new(1.0, 1.0, 1.0, alpha)
                        },
                        _ => BLACK,
                    };

                    draw_rectangle(px, py, cell_w, cell_h, color);
                }
            }
        }

        // Draw UI
        draw_text("WAGGLE MARKET", 20.0, 30.0, 30.0, WHITE);
        draw_text("Left Click: Add Source", 20.0, 50.0, 20.0, GRAY);
        draw_text(format!("Traders: {}", bees.len()).as_str(), 20.0, 70.0, 20.0, GRAY);
        draw_text(format!("Trades: {}", grid.trade_count).as_str(), market_x, 30.0, 20.0, WHITE);

        next_frame().await
    }
}

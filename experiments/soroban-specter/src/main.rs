mod fluid;
mod market;
mod soroban;

use fluid::FluidSolver;
use macroquad::prelude::*;
use market::{Market, Signal, Trader};
use soroban::Soroban;

// --- Constants ---
const FLUID_SIZE: usize = 128;
const BEAD_RADIUS: f32 = 12.0;
const BEAD_HEIGHT: f32 = 18.0;
const ROD_SPACING: f32 = 50.0; // Wider spacing for cleaner look over fluid
const ROD_COUNT: usize = 13;
const BEAM_Y: f32 = 300.0;
const FRAME_X: f32 = 50.0;
const FRAME_Y: f32 = 150.0;
const UPPER_BEAD_REST_Y: f32 = FRAME_Y + 30.0;
const UPPER_BEAD_ACTIVE_Y: f32 = BEAM_Y - BEAD_HEIGHT - 5.0;
const LOWER_BEAD_ACTIVE_START_Y: f32 = BEAM_Y + 5.0;

// --- Soroban Diff Logic ---
struct BeadChange {
    col_idx: usize,
    is_upper: bool,
    bead_index: Option<u8>, // For lower beads, which one moved? Or just general activity?
    // Simplified: Just position of activity.
    active: bool, // Moving to active or inactive?
}

fn diff_soroban(old: &Soroban, new: &Soroban) -> Vec<BeadChange> {
    let mut changes = Vec::new();
    for i in 0..13 {
        let o = &old.columns[i];
        let n = &new.columns[i];

        if o.upper_active != n.upper_active {
            changes.push(BeadChange {
                col_idx: i,
                is_upper: true,
                bead_index: None,
                active: n.upper_active,
            });
        }

        if o.lower_active != n.lower_active {
            // Determine which beads "moved".
            // If o.lower_active = 1 and n.lower_active = 2, bead #1 moved up.
            // If o.lower_active = 2 and n.lower_active = 1, bead #1 moved down.
            // Beads are 0-indexed from top (closest to beam).

            let start = o.lower_active.min(n.lower_active);
            let end = o.lower_active.max(n.lower_active);

            for b in start..end {
                changes.push(BeadChange {
                    col_idx: i,
                    is_upper: false,
                    bead_index: Some(b),
                    active: n.lower_active > o.lower_active, // Moving UP towards beam is "active" increase?
                                                             // Actually lower_active is count of beads up.
                });
            }
        }
    }
    changes
}

// --- Rendering ---

fn get_bead_pos(col_idx: usize, is_upper: bool, bead_idx: Option<u8>, active: bool) -> Vec2 {
    // Reverse index for visualization (Right to Left is standard for numbers)
    // But let's stick to array index for now to match logic, maybe reverse visually if needed.
    // Standard Soroban: Ones column is on the right.
    // market.rs likely adds to index 0 as ones?
    // soroban crate: index 0 is ones place.
    // Visual: Rightmost rod is index 0.

    let rod_x = FRAME_X + ((ROD_COUNT - 1 - col_idx) as f32) * ROD_SPACING + ROD_SPACING / 2.0;

    let y = if is_upper {
        if active {
            UPPER_BEAD_ACTIVE_Y
        } else {
            UPPER_BEAD_REST_Y
        }
    } else {
        let b = bead_idx.unwrap_or(0);
        // Wait, "active" in lower beads means PUSHED UP.
        // But lower beads have fixed slots.
        // Bead 0 is top-most of lower deck.
        // If lower_active is 2, then Bead 0 and Bead 1 are UP. Bead 2 and 3 are DOWN.

        // This function is for "Splash" location.
        // If bead_idx is 0, it is the one closest to beam.

        // A bead is "Active" (Up) or "Inactive" (Down).
        // If active:
        LOWER_BEAD_ACTIVE_START_Y + (b as f32) * BEAD_HEIGHT
        // If inactive (Down):
        // LOWER_BEAD_ACTIVE_START_Y + (b as f32) * BEAD_HEIGHT + 60.0
    };

    // Correction for Lower Down position:
    let final_y = if !is_upper && !active {
        LOWER_BEAD_ACTIVE_START_Y + (bead_idx.unwrap_or(0) as f32) * BEAD_HEIGHT + 60.0
    } else {
        y
    };

    vec2(rod_x, final_y)
}

fn draw_bead(x: f32, y: f32, color: Color) {
    draw_poly(x, y + BEAD_HEIGHT / 2.0, 6, BEAD_RADIUS, 90.0, color);
    // Highlight
    draw_poly(
        x - 3.0,
        y + BEAD_HEIGHT / 2.0 - 3.0,
        6,
        BEAD_RADIUS * 0.3,
        90.0,
        WHITE,
    );
}

fn draw_soroban(soroban: &Soroban) {
    // Frame
    draw_rectangle(
        FRAME_X - 20.0,
        FRAME_Y - 20.0,
        (ROD_COUNT as f32) * ROD_SPACING + 40.0,
        400.0,
        Color::new(0.4, 0.2, 0.1, 0.8), // Semi-transparent wood
    );

    // Beam
    draw_line(
        FRAME_X - 10.0,
        BEAM_Y,
        FRAME_X + (ROD_COUNT as f32) * ROD_SPACING + 10.0,
        BEAM_Y,
        5.0,
        BLACK,
    );

    for (i, col) in soroban.columns.iter().enumerate() {
        let rod_x = FRAME_X + ((ROD_COUNT - 1 - i) as f32) * ROD_SPACING + ROD_SPACING / 2.0;

        // Rod
        draw_line(rod_x, FRAME_Y, rod_x, FRAME_Y + 380.0, 3.0, DARKGRAY);

        // Upper Bead
        let upper_y = if col.upper_active {
            UPPER_BEAD_ACTIVE_Y
        } else {
            UPPER_BEAD_REST_Y
        };
        draw_bead(rod_x, upper_y, RED);

        // Lower Beads
        for b in 0..4 {
            let is_up = (b as u8) < col.lower_active;
            let y = if is_up {
                LOWER_BEAD_ACTIVE_START_Y + (b as f32) * BEAD_HEIGHT
            } else {
                LOWER_BEAD_ACTIVE_START_Y + (b as f32) * BEAD_HEIGHT + 60.0
            };
            draw_bead(rod_x, y, BLUE);
        }
    }
}

// --- Main ---

fn window_conf() -> Conf {
    Conf {
        window_title: "Soroban Specter".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> anyhow::Result<()> {
    // Init Fluid
    let mut fluid = FluidSolver::new(FLUID_SIZE, 0.1, 0.00001, 0.00001);

    // Texture for Fluid
    let mut image = Image::gen_image_color(FLUID_SIZE as u16, FLUID_SIZE as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Linear);

    // Init Market
    let mut market = Market::new();
    let mut trader = Trader::new(20);

    // Initial warm-up
    for _ in 0..50 {
        market.update();
        trader.process(&market);
    }

    let mut prev_soroban = trader.soroban.clone();

    loop {
        // --- Logic ---
        market.update();
        trader.process(&market);

        // Diff Soroban
        let changes = diff_soroban(&prev_soroban, &trader.soroban);
        prev_soroban = trader.soroban.clone();

        // Inject Fluid Forces
        for change in changes {
            // Calculate screen position of the bead
            // For lower beads, we need to know WHICH one moved.
            // The diff logic returned `bead_index`.
            // We also need to know if it moved Up (active) or Down (inactive).
            // `change.active` tells us the NEW state.
            // If it became active, it moved UP. If inactive, moved DOWN.

            let pos = get_bead_pos(
                change.col_idx,
                change.is_upper,
                change.bead_index,
                change.active,
            );

            // Map Screen Pos to Fluid Grid
            let fx = (pos.x / screen_width() * FLUID_SIZE as f32) as usize;
            let fy = (pos.y / screen_height() * FLUID_SIZE as f32) as usize;

            if fx < FLUID_SIZE && fy < FLUID_SIZE {
                // Add Density (Splash)
                fluid.add_density(fx, fy, 50.0);

                // Add Velocity (Direction of movement)
                // Upper bead: Active=Down, Inactive=Up.
                // Lower bead: Active=Up, Inactive=Down.

                let vy = if change.is_upper {
                    if change.active {
                        5.0
                    } else {
                        -5.0
                    } // Down / Up
                } else if change.active {
                    -5.0
                } else {
                    5.0
                }; // Up / Down

                fluid.add_velocity(fx, fy, 0.0, vy);
            }
        }

        // Mouse Interaction (for fun)
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let fx = (mx / screen_width() * FLUID_SIZE as f32) as usize;
            let fy = (my / screen_height() * FLUID_SIZE as f32) as usize;
            if fx < FLUID_SIZE && fy < FLUID_SIZE {
                fluid.add_density(fx, fy, 20.0);
            }
        }

        // Step Fluid
        fluid.step();

        // --- Render ---
        clear_background(BLACK);

        // Update Fluid Texture
        for y in 0..FLUID_SIZE {
            for x in 0..FLUID_SIZE {
                let idx = x + y * FLUID_SIZE;
                let d = fluid.density[idx];

                // Color mapping: Spectral (Blue -> Cyan -> White)
                let r = (d * 0.5).min(1.0);
                let g = (d * 1.5).min(1.0);
                let b = (d * 3.0).min(1.0);

                image.set_pixel(x as u32, y as u32, Color::new(r, g, b, 1.0));
            }
        }
        texture.update(&image);

        // Draw Fluid Background
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw Soroban Overlay
        draw_soroban(&trader.soroban);

        // Draw Info
        draw_text("SOROBAN SPECTER", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            format!("Price: {}", market.current_price).as_str(),
            20.0,
            60.0,
            20.0,
            GREEN,
        );
        draw_text(
            format!("SMA: {}", trader.current_sma).as_str(),
            20.0,
            80.0,
            20.0,
            YELLOW,
        );

        let signal_text = match trader.last_signal {
            Signal::Buy => "BUY",
            Signal::Sell => "SELL",
            Signal::Hold => "HOLD",
        };
        let signal_color = match trader.last_signal {
            Signal::Buy => GREEN,
            Signal::Sell => RED,
            Signal::Hold => LIGHTGRAY,
        };
        draw_text(signal_text, 20.0, 110.0, 40.0, signal_color);

        next_frame().await
    }
}

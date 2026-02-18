mod bio;
mod erosion;
mod leaf;

use bio::{BioAction, Plant};
use erosion::{erode_step, Seed};
use leaf::LeafMap;
use macroquad::prelude::*;

const MAP_WIDTH: usize = 400;
const MAP_HEIGHT: usize = 400;

#[macroquad::main("Chimera Erosion")]
async fn main() {
    let mut map = LeafMap::new(MAP_WIDTH, MAP_HEIGHT);
    map.generate_shape();

    let mut plants: Vec<Plant> = Vec::new();

    // Define Seeds
    // Moss: Thrives in water. Drinks it.
    let moss_dna = r#"
    strand main {
        # [H, W, E, Age]
        drop # Age
        dup  # Energy
        push(40) gt brz(check_water)

        # High Energy: Reproduce (3)
        drop drop drop
        push(3)
        jump(end)
    }

    strand check_water {
        # [H, W, E]
        drop # Energy
        dup # Water
        push(5) gt brz(dry)

        # Has Water: Drink (2)
        drop drop
        push(2)
        jump(end)
    }

    strand dry {
        # [H, W]
        drop drop
    }

    strand end {}
    "#;

    // Pine: Thrives on height. Grows roots.
    let pine_dna = r#"
    strand main {
        # [H, W, E, Age]
        drop # Age
        drop # Energy
        drop # Water

        dup # Height
        push(30) gt brz(low)

        # High Altitude: Grow Roots (1)
        drop
        push(1)
        jump(end)
    }

    strand low {
        drop
    }

    strand end {}
    "#;

    let seeds = vec![
        Seed {
            species: "Moss".to_string(),
            dna_source: moss_dna.to_string(),
        },
        Seed {
            species: "Pine".to_string(),
            dna_source: pine_dna.to_string(),
        },
    ];

    let mut image = Image::gen_image_color(
        MAP_WIDTH as u16,
        MAP_HEIGHT as u16,
        Color::new(0.0, 0.0, 0.0, 0.0),
    );
    let texture = Texture2D::from_image(&image);

    let mut eroding = true;
    let drops_per_frame = 1000;
    let mut show_plants = true;
    let mut planted_seeds_buffer = Vec::new();

    loop {
        if is_key_pressed(KeyCode::R) {
            map = LeafMap::new(MAP_WIDTH, MAP_HEIGHT);
            map.generate_shape();
            plants.clear();
        }
        if is_key_pressed(KeyCode::Space) {
            eroding = !eroding;
        }
        if is_key_pressed(KeyCode::D) {
            show_plants = !show_plants;
        }

        // Mouse interaction (Drop water manually)
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let screen_w = screen_width();
            let screen_h = screen_height();
            let scale = (screen_w / MAP_WIDTH as f32).min(screen_h / MAP_HEIGHT as f32) * 0.8;
            let offset_x = (screen_w - MAP_WIDTH as f32 * scale) / 2.0;
            let offset_y = (screen_h - MAP_HEIGHT as f32 * scale) / 2.0;

            let mx = (mouse_pos.0 - offset_x) / scale;
            let my = (mouse_pos.1 - offset_y) / scale;

            if mx >= 0.0 && mx < MAP_WIDTH as f32 && my >= 0.0 && my < MAP_HEIGHT as f32 {
                let x = mx as usize;
                let y = my as usize;
                // Just erode locally, no seed planting from mouse click for now
                crate::erosion::erode_at(&mut map, x, y);
            }
        }

        if eroding {
            // Erosion Step
            planted_seeds_buffer.clear();
            erode_step(&mut map, drops_per_frame, &seeds, &mut planted_seeds_buffer);

            // Plant new seeds
            for (x, y, seed_idx) in planted_seeds_buffer.iter() {
                if plants.len() < 5000 {
                    // Limit population
                    let seed = &seeds[*seed_idx];
                    plants.push(Plant::new(*x, *y, &seed.species, &seed.dna_source));
                }
            }

            // Bio Step
            let mut reproduction_queue = Vec::new();

            // Collect dead plant indices
            let mut dead_plants = Vec::new();

            for (i, plant) in plants.iter_mut().enumerate() {
                let action = plant.step(&mut map);

                if plant.energy <= 0 {
                    dead_plants.push(i);
                    continue;
                }

                match action {
                    BioAction::GrowRoots(amount) => {
                        let idx = plant.y * map.width + plant.x;
                        map.heightmap[idx] += amount;
                    }
                    BioAction::Drink(amount) => {
                        let idx = plant.y * map.width + plant.x;
                        if map.water[idx] > amount {
                            map.water[idx] -= amount;
                        } else {
                            map.water[idx] = 0.0;
                        }
                    }
                    BioAction::Reproduce => {
                        reproduction_queue.push((
                            plant.x,
                            plant.y,
                            plant.species.clone(),
                            plant.dna_source.clone(),
                        ));
                    }
                    BioAction::None => {}
                }
            }

            // Remove dead plants (reverse order to avoid index shifting)
            for i in dead_plants.iter().rev() {
                plants.remove(*i);
            }

            // Handle Reproduction
            for (px, py, species, source) in reproduction_queue {
                if plants.len() < 5000 {
                    // Spread nearby
                    let dx = macroquad::rand::gen_range(-3, 4);
                    let dy = macroquad::rand::gen_range(-3, 4);
                    let nx = (px as isize + dx).clamp(0, map.width as isize - 1) as usize;
                    let ny = (py as isize + dy).clamp(0, map.height as isize - 1) as usize;

                    if map.is_inside(nx, ny) {
                        plants.push(Plant::new(nx, ny, &species, &source));
                    }
                }
            }
        }

        // Visualization
        // Update texture
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let idx = y * MAP_WIDTH + x;
                if map.mask[idx] {
                    let h = map.heightmap[idx];
                    let w = map.water[idx];

                    let val = h.clamp(0.0, 1.0);
                    let r = (1.0 - val) * 0.8;
                    let g = 0.5 + val * 0.5;
                    let b = 0.1;
                    let mut color = Color::new(r, g, b, 1.0);

                    // Water
                    if w > 0.1 {
                        let water_alpha = (w * 0.5).min(0.8);
                        color = Color::new(
                            color.r * (1.0 - water_alpha) + 0.0 * water_alpha,
                            color.g * (1.0 - water_alpha) + 0.5 * water_alpha,
                            color.b * (1.0 - water_alpha) + 1.0 * water_alpha,
                            1.0,
                        );
                        // Natural evaporation/decay
                        map.water[idx] *= 0.95;
                    }
                    image.set_pixel(x as u32, y as u32, color);
                } else {
                    image.set_pixel(x as u32, y as u32, BLACK);
                }
            }
        }

        // Draw Plants
        if show_plants {
            for plant in &plants {
                let color = if plant.species == "Moss" {
                    GREEN
                } else {
                    DARKGREEN // Pine
                };
                image.set_pixel(plant.x as u32, plant.y as u32, color);
            }
        }

        texture.update(&image);

        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        let screen_w = screen_width();
        let screen_h = screen_height();
        let scale = (screen_w / MAP_WIDTH as f32).min(screen_h / MAP_HEIGHT as f32) * 0.8;
        let offset_x = (screen_w - MAP_WIDTH as f32 * scale) / 2.0;
        let offset_y = (screen_h - MAP_HEIGHT as f32 * scale) / 2.0;

        draw_texture_ex(
            &texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(MAP_WIDTH as f32 * scale, MAP_HEIGHT as f32 * scale)),
                ..Default::default()
            },
        );

        draw_text(
            &format!("Plants: {}", plants.len()),
            20.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Space: Pause | D: Debug | R: Reset",
            20.0,
            60.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}

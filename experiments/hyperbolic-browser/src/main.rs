mod fs_map;
mod render;
mod tiling;

use macroquad::prelude::*;
use num_complex::Complex;
use poincare_disk::{Mobius, Point};
use fs_map::{FsCache, RoomId};
use render::Renderer;

#[macroquad::main("Hyperbolic Browser")]
async fn main() -> anyhow::Result<()> {
    let mut fs_cache = FsCache::new();
    let mut renderer = Renderer::new();

    let cwd = std::env::current_dir()?;
    let mut current_room_id = RoomId { path: cwd, page: 0 };

    let mut view_transform = Mobius::identity();

    let mut is_animating = false;
    let mut anim_start_time = 0.0;
    let anim_duration = 0.5;
    let mut anim_source = Mobius::identity();
    let mut anim_target = Mobius::identity();
    let mut next_room_id = current_room_id.clone();

    loop {
        clear_background(BLACK);
        let screen_size = vec2(screen_width(), screen_height());
        let min_dim = screen_size.x.min(screen_size.y);
        let disk_radius = min_dim * 0.45;
        let screen_center = screen_size * 0.5;

        // Input
        if !is_animating && is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();

            // Map to disk coords
            let dx = (mx - screen_center.x) / disk_radius;
            let dy = -(my - screen_center.y) / disk_radius; // Flip Y
            let click_z = Complex::new(dx as f64, dy as f64);

            if click_z.norm() < 1.0 {
                let mut best_dir = None;
                // Center is always distance to 0
                let mut min_dist = poincare_disk::hyperbolic_dist(click_z, Point::new(0.0, 0.0));

                // Check neighbors
                if let Ok(room) = fs_cache.get_room(&current_room_id) {
                     for (dir, neighbor_opt) in room.neighbors.iter().enumerate() {
                         if neighbor_opt.is_some() {
                             let t = renderer.tiler.get_neighbor_transform(dir);
                             let pos = t.apply(Point::new(0.0, 0.0));

                             let d = poincare_disk::hyperbolic_dist(click_z, pos);
                             // Bias towards neighbors slightly to make clicking easier?
                             if d < min_dist {
                                 min_dist = d;
                                 best_dir = Some(dir);
                             }
                         }
                     }
                }

                if let Some(dir) = best_dir {
                    let room = fs_cache.get_room(&current_room_id).unwrap();
                    if let Some(target_id) = &room.neighbors[dir] {
                        is_animating = true;
                        anim_start_time = get_time();
                        next_room_id = target_id.clone();

                        anim_source = Mobius::identity();
                        let t_neighbor = renderer.tiler.get_neighbor_transform(dir);

                        // Inverse: (dz-b)/(-cz+a)
                        let inv = Mobius {
                            a: t_neighbor.d,
                            b: -t_neighbor.b,
                            c: -t_neighbor.c,
                            d: t_neighbor.a,
                        };
                        anim_target = inv;
                    }
                }
            }
        }

        // Update Animation
        if is_animating {
            let t = (get_time() - anim_start_time) / anim_duration;
            if t >= 1.0 {
                is_animating = false;
                current_room_id = next_room_id.clone();
                view_transform = Mobius::identity();
            } else {
                let t = t as f64;
                // Cubic ease out?
                // let t = 1.0 - (1.0 - t).powi(3);

                let a = lerp_c(anim_source.a, anim_target.a, t);
                let b = lerp_c(anim_source.b, anim_target.b, t);
                let c = lerp_c(anim_source.c, anim_target.c, t);
                let d = lerp_c(anim_source.d, anim_target.d, t);
                view_transform = Mobius { a, b, c, d };
            }
        }

        renderer.draw_scene(&mut fs_cache, &current_room_id, view_transform, screen_size);

        draw_text("Hyperbolic Browser", 20.0, 30.0, 30.0, WHITE);

        let path_str = current_room_id.path.display().to_string();
        let page_str = if current_room_id.page > 0 { format!(" (Page {})", current_room_id.page) } else { "".to_string() };
        draw_text(&format!("{}{}", path_str, page_str), 20.0, 60.0, 20.0, GRAY);

        if is_animating {
             draw_text("Warping...", 20.0, 90.0, 20.0, Color::new(1.0, 0.5, 0.0, 1.0));
        }

        next_frame().await
    }
}

fn lerp_c(a: Complex<f64>, b: Complex<f64>, t: f64) -> Complex<f64> {
    a * (1.0 - t) + b * t
}

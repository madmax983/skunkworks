use num_complex::Complex;
use ratatui::{
    style::Color,
    widgets::canvas::{Circle, Context, Line},
};
use std::f64::consts::PI;

use crate::ant::Ant;
use crate::dungeon::{Dungeon, TileType};
use poincare_disk::{neighbor_transform_a, Mobius, Point, TilingConsts};

pub fn draw_dungeon(
    ctx: &mut Context,
    dungeon: &Dungeon,
    start_path: &[usize],
    view_transform: &Mobius,
    consts: &TilingConsts,
    ants: &[Ant],
) {
    draw_tile_recursive(
        ctx,
        dungeon,
        start_path.to_vec(),
        *view_transform,
        consts,
        0,
        None,
        ants,
    );
}

fn draw_tile_recursive(
    ctx: &mut Context,
    dungeon: &Dungeon,
    path: Vec<usize>,
    transform: Mobius,
    consts: &TilingConsts,
    depth: usize,
    from_dir: Option<usize>,
    ants: &[Ant],
) {
    // 1. Cull check
    let screen_center = transform.apply(Point::new(0.0, 0.0));
    let v0_local = Complex::from_polar(consts.vertex_offset, PI / 4.0);
    let v0_screen = transform.apply(v0_local);
    let size = (v0_screen - screen_center).norm();

    if size < 0.02 {
        return;
    }
    if depth > 8 {
        return;
    }

    // 2. Get Tile Data (Clone to release borrow)
    let tile_data = dungeon.get_tile(&path).clone();
    let is_wall = matches!(tile_data.tile_type, TileType::Wall);

    // 3. Draw Geometry
    let mut screen_verts = [Point::default(); 4];
    for i in 0..4 {
        let angle = (i as f64 * PI / 2.0) + (PI / 4.0);
        let p_local = Complex::from_polar(consts.vertex_offset, angle);
        screen_verts[i] = transform.apply(p_local);
    }

    // Draw edges (Wireframe)
    let edge_color = if is_wall {
        Color::White
    } else {
        Color::DarkGray
    };

    for i in 0..4 {
        let p1 = screen_verts[i];
        let p2 = screen_verts[(i + 1) % 4];
        ctx.draw(&Line {
            x1: p1.re,
            y1: p1.im,
            x2: p2.re,
            y2: p2.im,
            color: edge_color,
        });
    }

    // Fill hint? (Cross for wall, Dot for food)
    if is_wall {
        ctx.draw(&Line {
            x1: screen_verts[0].re,
            y1: screen_verts[0].im,
            x2: screen_verts[2].re,
            y2: screen_verts[2].im,
            color: edge_color,
        });
        ctx.draw(&Line {
            x1: screen_verts[1].re,
            y1: screen_verts[1].im,
            x2: screen_verts[3].re,
            y2: screen_verts[3].im,
            color: edge_color,
        });
    } else if tile_data.has_food {
        ctx.draw(&Circle {
            x: screen_center.re,
            y: screen_center.im,
            radius: size * 0.3,
            color: Color::Red,
        });
    } else if tile_data.pheromone_food > 0.1 {
        ctx.draw(&Circle {
            x: screen_center.re,
            y: screen_center.im,
            radius: size * 0.1 * tile_data.pheromone_food.min(5.0),
            color: Color::Green,
        });
    }

    // 4. Draw Ants on this tile
    for ant in ants {
        if ant.path == path {
            // Map ant offset (local) to screen
            let ant_screen = transform.apply(ant.offset);
            let ant_color = if ant.carrying_food {
                Color::Magenta
            } else {
                Color::Yellow
            };
            ctx.draw(&Circle {
                x: ant_screen.re,
                y: ant_screen.im,
                radius: size * 0.2, // Relative size?
                color: ant_color,
            });
        }
    }

    // 5. Recurse
    if is_wall {
        return;
    }

    for i in 0..4 {
        if let Some(from) = from_dir {
            if i == from {
                continue;
            }
        }

        let step_a = neighbor_transform_a(i, consts);
        let step_transform = Mobius::translation(step_a);
        let child_transform = transform.then(&step_transform);
        let next_path = Dungeon::canonicalize_step(path.clone(), i);
        let next_from = (i + 2) % 4;

        draw_tile_recursive(
            ctx,
            dungeon,
            next_path,
            child_transform,
            consts,
            depth + 1,
            Some(next_from),
            ants,
        );
    }
}

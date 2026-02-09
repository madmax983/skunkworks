use num_complex::Complex;
use ratatui::{
    style::Color,
    widgets::canvas::{Context, Line, Circle},
};
use std::f64::consts::PI;

use crate::dungeon::{Dungeon, TileType};
use crate::game::Game;
use crate::entity::EntityKind;
use poincare_disk::{neighbor_transform_a, Mobius, Point, TilingConsts};

pub fn draw_dungeon(
    ctx: &mut Context,
    game: &Game,
    view_transform: &Mobius,
) {
    draw_tile_recursive(
        ctx,
        game,
        game.get_player().path.clone(),
        *view_transform,
        &game.tiling_consts,
        0,
        None,
    );
}

fn draw_tile_recursive(
    ctx: &mut Context,
    game: &Game,
    path: Vec<usize>,
    transform: Mobius,
    consts: &TilingConsts,
    depth: usize,
    from_dir: Option<usize>,
) {
    // 1. Check visibility/size
    let screen_center = transform.apply(Point::new(0.0, 0.0));
    let v0_local = Complex::from_polar(consts.vertex_offset, PI / 4.0);
    let v0_screen = transform.apply(v0_local);
    let size = (v0_screen - screen_center).norm();

    if size < 0.02 {
        return;
    }

    if depth > 10 {
        return;
    }

    // 2. Get Tile Data
    let tile = game.dungeon.get_tile(&path);
    let is_wall = match tile.tile_type {
        TileType::Wall => true,
        TileType::Floor => false,
    };

    // 3. Draw Geometry
    let mut screen_verts = [Point::default(); 4];
    for i in 0..4 {
        let angle = (i as f64 * PI / 2.0) + (PI / 4.0);
        let p_local = Complex::from_polar(consts.vertex_offset, angle);
        screen_verts[i] = transform.apply(p_local);
    }

    let color = if is_wall {
        Color::DarkGray
    } else {
        let c = tile.color_seed;
        match c % 6 {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            3 => Color::Yellow,
            4 => Color::Magenta,
            _ => Color::Cyan,
        }
    };

    for i in 0..4 {
        let p1 = screen_verts[i];
        let p2 = screen_verts[(i + 1) % 4];
        ctx.draw(&Line {
            x1: p1.re,
            y1: p1.im,
            x2: p2.re,
            y2: p2.im,
            color,
        });
    }

    if is_wall {
        ctx.draw(&Line {
            x1: screen_verts[0].re,
            y1: screen_verts[0].im,
            x2: screen_verts[2].re,
            y2: screen_verts[2].im,
            color,
        });
        ctx.draw(&Line {
            x1: screen_verts[1].re,
            y1: screen_verts[1].im,
            x2: screen_verts[3].re,
            y2: screen_verts[3].im,
            color,
        });
    }

    // Draw Entities in this tile
    // Iterate all entities (or just optimize if needed)
    for entity in &game.entities {
        if entity.path == path {
             let screen_pos = transform.apply(entity.offset);
             let color = match entity.kind {
                 EntityKind::Player => Color::Yellow,
                 EntityKind::Enemy => Color::Red,
                 EntityKind::Item => Color::Magenta,
             };
             let radius = match entity.kind {
                 EntityKind::Player => 0.02,
                 _ => 0.015,
             };

             ctx.draw(&Circle {
                 x: screen_pos.re,
                 y: screen_pos.im,
                 radius,
                 color,
             });
        }
    }

    // 4. Recurse to neighbors
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
        let next_from_dir = (i + 2) % 4;

        draw_tile_recursive(
            ctx,
            game,
            next_path,
            child_transform,
            consts,
            depth + 1,
            Some(next_from_dir),
        );
    }
}

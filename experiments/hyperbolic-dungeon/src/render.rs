use num_complex::Complex;
use ratatui::{
    style::Color,
    widgets::canvas::{Context, Line},
};
use std::f64::consts::PI;

use crate::dungeon::{Dungeon, TileType};
use poincare_disk::{neighbor_transform_a, Mobius, Point, TilingConsts};

pub fn draw_dungeon(
    ctx: &mut Context,
    dungeon: &Dungeon,
    player_path: &[usize],
    view_transform: &Mobius,
    consts: &TilingConsts,
) {
    // We start recursion from the player's current tile.
    // However, the view_transform maps the player's local coordinates to the screen.
    // If the player is at 'offset' in the current tile, view_transform should map 'offset' to (0,0).
    // The recursive drawer assumes it draws a tile centered at 0 in its local frame,
    // mapped to screen by 'transform'.

    // So 'transform' passed to draw_tile is view_transform.

    draw_tile_recursive(
        ctx,
        dungeon,
        player_path.to_vec(),
        *view_transform,
        consts,
        0,
        None,
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
) {
    // 1. Check visibility/size
    // Map center (0) to screen
    let screen_center = transform.apply(Point::new(0.0, 0.0));

    // Simple cull: if too far and small?
    // In Poincare disk, everything is inside unit circle.
    // We stop if the "size" of the tile is too small.
    // Approximate size: map a vertex and check distance to center.
    let v0_local = Complex::from_polar(consts.vertex_offset, PI / 4.0);
    let v0_screen = transform.apply(v0_local);
    let size = (v0_screen - screen_center).norm();

    if size < 0.02 {
        return;
    }

    // Depth limit as safety
    if depth > 10 {
        return;
    }

    // 2. Get Tile Data
    let tile = dungeon.get_tile(&path);
    let is_wall = match tile.tile_type {
        TileType::Wall => true,
        TileType::Floor => false,
    };

    // 3. Draw Geometry
    // Vertices of the square
    let mut screen_verts = [Point::default(); 4];
    for i in 0..4 {
        let angle = (i as f64 * PI / 2.0) + (PI / 4.0);
        let p_local = Complex::from_polar(consts.vertex_offset, angle);
        screen_verts[i] = transform.apply(p_local);
    }

    let color = if is_wall {
        Color::DarkGray
    } else {
        // Procedural color based on seed
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

    // Draw edges
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

    // If Wall, maybe draw an 'X' or fill (can't fill in Canvas easily)
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

    // 4. Recurse to neighbors
    // We recurse even if wall? Yes, because we might see *past* a wall?
    // In a dungeon, usually no. But for "impossible space" visualization, yes.
    // Or maybe Walls block visibility? Let's say yes for "Dungeon" feel.
    if is_wall {
        return;
    }

    for i in 0..4 {
        // Skip the direction we just came from to avoid infinite loops
        if let Some(from) = from_dir {
            if i == from {
                continue;
            }
        }

        let step_a = neighbor_transform_a(i, consts);
        let step_transform = Mobius::translation(step_a);
        let child_transform = transform.then(&step_transform);

        let next_path = Dungeon::canonicalize_step(path.clone(), i);

        // The neighbor i of current node will see current node as neighbor (i + 2) % 4.
        // So when recursing, the new 'from_dir' is (i + 2) % 4.
        let next_from_dir = (i + 2) % 4;

        draw_tile_recursive(
            ctx,
            dungeon,
            next_path,
            child_transform,
            consts,
            depth + 1,
            Some(next_from_dir),
        );
    }
}

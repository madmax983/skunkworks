use crate::origami::Mesh;
use macroquad::prelude::*;

pub fn generate_grid_mesh(cols: usize, rows: usize, width: f32, height: f32) -> Mesh {
    let mut mesh = Mesh::new();
    let dx = width / cols as f32;
    let dy = height / rows as f32;
    let offset_x = -width / 2.0;
    let offset_y = -height / 2.0;

    // Vertices
    for y in 0..=rows {
        for x in 0..=cols {
            let pos = vec3(offset_x + x as f32 * dx, offset_y + y as f32 * dy, 0.0);
            let uv = vec2(x as f32 / cols as f32, y as f32 / rows as f32);
            // Fix top edge
            let fixed = y == 0;
            mesh.add_vertex(pos, uv, fixed);
        }
    }

    // Faces (Triangles)
    for y in 0..rows {
        for x in 0..cols {
            let i0 = y * (cols + 1) + x;
            let i1 = i0 + 1;
            let i2 = (y + 1) * (cols + 1) + x;
            let i3 = i2 + 1;

            // i0 - i1
            // |  / |
            // i2 - i3
            // Triangles: (i0, i1, i2) and (i1, i3, i2)
            // Color based on checkerboard for visibility
            let color = if (x + y) % 2 == 0 {
                Color::new(0.9, 0.9, 0.9, 1.0)
            } else {
                Color::new(0.8, 0.8, 0.8, 1.0)
            };

            mesh.add_triangle(i0, i1, i2, color);
            mesh.add_triangle(i1, i3, i2, color);
        }
    }

    mesh
}

pub fn apply_syntax_creases(mesh: &mut Mesh, code: &str, cols: usize, rows: usize) {
    let chars: Vec<char> = code.chars().filter(|c| !c.is_whitespace()).collect();
    let mut char_idx = 0;

    // We map the linear code string to the grid cells (left-to-right, top-to-bottom)
    for y in 0..rows {
        for x in 0..cols {
            if char_idx >= chars.len() { break; }
            let c = chars[char_idx];
            char_idx += 1;

            // If syntax event, add a vertical crease to the RIGHT of this cell
            if c == '{' || c == '}' || c == '(' || c == ')' {
                 let v_top = y * (cols + 1) + (x + 1);
                 let v_bottom = (y + 1) * (cols + 1) + (x + 1);

                 if let Some(edge_idx) = mesh.get_edge_index(v_top, v_bottom) {
                     let bend = std::f32::consts::PI * 0.5; // 90 degrees
                     let (target, is_mtn) = if c == '{' || c == '(' {
                         (std::f32::consts::PI - bend, true) // Mountain
                     } else {
                         (std::f32::consts::PI + bend, false) // Valley
                     };

                     mesh.add_crease(edge_idx, target, 0.5, is_mtn);
                 }
            }

            // If ';', add a horizontal crease BELOW this cell?
            if c == ';' {
                 let v_left = (y + 1) * (cols + 1) + x;
                 let v_right = (y + 1) * (cols + 1) + (x + 1);

                 if let Some(edge_idx) = mesh.get_edge_index(v_left, v_right) {
                     // Mild fold
                      mesh.add_crease(edge_idx, std::f32::consts::PI + 0.2, 0.2, false);
                 }
            }
        }
    }
}

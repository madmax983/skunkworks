use macroquad::prelude::*;
use miller_lattice::Crystal;
use origami::{generate_miura_grid, MiuraParams, Orientation};
use physics_pbd::PbdSystem;

#[macroquad::main("Origami Lattice")]
async fn main() {
    let root_path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let crystal = Crystal::build_from_path(&root_path).unwrap_or_else(|_| Crystal::new());
    let atom_count = crystal.atoms.len();
    println!(
        "Built Crystal Lattice from {:?} with {} atoms.",
        root_path, atom_count
    );

    let cols = 15;
    let rows = 15;
    let w = cols + 1;

    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 1.2,
        orientation: Orientation::Horizontal,
    };

    let points = generate_miura_grid(params, (cols, rows), 0.5);
    let mut system = PbdSystem::new();
    let mut p_indices = Vec::with_capacity(points.len());

    for pos in &points {
        let p_idx = system.add_particle(*pos, 1.0);
        p_indices.push(p_idx);

        let is_corner = p_indices.len() - 1 == 0
            || p_indices.len() - 1 == cols
            || p_indices.len() - 1 == rows * w
            || p_indices.len() - 1 == rows * w + cols;

        if is_corner {
            system.add_pin_constraint(p_idx, *pos);
        }
    }

    let stiffness = 0.5;
    for y in 0..=rows {
        for x in 0..=cols {
            let idx = y * w + x;

            if x < cols {
                let r_idx = idx + 1;
                system.add_distance_constraint(p_indices[idx], p_indices[r_idx], stiffness);
            }

            if y < rows {
                let d_idx = idx + w;
                system.add_distance_constraint(p_indices[idx], p_indices[d_idx], stiffness);
            }

            if x < cols && y < rows {
                let br_idx = idx + w + 1;
                system.add_distance_constraint(p_indices[idx], p_indices[br_idx], stiffness * 0.5);
            }
        }
    }

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for atom in &crystal.atoms {
        min_x = min_x.min(atom.position.x as f32);
        max_x = max_x.max(atom.position.x as f32);
        min_y = min_y.min(atom.position.y as f32);
        max_y = max_y.max(atom.position.y as f32);
    }

    let crys_w = (max_x - min_x).max(1.0);
    let crys_h = (max_y - min_y).max(1.0);

    let mut cam = Camera3D {
        position: vec3(0.0, 15.0, 20.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    };

    let mut time = 0.0f32;

    loop {
        clear_background(BLACK);
        time += 0.016;

        for atom in &crystal.atoms {
            let normalized_x = (atom.position.x as f32 - min_x) / crys_w;
            let normalized_y = (atom.position.y as f32 - min_y) / crys_h;

            let grid_x = (normalized_x * (cols as f32)).round() as usize;
            let grid_y = (normalized_y * (rows as f32)).round() as usize;

            let grid_x = grid_x.clamp(0, cols);
            let grid_y = grid_y.clamp(0, rows);

            let idx = grid_y * w + grid_x;

            let force_mag = if atom.is_dir { 1.5 } else { -1.5 };
            let pulse = (time * 2.0 + (grid_x + grid_y) as f32 * 0.5).sin();

            system.particles[p_indices[idx]].vel.y += force_mag * pulse * 0.05;
        }

        system.step(0.016, 10);

        set_camera(&cam);

        for y in 0..rows {
            for x in 0..cols {
                let idx = y * w + x;
                let r_idx = idx + 1;
                let d_idx = idx + w;

                let p0 = system.particles[p_indices[idx]].pos;
                let p1 = system.particles[p_indices[r_idx]].pos;
                let p2 = system.particles[p_indices[d_idx]].pos;

                let mq_p0 = vec3(p0.x, p0.y, p0.z);
                let mq_p1 = vec3(p1.x, p1.y, p1.z);
                let mq_p2 = vec3(p2.x, p2.y, p2.z);

                draw_line_3d(mq_p0, mq_p1, Color::new(1.0, 1.0, 1.0, 0.5));
                draw_line_3d(mq_p0, mq_p2, Color::new(1.0, 1.0, 1.0, 0.5));
            }
        }

        for x in 0..cols {
            let idx = rows * w + x;
            let r_idx = idx + 1;
            let p0 = system.particles[p_indices[idx]].pos;
            let p1 = system.particles[p_indices[r_idx]].pos;
            draw_line_3d(
                vec3(p0.x, p0.y, p0.z),
                vec3(p1.x, p1.y, p1.z),
                Color::new(1.0, 1.0, 1.0, 0.5),
            );
        }
        for y in 0..rows {
            let idx = y * w + cols;
            let d_idx = idx + w;
            let p0 = system.particles[p_indices[idx]].pos;
            let p1 = system.particles[p_indices[d_idx]].pos;
            draw_line_3d(
                vec3(p0.x, p0.y, p0.z),
                vec3(p1.x, p1.y, p1.z),
                Color::new(1.0, 1.0, 1.0, 0.5),
            );
        }

        for atom in &crystal.atoms {
            let normalized_x = (atom.position.x as f32 - min_x) / crys_w;
            let normalized_y = (atom.position.y as f32 - min_y) / crys_h;

            let grid_x = (normalized_x * (cols as f32)).round() as usize;
            let grid_y = (normalized_y * (rows as f32)).round() as usize;
            let grid_x = grid_x.clamp(0, cols);
            let grid_y = grid_y.clamp(0, rows);

            let idx = grid_y * w + grid_x;
            let mesh_pos = system.particles[p_indices[idx]].pos;

            let color = if atom.is_dir {
                Color::new(0.0, 1.0, 0.0, 0.8)
            } else {
                Color::new(0.0, 0.5, 1.0, 0.8)
            };

            let mq_pos = vec3(mesh_pos.x, mesh_pos.y + 0.5, mesh_pos.z);
            draw_sphere(mq_pos, 0.2, None, color);
            draw_line_3d(
                mq_pos,
                vec3(mesh_pos.x, mesh_pos.y, mesh_pos.z),
                Color::new(1.0, 1.0, 1.0, 0.3),
            );
        }

        for (parent_idx, child_idx) in &crystal.bonds {
            let p_atom = &crystal.atoms[*parent_idx];
            let c_atom = &crystal.atoms[*child_idx];

            let get_mapped_pos = |atom: &miller_lattice::Atom| {
                let normalized_x = (atom.position.x as f32 - min_x) / crys_w;
                let normalized_y = (atom.position.y as f32 - min_y) / crys_h;
                let grid_x = ((normalized_x * (cols as f32)).round() as usize).clamp(0, cols);
                let grid_y = ((normalized_y * (rows as f32)).round() as usize).clamp(0, rows);
                let idx = grid_y * w + grid_x;
                let p = system.particles[p_indices[idx]].pos;
                vec3(p.x, p.y + 0.5, p.z)
            };

            let pos1 = get_mapped_pos(p_atom);
            let pos2 = get_mapped_pos(c_atom);

            draw_line_3d(pos1, pos2, Color::new(0.0, 1.0, 0.0, 0.2));
        }

        set_default_camera();

        draw_text(
            "Origami Lattice: Codebase Morphogenesis",
            10.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Codebase size: {} files/dirs", atom_count),
            10.0,
            60.0,
            20.0,
            GRAY,
        );
        draw_text(
            "Directories pull up, files pull down.",
            10.0,
            85.0,
            20.0,
            GRAY,
        );

        if is_mouse_button_down(MouseButton::Right) {
            let delta = mouse_position();
            cam.position.x += (delta.0 - 400.0) * 0.01;
            cam.position.y += (delta.1 - 300.0) * 0.01;
        }

        next_frame().await
    }
}

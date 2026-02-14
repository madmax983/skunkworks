use crate::origami::Mesh;
use macroquad::prelude::*;

pub fn solve_constraints(mesh: &mut Mesh, dt: f32) {
    // 1. Prediction
    for v in mesh.vertices.iter_mut() {
        if v.fixed { continue; }
        let vel = v.pos - v.old_pos;
        v.old_pos = v.pos;
        // Gravity? No, or very weak.
        // v.acc += vec3(0.0, 9.8, 0.0) * v.mass;

        v.pos += vel * 0.99 + v.acc * dt * dt;
        v.acc = Vec3::ZERO;
    }

    // 2. Iterative Solver
    let iterations = 10;
    for _ in 0..iterations {
        solve_distance_constraints(mesh);
        solve_bending_constraints(mesh);
    }
}

fn solve_distance_constraints(mesh: &mut Mesh) {
    for i in 0..mesh.edges.len() {
        let (p1_idx, p2_idx, target_len) = {
            let e = &mesh.edges[i];
            (e.a, e.b, e.length)
        };

        let p1 = mesh.vertices[p1_idx].pos;
        let p2 = mesh.vertices[p2_idx].pos;

        let w1 = if mesh.vertices[p1_idx].fixed { 0.0 } else { 1.0 / mesh.vertices[p1_idx].mass };
        let w2 = if mesh.vertices[p2_idx].fixed { 0.0 } else { 1.0 / mesh.vertices[p2_idx].mass };

        if w1 + w2 == 0.0 { continue; }

        let diff = p1 - p2;
        let dist = diff.length();
        if dist == 0.0 { continue; } // Avoid div by zero

        let correction = (dist - target_len) / (w1 + w2);
        let dir = diff / dist;

        if !mesh.vertices[p1_idx].fixed {
            mesh.vertices[p1_idx].pos -= dir * correction * w1;
        }
        if !mesh.vertices[p2_idx].fixed {
            mesh.vertices[p2_idx].pos += dir * correction * w2;
        }
    }
}

fn solve_bending_constraints(mesh: &mut Mesh) {
    // Simplified bending: Cross-product based constraint
    // For each crease, we have 4 vertices: p1, p2 (edge), p3 (wing A), p4 (wing B).
    // wait, edges have p2 (a) and p3 (b). wings are p1 and p4.

    for i in 0..mesh.creases.len() {
        let crease = &mesh.creases[i];
        let edge = &mesh.edges[crease.edge_index];

        if edge.wings[0].is_none() || edge.wings[1].is_none() { continue; }

        let idx_p1 = edge.wings[0].unwrap();
        let idx_p2 = edge.a; // Shared edge vertex 1
        let idx_p3 = edge.b; // Shared edge vertex 2
        let idx_p4 = edge.wings[1].unwrap();

        let p1 = mesh.vertices[idx_p1].pos;
        let p2 = mesh.vertices[idx_p2].pos;
        let p3 = mesh.vertices[idx_p3].pos;
        let p4 = mesh.vertices[idx_p4].pos;

        // Normals of the two triangles
        // T1: p2, p3, p1 (winding might be p2->p1->p3 or p2->p3->p1, need check)
        // Assuming consistent winding isn't guaranteed, we rely on current state?
        // Let's assume (p2, p3) is the shared edge vector.
        // n1 = (p1 - p2) x (p3 - p2) normalized?
        // n2 = (p3 - p2) x (p4 - p2) normalized?

        let v_edge = p3 - p2;
        if v_edge.length_squared() < 1e-6 { continue; }

        // Check winding: (p1, p2, p3) vs (p1, p3, p2)
        // If we use consistent indices logic, p1 is wing.
        // Correct order: p1, p2, p3 and p2, p4, p3 ?
        // Standard PBD Bending uses p1, p2, p3, p4 where p2, p3 is shared edge.
        // Normals: n1 = (p2-p1) x (p3-p1), n2 = (p2-p4) x (p3-p4).

        let p2_p1 = p2 - p1;
        let p3_p1 = p3 - p1;
        let mut n1 = p2_p1.cross(p3_p1);

        let p2_p4 = p2 - p4;
        let p3_p4 = p3 - p4;
        let mut n2 = p2_p4.cross(p3_p4);

        if n1.length_squared() < 1e-6 || n2.length_squared() < 1e-6 { continue; }
        n1 = n1.normalize();
        n2 = n2.normalize();

        let dot = n1.dot(n2).clamp(-1.0, 1.0);
        let cross = n1.cross(n2);
        let sign = if cross.dot(v_edge) > 0.0 { 1.0 } else { -1.0 };
        let current_angle = dot.acos() * sign; // -PI to PI

        // BUT: This angle is 0 if normals aligned (flat or folded 360?).
        // If flat, normals are parallel. dot = 1. acos(1) = 0.
        // Wait, if flat, n1 == n2 ?
        // If p1 and p4 are on opposite sides of edge p2-p3, normals should be same direction?
        // Yes, for a flat sheet, both triangles have normal 'up'.
        // So target angle for flat is 0.

        // Crease target angle:
        // Crease.target_angle is PI for flat in my definition?
        // Let's re-align.
        // In my logic: Flat = 0 (normals parallel).
        // Mountain = PI/2 (normals 90 deg).
        // Valley = -PI/2.
        // So target_angle needs conversion.
        // My crease struct says PI = flat.
        // So target = crease.current_target - std::f32::consts::PI;

        let target = crease.current_target - std::f32::consts::PI;

        // Correction
        // Simple approach: apply forces to rotate normals.
        // But for PBD we need positional updates.
        // Approximating: Move p1 and p4 to satisfy angle.

        // Simplified "Spring" on angle:
        // Just nudge p1 and p4 perpendicular to their triangles to change angle.

        let diff = current_angle - target;
        if diff.abs() < 0.01 { continue; }

        // Bias towards target.
        // We want to rotate n1 and n2 towards each other (or away).
        // A simple heuristic: Move p1 along n1 and p4 along n2?
        // No, moving p1 along n1 doesn't rotate n1. Moving p1 along normal rotates n1? No.
        // Moving p1 along n1 changes nothing (it's perpendicular).
        // Moving p1 along n1 *changes the plane*.

        // Standard formula from PBD paper is long.
        // Let's use the cross-edge distance constraint as a proxy?
        // Calculate target distance between p1 and p4 for the target dihedral angle.
        // h1 = distance of p1 from line p2-p3.
        // h2 = distance of p4 from line p2-p3.
        // dist_p1_p4^2 = h1^2 + h2^2 - 2*h1*h2*cos(dihedral_angle).
        // Wait, dihedral angle is angle between planes.
        // The angle at the hinge axis (inside the material) is PI - dihedral.
        // If normals are parallel (angle 0), then p1 and p4 are farthest apart (flat).
        // Angle at hinge is 180 (PI).
        // cos(PI) = -1. dist^2 = h1^2 + h2^2 + 2h1h2 = (h1+h2)^2. Correct.
        // If folded flat (normals anti-parallel, angle PI), hinge angle is 0.
        // cos(0) = 1. dist^2 = h1^2 + h2^2 - 2h1h2 = (h1-h2)^2. Correct.

        // So: internal_angle = PI - target (where target is 0 for flat).
        // Or using my mapping: target 0 is flat. internal angle = PI.
        // target PI/2 is mountain. internal angle = PI/2.

        // Calculate h1, h2
        // h1 = |(p1 - p2) x (p3 - p2)| / |p3 - p2|
        // Already computed cross products (unnormalized n1/n2 lengths).
        let area_x2_1 = (p2 - p1).cross(p3 - p1).length();
        let area_x2_2 = (p2 - p4).cross(p3 - p4).length();
        let len_edge = v_edge.length();

        let h1 = area_x2_1 / len_edge;
        let h2 = area_x2_2 / len_edge;

        // Inner angle theta = PI - target (approx).
        // Wait, target is signed (-PI to PI).
        // Target 0 (Flat) -> Theta PI.
        // Target PI/2 -> Theta PI/2.
        // Target -PI/2 -> Theta 3PI/2 (or -PI/2).

        let theta = std::f32::consts::PI - target;
        let target_dist_sq = h1*h1 + h2*h2 - 2.0*h1*h2*theta.cos();
        let target_dist = target_dist_sq.sqrt();

        // Enforce distance constraint between p1 and p4
        // But only correct if this doesn't break edge lengths?
        // It's an approximation.
        // Also p1 and p4 are constrained by edges (p1-p2, p1-p3 etc).
        // So this effectively bends the hinge.

        // Apply distance correction p1-p4
        let w1 = if mesh.vertices[idx_p1].fixed { 0.0 } else { 1.0 / mesh.vertices[idx_p1].mass };
        let w4 = if mesh.vertices[idx_p4].fixed { 0.0 } else { 1.0 / mesh.vertices[idx_p4].mass };
        if w1 + w4 == 0.0 { continue; }

        let diff_vec = p1 - p4;
        let current_dist = diff_vec.length();
        if current_dist == 0.0 { continue; }

        let correction = (current_dist - target_dist) / (w1 + w4) * crease.stiffness;
        let dir = diff_vec / current_dist;

        if !mesh.vertices[idx_p1].fixed {
            mesh.vertices[idx_p1].pos -= dir * correction * w1;
        }
        if !mesh.vertices[idx_p4].fixed {
            mesh.vertices[idx_p4].pos += dir * correction * w4;
        }
    }
}

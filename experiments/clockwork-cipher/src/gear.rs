use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

pub fn generate_gear_path(teeth: usize, module: f32, pressure_angle_deg: f32) -> Path {
    let mut builder = PathBuilder::new();

    // Generate full gear profile
    let points = generate_gear_points(teeth, module, pressure_angle_deg, 10);

    if points.is_empty() {
        return Path::default();
    }

    builder.move_to(points[0]);
    for point in points.iter().skip(1) {
        builder.line_to(*point);
    }
    builder.close();

    builder.build()
}

pub fn generate_gear_collider(teeth: usize, module: f32, pressure_angle_deg: f32) -> Collider {
    let pitch_radius = (teeth as f32 * module) / 2.0;
    let root_radius = pitch_radius - 1.25 * module;

    // 1. Central Disc (slightly smaller than root to avoid overlap issues with tooth roots)
    // Actually, make it exactly root_radius.
    let mut shapes = vec![(Vect::ZERO, 0.0, Collider::ball(root_radius * 0.99))];

    // 2. Teeth
    // Generate the polygon for ONE tooth, centered at angle 0.
    let tooth_points = generate_single_tooth_polygon(teeth, module, pressure_angle_deg, 4);

    if let Some(tooth_collider) = Collider::convex_hull(&tooth_points) {
        for i in 0..teeth {
            let angle = i as f32 * 2.0 * PI / teeth as f32;
            shapes.push((Vect::ZERO, angle, tooth_collider.clone()));
        }
    }

    Collider::compound(shapes)
}

fn generate_gear_points(
    teeth: usize,
    module: f32,
    pressure_angle_deg: f32,
    steps_per_tooth: usize,
) -> Vec<Vec2> {
    let mut points = Vec::new();
    let tooth_angle_step = 2.0 * PI / teeth as f32;

    for i in 0..teeth {
        let center_angle = i as f32 * tooth_angle_step;

        // Generate points for one tooth rotated by center_angle
        let tooth_poly =
            generate_single_tooth_polygon(teeth, module, pressure_angle_deg, steps_per_tooth);

        for p in tooth_poly {
            // Rotate p by center_angle
            let rotated = rotate_vec2(p, center_angle);
            points.push(rotated);
        }
    }
    points
}

fn generate_single_tooth_polygon(
    teeth: usize,
    module: f32,
    pressure_angle_deg: f32,
    steps: usize,
) -> Vec<Vec2> {
    let pitch_radius = (teeth as f32 * module) / 2.0;
    let base_radius = pitch_radius * (pressure_angle_deg.to_radians()).cos();
    let addendum = module;
    let dedendum = 1.25 * module;
    let outer_radius = pitch_radius + addendum;
    let root_radius = pitch_radius - dedendum;

    let mut points = Vec::new();

    // Calculate tooth half-thickness angle at base circle (or pitch circle logic)
    // Theta_pitch = pi / (2 * N)
    // Involute function inv(alpha) = tan(alpha) - alpha

    let alpha_pitch = pressure_angle_deg.to_radians();
    let inv_pitch = alpha_pitch.tan() - alpha_pitch;

    // Angle offset to the involute origin from the centerline of the tooth
    // half_thick_angle = pi/(2N) + inv(alpha_pitch)
    let mut half_thick_angle = PI / (2.0 * teeth as f32) + inv_pitch;

    // Apply Backlash (reduce tooth thickness)
    let backlash_angle = (PI / teeth as f32) * 0.1; // 10% of pitch angle
    half_thick_angle -= backlash_angle / 2.0;

    // Generate Left Side (from root to tip)
    // We iterate radius from root to outer

    // Helper to get angle on gear for a given radius r
    let get_angle_at_r = |r: f32| -> f32 {
        if r < base_radius {
            // Below base radius, straight radial line?
            // Correct would be some flank, but let's approximate as 0 deviation from base?
            // Or just project strictly radially from the base start point.
            // Angle at base radius:
            // inv(0) = 0. So angle is just half_thick_angle.
            return half_thick_angle;
        }
        // alpha_r = acos(Rb / r)
        let alpha_r = (base_radius / r).acos();
        let inv_r = alpha_r.tan() - alpha_r;
        half_thick_angle - inv_r
    };

    // 1. Left Flank (Root to Tip)
    // We assume the tooth is symmetric around Y axis? Or X axis?
    // Let's align it with X axis (angle 0).
    // Top half is Y > 0.

    // We go from Root up to Tip
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let r = root_radius + (outer_radius - root_radius) * t;
        let angle = get_angle_at_r(r); // This is positive

        // Point: angle is positive (Top half of tooth)
        points.push(Vec2::new(r * angle.cos(), r * angle.sin()));
    }

    // 2. Tip (Arc) - Optional, or just connect the last points
    // If we want a flat top or curved top.
    // Let's add one point at -angle?
    // No, we need to go down the other side.

    // 3. Right Flank (Tip to Root)
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let r = outer_radius - (outer_radius - root_radius) * t;
        let angle = -get_angle_at_r(r); // Negative angle (Bottom half)

        points.push(Vec2::new(r * angle.cos(), r * angle.sin()));
    }

    // 4. Root (Arc to next tooth?)
    // This polygon is for ONE tooth.
    // Ideally it should close at the root.
    // The previous loop ended at (root_radius, -angle).
    // The first point was (root_radius, +angle).
    // We can close it.

    points
}

fn rotate_vec2(v: Vec2, angle: f32) -> Vec2 {
    let cos = angle.cos();
    let sin = angle.sin();
    Vec2::new(v.x * cos - v.y * sin, v.x * sin + v.y * cos)
}

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::PI;

pub fn create_gear_path(radius: f32, teeth: u32) -> Path {
    let mut path_builder = PathBuilder::new();

    let root_radius = radius * 0.85;
    let outer_radius = radius;
    let tooth_width_angle = (2.0 * PI / (teeth as f32)) * 0.5; // 50% tooth, 50% gap
    let half_tooth = tooth_width_angle / 2.0;

    // Start at the first point
    // We will draw segments.

    for i in 0..teeth {
        let angle_center = i as f32 * (2.0 * PI / teeth as f32);

        // Tooth profile (trapezoidal approximation)
        let angle_root_start = angle_center - half_tooth * 1.2; // Slightly wider at root
        let angle_root_end = angle_center + half_tooth * 1.2;
        let angle_tip_start = angle_center - half_tooth * 0.8; // Narrower at tip
        let angle_tip_end = angle_center + half_tooth * 0.8;

        // Gap between this tooth and previous is handled by the line to root_start

        // Coordinates
        let p_root_start = Vec2::new(root_radius * angle_root_start.cos(), root_radius * angle_root_start.sin());
        let p_tip_start = Vec2::new(outer_radius * angle_tip_start.cos(), outer_radius * angle_tip_start.sin());
        let p_tip_end = Vec2::new(outer_radius * angle_tip_end.cos(), outer_radius * angle_tip_end.sin());
        let p_root_end = Vec2::new(root_radius * angle_root_end.cos(), root_radius * angle_root_end.sin());

        if i == 0 {
            path_builder.move_to(p_root_start);
        } else {
            path_builder.line_to(p_root_start);
        }

        path_builder.line_to(p_tip_start);
        path_builder.line_to(p_tip_end);
        path_builder.line_to(p_root_end);
    }

    path_builder.close();
    path_builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_path_generation() {
        // Just verify it doesn't panic and returns a path
        let path = create_gear_path(100.0, 12);
        assert!(path.0.iter().count() > 0);
    }
}

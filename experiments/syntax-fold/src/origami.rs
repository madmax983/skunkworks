use macroquad::prelude::*;
use crate::parser::{Scope, Line};

pub struct OrigamiMesh {
    pub line_height: f32,
    pub line_width: f32,
}

impl OrigamiMesh {
    pub fn new() -> Self {
        Self {
            line_height: 1.0,
            line_width: 20.0,
        }
    }

    /// Computes the Model Matrix for each line based on the fold state of scopes.
    pub fn calculate_transforms(
        &self,
        lines: &[Line],
        root_scope: &Scope,
        fold_states: &std::collections::HashMap<usize, f32> // Key: Scope start_line index, Value: 0.0-1.0
    ) -> Vec<Mat4> {
        let mut hinge_angles = vec![0.0f32; lines.len()];

        // 1. Assign target angles based on scopes
        self.apply_scope_angles(root_scope, fold_states, &mut hinge_angles);

        // 2. Compute transforms forward kinematics
        let mut transforms = Vec::with_capacity(lines.len());
        let mut current_transform = Mat4::IDENTITY;

        for i in 0..lines.len() {
            // The position of the current line is based on the previous chain
            transforms.push(current_transform);

            // Prepare for the next line
            // Move to the bottom of the current line
            let translate = Mat4::from_translation(vec3(0.0, -self.line_height, 0.0));

            // Rotate around the hinge at the bottom of this line
            let angle = hinge_angles[i].to_radians();
            let rotate = Mat4::from_rotation_x(angle);

            current_transform = current_transform * translate * rotate;
        }

        transforms
    }

    fn apply_scope_angles(
        &self,
        scope: &Scope,
        fold_states: &std::collections::HashMap<usize, f32>,
        hinge_angles: &mut Vec<f32>
    ) {
        // If this scope is folded (partially or fully)
        if let Some(&ratio) = fold_states.get(&scope.start_line) {
            // Check bounds to be safe
            if scope.start_line < hinge_angles.len() && scope.end_line < hinge_angles.len() {
                // Max fold angle (almost 180 to avoid Z-fighting, and 180 might flip normals weirdly)
                // Let's use 170 degrees for a visible "folded back" look
                let max_angle = 175.0;

                // Hinge after Header (Valley Fold)
                hinge_angles[scope.start_line] += ratio * -max_angle;

                // Hinge after Footer (Mountain Fold) - to bring the rest of the paper back to parallel
                hinge_angles[scope.end_line] += ratio * max_angle;
            }
        }

        // Recurse
        for child in &scope.children {
            self.apply_scope_angles(child, fold_states, hinge_angles);
        }
    }

    /// Checks flat-foldability of the current configuration.
    /// In this 1D strip model, flat-foldability implies that the net rotation for a "pleat" is 0.
    /// i.e., The paper returns to the original plane (or parallel to it).
    pub fn check_foldability(&self, lines: &[Line], root_scope: &Scope, fold_states: &std::collections::HashMap<usize, f32>) -> bool {
         let mut hinge_angles = vec![0.0f32; lines.len()];
         self.apply_scope_angles(root_scope, fold_states, &mut hinge_angles);

         // In a simple pleat system, the sum of angles should be roughly 0 (modulo 360)
         // if we consider the accumulated rotation.
         // However, we are just checking if every Valley has a corresponding Mountain?
         // Actually, our `apply_scope_angles` FORCES this by adding +Angle and -Angle.
         // So by definition, our procedural generation creates flat-foldable patterns.

         // Let's verify that the total rotation at the end is 0.
         let sum: f32 = hinge_angles.iter().sum();
         sum.abs() < 0.1
    }

    pub fn get_crease_pattern_info(&self, lines: &[Line], root_scope: &Scope, fold_states: &std::collections::HashMap<usize, f32>) -> Vec<(usize, String)> {
         let mut hinge_angles = vec![0.0f32; lines.len()];
         self.apply_scope_angles(root_scope, fold_states, &mut hinge_angles);

         hinge_angles.iter().enumerate().filter(|(_, &a)| a.abs() > 1.0).map(|(i, &a)| {
             let kind = if a < 0.0 { "VALLEY" } else { "MOUNTAIN" };
             (i, format!("{} ({:.1}°)", kind, a))
         }).collect()
    }
}

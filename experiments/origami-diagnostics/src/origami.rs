use macroquad::prelude::*;
use crate::diagnostics::Diagnostic;

const CONTEXT_LINES: usize = 3;
const LINE_HEIGHT: f32 = 20.0; // In texture pixels
const PAPER_WIDTH: f32 = 800.0; // In texture pixels
const WORLD_SCALE: f32 = 0.05; // Scaling pixel to world

#[derive(Clone, Debug)]
pub struct LineState {
    pub content: String,
    pub original_index: usize,
    pub is_folded: bool,
    pub is_error: bool,
}

pub struct PaperModel {
    pub lines: Vec<LineState>,
    pub mesh: Mesh,
    pub total_height: f32,
}

impl PaperModel {
    pub fn new(source_code: &str, diagnostics: &[Diagnostic]) -> Self {
        let source_lines: Vec<&str> = source_code.lines().collect();
        let mut lines = Vec::new();

        // Initialize lines
        for (i, content) in source_lines.iter().enumerate() {
            lines.push(LineState {
                content: content.to_string(),
                original_index: i,
                is_folded: true, // Default to folded
                is_error: false,
            });
        }

        // Mark error lines and context
        for diag in diagnostics {
            for span in &diag.spans {
                // Determine range
                let start = span.line_start.saturating_sub(1); // 1-based to 0-based
                let end = span.line_end.saturating_sub(1);

                // Mark error lines
                for i in start..=end {
                    if i < lines.len() {
                        lines[i].is_error = true;
                        lines[i].is_folded = false;
                    }
                }

                // Mark context
                let ctx_start = start.saturating_sub(CONTEXT_LINES);
                let ctx_end = (end + CONTEXT_LINES).min(lines.len() - 1);

                for i in ctx_start..=ctx_end {
                    if i < lines.len() {
                        lines[i].is_folded = false;
                    }
                }
            }
        }

        // If no diagnostics, unfold everything
        if diagnostics.is_empty() {
             for line in &mut lines {
                 line.is_folded = false;
             }
        }

        let mut model = PaperModel {
            lines,
            mesh: Mesh {
                vertices: Vec::new(),
                indices: Vec::new(),
                texture: None,
            },
            total_height: 0.0,
        };

        model.generate_mesh_indices();
        model.update_mesh(0.0); // Initial state
        model
    }

    fn generate_mesh_indices(&mut self) {
        self.mesh.indices.clear();
        let num_lines = self.lines.len();
        for i in 0..num_lines {
            let base = (i * 4) as u16;
            // Quad: 0, 1, 2, 0, 2, 3
            self.mesh.indices.push(base + 0);
            self.mesh.indices.push(base + 1);
            self.mesh.indices.push(base + 2);
            self.mesh.indices.push(base + 0);
            self.mesh.indices.push(base + 2);
            self.mesh.indices.push(base + 3);
        }
    }

    pub fn update_mesh(&mut self, t: f32) {
        // t goes from 0.0 (fully folded/compressed) to 1.0 (fully unfolded/flat)
        // Wait, the plan said "User adjusts t (fold/unfold)".
        // Let's say t=0 is FLAT (normal view), t=1 is FOLDED (origami mode).

        self.mesh.vertices.clear();

        let line_world_height = LINE_HEIGHT * WORLD_SCALE;
        let line_world_width = PAPER_WIDTH * WORLD_SCALE;

        // Correct approach: Calculate edge positions first.
        let mut edge_positions = Vec::new();
        let mut cursor_y = 0.0;
        let mut cursor_z = 0.0;

        edge_positions.push(vec3(0.0, cursor_y, cursor_z));

        for (i, line) in self.lines.iter().enumerate() {
            let h = line_world_height;
            let mut dy = h;
            let mut dz = 0.0;

            if line.is_folded {
                // If folded, we contract Y and expand Z
                // We want to alternate slopes.
                // Line 0: /
                // Line 1: \
                // Line 2: /

                // Calculate angle based on t
                // t=0 -> angle=0 (flat)
                // t=1 -> angle=80 deg (folded)
                let max_angle = 80.0f32.to_radians();
                let angle = max_angle * t;

                dy = h * angle.cos();
                let z_len = h * angle.sin();

                // Direction depends on index
                // Even index (0, 2): Go "Back" (Z decreases)? Or "Front"?
                // Let's say:
                // 0: / (Z increases)
                // 1: \ (Z decreases)
                if i % 2 == 0 {
                    dz = z_len;
                } else {
                    dz = -z_len;
                }
            }

            cursor_y -= dy; // Y goes down (negative)
            cursor_z += dz;

            edge_positions.push(vec3(0.0, cursor_y, cursor_z));
        }

        // Now build vertices
        for i in 0..self.lines.len() {
            let top_pos = edge_positions[i];
            let bottom_pos = edge_positions[i+1];

            let uv_y_start = i as f32 / self.lines.len() as f32;
            let uv_y_end = (i + 1) as f32 / self.lines.len() as f32;

            let w = line_world_width / 2.0;

            // 0: Top-Left
            self.mesh.vertices.push(Vertex {
                position: vec3(-w, top_pos.y, top_pos.z),
                uv: vec2(0.0, uv_y_start),
                color: WHITE.into(),
                normal: vec4(0.0, 0.0, 1.0, 0.0),
            });

            // 1: Top-Right
            self.mesh.vertices.push(Vertex {
                position: vec3(w, top_pos.y, top_pos.z),
                uv: vec2(1.0, uv_y_start),
                color: WHITE.into(),
                normal: vec4(0.0, 0.0, 1.0, 0.0),
            });

            // 2: Bottom-Right
            self.mesh.vertices.push(Vertex {
                position: vec3(w, bottom_pos.y, bottom_pos.z),
                uv: vec2(1.0, uv_y_end),
                color: WHITE.into(),
                normal: vec4(0.0, 0.0, 1.0, 0.0),
            });

            // 3: Bottom-Left
            self.mesh.vertices.push(Vertex {
                position: vec3(-w, bottom_pos.y, bottom_pos.z),
                uv: vec2(0.0, uv_y_end),
                color: WHITE.into(),
                normal: vec4(0.0, 0.0, 1.0, 0.0),
            });
        }

        self.total_height = cursor_y.abs();
    }
}

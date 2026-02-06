use glam::{Vec3, Mat4, Vec4Swizzles};
use ratatui::widgets::canvas::Context;
use ratatui::style::Color;
use crate::topology::figure_8_klein;
use crate::fs::FsNode;

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3) -> Self {
        Self {
            position,
            target,
            up: Vec3::Y,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    pub fn projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        Mat4::perspective_rh(45.0_f32.to_radians(), aspect_ratio, 0.1, 100.0)
    }
}

pub fn project(p: Vec3, view_proj: Mat4) -> Option<(f64, f64)> {
    let p4 = view_proj * p.extend(1.0);
    if p4.w <= 0.0 {
        return None; // Behind camera
    }
    let ndc = p4.xyz() / p4.w;
    Some((ndc.x as f64, ndc.y as f64))
}

pub fn draw_wireframe(ctx: &mut Context, camera: &Camera, aspect_ratio: f32) {
    let view_proj = camera.projection_matrix(aspect_ratio) * camera.view_matrix();

    let u_steps = 40;
    let v_steps = 20;
    let radius = 2.0;

    for j in 0..v_steps {
        let v = (j as f32 / v_steps as f32) * 2.0 * std::f32::consts::PI;
        let mut prev_point: Option<(f64, f64)> = None;

        for i in 0..=u_steps {
             let u = (i as f32 / u_steps as f32) * 2.0 * std::f32::consts::PI;
             let p = figure_8_klein(u, v, radius);

             if let Some((x, y)) = project(p, view_proj) {
                 if let Some((px, py)) = prev_point {
                     if (px - x).abs() < 1.0 && (py - y).abs() < 1.0 {
                        ctx.draw(&ratatui::widgets::canvas::Line {
                            x1: px, y1: py,
                            x2: x, y2: y,
                            color: Color::DarkGray,
                        });
                     }
                 }
                 prev_point = Some((x, y));
             } else {
                 prev_point = None;
             }
        }
    }

    for i in 0..u_steps {
        let u = (i as f32 / u_steps as f32) * 2.0 * std::f32::consts::PI;
        let mut prev_point: Option<(f64, f64)> = None;

        for j in 0..=v_steps {
            let v = (j as f32 / v_steps as f32) * 2.0 * std::f32::consts::PI;
             let p = figure_8_klein(u, v, radius);

             if let Some((x, y)) = project(p, view_proj) {
                 if let Some((px, py)) = prev_point {
                      if (px - x).abs() < 1.0 && (py - y).abs() < 1.0 {
                         ctx.draw(&ratatui::widgets::canvas::Line {
                             x1: px, y1: py,
                             x2: x, y2: y,
                             color: Color::DarkGray,
                         });
                      }
                 }
                 prev_point = Some((x, y));
             } else {
                 prev_point = None;
             }
        }
    }
}

// Removed lifetimes 'a, 'b.
pub fn draw_nodes(ctx: &mut Context, camera: &Camera, aspect_ratio: f32, nodes: &[FsNode], selected_idx: usize) {
    let view_proj = camera.projection_matrix(aspect_ratio) * camera.view_matrix();
    let radius = 2.0;

    for (i, node) in nodes.iter().enumerate() {
        let p = figure_8_klein(node.u, node.v, radius);

        if let Some((x, y)) = project(p, view_proj) {
            let is_selected = i == selected_idx;
            let color = if is_selected { Color::Yellow } else {
                if node.is_dir { Color::Cyan } else { Color::Green }
            };
            let r = if is_selected { 0.05 } else { 0.02 };

            ctx.draw(&ratatui::widgets::canvas::Circle {
                x, y, radius: r, color
            });

            if is_selected {
                 // Clone the name to create an owned Span, avoiding lifetime dependency on `nodes`.
                 ctx.print(x, y + 0.05, ratatui::text::Span::styled(node.name.clone(), ratatui::style::Style::default().fg(color).add_modifier(ratatui::style::Modifier::BOLD)));
            }
        }
    }
}

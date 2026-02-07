use crate::world::World;
use crate::topology::figure_8_klein;
use glam::{Mat4, Vec3, Vec4Swizzles};
use ratatui::style::Color;
use ratatui::widgets::canvas::{Context, Line};
use std::f32::consts::TAU;

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

pub fn draw_boids(
    ctx: &mut Context<'_>,
    camera: &Camera,
    aspect_ratio: f32,
    world: &World,
) {
    let view_proj = camera.projection_matrix(aspect_ratio) * camera.view_matrix();
    let radius = 2.0;

    // 1. Draw Wireframe (Low Res for performance)
    let u_steps = 30;
    let v_steps = 15;

    // Horizontal lines (along U)
    for j in 0..v_steps {
        let v = (j as f32 / v_steps as f32) * TAU;
        let mut prev_point: Option<(f64, f64)> = None;
        for i in 0..=u_steps {
            let u = (i as f32 / u_steps as f32) * TAU;
            let p = figure_8_klein(u, v, radius);
            if let Some((x, y)) = project(p, view_proj) {
                if let Some((px, py)) = prev_point {
                    if (px - x).abs() < 1.0 && (py - y).abs() < 1.0 {
                        ctx.draw(&Line {
                            x1: px,
                            y1: py,
                            x2: x,
                            y2: y,
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

    // Vertical lines (along V) - Optional, adds structure
    for i in (0..=u_steps).step_by(5) {
        let u = (i as f32 / u_steps as f32) * TAU;
         let mut prev_point: Option<(f64, f64)> = None;
         for j in 0..=v_steps {
            let v = (j as f32 / v_steps as f32) * TAU;
            let p = figure_8_klein(u, v, radius);
            if let Some((x, y)) = project(p, view_proj) {
                if let Some((px, py)) = prev_point {
                    if (px - x).abs() < 1.0 && (py - y).abs() < 1.0 {
                         ctx.draw(&Line {
                            x1: px,
                            y1: py,
                            x2: x,
                            y2: y,
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

    // 2. Draw Boids
    for boid in &world.boids {
        let p = figure_8_klein(boid.position.x, boid.position.y, radius);

        if let Some((x, y)) = project(p, view_proj) {
            let (char_str, color) = if boid.flash_timer > 0 {
                ("★".to_string(), Color::White)
            } else {
                (boid.dna.char_representation.to_string(), boid.dna.color)
            };

            ctx.print(
                x,
                y,
                ratatui::text::Span::styled(char_str, ratatui::style::Style::default().fg(color))
            );
        }
    }
}

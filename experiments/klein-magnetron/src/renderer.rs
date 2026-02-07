use crate::model::Platter;
use crate::topology::figure_8_klein;
use glam::{Mat4, Vec3, Vec4Swizzles};
use ratatui::style::Color;
use ratatui::widgets::canvas::{Context, Line};

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

pub fn draw_platter(
    ctx: &mut Context<'_>,
    camera: &Camera,
    aspect_ratio: f32,
    platter: &Platter,
    selected_idx: usize,
) {
    let view_proj = camera.projection_matrix(aspect_ratio) * camera.view_matrix();
    let radius = 2.0;

    // Draw Wireframe (Context)
    let u_steps = 40;
    let v_steps = 20;

    // Horizontal lines (along U)
    for j in 0..v_steps {
        let v = (j as f32 / v_steps as f32) * 2.0 * std::f32::consts::PI;
        let mut prev_point: Option<(f64, f64)> = None;
        for i in 0..=u_steps {
            let u = (i as f32 / u_steps as f32) * 2.0 * std::f32::consts::PI;
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

    // Draw Sectors
    for (i, sector) in platter.sectors.iter().enumerate() {
        let p = figure_8_klein(sector.u, sector.v, radius);

        if let Some((x, y)) = project(p, view_proj) {
            // Determine Color
            let mut color = if sector.magnetization > 0.8 {
                Color::Green
            } else if sector.magnetization > 0.4 {
                Color::Yellow
            } else {
                Color::Red
            };

            // Highlight selected
            if i == selected_idx {
                color = Color::Cyan;
            }

            // Draw as a small circle or symbol
            // Ratatui canvas symbols are limited resolution
            ctx.print(
                x,
                y,
                ratatui::text::Span::styled("■", ratatui::style::Style::default().fg(color)),
            );

            if i == selected_idx {
                // Clone label to avoid lifetime issues
                let label = sector.label.clone();
                ctx.print(
                    x,
                    y + 0.1,
                    ratatui::text::Span::styled(
                        label,
                        ratatui::style::Style::default()
                            .fg(Color::White)
                            .bg(Color::Black),
                    ),
                );
            }
        }
    }
}

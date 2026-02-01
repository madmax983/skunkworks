use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color},
    widgets::Widget,
};
use crate::solver::Fluid;
use std::f32::consts::PI;

pub struct FluidWidget<'a> {
    pub fluid: &'a Fluid,
}

impl<'a> Widget for FluidWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let n = self.fluid.size;

        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                // Map screen (x, y) to fluid grid (fx, fy)
                // For better visualization, let's scale coordinates?
                // No, 1-to-1 is easiest.

                let fx = (x - area.left()) as usize;
                // Terminal pixels are usually 2x taller than wide.
                // But fluid grid is square.
                // Let's just map direct.
                let fy = (y - area.top()) as usize;

                if fx >= n || fy >= n {
                    continue;
                }

                let idx = fx + fy * n;
                let d = self.fluid.density[idx];
                let vx = self.fluid.vx[idx];
                let vy = self.fluid.vy[idx];

                // Color mapping (Density)
                // Clamp density 0.0 - 1.0 (it can go higher)
                let d_clamped = d.min(1.0).max(0.0);

                let bg = if d_clamped > 0.8 {
                    Color::White
                } else if d_clamped > 0.6 {
                    Color::Cyan
                } else if d_clamped > 0.4 {
                    Color::Blue
                } else if d_clamped > 0.2 {
                    Color::DarkGray
                } else if d_clamped > 0.05 {
                    Color::Rgb(20, 20, 50) // Very dark blue
                } else {
                    Color::Reset
                };

                // Character mapping (Velocity)
                let cell = buf.cell_mut((x, y));
                if let Some(cell) = cell {
                    cell.set_bg(bg);

                    // Only draw arrows if there is some density or high velocity
                    if d_clamped > 0.05 || vx.abs() > 0.1 || vy.abs() > 0.1 {
                        let arrow = get_arrow(vx, vy);
                        cell.set_char(arrow);

                        // Contrast text color
                        if bg == Color::White || bg == Color::Cyan {
                            cell.set_fg(Color::Black);
                        } else {
                            cell.set_fg(Color::Gray);
                        }
                    } else {
                        cell.set_char(' ');
                    }
                }
            }
        }
    }
}

fn get_arrow(vx: f32, vy: f32) -> char {
    let mag = (vx * vx + vy * vy).sqrt();
    if mag < 0.01 {
        return ' ';
    }

    let angle = vy.atan2(vx); // -PI to PI

    // 8 directions
    // 0 is Right (1, 0)
    // PI/2 is Down (0, 1) in screen coords?
    // Wait, in solver:
    // y goes 0..N.
    // In TUI: y goes Top..Bottom (increase).
    // So +y is Down.
    // atan2(y, x).

    // -PI   .. -7/8 PI -> Left
    // -7/8 PI .. -5/8 PI -> Up-Left
    // -5/8 PI .. -3/8 PI -> Up
    // -3/8 PI .. -1/8 PI -> Up-Right
    // -1/8 PI .. 1/8 PI -> Right
    // 1/8 PI .. 3/8 PI -> Down-Right
    // 3/8 PI .. 5/8 PI -> Down
    // 5/8 PI .. 7/8 PI -> Down-Left
    // 7/8 PI .. PI -> Left

    let step = PI / 4.0;
    let a = angle + step / 2.0; // Offset to center intervals

    // Normalize to 0..2PI
    let a = if a < 0.0 { a + 2.0 * PI } else { a };

    let sector = (a / step).floor() as usize;

    match sector % 8 {
        0 => '→',
        1 => '↘',
        2 => '↓',
        3 => '↙',
        4 => '←',
        5 => '↖',
        6 => '↑',
        7 => '↗',
        _ => '?',
    }
}

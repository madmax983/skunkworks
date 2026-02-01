use crate::lbm::Fluid;
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

pub struct FluidWidget<'a> {
    pub fluid: &'a Fluid,
}

impl<'a> Widget for FluidWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let fluid = self.fluid;

        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                // Map to fluid coordinates (2x4 block)
                // Fluid is 0-indexed. Area is absolute.
                // We assume fluid 0,0 corresponds to area.left(), area.top().
                let rel_x = (x - area.left()) as usize;
                let rel_y = (y - area.top()) as usize;

                let base_fx = rel_x * 2;
                let base_fy = rel_y * 4;

                if base_fx + 1 >= fluid.width || base_fy + 3 >= fluid.height {
                    continue;
                }

                let mut mask: u32 = 0;
                let mut curl_sum = 0.0;
                let mut count = 0;
                let mut obstacle_count = 0;

                // 2x4 Grid
                // 0 3
                // 1 4
                // 2 5
                // 6 7
                // (Using standard Braille dot numbering 1-8 mapped to 0-7 indices here)
                // Dot 1 (0): 0x01
                // Dot 2 (1): 0x02
                // Dot 3 (2): 0x04
                // Dot 7 (6): 0x40 (Bottom-most Left)
                // Dot 4 (3): 0x08 (Top Right)
                // Dot 5 (4): 0x10
                // Dot 6 (5): 0x20
                // Dot 8 (7): 0x80 (Bottom-most Right)

                // Offsets (dx, dy)
                let offsets = [
                    (0, 0, 0x01), // Dot 1
                    (0, 1, 0x02), // Dot 2
                    (0, 2, 0x04), // Dot 3
                    (0, 3, 0x40), // Dot 7
                    (1, 0, 0x08), // Dot 4
                    (1, 1, 0x10), // Dot 5
                    (1, 2, 0x20), // Dot 6
                    (1, 3, 0x80), // Dot 8
                ];

                for &(dx, dy, bit) in &offsets {
                    let fx = base_fx + dx;
                    let fy = base_fy + dy;

                    let idx = fy * fluid.width + fx;
                    if fluid.obstacles[idx] {
                        mask |= bit;
                        obstacle_count += 1;
                    } else {
                        // Visualize Curl or Speed?
                        // Let's visualize curl intensity.
                        let curl = fluid.get_curl(fx, fy);
                        let speed = (fluid.u_x[idx].powi(2) + fluid.u_y[idx].powi(2)).sqrt();

                        curl_sum += curl;
                        count += 1;

                        // Threshold for dot
                        if curl.abs() > 0.005 || speed > 0.05 {
                            mask |= bit;
                        }
                    }
                }

                if mask == 0 {
                    continue; // Empty cell
                }

                // Color Logic
                let color = if obstacle_count > 4 {
                    Color::White
                } else if count > 0 {
                    let avg_curl = curl_sum / count as f32;
                    if avg_curl > 0.001 {
                        Color::Red
                    } else if avg_curl < -0.001 {
                        Color::Cyan
                    } else {
                        Color::DarkGray // Just speed/pressure
                    }
                } else {
                    Color::Gray
                };

                // Unicode Braille: 0x2800 + mask
                // Rust char from u32
                if let Some(c) = char::from_u32(0x2800 + mask) {
                    // ratatui 0.29+ uses cell_mut((x,y))
                    buf.cell_mut((x, y)).unwrap().set_char(c).set_fg(color);
                }
            }
        }
    }
}

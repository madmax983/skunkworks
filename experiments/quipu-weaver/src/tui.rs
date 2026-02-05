use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Widget,
};
use crate::quipu::{Quipu, Cord, Knot};

pub struct QuipuWidget<'a> {
    pub quipu: &'a Quipu,
    pub scroll_x: u16,
    pub scroll_y: u16,
}

impl<'a> Widget for QuipuWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Simple Layout Engine
        // 1. Draw Primary Cord (Horizontal Line)
        // 2. Traverse children and draw them.

        let primary_y = area.top() + 2 - (self.scroll_y as u16); // Leave space for title
        let start_x = area.left() + 2 - (self.scroll_x as u16);

        // Draw Primary Cord Line
        // We don't know the width yet, but let's draw a long line.
        // Or better, calculate width first.
        let total_width = measure_width(&self.quipu.primary_cord);

        if primary_y >= area.top() && primary_y < area.bottom() {
             for x in start_x..(start_x + total_width).min(area.right()) {
                 if x >= area.left() {
                     buf.cell_mut((x, primary_y)).unwrap().set_symbol("━").set_style(Style::default().fg(Color::White));
                 }
             }
        }

        // Draw Children
        let mut current_x = start_x;
        for child in &self.quipu.primary_cord.children {
            let child_width = measure_width(child);
            let cord_x = current_x + (child_width / 2); // Center the cord in its allocated space

            draw_cord(child, cord_x, primary_y, buf, area);

            current_x += child_width;
        }
    }
}

fn measure_width(cord: &Cord) -> u16 {
    if cord.children.is_empty() {
        return 4; // Min spacing
    }
    let mut w = 0;
    for child in &cord.children {
        w += measure_width(child);
    }
    // Add spacing between children?
    w + 2
}

fn draw_cord(cord: &Cord, x: u16, start_y: u16, buf: &mut Buffer, area: Rect) {
    // Draw the vertical line
    // How long? Depends on knots.
    // Knots are at positions (powers of 10).
    // Let's map powers to Y positions.
    // Power 0 (Units) is at bottom.
    // Power 4 is at top.
    // Let's say max power is 5.
    // Y = start_y + 1 + (MaxPower - Power) * 2

    // Find max power in this cord to determine length?
    // Or just fixed length?
    // Let's do fixed length for now, or dynamic.
    let length = 30;

    // Draw vertical string
    for i in 1..length {
        let y = start_y + i;
        if x >= area.left() && x < area.right() && y >= area.top() && y < area.bottom() {
             buf.cell_mut((x, y)).unwrap().set_symbol("│").set_style(Style::default().fg(match cord.color {
                 crate::quipu::Color::Red => Color::Red,
                 crate::quipu::Color::Blue => Color::Blue,
                 crate::quipu::Color::Green => Color::Green,
                 _ => Color::DarkGray,
             }));
        }
    }

    // Draw Label if exists
    if let Some(label) = &cord.label {
         let label_y = start_y + 1;
         if x + 1 < area.right() && label_y < area.bottom() {
             buf.set_string(x + 1, label_y, label, Style::default().add_modifier(Modifier::DIM));
         }
    }

    // Draw Knots
    // Map power to Y.
    let max_power = 5;

    // Sort knots by power descending so high powers are at top
    let mut sorted_knots = cord.knots.clone();
    sorted_knots.sort_by(|a, b| b.0.cmp(&a.0));

    let mut last_power = 999;
    let mut offset = 0;

    for (power, knot) in sorted_knots {
        // Clamp power
        let p = if power > max_power { max_power } else { power };

        // Group same powers together
        if p == last_power {
            offset += 1;
        } else {
            offset = 0;
            last_power = p;
        }

        // Calculate Y: Top is Power 5.
        // We multiply by 4 to allow space for clusters (multiple single knots)
        let y = start_y + 3 + (max_power - p) as u16 * 4 + offset;

        if x >= area.left() && x < area.right() && y >= area.top() && y < area.bottom() {
            let (symbol, color) = match knot {
                Knot::Single => ("●", Color::White), // Single knot
                Knot::FigureEight => ("∞", Color::Cyan), // 1
                Knot::Long(t) => ( match t {
                    2 => "══", 3 => "≡", 4 => "≣", _ => "§"
                }, Color::Yellow),
            };

            buf.cell_mut((x, y)).unwrap().set_symbol(symbol).set_style(Style::default().fg(color));
        }
    }

    // Draw Subsidiaries
    // They branch off.
    // Let's just draw them recursively shifted right?
    // But we already allocated width for them in `measure_width`?
    // Wait, `measure_width` sums children widths.
    // If this cord has children (subsidiaries), we render them.
    // But where?
    // If `draw_cord` is called for a child of Primary, `x` is centered.
    // Its children should be drawn relative to it?
    // This tree layout logic is tricky.

    // Simplification: `measure_width` handles the spacing.
    // If a cord has children, we draw the cord line, then draw children to the right?
    // Or underneath?

    // Let's assume the "Simple Layout":
    // Primary has children.
    // Those children are pendants.
    // If those pendants have children, we ignore them for now or draw them simply.

    if !cord.children.is_empty() {
        // Draw subsidiaries branching off
        let mut sub_y = start_y + 5;
        for child in &cord.children {
            // Branch line
            if x+1 < area.right() && sub_y < area.bottom() {
                buf.set_string(x, sub_y, "├─", Style::default());
            }
            draw_cord(child, x + 2, sub_y, buf, area);
            sub_y += 10; // Vertical spacing for subsidiaries
        }
    }
}

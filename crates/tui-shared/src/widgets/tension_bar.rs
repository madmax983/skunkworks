use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols::block,
    widgets::{Block, Borders, Widget},
};

pub struct TensionBar {
    tension: f64,
}

impl TensionBar {
    pub fn new(tension: f64) -> Self {
        Self { tension }
    }
}

impl Widget for TensionBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let tension = self.tension.clamp(0.0, 1.0);

        // Gradient Calculation: Green -> Yellow -> Red
        let (r, g, b) = if tension < 0.5 {
            // Green (0, 255, 0) to Yellow (255, 255, 0)
            let t = tension * 2.0;
            ((255.0 * t) as u8, 255, 0)
        } else {
            // Yellow (255, 255, 0) to Red (255, 0, 0)
            let t = (tension - 0.5) * 2.0;
            (255, (255.0 * (1.0 - t)) as u8, 0)
        };
        let color = Color::Rgb(r, g, b);

        let container = Block::default().borders(Borders::ALL).title("TENS");
        let inner_area = container.inner(area);
        container.render(area, buf);

        if inner_area.height < 1 {
            return;
        }

        let precise_height = inner_area.height as f64 * tension;
        let full_blocks = precise_height.floor() as u16;
        let remainder = precise_height - full_blocks as f64;

        // Draw full blocks
        for y in 0..full_blocks {
            let draw_y = inner_area.y + inner_area.height - 1 - y;
            if draw_y >= inner_area.y + inner_area.height {
                continue;
            }

            for x in inner_area.x..inner_area.x + inner_area.width {
                let cell = &mut buf[(x, draw_y)];
                cell.set_symbol(block::FULL);
                cell.set_fg(color);
            }
        }

        // Draw partial block
        if remainder > 0.0 && full_blocks < inner_area.height {
            let draw_y = inner_area.y + inner_area.height - 1 - full_blocks;

            // Lower blocks grow from bottom
            let symbol = if remainder < 0.125 {
                block::ONE_EIGHTH
            } else if remainder < 0.25 {
                block::ONE_QUARTER
            } else if remainder < 0.375 {
                block::THREE_EIGHTHS
            } else if remainder < 0.5 {
                block::HALF
            } else if remainder < 0.625 {
                block::FIVE_EIGHTHS
            } else if remainder < 0.75 {
                block::THREE_QUARTERS
            } else if remainder < 0.875 {
                block::SEVEN_EIGHTHS
            } else {
                block::FULL
            };

            for x in inner_area.x..inner_area.x + inner_area.width {
                let cell = &mut buf[(x, draw_y)];
                cell.set_symbol(symbol);
                cell.set_fg(color);
            }
        }
    }
}

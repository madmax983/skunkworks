use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
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

        let color = if tension > 0.8 {
            Color::Red
        } else if tension > 0.5 {
            Color::Yellow
        } else {
            Color::Green
        };

        let block = Block::default().borders(Borders::ALL).title("TENS"); // Short title for narrow bar

        let inner_area = block.inner(area);
        block.render(area, buf);

        if inner_area.height < 1 {
            return;
        }

        let fill_height = (inner_area.height as f64 * tension).round() as u16;

        for y in 0..fill_height {
            // Draw from bottom up
            let draw_y = inner_area.y + inner_area.height - 1 - y;
            if draw_y >= inner_area.y + inner_area.height {
                continue;
            } // Safety

            for x in inner_area.x..inner_area.x + inner_area.width {
                buf[(x, draw_y)].set_style(Style::default().bg(color));
            }
        }
    }
}

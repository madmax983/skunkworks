use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    symbols::block,
    widgets::{Block, Borders, Widget},
};

/// A vertical progress bar that visualizes "tension" or "stress" levels.
///
/// The `TensionBar` renders a gradient-colored bar that fills from bottom to top.
/// It uses fractional block characters for high-resolution rendering, ensuring
/// smooth transitions between integer values.
///
/// # Visual Style
///
/// - **Gradient**: The color shifts dynamically based on the tension value:
///   - **0.0 - 0.5**: Green → Yellow
///   - **0.5 - 1.0**: Yellow → Red
/// - **Precision**: Uses `ratatui`'s partial block symbols (e.g., `▂`, `▃`, `▄`) to represent
///   fractional values, allowing for smoother animations than standard full-block bars.
///
/// # Examples
///
/// ```
/// use tui_shared::TensionBar;
/// use ratatui::{layout::Rect, buffer::Buffer, widgets::{Widget, Block, Borders}};
///
/// // Create a bar with 75% tension (Red-ish)
/// let widget = TensionBar::new(0.75)
///     .block(Block::default().title("Stress").borders(Borders::ALL));
///
/// // In a real app, you'd render this to a frame
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
/// widget.render(Rect::new(0, 0, 10, 10), &mut buffer);
/// ```
pub struct TensionBar<'a> {
    tension: f64,
    block: Option<Block<'a>>,
}

impl<'a> TensionBar<'a> {
    /// Creates a new `TensionBar` with the specified tension level.
    ///
    /// # Arguments
    ///
    /// * `tension` - A value between 0.0 (empty) and 1.0 (full). Values outside this range
    ///   are clamped.
    pub fn new(tension: f64) -> Self {
        Self { tension, block: None }
    }

    /// Sets a custom block for the widget.
    ///
    /// Defaults to a bordered block with title "TENSION".
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for TensionBar<'a> {
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

        let block = self.block.unwrap_or_else(|| {
            Block::default().borders(Borders::ALL).title("TENSION")
        });

        let inner_area = block.inner(area);
        block.render(area, buf);

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
            // Uses <= to ensure exact fractions (e.g., 0.5) map to the corresponding block (HALF)
            let symbol = if remainder <= 0.125 {
                block::ONE_EIGHTH
            } else if remainder <= 0.25 {
                block::ONE_QUARTER
            } else if remainder <= 0.375 {
                block::THREE_EIGHTHS
            } else if remainder <= 0.5 {
                block::HALF
            } else if remainder <= 0.625 {
                block::FIVE_EIGHTHS
            } else if remainder <= 0.75 {
                block::THREE_QUARTERS
            } else if remainder <= 0.875 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, buffer::Buffer, layout::Rect, Terminal};

    fn render_tension(tension: f64, width: u16, height: u16) -> Buffer {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let area = Rect::new(0, 0, width, height);
                let widget = TensionBar::new(tension);
                f.render_widget(widget, area);
            })
            .unwrap();

        terminal.backend().buffer().clone()
    }

    #[test]
    fn test_render_zero_tension() {
        let buffer = render_tension(0.0, 10, 10);
        let cell = &buffer[(1, 8)];
        assert_eq!(cell.symbol(), " "); // Empty
    }

    #[test]
    fn test_render_full_tension() {
        let buffer = render_tension(1.0, 10, 10);
        for y in 1..9 {
            let cell = &buffer[(1, y)];
            assert_eq!(cell.symbol(), block::FULL);
        }
    }

    #[test]
    fn test_render_half_tension() {
        let buffer = render_tension(0.5, 10, 10);
        for y in 5..9 {
            let cell = &buffer[(1, y)];
            assert_eq!(cell.symbol(), block::FULL, "Row {} should be full", y);
        }
        let cell = &buffer[(1, 4)];
        assert_eq!(cell.symbol(), " ", "Row 4 should be empty");
    }

    #[test]
    fn test_render_partial_blocks() {
        let buffer = render_tension(0.0625, 10, 10);
        let cell = &buffer[(1, 8)]; // Bottom row
        assert_eq!(
            cell.symbol(),
            block::HALF,
            "Expected HALF block for 0.5 remainder (Wait, logic?)"
        );
        // My test logic in thought was wrong?
        // 0.0625 * 8 = 0.5. Remainder 0.5.
        // <= 0.5 -> HALF. Correct.
    }
}

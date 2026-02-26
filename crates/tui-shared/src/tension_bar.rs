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
/// use ratatui::{layout::Rect, buffer::Buffer, widgets::Widget};
///
/// // Create a bar with 75% tension (Red-ish)
/// let widget = TensionBar::new(0.75);
///
/// // In a real app, you'd render this to a frame
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
/// widget.render(Rect::new(0, 0, 10, 10), &mut buffer);
/// ```
pub struct TensionBar {
    tension: f64,
}

impl TensionBar {
    /// Creates a new `TensionBar` with the specified tension level.
    ///
    /// # Arguments
    ///
    /// * `tension` - A value between 0.0 (empty) and 1.0 (full). Values outside this range
    ///   are clamped.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::TensionBar;
    /// let bar = TensionBar::new(0.5); // Half-full, Yellow
    /// ```
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
        // Inner area height is 10 - 2 (borders) = 8.
        // Tension 0.0 -> height 0.0. No blocks drawn.
        // Check cell at (1, 8) (bottom-left inside border)
        // (0,0) is top-left.
        // Borders are at x=0, x=9, y=0, y=9.
        // Inner: x=1..9, y=1..9.
        // Bottom row inside is y=8.
        let cell = &buffer[(1, 8)];
        assert_eq!(cell.symbol(), " "); // Empty
    }

    #[test]
    fn test_render_full_tension() {
        let buffer = render_tension(1.0, 10, 10);
        // Inner height 8. Full tension -> 8 blocks.
        // Should fill from y=8 up to y=1.
        for y in 1..9 {
            let cell = &buffer[(1, y)];
            assert_eq!(cell.symbol(), block::FULL);
        }
    }

    #[test]
    fn test_render_half_tension() {
        let buffer = render_tension(0.5, 10, 10);
        // Inner height 8. 0.5 * 8 = 4 blocks.
        // Filled from bottom (y=8) up 4 rows: y=8, 7, 6, 5.
        // y=4 should be empty.

        // Check filled
        for y in 5..9 {
            let cell = &buffer[(1, y)];
            assert_eq!(cell.symbol(), block::FULL, "Row {} should be full", y);
        }

        // Check empty
        let cell = &buffer[(1, 4)];
        assert_eq!(cell.symbol(), " ", "Row 4 should be empty");
    }

    #[test]
    fn test_render_partial_blocks() {
        // Case 1: Exactly 0.5 blocks (HALF)
        // Tension 1/16 = 0.0625. Height 8. Fill = 0.5.
        // Remainder = 0.5.
        // Expect block::HALF.
        let buffer = render_tension(0.0625, 10, 10);
        let cell = &buffer[(1, 8)]; // Bottom row
        assert_eq!(
            cell.symbol(),
            block::HALF,
            "Expected HALF block for 0.5 remainder"
        );

        // Case 2: 4.5 blocks (HALF)
        // Tension 9/16 = 0.5625. Height 8. Fill = 4.5.
        // Remainder = 0.5.
        let buffer = render_tension(0.5625, 10, 10);
        // y=8,7,6,5 are FULL.
        // y=4 is HALF.
        assert_eq!(buffer[(1, 5)].symbol(), block::FULL);
        assert_eq!(
            buffer[(1, 4)].symbol(),
            block::HALF,
            "Expected HALF block at top for 0.5 remainder"
        );

        // Case 3: Exactly 0.125 blocks (ONE_EIGHTH)
        // Tension 1/64 = 0.015625. Height 8. Fill = 0.125.
        // Remainder = 0.125.
        let buffer = render_tension(0.015625, 10, 10);
        let cell = &buffer[(1, 8)];
        assert_eq!(
            cell.symbol(),
            block::ONE_EIGHTH,
            "Expected ONE_EIGHTH block for 0.125 remainder"
        );
    }

    #[test]
    fn test_small_area() {
        // Height 2. Borders take 2. Inner height 0.
        // Should return early and not panic.
        let _ = render_tension(0.5, 5, 2);

        // Height 1.
        let _ = render_tension(0.5, 5, 1);

        // Height 3. Inner height 1.
        let buffer = render_tension(0.5, 5, 3);
        // 0.5 * 1 = 0.5 block.
        // 0.5 remainder -> HALF (due to <= 0.5).
        // Inner y range: 1..2 (height 1).
        // y=1.
        assert_eq!(buffer[(1, 1)].symbol(), block::HALF);
    }

    #[test]
    fn test_gradient_colors() {
        // Green (< 0.5)
        let _ = render_tension(0.0, 10, 10);

        // Let's use tension 0.1 (Greenish).
        let buffer = render_tension(0.1, 10, 10);
        // 0.1 * 8 = 0.8 blocks. ONE_ROW.
        let cell = &buffer[(1, 8)];
        let fg = cell.fg;
        // 0.1 < 0.5.
        // t = 0.1 * 2.0 = 0.2.
        // r = 255 * 0.2 = 51. g = 255. b = 0.
        assert_eq!(fg, Color::Rgb(51, 255, 0));

        // Red (> 0.5)
        // Tension 1.0.
        let buffer = render_tension(1.0, 10, 10);
        let cell = &buffer[(1, 8)];
        // t = (1.0 - 0.5) * 2.0 = 1.0.
        // r = 255. g = 255 * (1.0 - 1.0) = 0. b = 0.
        assert_eq!(cell.fg, Color::Rgb(255, 0, 0));
    }
}

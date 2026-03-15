//! # Tension Bar 🌡️
//!
//! Provides the [`TensionBar`] widget to visualize continuous stress or progress metrics.
//!
//! The `TensionBar` differs from standard terminal progress bars by rendering vertically
//! using fractional block characters (`▂`, `▃`, `▄`) to create high-resolution, smooth
//! animations even within a low-resolution terminal grid. It automatically applies a
//! color gradient (Cyan → Yellow → Red) as the tension value increases.

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
///   - **0.0 - 0.5**: Cyan → Yellow
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
    ///   will be clamped during rendering.
    pub fn new(tension: f64) -> Self {
        Self {
            tension,
            block: None,
        }
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

        // Gradient Calculation: Cyan -> Yellow -> Red
        let (r, g, b) = if tension < 0.5 {
            // Cyan (0, 255, 255) to Yellow (255, 255, 0)
            let t = tension * 2.0;
            ((255.0 * t) as u8, 255, (255.0 * (1.0 - t)) as u8)
        } else {
            // Yellow (255, 255, 0) to Red (255, 0, 0)
            let t = (tension - 0.5) * 2.0;
            (255, (255.0 * (1.0 - t)) as u8, 0)
        };
        let color = Color::Rgb(r, g, b);

        let block = self
            .block
            .unwrap_or_else(|| Block::default().borders(Borders::ALL).title("TENSION"));

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
    fn test_tension_bar_custom_block() {
        let block = Block::default()
            .title("Custom Tension")
            .borders(Borders::BOTTOM);
        let widget = TensionBar::new(0.5).block(block);

        let width = 10;
        let height = 10;
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let area = Rect::new(0, 0, width, height);
                f.render_widget(widget, area);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        // Custom title rendered at top left since borders=BOTTOM means no top border, rendering title at (0, 0)
        assert_eq!(buffer[(0, 0)].symbol(), "C");
        assert_eq!(buffer[(1, 0)].symbol(), "u");
        assert_eq!(buffer[(2, 0)].symbol(), "s");
    }

    #[test]
    fn test_tension_bar_zero_height() {
        // Test early return when inner_area.height < 1
        let widget = TensionBar::new(1.0); // full tension

        let backend = TestBackend::new(10, 2); // only 2 height, borders take 2, inner height 0
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let area = Rect::new(0, 0, 10, 2);
                f.render_widget(widget, area);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        // Since inner height is 0, no blocks should be drawn inside (y=0 and y=1 are borders)
        assert_ne!(buffer[(1, 1)].symbol(), block::FULL);
    }

    #[test]
    fn test_render_partial_blocks() {
        // We have height = 8 blocks for inner height.
        // We test multiple fraction values to trigger each branch.
        // Note: inner_height = 8.
        // tension * 8 = precise_height
        // precise_height = full_blocks + remainder

        let cases = vec![
            (0.0625, block::HALF),        // precise_height = 0.5 -> full=0, rem=0.5 -> HALF
            (0.01, block::ONE_EIGHTH),    // 0.08 rem -> ONE_EIGHTH
            (0.025, block::ONE_QUARTER),  // 0.2 rem -> ONE_QUARTER
            (0.04, block::THREE_EIGHTHS), // 0.32 rem -> THREE_EIGHTHS
            (0.075, block::FIVE_EIGHTHS), // 0.6 rem -> FIVE_EIGHTHS
            (0.09, block::THREE_QUARTERS), // 0.72 rem -> THREE_QUARTERS
            (0.10, block::SEVEN_EIGHTHS), // 0.8 rem -> SEVEN_EIGHTHS
            (0.12, block::FULL),          // 0.96 rem -> FULL
        ];

        for (tension, expected_symbol) in cases {
            let buffer = render_tension(tension, 10, 10);
            let cell = &buffer[(1, 8)]; // Bottom row
            assert_eq!(
                cell.symbol(),
                expected_symbol,
                "Expected {} block for tension {}",
                expected_symbol,
                tension
            );
        }
    }

    #[test]
    fn test_tension_bar_out_of_bounds_draw() {
        // Trigger condition: draw_y >= inner_area.y + inner_area.height
        // This is theoretically guarded by full_blocks < inner_area.height and math,
        // but we can try to force it by having tension > 1.0 (though it's clamped to 1.0)
        // Let's ensure the clamp works properly.
        let buffer = render_tension(2.0, 10, 10);
        // If tension is exactly 1.0, full_blocks = 8, inner_height = 8
        // draw_y = inner_y + inner_height - 1 - y
        // Highest y is 7.
        // inner_y = 1, inner_height = 8.
        // inner_y + inner_height = 9.
        // Highest draw_y = 1 + 8 - 1 - 7 = 1.
        // It should never draw outside the block borders (y=0 or y=9).

        // Ensure top border is preserved (not overwritten by tension blocks)
        assert_ne!(buffer[(1, 0)].symbol(), block::FULL);
        // Ensure bottom border is preserved
        assert_ne!(buffer[(1, 9)].symbol(), block::FULL);
    }
}

#[cfg(test)]
mod havoc_tests {
    use super::*;
    use proptest::prelude::*;
    use ratatui::{layout::Rect, buffer::Buffer, widgets::{Widget, Block}};

    proptest! {
        #[test]
        #[should_panic]
        fn fuzz_tension_bar_underflow(
            y in 65530u16..=65535,
            height in 1u16..=10,
            tension in 0.0f64..=1.0f64
        ) {
            // Bypass Rect::new which clips height and prevents the overflow from being reachable.
            let area = Rect { x: 0, y, width: 10, height };
            let mut buffer = Buffer::empty(area);
            let widget = TensionBar::new(tension).block(Block::default());
            widget.render(area, &mut buffer);
        }
    }
}

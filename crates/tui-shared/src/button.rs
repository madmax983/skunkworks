//! # TUI Button 🔘
//!
//! Provides the [`Button`] widget for interactive terminal applications.
//!
//! The `Button` component is a simplified, concrete widget for displaying a button-like
//! rectangle on screen. It has been stripped of complex state and generic traits
//! to keep it simple, direct, and explicit.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Widget},
};

/// A reusable Button component for TUI applications.
///
/// The button accepts a label, optional icon, custom block borders,
/// and an explicit `ratatui::style::Style` to dictate its appearance.
///
/// # Example
///
/// ```
/// use tui_shared::Button;
/// use ratatui::widgets::Widget;
/// use ratatui::layout::Rect;
/// use ratatui::buffer::Buffer;
/// use ratatui::style::{Color, Style};
///
/// let button = Button::new("Submit")
///     .style(Style::default().bg(Color::Blue).fg(Color::Black))
///     .icon("🚀");
///
/// // Render
/// let area = Rect::new(0, 0, 10, 3);
/// let mut buffer = Buffer::empty(area);
/// button.render(area, &mut buffer);
/// ```
pub struct Button<'a> {
    label: String,
    style: Style,
    icon: Option<String>,
    block: Option<Block<'a>>,
}

impl<'a> Button<'a> {
    /// Creates a new `Button` with the given label.
    ///
    /// By default, the button has a blue background and black text.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::Button;
    ///
    /// let button = Button::new("Click Me");
    /// ```
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            style: Style::default().bg(Color::Blue).fg(Color::Black).add_modifier(Modifier::BOLD),
            icon: None,
            block: None,
        }
    }

    /// Sets the visual style of the button explicitly.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::Button;
    /// use ratatui::style::{Color, Style};
    ///
    /// let button = Button::new("Delete").style(Style::default().bg(Color::Red).fg(Color::White));
    /// ```
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Adds an icon to the left of the label.
    ///
    /// The icon is prepended to the label text with a space separator.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::Button;
    ///
    /// let button = Button::new("Save").icon("💾");
    /// ```
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Sets a custom block for the button.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::Button;
    /// use ratatui::widgets::{Block, Borders};
    ///
    /// let block = Block::default().borders(Borders::BOTTOM);
    /// let button = Button::new("Custom").block(block);
    /// ```
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Toggles the button's "active" appearance quickly.
    ///
    /// - `true`: Sets style to Black text on a Yellow background (High visibility).
    /// - `false`: Sets style to Gray text on a default background (Low visibility).
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::Button;
    ///
    /// let is_on = true;
    /// let button = Button::new("Power").active(is_on);
    /// ```
    pub fn active(mut self, is_active: bool) -> Self {
        if is_active {
            self.style = Style::default().bg(Color::Yellow).fg(Color::Black).add_modifier(Modifier::BOLD);
        } else {
            self.style = Style::default().fg(Color::Gray).bg(Color::Reset);
        }
        self
    }
}

impl<'a> Widget for Button<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        if self.block.is_none() {
            self.block = Some(Block::default().borders(Borders::ALL));
        }

        let block = self
            .block
            .take()
            .unwrap_or_else(|| Block::default().borders(Borders::ALL))
            .style(self.style);

        // Ensure rendering block doesn't go out of bounds of the current buffer
        let safe_area = area.intersection(buf.area);
        if safe_area.width == 0 || safe_area.height == 0 {
            return;
        }

        let inner_area = block.inner(safe_area);
        block.render(safe_area, buf);

        if inner_area.height == 0 || inner_area.width == 0 {
            return;
        }

        let text_area = Rect {
            x: inner_area.x,
            y: inner_area.y + (inner_area.height.saturating_sub(1)) / 2,
            width: inner_area.width,
            height: 1,
        };

        let content = if let Some(icon) = self.icon {
            format!("{} {}", icon, self.label)
        } else {
            self.label
        };

        let line = Line::from(content);
        let x_offset = (text_area.width.saturating_sub(line.width() as u16)) / 2;

        if text_area.y < buf.area.bottom() {
            buf.set_line(text_area.x + x_offset, text_area.y, &line, text_area.width);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::style::Color;

    #[test]
    fn test_button_rendering() {
        let button = Button::new("Click Me");

        let area = Rect::new(0, 0, 20, 3);
        let mut buffer = Buffer::empty(area);

        button.render(area, &mut buffer);

        // Check border style (Default is Blue bg, Black fg)
        let cell = &buffer[(0, 0)];
        assert_eq!(cell.fg, Color::Black);
        assert_eq!(cell.bg, Color::Blue);

        // Check text content
        // Text is centered. Width 20, text "Click Me" (8 chars).
        // Inner width 18. Padding (18-8)/2 = 5.
        // x = 1 + 5 = 6.
        let cell = &buffer[(6, 1)];
        assert_eq!(cell.symbol(), "C");
    }

    #[test]
    fn test_button_custom_style() {
        let button = Button::new("Del")
            .style(Style::default().bg(Color::LightRed).fg(Color::White))
            .icon("X");

        let area = Rect::new(0, 0, 20, 3);
        let mut buffer = Buffer::empty(area);

        button.render(area, &mut buffer);

        let cell = &buffer[(0, 0)];
        assert_eq!(cell.fg, Color::White);
        assert_eq!(cell.bg, Color::LightRed);

        // Check icon
        // "X Del" -> 1 + 1 + 3 = 5 chars.
        // Inner width 18. Padding (18-5)/2 = 6 (trunc).
        // x = 1 + 6 = 7.
        let cell_icon = &buffer[(7, 1)];
        assert_eq!(cell_icon.symbol(), "X");

        let cell_text = &buffer[(9, 1)];
        assert_eq!(cell_text.symbol(), "D");
    }

    #[test]
    fn test_active_mapping() {
        let active_btn = Button::new("On").active(true);

        let area = Rect::new(0, 0, 10, 3);
        let mut buffer = Buffer::empty(area);
        active_btn.render(area, &mut buffer);

        let cell = &buffer[(0, 0)];
        assert_eq!(cell.fg, Color::Black);
        assert_eq!(cell.bg, Color::Yellow);

        let inactive_btn = Button::new("Off").active(false);
        let mut buffer = Buffer::empty(area);
        inactive_btn.render(area, &mut buffer);
        let cell = &buffer[(0, 0)];
        assert_eq!(cell.fg, Color::Gray);
        assert_eq!(cell.bg, Color::Reset);
    }

    #[test]
    fn test_button_custom_block() {
        use ratatui::widgets::Borders;

        let block = Block::default().title("Custom").borders(Borders::BOTTOM);
        let button = Button::new("Test").block(block);

        let area = Rect::new(0, 0, 10, 3);
        let mut buffer = Buffer::empty(area);
        button.render(area, &mut buffer);

        // We supplied a custom block with bottom border only and title "Custom".
        // A block with borders=BOTTOM leaves the top open, but title is rendered at top left (0, 0).
        let cell = &buffer[(0, 0)];
        assert_eq!(cell.symbol(), "C");
        let cell = &buffer[(1, 0)];
        assert_eq!(cell.symbol(), "u");
    }
}

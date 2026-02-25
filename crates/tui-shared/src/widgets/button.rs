use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

/// A simple, interactive button widget for TUI applications.
///
/// The `Button` widget renders a label inside a bordered block. It supports
/// visual feedback for "hover" and "active" (clicked) states, making it easier
/// to build interactive menus or forms.
///
/// # States
///
/// - **Default**: Gray borders, White text.
/// - **Hovered**: Dark Gray background, Bold White text (indicates selection).
/// - **Active**: Yellow background, Black text (indicates activation/click).
///
/// # Examples
///
/// ```
/// use tui_shared::widgets::Button;
/// use ratatui::{layout::Rect, buffer::Buffer, widgets::Widget};
///
/// // Create a button that is currently being hovered
/// let btn = Button::new("Click Me")
///     .hovered(true);
///
/// // Render it
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 12, 3));
/// btn.render(Rect::new(0, 0, 12, 3), &mut buffer);
/// ```
pub struct Button<'a> {
    label: &'a str,
    is_active: bool,
    is_hovered: bool,
}

impl<'a> Button<'a> {
    /// Creates a new button with the given label text.
    ///
    /// The button starts in the default state (not hovered, not active).
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            is_active: false,
            is_hovered: false,
        }
    }

    /// Sets the "active" state of the button.
    ///
    /// Use this to visualize a button being pressed or triggered.
    /// - **True**: Renders with a high-contrast Yellow background.
    /// - **False**: Renders normally (or hovered).
    pub fn active(mut self, active: bool) -> Self {
        self.is_active = active;
        self
    }

    /// Sets the "hovered" state of the button.
    ///
    /// Use this to visualize the button currently under the cursor or selected.
    /// - **True**: Renders with a Dark Gray background and bold text.
    /// - **False**: Renders normally.
    ///
    /// *Note: If both `hovered` and `active` are true, `active` takes precedence.*
    pub fn hovered(mut self, hovered: bool) -> Self {
        self.is_hovered = hovered;
        self
    }
}

impl<'a> Widget for Button<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut style = Style::default().fg(Color::White);
        let mut block_style = Style::default().fg(Color::Gray);

        if self.is_hovered {
            // Hover: Distinct Dark Gray background
            style = style
                .bg(Color::DarkGray)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD);
            block_style = block_style.fg(Color::White).add_modifier(Modifier::BOLD);
        }

        if self.is_active {
            // Active (Click): High contrast Yellow
            style = style
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD);
            block_style = block_style.fg(Color::Yellow);
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(block_style);

        let paragraph = Paragraph::new(self.label)
            .block(block)
            .style(style)
            .alignment(Alignment::Center);

        paragraph.render(area, buf);
    }
}

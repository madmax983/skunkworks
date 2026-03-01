use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Widget},
};

/// Represents the interaction state of a [`Button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonState {
    /// The default state.
    #[default]
    Normal,
    /// The button is being hovered over (e.g., by a mouse or selection).
    Hovered,
    /// The button is currently being pressed.
    Clicked,
    /// The button is disabled and cannot be interacted with.
    Disabled,
}

/// Defines the visual style variant of a [`Button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonStyle {
    /// The primary action button (Blue background).
    #[default]
    Primary,
    /// A secondary action button (Gray/White).
    Secondary,
    /// A ghost button with only an outline.
    Outline,
    /// A destructive action button (Red).
    Danger,
    /// A warning or high-attention button (Yellow).
    Warning,
    /// A success or completion button (Green).
    Success,
}

/// A reusable Button component for TUI applications.
///
/// The button supports different [styles](ButtonStyle) and [states](ButtonState),
/// automatically handling the visual changes for hover and click effects.
///
/// # Example
///
/// ```
/// use tui_shared::{Button, ButtonStyle, ButtonState};
/// use ratatui::widgets::Widget;
/// use ratatui::layout::Rect;
/// use ratatui::buffer::Buffer;
///
/// let button = Button::new("Submit")
///     .style_variant(ButtonStyle::Primary)
///     .state(ButtonState::Hovered)
///     .icon("🚀");
///
/// // Render
/// let area = Rect::new(0, 0, 10, 3);
/// let mut buffer = Buffer::empty(area);
/// button.render(area, &mut buffer);
/// ```
pub struct Button<'a> {
    label: String,
    state: ButtonState,
    style_variant: ButtonStyle,
    icon: Option<String>,
    block: Option<Block<'a>>,
}

impl<'a> Button<'a> {
    /// Creates a new `Button` with the given label.
    ///
    /// The button defaults to [`ButtonStyle::Primary`] and [`ButtonState::Normal`].
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state: ButtonState::Normal,
            style_variant: ButtonStyle::Primary,
            icon: None,
            block: None,
        }
    }

    /// Explicitly sets the button's state.
    pub fn state(mut self, state: ButtonState) -> Self {
        self.state = state;
        self
    }

    /// Sets the visual style variant.
    pub fn style_variant(mut self, variant: ButtonStyle) -> Self {
        self.style_variant = variant;
        self
    }

    /// Adds an icon to the left of the label.
    ///
    /// The icon is prepended to the label text with a space separator.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Sets a custom block for the button.
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Toggles the button's "active" appearance.
    ///
    /// This is a convenience method for legacy compatibility or simple toggle states.
    ///
    /// - `true`: Sets style to [`ButtonStyle::Warning`] (High visibility/Yellow) to represent an active state.
    /// - `false`: Sets style to [`ButtonStyle::Outline`] (Low visibility/Gray) to represent an inactive state.
    pub fn active(mut self, is_active: bool) -> Self {
        if is_active {
            self.style_variant = ButtonStyle::Warning;
        } else {
            self.style_variant = ButtonStyle::Outline;
        }
        self
    }
}

impl<'a> Widget for Button<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let (fg, bg, modifier) = match (self.style_variant, self.state) {
            // Disabled
            (_, ButtonState::Disabled) => (Color::DarkGray, Color::Black, Modifier::empty()),

            // Primary (Blue)
            (ButtonStyle::Primary, ButtonState::Normal) => {
                (Color::Black, Color::Blue, Modifier::BOLD)
            }
            (ButtonStyle::Primary, ButtonState::Hovered) => {
                (Color::Black, Color::Cyan, Modifier::BOLD) // Brighter blue/cyan
            }
            (ButtonStyle::Primary, ButtonState::Clicked) => {
                (Color::Blue, Color::White, Modifier::BOLD)
            }

            // Secondary (Gray)
            (ButtonStyle::Secondary, ButtonState::Normal) => {
                (Color::White, Color::DarkGray, Modifier::empty())
            }
            (ButtonStyle::Secondary, ButtonState::Hovered) => {
                (Color::Black, Color::Gray, Modifier::empty()) // Brighter
            }
            (ButtonStyle::Secondary, ButtonState::Clicked) => {
                (Color::Black, Color::White, Modifier::BOLD)
            }

            // Outline (Ghost)
            (ButtonStyle::Outline, ButtonState::Normal) => {
                (Color::Gray, Color::Reset, Modifier::empty())
            }
            (ButtonStyle::Outline, ButtonState::Hovered) => (
                Color::White,
                Color::Reset,
                Modifier::BOLD | Modifier::UNDERLINED,
            ),
            (ButtonStyle::Outline, ButtonState::Clicked) => {
                (Color::Black, Color::White, Modifier::BOLD)
            }

            // Danger (Red)
            (ButtonStyle::Danger, ButtonState::Normal) => {
                (Color::White, Color::Red, Modifier::BOLD)
            }
            (ButtonStyle::Danger, ButtonState::Hovered) => {
                (Color::White, Color::LightRed, Modifier::BOLD) // Lighter red
            }
            (ButtonStyle::Danger, ButtonState::Clicked) => {
                (Color::Red, Color::White, Modifier::BOLD)
            }

            // Warning (Yellow)
            (ButtonStyle::Warning, ButtonState::Normal) => {
                (Color::Black, Color::Yellow, Modifier::BOLD)
            }
            (ButtonStyle::Warning, ButtonState::Hovered) => {
                (Color::Black, Color::LightYellow, Modifier::BOLD) // Lighter yellow
            }
            (ButtonStyle::Warning, ButtonState::Clicked) => {
                (Color::Yellow, Color::Black, Modifier::BOLD)
            }

            // Success (Green)
            (ButtonStyle::Success, ButtonState::Normal) => {
                (Color::Black, Color::Green, Modifier::BOLD)
            }
            (ButtonStyle::Success, ButtonState::Hovered) => {
                (Color::Black, Color::LightGreen, Modifier::BOLD)
            }
            (ButtonStyle::Success, ButtonState::Clicked) => {
                (Color::Green, Color::White, Modifier::BOLD)
            }
        };

        let style = Style::default().fg(fg).bg(bg).add_modifier(modifier);

        if self.block.is_none() {
            self.block = Some(Block::default().borders(Borders::ALL));
        }

        let block = self.block.take().unwrap().style(style);
        let inner_area = block.inner(area);
        block.render(area, buf);

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

        if text_area.width > 0 {
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
        let button = Button::new("Click Me")
            .style_variant(ButtonStyle::Primary)
            .state(ButtonState::Normal);

        let area = Rect::new(0, 0, 20, 3);
        let mut buffer = Buffer::empty(area);

        button.render(area, &mut buffer);

        // Check border style (Primary Normal -> Blue)
        let cell = &buffer[(0, 0)];
        assert_eq!(cell.fg, Color::Black); // Text color for Primary Normal is Black
        assert_eq!(cell.bg, Color::Blue); // Bg color for Primary Normal is Blue

        // Check text content
        // Text is centered. Width 20, text "Click Me" (8 chars).
        // Inner width 18. Padding (18-8)/2 = 5.
        // x = 1 + 5 = 6.
        let cell = &buffer[(6, 1)];
        assert_eq!(cell.symbol(), "C");
    }

    #[test]
    fn test_button_danger_hovered() {
        let button = Button::new("Del") // Short label to fit easily
            .style_variant(ButtonStyle::Danger)
            .state(ButtonState::Hovered)
            .icon("X"); // Simple ascii icon to avoid emoji width issues

        let area = Rect::new(0, 0, 20, 3);
        let mut buffer = Buffer::empty(area);

        button.render(area, &mut buffer);

        // Danger Hovered -> LightRed bg, White fg
        let cell = &buffer[(0, 0)];
        assert_eq!(cell.fg, Color::White);
        assert_eq!(cell.bg, Color::LightRed);

        // Check icon
        // "X Del" -> 1 + 1 + 3 = 5 chars.
        // Inner width 18. Padding (18-5)/2 = 6 (trunc).
        // x = 1 + 6 = 7.
        // Let's check if 'X' is at (7, 1)
        let cell_icon = &buffer[(7, 1)];
        assert_eq!(cell_icon.symbol(), "X");

        let cell_text = &buffer[(9, 1)];
        assert_eq!(cell_text.symbol(), "D");
    }

    #[test]
    fn test_active_mapping() {
        let active_btn = Button::new("On").active(true);
        // We can't access style_variant directly since it's private,
        // so we render it and check the color which maps to the style.
        // Warning Normal -> Yellow fg, Black bg

        let area = Rect::new(0, 0, 10, 3);
        let mut buffer = Buffer::empty(area);
        active_btn.render(area, &mut buffer);

        let cell = &buffer[(0, 0)];
        assert_eq!(cell.fg, Color::Black);
        assert_eq!(cell.bg, Color::Yellow);

        let inactive_btn = Button::new("Off").active(false);
        // Outline Normal -> Gray fg, Reset bg
        let mut buffer = Buffer::empty(area);
        inactive_btn.render(area, &mut buffer);
        let cell = &buffer[(0, 0)];
        assert_eq!(cell.fg, Color::Gray);
        assert_eq!(cell.bg, Color::Reset);
    }

    #[test]
    fn test_button_success_rendering() {
        let button = Button::new("Go")
            .style_variant(ButtonStyle::Success)
            .state(ButtonState::Normal);

        let area = Rect::new(0, 0, 10, 3);
        let mut buffer = Buffer::empty(area);

        button.render(area, &mut buffer);

        let cell = &buffer[(0, 0)];
        assert_eq!(cell.fg, Color::Black);
        assert_eq!(cell.bg, Color::Green);
    }
}

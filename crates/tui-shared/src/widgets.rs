use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Widget},
};

/// A widget that displays a list of log messages with automatic color coding.
///
/// Use `LogList::new` to create a new instance, passing a vector of strings.
/// Messages containing "Error" will be red, "Warning" yellow, and "Note" blue.
pub struct LogList<'a> {
    items: Vec<String>,
    block: Option<Block<'a>>,
}

impl<'a> LogList<'a> {
    /// Creates a new `LogList` with the given items.
    pub fn new(items: Vec<String>) -> Self {
        Self { items, block: None }
    }

    /// Sets the block for the widget (e.g., borders and title).
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Helper to set a block with a title and borders.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.block = Some(Block::default().borders(Borders::ALL).title(title.into()));
        self
    }
}

impl<'a> Widget for LogList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|s| {
                let s_lower = s.to_lowercase();
                let (style, prefix) = if s_lower.contains("error") {
                    (Style::default().fg(Color::Red), "❌ ")
                } else if s_lower.contains("warning") {
                    (Style::default().fg(Color::Yellow), "⚠️ ")
                } else if s_lower.contains("note") || s_lower.contains("info") {
                    (Style::default().fg(Color::Blue), "ℹ️ ")
                } else if s_lower.contains("success") {
                    (Style::default().fg(Color::Green), "✅ ")
                } else {
                    (Style::default(), "")
                };

                let content = Line::from(vec![
                    Span::styled(prefix, style),
                    Span::styled(s.as_str(), style),
                ]);
                ListItem::new(content)
            })
            .collect();

        let mut list = List::new(items);
        if let Some(block) = self.block {
            list = list.block(block);
        }
        list.render(area, buf);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonState {
    #[default]
    Normal,
    Hovered,
    Clicked,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonStyle {
    #[default]
    Primary,
    Secondary,
    Outline,
    Danger,
}

/// A reusable Button component for Arthropod UI.
///
/// Supports hover, click, and disabled states with visual feedback.
pub struct Button<'a> {
    label: String,
    state: ButtonState,
    style_variant: ButtonStyle,
    icon: Option<String>,
    block: Option<Block<'a>>,
}

impl<'a> Button<'a> {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state: ButtonState::Normal,
            style_variant: ButtonStyle::Primary,
            icon: None,
            block: None,
        }
    }

    pub fn hovered(mut self, hovered: bool) -> Self {
        if self.state != ButtonState::Disabled {
            self.state = if hovered {
                ButtonState::Hovered
            } else {
                ButtonState::Normal
            };
        }
        self
    }

    pub fn clicked(mut self, clicked: bool) -> Self {
        if self.state != ButtonState::Disabled {
            self.state = if clicked {
                ButtonState::Clicked
            } else {
                self.state
            };
        }
        self
    }

    pub fn state(mut self, state: ButtonState) -> Self {
        self.state = state;
        self
    }

    pub fn style_variant(mut self, variant: ButtonStyle) -> Self {
        self.style_variant = variant;
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for Button<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let (fg, bg, modifier) = match (self.style_variant, self.state) {
            // Disabled
            (_, ButtonState::Disabled) => (Color::DarkGray, Color::Black, Modifier::empty()),

            // Primary
            (ButtonStyle::Primary, ButtonState::Normal) => {
                (Color::Black, Color::Blue, Modifier::BOLD)
            }
            (ButtonStyle::Primary, ButtonState::Hovered) => {
                (Color::Black, Color::LightBlue, Modifier::BOLD)
            }
            (ButtonStyle::Primary, ButtonState::Clicked) => {
                (Color::White, Color::Blue, Modifier::BOLD)
            }

            // Secondary
            (ButtonStyle::Secondary, ButtonState::Normal) => {
                (Color::White, Color::DarkGray, Modifier::empty())
            }
            (ButtonStyle::Secondary, ButtonState::Hovered) => {
                (Color::White, Color::Gray, Modifier::empty())
            }
            (ButtonStyle::Secondary, ButtonState::Clicked) => {
                (Color::Black, Color::White, Modifier::BOLD)
            }

            // Outline
            (ButtonStyle::Outline, ButtonState::Normal) => {
                (Color::Gray, Color::Reset, Modifier::empty())
            }
            (ButtonStyle::Outline, ButtonState::Hovered) => {
                (Color::White, Color::Reset, Modifier::BOLD)
            }
            (ButtonStyle::Outline, ButtonState::Clicked) => {
                (Color::Green, Color::Reset, Modifier::BOLD)
            }

            // Danger
            (ButtonStyle::Danger, ButtonState::Normal) => {
                (Color::White, Color::Red, Modifier::BOLD)
            }
            (ButtonStyle::Danger, ButtonState::Hovered) => {
                (Color::White, Color::LightRed, Modifier::BOLD)
            }
            (ButtonStyle::Danger, ButtonState::Clicked) => (
                Color::Black,
                Color::Red,
                Modifier::BOLD | Modifier::REVERSED,
            ),
        };

        let style = Style::default().fg(fg).bg(bg).add_modifier(modifier);

        if self.block.is_none() {
            let borders = if self.style_variant == ButtonStyle::Outline {
                Borders::ALL
            } else {
                Borders::ALL
            };
            self.block = Some(Block::default().borders(borders));
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

        // Manual rendering of the line to ensure it fits and is centered
        buf.set_line(text_area.x + x_offset, text_area.y, &line, text_area.width);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::style::Color;

    #[test]
    fn test_log_list_rendering() {
        let items = vec![
            "Error: Something went wrong".to_string(),
            "Warning: Be careful".to_string(),
            "Success: It worked".to_string(),
            "Normal message".to_string(),
        ];
        let log_list = LogList::new(items);
        let area = Rect::new(0, 0, 40, 5);
        let mut buffer = Buffer::empty(area);

        log_list.render(area, &mut buffer);

        // Check Error line
        // "❌ " is usually handled as 2 cells wide for the emoji.
        let cell = &buffer[(0, 0)];
        assert_eq!(cell.symbol(), "❌");
        assert_eq!(cell.fg, Color::Red);

        // Check Warning line
        let cell = &buffer[(0, 1)];
        assert_eq!(cell.symbol(), "⚠️");
        assert_eq!(cell.fg, Color::Yellow);

        // Check Success line
        let cell = &buffer[(0, 2)];
        assert_eq!(cell.symbol(), "✅");
        assert_eq!(cell.fg, Color::Green);

        // Check Normal line
        let cell = &buffer[(0, 3)];
        assert_ne!(cell.symbol(), "❌");
        assert_ne!(cell.symbol(), "⚠️");
        assert_ne!(cell.symbol(), "✅");
        assert_eq!(cell.fg, Color::Reset);
    }

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
}

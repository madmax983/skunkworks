//! # Log List 📜
//!
//! Provides the [`LogList`] widget for displaying system messages and activity logs.
//!
//! Rather than requiring manual styling for every list item, `LogList` automatically parses
//! keywords within strings to apply appropriate severity colors (e.g. Red for "Error",
//! Yellow for "Warning") and prefixes the messages with semantic icons (❌, ⚠️, ✅, ℹ️).
//! It is ideal for monitoring system status in a running TUI application.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Widget},
};

/// A widget that displays a list of log messages with automatic color coding.
///
/// Use [`LogList::new`] to create a new instance, passing a vector of strings.
/// Messages containing specific keywords will be styled automatically:
/// - "Error": Red text with ❌ prefix
/// - "Warning": Yellow text with ⚠️ prefix
/// - "Success": Green text with ✅ prefix
/// - "Note" or "Info": Blue text with ℹ️ prefix
///
/// # Example
///
/// ```
/// use tui_shared::LogList;
/// use ratatui::widgets::Widget;
/// use ratatui::layout::Rect;
/// use ratatui::buffer::Buffer;
///
/// let logs = vec![
///     "Error: Connection failed".to_string(),
///     "Success: Data saved".to_string(),
/// ];
/// let widget = LogList::new(logs).with_title("System Logs");
///
/// // Render to buffer (usually done by Terminal::draw)
/// let area = Rect::new(0, 0, 20, 10);
/// let mut buffer = Buffer::empty(area);
/// widget.render(area, &mut buffer);
/// ```
pub struct LogList<'a> {
    items: Vec<String>,
    block: Option<Block<'a>>,
}

impl<'a> LogList<'a> {
    /// Creates a new `LogList` with the given items.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::LogList;
    ///
    /// let logs = vec!["System started".to_string(), "Error: timeout".to_string()];
    /// let log_list = LogList::new(logs);
    /// ```
    pub fn new(items: Vec<String>) -> Self {
        Self { items, block: None }
    }

    /// Helper to set a block with a title and all borders.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::LogList;
    ///
    /// let logs = vec!["Warning: low memory".to_string()];
    /// let log_list = LogList::new(logs).with_title("Warnings");
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.block = Some(Block::default().borders(Borders::ALL).title(title.into()));
        self
    }
}

impl<'a> Widget for LogList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items = self.items.iter().map(|s| {
            // Optimization: avoid `s.to_lowercase()` to reduce heap allocations per render frame.
            // Using case-insensitive ascii checks works because our keywords are ascii.
            let contains_ignore_case = |keyword: &str| -> bool {
                // If it's a hot path, a simple ascii substring check is much faster than regex
                // or building a new String via `to_lowercase()`.
                s.as_bytes().windows(keyword.len()).any(|window| {
                    window
                        .iter()
                        .zip(keyword.as_bytes())
                        .all(|(&c, &k)| c.to_ascii_lowercase() == k)
                })
            };

            let (style, prefix) = if contains_ignore_case("error") {
                (
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    "❌ ",
                )
            } else if contains_ignore_case("warning") {
                (Style::default().fg(Color::Yellow), "⚠️ ")
            } else if contains_ignore_case("note") || contains_ignore_case("info") {
                (Style::default().fg(Color::Blue), "ℹ️ ")
            } else if contains_ignore_case("success") {
                (
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                    "✅ ",
                )
            } else {
                (Style::default(), "")
            };

            let content = Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(s.as_str(), style),
            ]);
            ListItem::new(content)
        });

        let mut list = List::new(items);
        if let Some(block) = self.block {
            list = list.block(block);
        }
        list.render(area, buf);
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
    fn test_log_list_info_note_rendering() {
        let items = vec![
            "Info: Something happened".to_string(),
            "Note: Pay attention".to_string(),
        ];
        let log_list = LogList::new(items).with_title("Logs");
        let area = Rect::new(0, 0, 40, 5);
        let mut buffer = Buffer::empty(area);

        log_list.render(area, &mut buffer);

        // Render with title means logs start at y=1, and x=1
        // Check Info line
        let cell = &buffer[(1, 1)];
        assert_eq!(cell.symbol(), "ℹ\u{fe0f}");
        assert_eq!(cell.fg, Color::Blue);

        // Check Note line
        let cell = &buffer[(1, 2)];
        assert_eq!(cell.symbol(), "ℹ\u{fe0f}");
        assert_eq!(cell.fg, Color::Blue);
    }
}

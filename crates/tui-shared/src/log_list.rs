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

use std::borrow::Cow;

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
    items: Vec<Cow<'a, str>>,
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
    /// let logs: Vec<&str> = vec!["System started", "Error: timeout"];
    /// let log_list = LogList::new(logs);
    /// ```
    pub fn new<I>(items: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Cow<'a, str>>,
    {
        Self {
            items: items.into_iter().map(|i| i.into()).collect(),
            block: None,
        }
    }

    /// Helper to set a block with a title and all borders.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_shared::LogList;
    ///
    /// let logs: Vec<&str> = vec!["Warning: low memory"];
    /// let log_list = LogList::new(logs).with_title("Warnings");
    /// ```
    pub fn with_title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.block = Some(Block::default().borders(Borders::ALL).title(title.into()));
        self
    }
}

impl<'a> Widget for LogList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items = self.items.iter().map(|s| {
            let (style, prefix) = get_log_style_and_prefix(s);

            let content = Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(s.as_ref(), style),
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

fn get_log_style_and_prefix(s: &str) -> (Style, &'static str) {
    // Optimization: avoid `s.to_lowercase()` to reduce heap allocations per render frame.
    // Using case-insensitive ascii checks works because our keywords are ascii.
    let contains_ignore_case = |keyword: &str| -> bool {
        // Optimization: Uses LLVM-optimized vector instructions for zero-cost abstraction performance gains.
        // If it's a hot path, a simple ascii substring check is much faster than regex
        // or building a new String via `to_lowercase()`.
        if s.len() < keyword.len() {
            return false;
        }
        let first_byte = keyword.as_bytes()[0]; // assumes keyword is lowercase ASCII
        s.as_bytes().windows(keyword.len()).any(|window| {
            // Fast-path: quickly check the first character before doing the full slice comparison
            window[0].to_ascii_lowercase() == first_byte
                && window.eq_ignore_ascii_case(keyword.as_bytes())
        })
    };

    if contains_ignore_case("error") {
        (
            Style::default().fg(Color::Red).bg(Color::Black).add_modifier(Modifier::BOLD),
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::style::Color;

    #[test]
    fn test_case_insensitive_fast_path() {
        let (_style, prefix) = super::get_log_style_and_prefix("eRrOr: something");
        assert_eq!(prefix, "❌ ");
        let (_style, prefix) = super::get_log_style_and_prefix("SUCCESS! done");
        assert_eq!(prefix, "✅ ");
    }

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

    #[test]
    fn test_table_driven_log_list_rendering() {
        struct TestCase {
            message: &'static str,
            expected_symbol: &'static str,
            expected_color: Color,
        }

        let test_cases = vec![
            TestCase {
                message: "This is an error message",
                expected_symbol: "❌",
                expected_color: Color::Red,
            },
            TestCase {
                message: "Error in system",
                expected_symbol: "❌",
                expected_color: Color::Red,
            },
            TestCase {
                message: "System warning: low disk space",
                expected_symbol: "⚠️",
                expected_color: Color::Yellow,
            },
            TestCase {
                message: "Success! Operation completed",
                expected_symbol: "✅",
                expected_color: Color::Green,
            },
            TestCase {
                message: "Note: please restart",
                expected_symbol: "ℹ\u{fe0f}",
                expected_color: Color::Blue,
            },
            TestCase {
                message: "Info about the process",
                expected_symbol: "ℹ\u{fe0f}",
                expected_color: Color::Blue,
            },
            TestCase {
                message: "Plain message with no keywords",
                expected_symbol: "P",
                expected_color: Color::Reset,
            },
            TestCase {
                message: "ERROR in all caps",
                expected_symbol: "❌",
                expected_color: Color::Red,
            },
            TestCase {
                message: "warning in lowercase",
                expected_symbol: "⚠️",
                expected_color: Color::Yellow,
            },
            TestCase {
                message: "SuCcEsS with mixed case",
                expected_symbol: "✅",
                expected_color: Color::Green,
            },
        ];

        for (i, case) in test_cases.iter().enumerate() {
            let log_list = LogList::new(vec![case.message.to_string()]);
            let area = Rect::new(0, 0, 40, 1);
            let mut buffer = Buffer::empty(area);

            log_list.render(area, &mut buffer);

            let cell = &buffer[(0, 0)];
            assert_eq!(
                cell.symbol(),
                case.expected_symbol,
                "Failed on test case {}: {}",
                i,
                case.message
            );
            assert_eq!(
                cell.fg, case.expected_color,
                "Failed on test case {}: {}",
                i, case.message
            );
        }
    }
}

use ratatui::{
    style::{Color, Style},
    widgets::{Block, List, ListItem, Widget},
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
}

impl<'a> Widget for LogList<'a> {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|s| {
                let style = if s.contains("Error") {
                    Style::default().fg(Color::Red)
                } else if s.contains("Warning") {
                    Style::default().fg(Color::Yellow)
                } else if s.contains("Note") {
                    Style::default().fg(Color::Blue)
                } else {
                    Style::default()
                };
                ListItem::new(s.as_str()).style(style)
            })
            .collect();

        let mut list = List::new(items);
        if let Some(block) = self.block {
            list = list.block(block);
        }
        list.render(area, buf);
    }
}

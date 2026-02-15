use ratatui::{
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, List, ListItem, Widget},
    buffer::Buffer,
    layout::{Rect},
    text::Line,
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

/// A reusable Button component for Arthropod UI.
///
/// Supports hover and click states with visual feedback.
pub struct Button<'a> {
    label: String,
    is_hovered: bool,
    is_clicked: bool,
    block: Option<Block<'a>>,
}

impl<'a> Button<'a> {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            is_hovered: false,
            is_clicked: false,
            block: None,
        }
    }

    pub fn hovered(mut self, hovered: bool) -> Self {
        self.is_hovered = hovered;
        self
    }

    pub fn clicked(mut self, clicked: bool) -> Self {
        self.is_clicked = clicked;
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for Button<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let style = if self.is_clicked {
            Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
        } else if self.is_hovered {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };

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

        let line = Line::from(self.label);
        let x_offset = (text_area.width.saturating_sub(line.width() as u16)) / 2;
        buf.set_line(text_area.x + x_offset, text_area.y, &line, text_area.width);
    }
}

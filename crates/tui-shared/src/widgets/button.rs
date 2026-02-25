use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct Button<'a> {
    label: &'a str,
    is_active: bool,
    is_hovered: bool,
}

impl<'a> Button<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            is_active: false,
            is_hovered: false,
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.is_active = active;
        self
    }

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

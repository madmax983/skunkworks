use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Button<'a> {
    pub label: &'a str,
    pub active: bool,
    pub hovered: bool,
}

impl<'a> Button<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            active: false,
            hovered: false,
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    pub fn render(self, f: &mut Frame, area: Rect) {
        let mut style = Style::default().fg(Color::White);
        let mut block_style = Style::default().fg(Color::Gray);

        if self.hovered {
            block_style = block_style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
            style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }

        if self.active {
            style = style.bg(Color::Yellow).fg(Color::Black);
            block_style = block_style.fg(Color::Yellow);
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(block_style);

        let paragraph = Paragraph::new(self.label)
            .block(block)
            .style(style)
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(paragraph, area);
    }
}

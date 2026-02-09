use ratatui::{
    layout::Rect,
    style::{Color, Style, Modifier},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::sim::Grid;

pub fn draw_grid(f: &mut Frame, grid: &Grid, area: Rect, cursor: Option<(usize, usize)>) {
    let mut lines = Vec::new();

    // Determine view window based on area size vs grid size
    // For now, just render top-left corner fitting in area
    // TODO: Implement scrolling based on cursor
    let view_height = area.height.saturating_sub(2) as usize;
    let view_width = area.width.saturating_sub(2) as usize;

    let render_height = grid.height.min(view_height);
    let render_width = grid.width.min(view_width);

    for y in 0..render_height {
        let mut spans = Vec::new();
        for x in 0..render_width {
            let is_cursor = cursor.is_some_and(|(cx, cy)| cx == x && cy == y);

            if let Some(cell) = grid.get(x, y) {
                let (char, color) = if cell.water > 0.5 {
                    ('≈', Color::Blue)
                } else if cell.water > 0.05 {
                    ('~', Color::Cyan)
                } else if cell.terrain > 3.0 {
                    ('^', Color::White)
                } else if cell.terrain > 1.0 {
                    ('n', Color::LightRed) // Hills
                } else {
                    ('.', Color::Green) // Plains
                };

                let mut style = Style::default().fg(color);
                if is_cursor {
                    style = style.add_modifier(Modifier::REVERSED);
                }

                spans.push(Span::styled(char.to_string(), style));
            }
        }
        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(Text::from(lines))
        .block(Block::default().title("Tactical Tide (WASD: Move, Space: Water, Enter: Mountain)").borders(Borders::ALL));

    f.render_widget(paragraph, area);
}

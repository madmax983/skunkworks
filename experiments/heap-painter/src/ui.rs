use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::painter::Painter;

pub fn draw(f: &mut Frame, painter: &Painter) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let main_area = chunks[0];
    let status_area = chunks[1];

    let mut lines = Vec::new();
    let mut current_line_spans = Vec::new();

    // We assume the heap width fits in the view for the best effect.
    // If not, it will just be scrollable or cutoff, but let's just render it.

    for (i, cell) in painter.heap.grid.iter().enumerate() {
        if i > 0 && i % painter.heap.width == 0 {
             lines.push(Line::from(current_line_spans.clone()));
             current_line_spans.clear();
        }

        let (symbol, style) = if let Some(id) = cell {
            if let Some(alloc) = painter.heap.allocations.get(id) {
                 // Color logic:
                // New allocations are White/Yellow (Hot)
                // Middle age are RGB based on seed
                // Old are Dark Blue/Gray (Cold/Dead)

                let color = if alloc.age < 2 {
                    Color::White
                } else if alloc.age < 5 {
                    Color::Yellow
                } else {
                    // Seed mapping
                    match alloc.color_seed % 6 {
                        0 => Color::Red,
                        1 => Color::Green,
                        2 => Color::Blue,
                        3 => Color::Magenta,
                        4 => Color::Cyan,
                        _ => Color::LightRed,
                    }
                };

                // Shade based on age?
                // Ratatui colors are limited unless using RGB.
                // Let's stick to basic colors for broad compatibility/simplicity.

                ("█", Style::default().fg(color))
            } else {
                // This shouldn't happen if logic is correct
                ("?", Style::default().fg(Color::Red))
            }
        } else {
            ("·", Style::default().fg(Color::DarkGray))
        };

        current_line_spans.push(Span::styled(symbol, style));
    }
    // Push last line
    if !current_line_spans.is_empty() {
        lines.push(Line::from(current_line_spans));
    }

    let heap_widget = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" Memory Canvas (Heap State) "));

    f.render_widget(heap_widget, main_area);

    // Status Bar
    let status_text = format!(
        "Mode: {} | Frame: {} | Allocations: {} | Frag: {:.2} | [Q]uit [SPACE]Switch Mode",
        painter.mode, painter.frame, painter.allocated_ids.len(), painter.heap.fragmentation()
    );
    let status = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL).title(" Status "));

    f.render_widget(status, status_area);
}

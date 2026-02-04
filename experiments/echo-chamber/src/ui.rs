use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(
    f: &mut Frame,
    width: usize,
    height: usize,
    pressure: &[f32],
    walls: &[bool],
    listener: (usize, usize),
    source: (usize, usize), // Cursor/Source
    info_text: &str,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let grid_area = chunks[0];

    // We render the grid as lines of text
    // We assume the terminal size is sufficient.

    let mut lines = Vec::with_capacity(height);

    for y in 0..height {
        let mut spans = Vec::with_capacity(width);
        for x in 0..width {
            let idx = y * width + x;

            // Default style
            let mut style = Style::default();
            let mut ch = ' ';

            // Check Walls
            if walls.get(idx).copied().unwrap_or(false) {
                style = style.bg(Color::White); // Wall is white block
                ch = ' ';
            } else {
                // Check Pressure
                let p = pressure.get(idx).copied().unwrap_or(0.0);

                // Visualization:
                // p > 0: Red tint
                // p < 0: Blue tint
                // We use RGB for smooth gradient

                let intensity = (p.abs() * 255.0).clamp(0.0, 255.0) as u8;

                if intensity > 10 {
                    if p > 0.0 {
                        // Red
                        style = style.bg(Color::Rgb(intensity, 0, 0));
                    } else {
                        // Blue
                        style = style.bg(Color::Rgb(0, 0, intensity));
                    }
                } else {
                    style = style.bg(Color::Black);
                }

                // Overlays
                if (x, y) == listener {
                    ch = 'L';
                    style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
                } else if (x, y) == source {
                    ch = 'S';
                    style = style.fg(Color::Green).add_modifier(Modifier::BOLD);
                }
            }

            spans.push(Span::styled(ch.to_string(), style));
        }
        lines.push(Line::from(spans));
    }

    let grid_widget = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Echo Chamber (FDTD Acoustic Simulation)"));

    f.render_widget(grid_widget, grid_area);

    // Info Bar
    let info_widget = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(info_widget, chunks[1]);
}

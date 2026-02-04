use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::game::Game;

pub fn draw(f: &mut Frame, game: &Game) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Stats
            Constraint::Min(0),    // Code
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // Stats
    let wpm = game.wpm();
    let accuracy = game.accuracy();
    let stats_text = format!(
        " WPM: {:.1} | Accuracy: {:.1}% | Progress: {:.0}%",
        wpm,
        accuracy,
        game.progress() * 100.0
    );

    let stats_block = Block::default()
        .borders(Borders::ALL)
        .title(" Code Sprint 🌟 ");

    let stats_paragraph = Paragraph::new(stats_text)
        .block(stats_block)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(stats_paragraph, chunks[0]);

    // Code Area
    let code_area = chunks[1];
    let code_block = Block::default()
        .borders(Borders::ALL)
        .title(" Type the Code ");

    // Render the code
    let mut lines: Vec<Line> = Vec::new();
    let mut current_line_spans: Vec<Span> = Vec::new();

    let input_chars: Vec<char> = game.input.chars().collect();
    let snippet_chars: Vec<char> = game.snippet.chars().collect();

    // We process the snippet char by char to build spans
    for (i, &s_char) in snippet_chars.iter().enumerate() {
        let style = if i < input_chars.len() {
            let i_char = input_chars[i];
            if i_char == s_char {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Red).bg(Color::DarkGray)
            }
        } else {
            // Not typed yet
            Style::default().fg(Color::Gray)
        };

        if s_char == '\n' {
            lines.push(Line::from(current_line_spans.clone()));
            current_line_spans.clear();
        } else {
            current_line_spans.push(Span::styled(s_char.to_string(), style));
        }
    }

    // Push last line
    if !current_line_spans.is_empty() {
        lines.push(Line::from(current_line_spans));
    }

    let p = Paragraph::new(lines).block(code_block);
    f.render_widget(p, code_area);

    // Cursor positioning
    if !game.finished {
        let (cx, cy) = calculate_cursor_pos(&game.snippet, game.input.len());
        // Offset by block border (1, 1) + area position
        let area_x = code_area.x + 1;
        let area_y = code_area.y + 1;

        // In newer ratatui, use set_cursor_position
        f.set_cursor_position((area_x + cx, area_y + cy));
    }

    // Footer
    let footer_text = if game.finished {
        " Finished! Press [Enter] for next snippet, [Esc] to quit. "
    } else {
        " Type the code exactly. [Esc] to quit, [Tab] to skip. "
    };

    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(footer, chunks[2]);
}

fn calculate_cursor_pos(snippet: &str, input_len: usize) -> (u16, u16) {
    let mut x = 0;
    let mut y = 0;

    for (i, c) in snippet.chars().enumerate() {
        if i == input_len {
            return (x, y);
        }
        if c == '\n' {
            x = 0;
            y += 1;
        } else {
            x += 1;
        }
    }
    (x, y)
}

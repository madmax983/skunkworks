use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::scanner::FileNode;

pub struct AppState {
    pub files: Vec<FileNode>,
    pub selected_index: usize,
    pub entropy: f32,
    pub inspector_view: bool,
    pub decayed_content: Option<String>,
    pub scroll_offset: u16,
}

pub fn draw_ui(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    if state.inspector_view {
        draw_inspector(f, chunks[0], state);
    } else {
        draw_grid(f, chunks[0], state);
    }

    draw_footer(f, chunks[1], state);
}

fn draw_grid(f: &mut Frame, area: Rect, state: &AppState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Digital Decay Archive ");

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    if inner_area.width < 2 || inner_area.height < 1 {
        return;
    }

    // Calculate grid dimensions
    let cols = (inner_area.width / 2) as usize; // 2 chars per file block
    let rows = inner_area.height as usize;
    let page_size = cols * rows;

    if page_size == 0 {
        return;
    }

    // Simple pagination based on selection
    let start_index = (state.selected_index / page_size) * page_size;

    let mut lines = Vec::new();
    let mut current_line_spans = Vec::new();

    for (i, file) in state.files.iter().enumerate().skip(start_index).take(page_size) {
        let is_selected = i == state.selected_index;

        let color = if file.health < 0.5 {
            Color::Red
        } else if file.health < 0.9 {
            Color::Yellow
        } else {
            Color::Green
        };

        let style = if is_selected {
            Style::default().fg(color).add_modifier(Modifier::REVERSED | Modifier::BOLD)
        } else {
            Style::default().fg(color)
        };

        // Representation: █ or ▒ based on health
        let char_code = if file.health < 0.3 { "░" } else if file.health < 0.7 { "▒" } else { "█" };

        current_line_spans.push(Span::styled(char_code, style));
        // Add a space for visual separation
        current_line_spans.push(Span::raw(" "));

        // Check if we filled the row (cols is capacity in *items*, item width is 2 chars)
        if current_line_spans.len() >= cols * 2 {
            lines.push(Line::from(current_line_spans.clone()));
            current_line_spans.clear();
        }
    }

    // Flush remaining
    if !current_line_spans.is_empty() {
        lines.push(Line::from(current_line_spans));
    }

    let p = Paragraph::new(lines);
    f.render_widget(p, inner_area);
}

fn draw_inspector(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);

    // Left: Details
    if let Some(file) = state.files.get(state.selected_index) {
        let details = vec![
            Line::from(vec![Span::raw("Path: "), Span::styled(file.path.to_string_lossy(), Style::default().fg(Color::Cyan))]),
            Line::from(vec![Span::raw(format!("Size: {} bytes", file.size))]),
            Line::from(vec![Span::raw(format!("Health: {:.2}%", file.health * 100.0))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled("Entropy Level", Style::default().fg(Color::Red)), Span::raw(format!(": {:.2}", state.entropy))]),
        ];

        let block = Block::default().borders(Borders::ALL).title(" Metadata ");
        let p = Paragraph::new(details).block(block).wrap(Wrap { trim: true });
        f.render_widget(p, chunks[0]);
    }

    // Right: Content
    let content_block = Block::default().borders(Borders::ALL).title(" Sector View ");
    let content_text = state.decayed_content.as_deref().unwrap_or("Loading sector...");

    // TODO: Syntax highlighting or hex dump formatting could go here
    let p = Paragraph::new(content_text)
        .block(content_block)
        .wrap(Wrap { trim: false })
        .scroll((state.scroll_offset, 0));

    f.render_widget(p, chunks[1]);
}

fn draw_footer(f: &mut Frame, area: Rect, state: &AppState) {
    let text = if state.inspector_view {
        "ESC: Grid | ↑/↓: Scroll | +/-: Entropy | R: Repair | Q: Quit"
    } else {
        "ENTER: Inspect | Arrow Keys: Navigate | +/-: Entropy | Q: Quit"
    };

    let p = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::TOP));
    f.render_widget(p, area);
}

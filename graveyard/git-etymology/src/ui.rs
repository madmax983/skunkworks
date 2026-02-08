use crate::etym::{ChangeType, TraceEntry};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

#[derive(Debug, PartialEq)]
pub enum AppMode {
    Browsing, // Selecting a line
    Tracing,  // Viewing the history
}

pub struct AppState {
    pub file_path: String,
    pub file_lines: Vec<String>,
    pub list_state: ListState,

    pub trace: Vec<TraceEntry>,
    pub trace_state: ListState,

    pub mode: AppMode,
    pub status_msg: String,
}

impl AppState {
    pub fn new(path: String, lines: Vec<String>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            file_path: path,
            file_lines: lines,
            list_state,
            trace: Vec::new(),
            trace_state: ListState::default(),
            mode: AppMode::Browsing,
            status_msg: "Use Up/Down to select line, Enter to trace.".to_string(),
        }
    }

    pub fn next_line(&mut self) {
        if self.mode == AppMode::Browsing {
            let i = match self.list_state.selected() {
                Some(i) => {
                    if i >= self.file_lines.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.list_state.select(Some(i));
        } else {
            let i = match self.trace_state.selected() {
                Some(i) => {
                    if self.trace.is_empty() {
                        0
                    } else if i >= self.trace.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.trace_state.select(Some(i));
        }
    }

    pub fn prev_line(&mut self) {
        if self.mode == AppMode::Browsing {
            let i = match self.list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.file_lines.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.list_state.select(Some(i));
        } else {
            let i = match self.trace_state.selected() {
                Some(i) => {
                    if self.trace.is_empty() {
                        0
                    } else if i == 0 {
                        self.trace.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.trace_state.select(Some(i));
        }
    }
}

pub fn draw_ui(f: &mut Frame, app: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.area());

    draw_file_viewer(f, app, chunks[0]);
    draw_trace_viewer(f, app, chunks[1]);
}

fn draw_file_viewer(f: &mut Frame, app: &mut AppState, area: Rect) {
    let items: Vec<ListItem> = app
        .file_lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let num = Span::styled(
                format!("{:4} ", i + 1),
                Style::default().fg(Color::DarkGray),
            );
            let content = Span::raw(line);
            ListItem::new(Line::from(vec![num, content]))
        })
        .collect();

    let border_style = if app.mode == AppMode::Browsing {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Gray)
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" File: {} ", app.file_path))
                .border_style(border_style),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, &mut app.list_state);
}

fn draw_trace_viewer(f: &mut Frame, app: &mut AppState, area: Rect) {
    let border_style = if app.mode == AppMode::Tracing {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Gray)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Etymology Trace ")
        .border_style(border_style);

    if app.trace.is_empty() {
        let p = Paragraph::new(app.status_msg.clone())
            .block(block)
            .wrap(Wrap { trim: true });
        f.render_widget(p, area);
        return;
    }

    let items: Vec<ListItem> = app
        .trace
        .iter()
        .map(|entry| {
            let header = Line::from(vec![
                Span::styled(
                    format!("{} ", &entry.short_hash),
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled(
                    format!("{} ", entry.date.format("%Y-%m-%d")),
                    Style::default().fg(Color::Blue),
                ),
                Span::styled(
                    format!("{} ", entry.author),
                    Style::default().fg(Color::Green),
                ),
            ]);

            let change_style = match entry.change_type {
                ChangeType::Genesis => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ChangeType::Shift => Style::default().fg(Color::Magenta),
                ChangeType::Inertia => Style::default().fg(Color::DarkGray),
            };

            let type_str = match entry.change_type {
                ChangeType::Genesis => "★ GENESIS",
                ChangeType::Shift => "⚡ SHIFT",
                ChangeType::Inertia => "⬇ INERTIA",
            };

            let meta = Line::from(vec![
                Span::styled(type_str, change_style),
                Span::raw(format!(" (Line {})", entry.line_num)),
            ]);

            let content = Line::from(Span::styled(
                format!("  {}", entry.line_content),
                Style::default().fg(Color::White),
            ));
            let msg = Line::from(Span::styled(
                format!("  \"{}\"", entry.message),
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::ITALIC),
            ));

            // Multiline items are hard in List, so we cheat by just doing lines?
            // Or we use a simple representation.
            // Let's stick to 2 lines per entry if possible, or just the content.

            // Actually ListItems can be height 1+.
            // But let's keep it compact.

            ListItem::new(vec![
                header,
                meta,
                content,
                msg,
                Line::from(""), // Spacer
            ])
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().bg(Color::Rgb(50, 50, 50)));

    f.render_stateful_widget(list, area, &mut app.trace_state);
}

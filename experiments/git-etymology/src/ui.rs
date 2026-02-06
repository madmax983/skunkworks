use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap, ListState, Table, Row, Cell, TableState},
    Frame,
};
use crate::etym::{TraceEntry, ChangeType};

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
    pub trace_state: TableState,

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
            trace_state: TableState::default(),
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
                    if self.trace.is_empty() { 0 }
                    else if i >= self.trace.len() - 1 { 0 }
                    else { i + 1 }
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
                    if self.trace.is_empty() { 0 }
                    else if i == 0 { self.trace.len() - 1 }
                    else { i - 1 }
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
                .bg(Color::Rgb(40, 44, 52))
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
        let p = Paragraph::new(app.status_msg.clone()).block(block).wrap(Wrap { trim: true });
        f.render_widget(p, area);
        return;
    }

    let rows: Vec<Row> = app.trace.iter().map(|entry| {
        let (type_str, type_color) = match entry.change_type {
            ChangeType::Genesis => ("★ GENESIS", Color::Yellow),
            ChangeType::Shift => ("⚡ SHIFT", Color::Cyan),
            ChangeType::Inertia => ("⬇ INERTIA", Color::DarkGray),
        };

        Row::new(vec![
            Cell::from(Span::styled(type_str, Style::default().fg(type_color).add_modifier(Modifier::BOLD))),
            Cell::from(Span::styled(entry.short_hash.clone(), Style::default().fg(Color::Yellow))),
            Cell::from(Span::styled(entry.date.format("%Y-%m-%d").to_string(), Style::default().fg(Color::Blue))),
            Cell::from(Span::styled(entry.author.clone(), Style::default().fg(Color::Green))),
            Cell::from(Span::raw(entry.line_content.clone())),
        ])
    }).collect();

    let table = Table::new(rows, [
        Constraint::Length(12),
        Constraint::Length(8),
        Constraint::Length(11),
        Constraint::Length(15),
        Constraint::Min(0),
    ])
    .header(
        Row::new(vec!["Type", "Hash", "Date", "Author", "Content"])
            .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::UNDERLINED))
            .bottom_margin(1)
    )
    .block(block)
    .row_highlight_style(Style::default().bg(Color::Rgb(40, 44, 52)).add_modifier(Modifier::BOLD));

    f.render_stateful_widget(table, area, &mut app.trace_state);
}

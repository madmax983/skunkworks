mod sexagesimal;
mod series;

use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem},
    Frame,
};
use tui_shared::Tui;

use sexagesimal::Sexagesimal;
use series::TimeSeries;

struct App {
    data: TimeSeries,
    sma: TimeSeries,
    window_size: usize,
    exit: bool,
    scroll_offset: usize,
}

impl App {
    fn new() -> Self {
        let len = 100;
        let data = TimeSeries::generate_sine_wave(len);
        let window_size = 5;
        let sma = data.simple_moving_average(window_size);

        Self {
            data,
            sma,
            window_size,
            exit: false,
            scroll_offset: 0,
        }
    }

    fn update_sma(&mut self) {
        if self.window_size == 0 {
            self.window_size = 1;
        }
        if self.window_size > self.data.data.len() {
            self.window_size = self.data.data.len();
        }
        self.sma = self.data.simple_moving_average(self.window_size);
    }

    fn run(&mut self, tui: &mut Tui) -> Result<()> {
        while !self.exit {
            tui.terminal.draw(|f| self.ui(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            self.window_size += 1;
                            self.update_sma();
                        }
                        KeyCode::Char('-') | KeyCode::Char('_') => {
                            if self.window_size > 1 {
                                self.window_size -= 1;
                                self.update_sma();
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if self.scroll_offset < self.data.data.len().saturating_sub(1) {
                                self.scroll_offset += 1;
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if self.scroll_offset > 0 {
                                self.scroll_offset -= 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(f.area());

        self.render_tablet(f, chunks[0]);
        self.render_chart(f, chunks[1]);
    }

    fn render_tablet(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let items: Vec<ListItem> = self.data.data.iter().enumerate()
            .skip(self.scroll_offset)
            .map(|(i, val)| {
                let idx_sex = Sexagesimal::from_u64(i as u64);
                let content = Line::from(vec![
                    Span::raw(format!("{}: ", idx_sex)).gray(),
                    Span::raw(format!("{}", val)).yellow(),
                ]);
                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Cuneiform Tablet"))
            .highlight_style(Style::default().add_modifier(ratatui::style::Modifier::BOLD));

        f.render_widget(list, area);
    }

    fn render_chart(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let raw_data: Vec<(f64, f64)> = self.data.data.iter().enumerate()
            .map(|(i, val)| (i as f64, val.to_f64()))
            .collect();

        let sma_data: Vec<(f64, f64)> = self.sma.data.iter().enumerate()
            .map(|(i, val)| ((i + self.window_size - 1) as f64, val.to_f64()))
            .collect();

        let x_labels = vec![
            Span::styled(" ", Style::default().add_modifier(ratatui::style::Modifier::BOLD)),
            Span::raw(format!("{}", Sexagesimal::from_u64(50))),
            Span::styled(format!("{}", Sexagesimal::from_u64(100)), Style::default().add_modifier(ratatui::style::Modifier::BOLD)),
        ];

        let datasets = vec![
            Dataset::default()
                .name("Raw Data")
                .marker(symbols::Marker::Dot)
                .graph_type(GraphType::Scatter)
                .style(Style::default().fg(Color::Cyan))
                .data(&raw_data),
            Dataset::default()
                .name(format!("SMA({})", Sexagesimal::from_u64(self.window_size as u64)))
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Yellow))
                .data(&sma_data),
        ];

        let chart = Chart::new(datasets)
            .block(Block::default().borders(Borders::ALL).title("Saros Cycle Analysis"))
            .x_axis(Axis::default()
                .title("Time (t)")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0])
                .labels(x_labels))
            .y_axis(Axis::default()
                .title("Magnitude")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 25.0]) // Sine wave scaled [0, 20]
                .labels(vec![
                    Span::raw(" "), // empty
                    Span::raw(format!("{}", Sexagesimal::from_u64(10))),
                    Span::raw(format!("{}", Sexagesimal::from_u64(20))),
                ]));

        f.render_widget(chart, area);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let res = app.run(&mut tui);
    tui.exit()?;
    res
}

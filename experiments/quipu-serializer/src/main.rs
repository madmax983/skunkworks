use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use quipu::{Cord, Knot, Quipu};
use quipu_serializer::ser::to_quipu;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use serde_json::Value;
use std::io;

enum Mode {
    Calculator,
    Serializer,
}

struct App {
    mode: Mode,
    // Calculator State
    calc_input_a: String,
    calc_input_b: String,
    calc_focus: usize, // 0=A, 1=B

    // Serializer State
    ser_input: String,
    ser_output: Option<Quipu>,
    ser_scroll: u16,
}

impl App {
    fn new() -> Self {
        Self {
            mode: Mode::Calculator,
            calc_input_a: String::new(),
            calc_input_b: String::new(),
            calc_focus: 0,
            ser_input: String::new(),
            ser_output: None,
            ser_scroll: 0,
        }
    }

    fn update_serializer(&mut self) {
        if let Ok(v) = serde_json::from_str::<Value>(&self.ser_input) {
            if let Ok(q) = to_quipu(&v) {
                self.ser_output = Some(q);
            } else {
                self.ser_output = None;
            }
        } else {
            self.ser_output = None;
        }
    }
}

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                Mode::Calculator => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Tab => {
                        app.mode = Mode::Serializer;
                    }
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        if app.calc_focus == 0 {
                            app.calc_input_a.push(c);
                        } else {
                            app.calc_input_b.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        if app.calc_focus == 0 {
                            app.calc_input_a.pop();
                        } else {
                            app.calc_input_b.pop();
                        }
                    }
                    KeyCode::Enter | KeyCode::Down | KeyCode::Up => {
                        app.calc_focus = 1 - app.calc_focus;
                    }
                    _ => {}
                },
                Mode::Serializer => match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Tab => {
                        app.mode = Mode::Calculator;
                    }
                    KeyCode::Char(c) => {
                        app.ser_input.push(c);
                        app.update_serializer();
                    }
                    KeyCode::Backspace => {
                        app.ser_input.pop();
                        app.update_serializer();
                    }
                    KeyCode::Up => {
                        if app.ser_scroll > 0 {
                            app.ser_scroll -= 1;
                        }
                    }
                    KeyCode::Down => {
                        app.ser_scroll += 1;
                    }
                    _ => {}
                },
            }
        }
    }

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn map_color(c: quipu::Color) -> Color {
    match c {
        quipu::Color::Natural => Color::Gray,
        quipu::Color::Red => Color::Red,
        quipu::Color::Green => Color::Green,
        quipu::Color::Blue => Color::Blue,
        quipu::Color::Yellow => Color::Yellow,
        quipu::Color::Black => Color::DarkGray,
        quipu::Color::White => Color::White,
    }
}

fn render_cord_text(cord: &Cord, indent_level: usize) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let indent_str = "  ".repeat(indent_level);

    // Color Label
    if cord.color != quipu::Color::Natural {
        lines.push(Line::from(vec![
            Span::raw(indent_str.clone()),
            Span::styled(
                format!("[{:?}]", cord.color),
                Style::default().fg(map_color(cord.color)),
            ),
        ]));
    }

    // Main Cord String
    if cord.clusters.is_empty() {
        lines.push(Line::from(vec![
            Span::raw(indent_str.clone()),
            Span::styled("  │", Style::default().fg(Color::DarkGray)),
            Span::raw(" (empty)"),
        ]));
    } else {
        // Display from Top (highest power) to Bottom (units)
        for (i, cluster) in cord.clusters.iter().enumerate().rev() {
            let mut spans = Vec::new();
            spans.push(Span::raw(indent_str.clone()));
            spans.push(Span::raw("  ")); // Spacing for cord

            if cluster.is_empty() {
                spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
            } else {
                spans.push(Span::styled("│ ", Style::default().fg(Color::DarkGray)));
                for (j, knot) in cluster.iter().enumerate() {
                    let (symbol, color) = match knot {
                        Knot::Simple => ("●".to_string(), Color::Yellow),
                        Knot::Long(v) => (format!("≡{}", v), Color::Green),
                        Knot::FigureEight => ("∞".to_string(), Color::Cyan),
                    };
                    spans.push(Span::styled(symbol, Style::default().fg(color)));
                    if j < cluster.len() - 1 {
                        spans.push(Span::raw(" "));
                    }
                }
            }
            lines.push(Line::from(spans));

            // Spacer between clusters (verticality)
            if i > 0 {
                lines.push(Line::from(vec![
                    Span::raw(indent_str.clone()),
                    Span::raw("    "),
                    Span::styled("│", Style::default().fg(Color::DarkGray)),
                ]));
            }
        }
    }

    // Tail of main cord
    lines.push(Line::from(vec![
        Span::raw(indent_str.clone()),
        Span::raw("    "),
        Span::styled("▼", Style::default().fg(Color::DarkGray)),
    ]));

    // Subsidiaries
    if !cord.subsidiaries.is_empty() {
        // Connector
        lines.push(Line::from(vec![
            Span::raw(indent_str.clone()),
            Span::styled("  └─Subsidiaries:", Style::default().fg(Color::DarkGray)),
        ]));
        for sub in &cord.subsidiaries {
            lines.extend(render_cord_text(sub, indent_level + 1));
            // Add a spacer line between siblings?
            lines.push(Line::from(""));
        }
    }

    lines
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title_text = match app.mode {
        Mode::Calculator => "🧮 QUIPU CALCULATOR",
        Mode::Serializer => "📜 QUIPU SERIALIZER",
    };

    let header = Paragraph::new(title_text)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    match app.mode {
        Mode::Calculator => render_calculator(f, chunks[1], app),
        Mode::Serializer => render_serializer(f, chunks[1], app),
    }

    let footer_text = match app.mode {
        Mode::Calculator => {
            "Q/Esc: Quit | Tab: Switch to Serializer | Digits: Input | Enter/Arrows: Focus"
        }
        Mode::Serializer => {
            "Esc: Quit | Tab: Switch to Calculator | Type JSON | Up/Down: Scroll Output"
        }
    };

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn render_calculator(f: &mut Frame, area: Rect, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // Input A
            Constraint::Percentage(25), // Input B
            Constraint::Percentage(30), // Result
            Constraint::Percentage(20), // Legend
        ])
        .split(area);

    // Input A
    let val_a = app.calc_input_a.parse::<u64>().unwrap_or(0);
    let cord_a = Cord::from(val_a);
    let style_a = if app.calc_focus == 0 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let border_style_a = if app.calc_focus == 0 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let block_a = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style_a)
        .title(format!(" Input A: {} ", app.calc_input_a));

    f.render_widget(
        Paragraph::new(render_cord_text(&cord_a, 0))
            .block(block_a)
            .style(style_a),
        main_chunks[0],
    );

    // Input B
    let val_b = app.calc_input_b.parse::<u64>().unwrap_or(0);
    let cord_b = Cord::from(val_b);
    let style_b = if app.calc_focus == 1 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let border_style_b = if app.calc_focus == 1 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let block_b = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style_b)
        .title(format!(" Input B: {} ", app.calc_input_b));
    f.render_widget(
        Paragraph::new(render_cord_text(&cord_b, 0))
            .block(block_b)
            .style(style_b),
        main_chunks[1],
    );

    // Result
    let cord_sum = cord_a.clone() + cord_b.clone();
    let block_sum = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Sum (A+B): {} ", cord_sum.value()))
        .style(Style::default().fg(Color::Green));
    f.render_widget(
        Paragraph::new(render_cord_text(&cord_sum, 0)).block(block_sum),
        main_chunks[2],
    );

    // Legend
    let legend_text = vec![
        Line::from(Span::styled(
            "Legend",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("●", Style::default().fg(Color::Yellow)),
            Span::raw(" = 1 (Tens+)"),
        ]),
        Line::from(vec![
            Span::styled("≡N", Style::default().fg(Color::Green)),
            Span::raw(" = N (Units)"),
        ]),
        Line::from(vec![
            Span::styled("∞", Style::default().fg(Color::Cyan)),
            Span::raw(" = 1 (Units)"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("│", Style::default().fg(Color::DarkGray)),
            Span::raw(" = Cord"),
        ]),
        Line::from(vec![
            Span::styled("[Color]", Style::default().fg(Color::Magenta)),
            Span::raw(" = Type/Metadata"),
        ]),
    ];
    let block_legend = Block::default().borders(Borders::ALL).title(" Guide ");
    f.render_widget(
        Paragraph::new(legend_text).block(block_legend),
        main_chunks[3],
    );
}

fn render_serializer(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // JSON Input
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(" JSON Input ")
        .style(Style::default().fg(Color::White)); // Explicit white

    f.render_widget(
        Paragraph::new(app.ser_input.clone())
            .block(input_block)
            .wrap(Wrap { trim: true }),
        chunks[0],
    );

    // Quipu Output
    let output_block = Block::default()
        .borders(Borders::ALL)
        .title(" Quipu Output ");

    let mut text_lines = Vec::new();
    if let Some(q) = &app.ser_output {
        text_lines.push(Line::from(Span::styled(
            format!("Quipu with {} pendant cord(s):", q.cords.len()),
            Style::default().add_modifier(Modifier::BOLD),
        )));
        text_lines.push(Line::from(""));

        for (i, cord) in q.cords.iter().enumerate() {
            text_lines.push(Line::from(Span::styled(
                format!("Pendant Cord {}: (Value: {})", i, cord.value()),
                Style::default().fg(Color::Cyan),
            )));
            let cord_lines = render_cord_text(cord, 0);
            text_lines.extend(cord_lines);
            text_lines.push(Line::from("")); // Spacing
        }
    } else if app.ser_input.is_empty() {
        text_lines.push(Line::from(Span::styled(
            "Type JSON to see Quipu...",
            Style::default().fg(Color::Gray),
        )));
    } else {
        text_lines.push(Line::from(Span::styled(
            "Invalid JSON",
            Style::default().fg(Color::Red),
        )));
    }

    f.render_widget(
        Paragraph::new(text_lines)
            .block(output_block)
            .scroll((app.ser_scroll, 0)),
        chunks[1],
    );
}

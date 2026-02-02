pub mod model;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use model::EnigmaMachine;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};
use tui_shared::Tui;

struct App {
    machine: EnigmaMachine,
    history: Vec<(char, char)>,
    last_output: Option<char>,
    selected_rotor: usize, // 0=Left, 1=Middle, 2=Right
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            machine: EnigmaMachine::new(),
            history: Vec::new(),
            last_output: None,
            selected_rotor: 2, // Start with Right rotor focused
            should_quit: false,
        }
    }

    fn on_key(&mut self, c: char) {
        let input = c.to_ascii_uppercase();
        let output = self.machine.encrypt_char(input);
        self.last_output = Some(output);
        self.history.push((input, output));
        // Keep history manageable
        if self.history.len() > 50 {
            self.history.remove(0);
        }
    }

    fn rotate_rotor(&mut self, direction: i8) {
        let rotor = match self.selected_rotor {
            0 => &mut self.machine.left,
            1 => &mut self.machine.middle,
            2 => &mut self.machine.right,
            _ => return,
        };

        if direction > 0 {
            rotor.position = (rotor.position + 1) % 26;
        } else {
            rotor.position = (rotor.position + 25) % 26;
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    loop {
        tui.terminal.draw(|f| ui(f, &app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => app.should_quit = true,
                        KeyCode::Char('q')
                            if key
                                .modifiers
                                .contains(crossterm::event::KeyModifiers::CONTROL) =>
                        {
                            app.should_quit = true;
                        }
                        KeyCode::Left => {
                            if app.selected_rotor > 0 {
                                app.selected_rotor -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if app.selected_rotor < 2 {
                                app.selected_rotor += 1;
                            }
                        }
                        KeyCode::Up => app.rotate_rotor(1),
                        KeyCode::Down => app.rotate_rotor(-1),
                        KeyCode::Char(c) if c.is_ascii_alphabetic() => app.on_key(c),
                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(10), // Rotors & Lamps
            Constraint::Min(0),     // History
        ])
        .split(f.area());

    // 1. Header
    let title = Paragraph::new("ENIGMA MACHINE SIMULATOR")
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // 2. Middle Section (Rotors | Lamps)
    let mid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[1]);

    // Rotors
    f.render_widget(RotorWidget { app }, mid_chunks[0]);

    // Lamps
    f.render_widget(LampboardWidget { app }, mid_chunks[1]);

    // 3. History
    let history_text: Vec<Line> = app
        .history
        .iter()
        .rev()
        .take(10)
        .map(|(in_char, out_char)| {
            Line::from(vec![
                Span::raw("Input: "),
                Span::styled(in_char.to_string(), Style::default().fg(Color::Green)),
                Span::raw(" -> Output: "),
                Span::styled(out_char.to_string(), Style::default().fg(Color::Yellow)),
            ])
        })
        .collect();

    let history_block = Block::default().borders(Borders::ALL).title("Log");
    let p = Paragraph::new(history_text).block(history_block);
    f.render_widget(p, chunks[2]);
}

struct RotorWidget<'a> {
    app: &'a App,
}

impl<'a> Widget for RotorWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL).title("Rotors");
        let inner_area = block.inner(area);
        block.render(area, buf);

        // Calculate positions
        // We want to display 3 windows [ L ] [ M ] [ R ]
        // Each window shows Previous, Current, Next letter.

        let rotors = [
            &self.app.machine.left,
            &self.app.machine.middle,
            &self.app.machine.right,
        ];
        let labels = ["I", "II", "III"]; // Simplified, assuming default setup

        let center_y = inner_area.top() + inner_area.height / 2;
        // Space them out evenly
        let spacing = inner_area.width / 3;

        for (i, rotor) in rotors.iter().enumerate() {
            let center_x = inner_area.left() + (i as u16 * spacing) + spacing / 2;

            // Highlight if selected
            let style = if self.app.selected_rotor == i {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };

            // Previous
            let prev = (rotor.position + 25) % 26;
            buf.set_string(
                center_x - 1,
                center_y - 1,
                format!("{}", (prev as u8 + b'A') as char),
                Style::default().fg(Color::DarkGray),
            );

            // Current (Window)
            let curr = rotor.position;
            buf.set_string(
                center_x - 1,
                center_y,
                format!("[{}]", (curr as u8 + b'A') as char),
                style,
            );

            // Next
            let next = (rotor.position + 1) % 26;
            buf.set_string(
                center_x - 1,
                center_y + 1,
                format!("{}", (next as u8 + b'A') as char),
                Style::default().fg(Color::DarkGray),
            );

            // Label
            buf.set_string(center_x - 1, center_y - 3, labels[i], Style::default());
        }

        // Instructions
        buf.set_string(
            inner_area.left() + 1,
            inner_area.bottom() - 1,
            "Arrows: Move/Rotate",
            Style::default().fg(Color::DarkGray),
        );
    }
}

struct LampboardWidget<'a> {
    app: &'a App,
}

impl<'a> Widget for LampboardWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL).title("Lampboard");
        let inner_area = block.inner(area);
        block.render(area, buf);

        // QWERTY layout
        let rows = [
            "QWERTYUIO",
            "ASDFGHJKL",
            "ZXCVBNM P", // P added to fill padding or handled carefully
        ];

        let start_y = inner_area.top() + 1;

        for (r, row_str) in rows.iter().enumerate() {
            let row_y = start_y + (r as u16 * 2);
            let mut start_x = inner_area.left() + 2 + (r as u16 * 2); // Indent rows like a keyboard

            for c in row_str.chars() {
                if c == ' ' {
                    continue;
                }

                let is_lit = self.app.last_output == Some(c);

                let style = if is_lit {
                    Style::default()
                        .bg(Color::Yellow)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                buf.set_string(start_x, row_y, format!("({})", c), style);
                start_x += 4;
            }
        }
    }
}

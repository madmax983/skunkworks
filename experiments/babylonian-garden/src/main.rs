use std::io;
use std::process::Command;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame, Terminal,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line as TextLine, Span},
    widgets::canvas::{Canvas, Line as CanvasLine},
    widgets::{Block, Borders, Paragraph},
};
use tui_shared::Tui;

mod lsystem;
mod sexagesimal;
mod turtle;

use lsystem::LSystem;
use sexagesimal::Sexagesimal;
use turtle::{Line, Turtle};

struct CommitPlant {
    hash: String,
    message: String,
    author: String,
    date: String,
    sexagesimal: Sexagesimal,
    lsystem: LSystem,
    lines: Vec<Line>,
    angle: f64,
}

struct App {
    plants: Vec<CommitPlant>,
    current_plant_index: usize,
    iterations: u32,
    zoom: f64,
    pan_x: f64,
    pan_y: f64,
    exit: bool,
}

impl App {
    fn new() -> Result<Self> {
        let mut plants = Self::harvest_git_history()?;
        // Ensure plants have initial lines
        for plant in &mut plants {
            if plant.lines.is_empty() {
                 let instructions = plant.lsystem.expand(4);
                 let mut turtle = Turtle::new(0.0, -50.0, -90.0_f64.to_radians(), 2.0, plant.angle);
                 plant.lines = turtle.interpret(&instructions);
            }
        }

        Ok(Self {
            plants,
            current_plant_index: 0,
            iterations: 4,
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
            exit: false,
        })
    }

    fn harvest_git_history() -> Result<Vec<CommitPlant>> {
        let output = Command::new("git")
            .args(&["log", "-n", "10", "--pretty=format:%H|%s|%an|%ad"])
            .output();

        let mut plants = Vec::new();

        if let Ok(output) = output {
            let stdout = String::from_utf8(output.stdout).unwrap_or_default();

            for line in stdout.lines() {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() < 4 {
                    continue;
                }

                let hash = parts[0];
                let message = parts[1];
                let author = parts[2];
                let date = parts[3];

                let hash_prefix = &hash[0..8];
                let val = u64::from_str_radix(hash_prefix, 16).unwrap_or(0);
                let sexagesimal = Sexagesimal::from_u64(val);

                let mut rule_str = "F".to_string();
                for &digit in &sexagesimal.digits {
                    match digit % 4 {
                        0 => rule_str.push_str("[+F]"),
                        1 => rule_str.push_str("[-F]"),
                        2 => rule_str.push_str("F"),
                        3 => rule_str.push_str("[F]"),
                        _ => {}
                    }
                    if digit > 30 {
                       rule_str.push_str("F");
                    }
                }

                if rule_str == "F" {
                    rule_str = "F[+F]F[-F]F".to_string();
                }

                let axiom = "X";
                let x_rule = format!("F-[[X]+X]+F[+FX]-X");
                let f_rule = rule_str;

                let rules = vec![('X', x_rule.as_str()), ('F', f_rule.as_str())];

                let last_digit = *sexagesimal.digits.last().unwrap_or(&25);
                let angle = (20.0 + (last_digit as f64 % 30.0)).to_radians();

                let lsystem = LSystem::new(axiom, rules);

                let instructions = lsystem.expand(4);
                let mut turtle = Turtle::new(0.0, -50.0, -90.0_f64.to_radians(), 2.0, angle);
                let lines = turtle.interpret(&instructions);

                plants.push(CommitPlant {
                    hash: hash.to_string(),
                    message: message.to_string(),
                    author: author.to_string(),
                    date: date.to_string(),
                    sexagesimal,
                    lsystem,
                    lines,
                    angle,
                });
            }
        }

        if plants.is_empty() {
             let val = 123456789;
             let sexagesimal = Sexagesimal::from_u64(val);
             let rules = vec![('X', "F-[[X]+X]+F[+FX]-X"), ('F', "FF")];
             let lsystem = LSystem::new("X", rules);
             let angle = 25.0_f64.to_radians();

             let instructions = lsystem.expand(4);
             let mut turtle = Turtle::new(0.0, -50.0, -90.0_f64.to_radians(), 2.0, angle);
             let lines = turtle.interpret(&instructions);

             plants.push(CommitPlant {
                 hash: "000000".to_string(),
                 message: "No Git History Found".to_string(),
                 author: "System".to_string(),
                 date: "Now".to_string(),
                 sexagesimal,
                 lsystem,
                 lines,
                 angle,
             });
        }

        Ok(plants)
    }

    fn regenerate(&mut self) {
        let plant = &mut self.plants[self.current_plant_index];
        let instructions = plant.lsystem.expand(self.iterations);

        let step_size = 50.0 / (self.iterations as f64 * 2.0).max(1.0);

        let mut turtle = Turtle::new(0.0, -50.0, -90.0_f64.to_radians(), step_size, plant.angle);
        plant.lines = turtle.interpret(&instructions);
    }

    fn run<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()>
    where B::Error: std::fmt::Debug
    {
        while !self.exit {
            terminal.draw(|frame| self.ui(frame))
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
                        KeyCode::Tab => {
                            self.current_plant_index = (self.current_plant_index + 1) % self.plants.len();
                            self.iterations = 4;
                            self.regenerate();
                        }
                        KeyCode::BackTab => {
                             if self.current_plant_index == 0 {
                                 self.current_plant_index = self.plants.len() - 1;
                             } else {
                                 self.current_plant_index -= 1;
                             }
                             self.iterations = 4;
                             self.regenerate();
                        }
                        KeyCode::Char(' ') => {
                            self.iterations += 1;
                            if self.iterations > 7 {
                                self.iterations = 0;
                            }
                            self.regenerate();
                        }
                         KeyCode::Char('r') => {
                            self.iterations = 0;
                            self.regenerate();
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => self.zoom *= 1.1,
                        KeyCode::Char('-') | KeyCode::Char('_') => self.zoom /= 1.1,
                        KeyCode::Left => self.pan_x -= 10.0 / self.zoom,
                        KeyCode::Right => self.pan_x += 10.0 / self.zoom,
                        KeyCode::Up => self.pan_y += 10.0 / self.zoom,
                        KeyCode::Down => self.pan_y -= 10.0 / self.zoom,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn ui(&self, frame: &mut Frame) {
         let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(frame.area());

        let plant = &self.plants[self.current_plant_index];

        let sexagesimal_str = format!("{}", plant.sexagesimal);

        let info = vec![
            TextLine::from(vec![Span::raw("🧬 Babylonian Garden 🌿").bold().green()]),
            TextLine::from(""),
            TextLine::from(vec![Span::raw("Commit: ").bold(), Span::raw(&plant.hash).yellow()]),
            TextLine::from(vec![Span::raw("Author: ").bold(), Span::raw(&plant.author).cyan()]),
            TextLine::from(vec![Span::raw("Date:   ").bold(), Span::raw(&plant.date).blue()]),
            TextLine::from(""),
            TextLine::from(vec![Span::raw("Message:").bold()]),
            TextLine::from(Span::raw(&plant.message).italic()),
            TextLine::from(""),
            TextLine::from(vec![Span::raw("Sexagesimal DNA:").bold().magenta()]),
            TextLine::from(sexagesimal_str),
            TextLine::from(""),
             TextLine::from(vec![Span::raw("L-System Rules:").bold()]),
             TextLine::from(format!("Axiom: {}", plant.lsystem.axiom)),
        ];

        let mut rules_text = Vec::new();
        for (k, v) in &plant.lsystem.rules {
             rules_text.push(TextLine::from(format!("{} -> {}", k, v)));
        }

        let mut full_info = info;
        full_info.extend(rules_text);

        full_info.extend(vec![
            TextLine::from(""),
            TextLine::from(format!("Iterations: {}", self.iterations)),
            TextLine::from(format!("Lines: {}", plant.lines.len())),
            TextLine::from(""),
            TextLine::from("Controls:"),
            TextLine::from(" [Tab]   Next Plant"),
            TextLine::from(" [Space] Grow"),
            TextLine::from(" [Arrow] Pan"),
            TextLine::from(" [+/-]   Zoom"),
            TextLine::from(" [q]     Quit"),
        ]);

        let info_block = Paragraph::new(full_info)
            .block(Block::default().borders(Borders::ALL).title("Cuneiform Chronicle"))
            .wrap(ratatui::widgets::Wrap { trim: true });

        frame.render_widget(info_block, chunks[0]);

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("The Hanging Garden"),
            )
            .x_bounds([
                -100.0 / self.zoom + self.pan_x,
                100.0 / self.zoom + self.pan_x,
            ])
            .y_bounds([
                -100.0 / self.zoom + self.pan_y,
                100.0 / self.zoom + self.pan_y,
            ])
            .paint(|ctx| {
                for line in &plant.lines {
                    ctx.draw(&CanvasLine {
                        x1: line.x1,
                        y1: line.y1,
                        x2: line.x2,
                        y2: line.y2,
                        color: Color::Green,
                    });
                }
            });

        frame.render_widget(canvas, chunks[1]);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new()?;

    app.run(&mut tui.terminal)?;

    Ok(())
}

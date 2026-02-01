#![allow(clippy::collapsible_if)]
use std::io::{self, stdout};
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    layout::{Constraint, Direction, Layout},
    style::{Color, Stylize},
    text::{Line as TextLine, Span},
    widgets::canvas::{Canvas, Line as CanvasLine},
    widgets::{Block, Borders, Paragraph},
};

mod lsystem;
mod turtle;

use lsystem::LSystem;
use turtle::{Line, Turtle};

struct Preset {
    name: String,
    axiom: String,
    rules: Vec<(char, &'static str)>,
    angle: f64,
    start_angle: f64,
}

struct App {
    presets: Vec<Preset>,
    current_preset_index: usize,
    iterations: u32,
    lines: Vec<Line>,

    // Camera
    zoom: f64,
    pan_x: f64,
    pan_y: f64,

    exit: bool,
}

impl App {
    fn new() -> Self {
        let presets = vec![
            Preset {
                name: "Dragon Curve".to_string(),
                axiom: "FX".to_string(),
                rules: vec![('X', "X+YF+"), ('Y', "-FX-Y")],
                angle: 90.0f64.to_radians(),
                start_angle: 0.0,
            },
            Preset {
                name: "Sierpinski Triangle".to_string(),
                axiom: "F-G-G".to_string(),
                rules: vec![('F', "F-G+F+G-F"), ('G', "GG")],
                angle: 120.0f64.to_radians(),
                start_angle: 0.0,
            },
            Preset {
                name: "Fractal Plant".to_string(),
                axiom: "X".to_string(),
                rules: vec![('X', "F+[[X]-X]-F[-FX]+X"), ('F', "FF")],
                angle: 25.0f64.to_radians(),
                start_angle: -90.0f64.to_radians(),
            },
            Preset {
                name: "Koch Curve".to_string(),
                axiom: "F".to_string(),
                rules: vec![('F', "F+F-F-F+F")],
                angle: 90.0f64.to_radians(),
                start_angle: 0.0,
            },
            Preset {
                name: "Gosper Curve".to_string(),
                axiom: "A".to_string(),
                rules: vec![('A', "A-B--B+A++AA+B-"), ('B', "+A-BB--B-A++A+B")],
                angle: 60.0f64.to_radians(),
                start_angle: 0.0,
            },
        ];

        let mut app = Self {
            presets,
            current_preset_index: 0,
            iterations: 4,
            lines: Vec::new(),
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
            exit: false,
        };
        app.regenerate();
        app
    }

    fn regenerate(&mut self) {
        let preset = &self.presets[self.current_preset_index];
        let system = LSystem::new(&preset.axiom, preset.rules.clone());
        let instructions = system.expand(self.iterations);

        // Auto-scale step size based on iterations to keep it roughly visible
        // This is a heuristic
        let step_size = 100.0 / (self.iterations as f64 * 2.0).max(1.0);

        let mut turtle = Turtle::new(0.0, 0.0, preset.start_angle, step_size, preset.angle);
        self.lines = turtle.interpret(&instructions);
    }

    fn next_preset(&mut self) {
        self.current_preset_index = (self.current_preset_index + 1) % self.presets.len();
        self.iterations = 4; // Reset iterations
        self.zoom = 1.0;
        self.pan_x = 0.0;
        self.pan_y = 0.0;
        self.regenerate();
    }

    fn run(&mut self) -> io::Result<()> {
        let mut terminal = Terminal::new(ratatui::backend::CrosstermBackend::new(stdout()))?;

        while !self.exit {
            terminal.draw(|frame| self.ui(frame))?;
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
                        KeyCode::Tab => self.next_preset(),
                        KeyCode::Char(' ') => {
                            self.iterations += 1;
                            if self.iterations > 8 {
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
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(frame.area());

        // Left Panel: Info & Controls
        let preset = &self.presets[self.current_preset_index];
        let rules_text: Vec<TextLine> = preset
            .rules
            .iter()
            .map(|(k, v)| TextLine::from(format!("{} -> {}", k, v)))
            .collect();

        let mut full_info = vec![
            TextLine::from(vec![Span::raw("L-System Garden").bold().green()]),
            TextLine::from(""),
            TextLine::from(vec![
                Span::raw("Preset: "),
                Span::raw(&preset.name).yellow(),
            ]),
            TextLine::from(format!("Axiom: {}", preset.axiom)),
            TextLine::from("Rules:"),
        ];

        full_info.extend(rules_text);
        full_info.extend(vec![
            TextLine::from(""),
            TextLine::from(format!("Iterations: {}", self.iterations)),
            TextLine::from(format!("Lines: {}", self.lines.len())),
            TextLine::from(""),
            TextLine::from("Controls:"),
            TextLine::from(" [Tab]   Next Preset"),
            TextLine::from(" [Space] Iterate +1"),
            TextLine::from(" [r]     Reset Iter"),
            TextLine::from(" [+/-]   Zoom"),
            TextLine::from(" [Arrows] Pan"),
            TextLine::from(" [q/Esc] Quit"),
        ]);

        let info_block = Paragraph::new(full_info)
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        frame.render_widget(info_block, chunks[0]);

        // Right Panel: Canvas
        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Visualization"),
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
                for line in &self.lines {
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

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut app = App::new();
    let res = app.run();

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

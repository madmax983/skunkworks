use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{Block, Borders, Paragraph, canvas::{Canvas, Points}},
    Terminal,
};
use resonance_audio::audio::AudioCommand;
use crossbeam_channel::{Receiver, Sender};
use std::time::Duration;
use std::collections::HashMap;
use crate::parser::CodeEntity;

pub struct App {
    pub grid_width: usize,
    pub grid_height: usize,
    pub listener_x: usize,
    pub listener_y: usize,
    pub snapshot: Vec<f32>,
    pub cmd_tx: Sender<AudioCommand>,
    pub snap_rx: Receiver<Vec<f32>>,
    pub entities: Vec<CodeEntity>,
    pub lines: Vec<String>,
}

impl App {
    pub fn new(
        width: usize,
        height: usize,
        cmd_tx: Sender<AudioCommand>,
        snap_rx: Receiver<Vec<f32>>,
        entities: Vec<CodeEntity>,
        lines: Vec<String>,
    ) -> Self {
        Self {
            grid_width: width,
            grid_height: height,
            listener_x: width / 2,
            listener_y: height / 2,
            snapshot: vec![0.0; width * height],
            cmd_tx,
            snap_rx,
            entities,
            lines,
        }
    }

    pub fn run(mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.run_loop(&mut terminal);

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen
        )?;
        terminal.show_cursor()?;

        res
    }

    fn run_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
        loop {
            // Poll for snapshots
            while let Ok(snap) = self.snap_rx.try_recv() {
                self.snapshot = snap;
            }

            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(f.area());

                // Code View
                // Limit lines to grid height for now
                let view_height = self.grid_height.min(self.lines.len());
                let code_text: String = self.lines.iter().take(view_height).cloned().collect::<Vec<_>>().join("\n");

                let code_block = Paragraph::new(code_text)
                    .block(Block::default().borders(Borders::ALL).title("Source Code"));
                f.render_widget(code_block, chunks[0]);

                // Resonance View Split
                let right_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(0), Constraint::Length(3)])
                    .split(chunks[1]);

                let canvas = Canvas::default()
                    .block(Block::default().borders(Borders::ALL).title("Resonance Grid"))
                    .x_bounds([0.0, self.grid_width as f64])
                    .y_bounds([0.0, self.grid_height as f64])
                    .paint(|ctx| {
                        // Draw energy
                        let mut color_points: HashMap<(u8, u8, u8), Vec<(f64, f64)>> = HashMap::new();

                        for y in 0..self.grid_height {
                            for x in 0..self.grid_width {
                                let idx = y * self.grid_width + x;
                                let val = self.snapshot[idx];
                                if val.abs() > 0.05 {
                                    let (r, g, b) = if val > 0.0 {
                                        (0, (val.min(1.0) * 255.0) as u8, 0)
                                    } else {
                                        (0, 0, ((-val).min(1.0) * 255.0) as u8)
                                    };
                                    // Invert Y because canvas Y goes up
                                    color_points.entry((r, g, b)).or_default().push((x as f64, (self.grid_height - 1 - y) as f64));
                                }
                            }
                        }

                        for ((r, g, b), points) in color_points {
                            ctx.draw(&Points {
                                coords: &points,
                                color: Color::Rgb(r, g, b),
                            });
                        }

                        // Draw Listener
                        ctx.draw(&Points {
                            coords: &[(self.listener_x as f64, (self.grid_height - 1 - self.listener_y) as f64)],
                            color: Color::Yellow,
                        });
                    });

                f.render_widget(canvas, right_chunks[0]);

                // Status Bar
                let mut entity_name = "Global Scope";
                for entity in &self.entities {
                    let start_y = entity.start.line.saturating_sub(1);
                    let end_y = entity.end.line.saturating_sub(1);
                    // Simple check if listener is within vertical range of entity
                    if self.listener_y >= start_y && self.listener_y <= end_y {
                         entity_name = &entity.name;
                    }
                }

                let status = Paragraph::new(format!("Pos: {},{} | Entity: {}", self.listener_x, self.listener_y, entity_name))
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(status, right_chunks[1]);
            })?;

            if event::poll(Duration::from_millis(10))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left => self.listener_x = self.listener_x.saturating_sub(1),
                        KeyCode::Right => self.listener_x = (self.listener_x + 1).min(self.grid_width - 1),
                        KeyCode::Up => self.listener_y = self.listener_y.saturating_sub(1),
                        KeyCode::Down => self.listener_y = (self.listener_y + 1).min(self.grid_height - 1),
                        KeyCode::Char(' ') => {
                            self.cmd_tx.send(AudioCommand::Pluck {
                                x: self.listener_x,
                                y: self.listener_y,
                                strength: 1.0,
                            })?;
                        },
                        KeyCode::Char('w') => {
                             self.cmd_tx.send(AudioCommand::AddWall {
                                x: self.listener_x,
                                y: self.listener_y,
                            })?;
                        },
                         KeyCode::Char('e') => {
                             self.cmd_tx.send(AudioCommand::RemoveWall {
                                x: self.listener_x,
                                y: self.listener_y,
                            })?;
                        },
                        _ => {}
                    }

                    // Update listener
                    self.cmd_tx.send(AudioCommand::MoveListener {
                        x: self.listener_x,
                        y: self.listener_y,
                    })?;
                }
            }
        }
    }
}

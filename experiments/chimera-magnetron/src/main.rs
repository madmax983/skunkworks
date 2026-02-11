use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear},
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;
use rand::Rng;

mod platter;
use platter::Platter;

use chimera_lang::{
    vm::{ChimeraVM, GRID_SIZE, Value},
    ast::{Dna, Helix, Strand, Gene, Nucleotide},
    opcode::OpCode,
};

struct App {
    vm: ChimeraVM,
    platter: Platter,
    head_pos: (usize, usize),
    running: bool,
    last_tick: Instant,
    time_scale: f32,
    mutation_log: Vec<String>,
}

impl App {
    fn new() -> Self {
        // Initialize simple DNA: [ push(10) consume() jump(0) ]
        // A simple "eater" organism.
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
            Gene { op: OpCode::Consume, args: vec![] }, // Eat energy (magnetization)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
            Gene { op: OpCode::GWrite, args: vec![] }, // Write to grid (stress medium)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
        ];

        // Make multiple strands to fill the grid a bit
        let mut strands = Vec::new();
        for _ in 0..4 {
            strands.push(Strand { genes: genes.clone() });
        }

        let dna = Dna { helix: Helix { strands } };
        let mut vm = ChimeraVM::new(dna);

        // Seed grid with some values
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let x = rng.gen_range(0..GRID_SIZE);
            let y = rng.gen_range(0..GRID_SIZE);
            vm.grid[y][x] = Value::Int(rng.gen_range(1..100));
        }

        Self {
            vm,
            platter: Platter::new(GRID_SIZE, GRID_SIZE), // 16x16
            head_pos: (0, 0),
            running: true,
            last_tick: Instant::now(),
            time_scale: 1.0,
            mutation_log: Vec::new(),
        }
    }

    fn run(&mut self, tui: &mut Tui) -> Result<()> {
        while self.running {
            tui.terminal.draw(|f| self.ui(f))?;

            let timeout = Duration::from_millis(16);
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.running = false,
                        KeyCode::Left => {
                            if self.head_pos.0 > 0 { self.head_pos.0 -= 1; }
                        }
                        KeyCode::Right => {
                            if self.head_pos.0 < GRID_SIZE - 1 { self.head_pos.0 += 1; }
                        }
                        KeyCode::Up => {
                            if self.head_pos.1 > 0 { self.head_pos.1 -= 1; }
                        }
                        KeyCode::Down => {
                            if self.head_pos.1 < GRID_SIZE - 1 { self.head_pos.1 += 1; }
                        }
                        KeyCode::Char(' ') => {
                            if let Some(sector) = self.platter.get_sector_mut(self.head_pos.0, self.head_pos.1) {
                                sector.scrub(); // Restore magnetization
                                // Also clear VM memory at this location (Head destroys data/organisms)
                                self.vm.grid[self.head_pos.1][self.head_pos.0] = Value::Int(0);
                                self.mutation_log.push(format!("SCRUBbed sector {},{}", self.head_pos.0, self.head_pos.1));
                            }
                        }
                        KeyCode::Char('+') => self.time_scale *= 1.5,
                        KeyCode::Char('-') => self.time_scale *= 0.75,
                        _ => {}
                    }
                }
            }

            let now = Instant::now();
            let dt = now.duration_since(self.last_tick).as_secs_f32();
            self.last_tick = now;

            // Update Physics
            let mutations = self.platter.update(dt * self.time_scale);

            // Apply mutations to VM
            let mut rng = rand::thread_rng();
            for (x, y) in mutations {
                // Flip a bit / Mutate value
                let val = &mut self.vm.grid[y][x];
                match val {
                    Value::Int(n) => *n ^= 1 << rng.gen_range(0..62),
                    Value::Str(s) => {
                        // Corrupt string
                        if !s.is_empty() {
                            // Replace with random char
                             *val = Value::Str("Glitch".to_string());
                        }
                    },
                    _ => *val = Value::Int(rng.gen()),
                }
                self.mutation_log.push(format!("BIT ROT at {},{}", x, y));
                if self.mutation_log.len() > 10 {
                    self.mutation_log.remove(0);
                }
            }

            // Update VM
            self.vm.step();

            // Interaction: Organisms consume magnetization
            // Scan grid: active cells consume nearby magnetization
            // Use execution_trail or ip location
            let (ip_y, ip_x) = self.vm.context_loc; // or use IP to track active execution
            // Actually, let's look at execution_trail for active areas
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    if self.vm.execution_trail[y * GRID_SIZE + x] > 0 {
                        if let Some(sector) = self.platter.get_sector_mut(x, y) {
                            // Eating magnetization
                            sector.magnetization -= 0.05 * dt * self.time_scale;
                            // Gain energy?
                            self.vm.energy = self.vm.energy.saturating_add(1);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(f.area());

        // Left Panel: The Grid (Platter + VM)
        self.render_grid(f, chunks[0]);

        // Right Panel: Info & Logs
        self.render_info(f, chunks[1]);
    }

    fn render_grid(&self, f: &mut Frame, area: Rect) {
        let block = Block::default().borders(Borders::ALL).title("Platter Map");
        let inner_area = block.inner(area);
        f.render_widget(block, area);

        // We can use a Paragraph with multiple lines to render the grid
        let mut lines = Vec::new();

        for y in 0..GRID_SIZE {
            let mut spans = Vec::new();
            for x in 0..GRID_SIZE {
                let sector = self.platter.get_sector(x, y).unwrap();
                let vm_val = &self.vm.grid[y][x];

                // Background color based on magnetization
                let bg_color = if sector.magnetization > 0.8 {
                    Color::Black
                } else if sector.magnetization > 0.5 {
                    Color::Rgb(50, 50, 0) // Dim Yellow
                } else if sector.magnetization > 0.2 {
                     Color::Rgb(100, 50, 0) // Orange
                } else {
                    Color::Rgb(100, 0, 0) // Red
                };

                // Foreground color based on VM content
                let fg_color = match vm_val {
                    Value::Int(0) => Color::DarkGray,
                    _ => Color::Green,
                };

                // Character to render
                let ch = if x == self.head_pos.0 && y == self.head_pos.1 {
                    "HEAD" // 4 chars wide?
                } else {
                    match vm_val {
                        Value::Int(0) => " .  ",
                        Value::Int(n) => " #  ", // Placeholder
                        Value::Str(s) => " S  ",
                        _ => " ?  ",
                    }
                };

                let ch = if x == self.head_pos.0 && y == self.head_pos.1 {
                     " [H]"
                } else if let Value::Int(n) = vm_val {
                    if *n == 0 { " .  " } else { " #  " }
                } else {
                    " $  "
                };

                // Color overrides
                let final_bg = if x == self.head_pos.0 && y == self.head_pos.1 {
                    Color::Cyan
                } else {
                    bg_color
                };

                spans.push(Span::styled(ch, Style::default().bg(final_bg).fg(fg_color)));
            }
            lines.push(Line::from(spans));
        }

        f.render_widget(Paragraph::new(lines), inner_area);
    }

    fn render_info(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(10), Constraint::Min(0)])
            .split(area);

        // Status
        let status_text = vec![
            Line::from(format!("Time Scale: {:.1}x", self.time_scale)),
            Line::from(format!("Head Pos: {},{}", self.head_pos.0, self.head_pos.1)),
            Line::from(format!("VM Energy: {}", self.vm.energy)),
            Line::from(format!("VM IP: {:?}", self.vm.ip)),
            Line::from(""),
            Line::from("Controls:"),
            Line::from("Arrows: Move Head"),
            Line::from("Space: Scrub (Heal Platter / Kill Agent)"),
            Line::from("+/-: Time Scale"),
            Line::from("Q: Quit"),
        ];

        f.render_widget(
            Paragraph::new(status_text).block(Block::default().borders(Borders::ALL).title("Status")),
            chunks[0],
        );

        // Mutation Log
        let log_lines: Vec<Line> = self.mutation_log.iter().rev()
            .map(|s| Line::from(Span::styled(s, Style::default().fg(Color::Red))))
            .collect();

        f.render_widget(
            Paragraph::new(log_lines).block(Block::default().borders(Borders::ALL).title("Events")),
            chunks[1],
        );
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let res = app.run(&mut tui);
    tui.exit()?;
    res
}

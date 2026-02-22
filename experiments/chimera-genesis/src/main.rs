use anyhow::Result;
use chimera_lang::prelude::*;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{self};
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Cell {
    pub vm: ChimeraVM,
    pub state: bool,
}

pub struct GenesisGrid {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub dna: Dna,
}

impl GenesisGrid {
    pub fn new(width: usize, height: usize, dna: Dna) -> Self {
        let mut cells = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            let vm = ChimeraVM::new(dna.clone());
            // Random initial state (approx 20% alive)
            let alive = rand::random::<f32>() < 0.2;
            cells.push(Cell { vm, state: alive });
        }
        GenesisGrid {
            cells,
            width,
            height,
            dna,
        }
    }

    pub fn update(&mut self) {
        let mut next_states = vec![false; self.width * self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;

                // Collect neighbors first to avoid borrow conflicts
                let neighbors = self.get_neighbors(x, y);

                let cell = &mut self.cells[idx];

                // Stack Setup for DNA Execution
                // DNA expects: [N1..N8, Self, 0, 0] (Top is 0)
                // GWrite pops x(0), y(0), val(Self).

                // 1. Push Neighbors
                for n in neighbors {
                    cell.vm.stack.push(Value::Int(if n { 1 } else { 0 }));
                }

                // 2. Push Self and GWrite args
                let self_val = if cell.state { 1 } else { 0 };
                cell.vm.stack.push(Value::Int(self_val)); // val
                cell.vm.stack.push(Value::Int(0)); // y
                cell.vm.stack.push(Value::Int(0)); // x

                // Step VM
                // 100 steps sufficient for Conway logic
                for _ in 0..100 {
                    cell.vm.step();
                }

                // Pop result
                if let Some(res) = cell.vm.stack.pop() {
                     match res {
                         Value::Int(n) if n > 0 => next_states[idx] = true,
                         _ => next_states[idx] = false,
                     }
                } else {
                    next_states[idx] = false;
                }

                // Reset VM for next tick (Energy, IP, Stack)
                cell.vm.stack.clear();
                cell.vm.energy = 1000;
                cell.vm.ip = (0, 0);
            }
        }

        for (i, state) in next_states.into_iter().enumerate() {
            self.cells[i].state = state;
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<bool> {
        let mut neighbors = Vec::new();
        // Moore Neighborhood (8)
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = (x as isize + dx).rem_euclid(self.width as isize) as usize;
                let ny = (y as isize + dy).rem_euclid(self.height as isize) as usize;
                neighbors.push(self.cells[ny * self.width + nx].state);
            }
        }
        neighbors
    }
}

fn create_conway_dna() -> Dna {
    // Conway's Game of Life implemented in Chimera Assembly
    let genes = vec![
        // 1. Store Self state to VM Grid (0,0)
        // Stack at start: [N1..N8, Self, 0, 0] (Top is 0)
        Gene { op: OpCode::GWrite, args: vec![] },
        // Stack now: [N1..N8] (Top is N8)

        // 2. Sum Neighbors (Consumes 8 values, pushes 1 sum)
        Gene { op: OpCode::Add, args: vec![] },
        Gene { op: OpCode::Add, args: vec![] },
        Gene { op: OpCode::Add, args: vec![] },
        Gene { op: OpCode::Add, args: vec![] },
        Gene { op: OpCode::Add, args: vec![] },
        Gene { op: OpCode::Add, args: vec![] },
        Gene { op: OpCode::Add, args: vec![] }, // Sum (S) on stack

        // 3. Logic: Is3 || (Is2 && Self)
        Gene { op: OpCode::Dup, args: vec![] }, // [S, S]
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(3)] }, // [S, S, 3]
        Gene { op: OpCode::Eq, args: vec![] }, // [S, Is3]
        Gene { op: OpCode::Swap, args: vec![] }, // [Is3, S]
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] }, // [Is3, S, 2]
        Gene { op: OpCode::Eq, args: vec![] }, // [Is3, Is2]

        // Retrieve Self from Grid(0,0)
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // [Is3, Is2, 0]
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // [Is3, Is2, 0, 0]
        Gene { op: OpCode::GRead, args: vec![] }, // [Is3, Is2, Self]

        Gene { op: OpCode::Mul, args: vec![] }, // [Is3, Is2 && Self] (Simulate AND)
        Gene { op: OpCode::Add, args: vec![] }, // [Is3 + (Is2 && Self)] (Simulate OR)

        // Result > 0?
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Gt, args: vec![] }, // > 0 -> 1 (True)
    ];

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize Simulation
    let width = 60;
    let height = 30;
    let dna = create_conway_dna();
    let mut grid = GenesisGrid::new(width, height, dna.clone());

    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    let mut running = true;
    let mut paused = false;

    while running {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(f.area());

            // Header
            let header = Paragraph::new(Line::from(vec![
                Span::styled("Chimera Genesis 🧬", Style::default().fg(Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD)),
                Span::raw(" - Evolutionary Cellular Automata"),
            ]));
            f.render_widget(header, chunks[0]);

            // Grid Rendering
            let mut lines = Vec::new();
            for y in 0..grid.height {
                let mut spans = Vec::new();
                for x in 0..grid.width {
                    let idx = y * grid.width + x;
                    let cell = &grid.cells[idx];
                    let ch = if cell.state { "██" } else { "  " };
                    let color = if cell.state { Color::Green } else { Color::DarkGray };
                    spans.push(Span::styled(ch, Style::default().fg(color)));
                }
                lines.push(Line::from(spans));
            }

            let grid_block = Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL).title("Petri Dish"));
            f.render_widget(grid_block, chunks[1]);

            // Footer
            let status = if paused { "PAUSED" } else { "RUNNING" };
            let footer = Paragraph::new(format!("Q: Quit | R: Reset | Space: Pause | Status: {}", status));
            f.render_widget(footer, chunks[2]);
        })?;

        // Input Handling
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => running = false,
                        KeyCode::Char('r') => {
                            grid = GenesisGrid::new(width, height, dna.clone());
                        }
                        KeyCode::Char(' ') => {
                            paused = !paused;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Simulation Step
        if !paused && last_tick.elapsed() >= tick_rate {
            grid.update();
            last_tick = Instant::now();
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oscillator() {
        let dna = create_conway_dna();
        let mut grid = GenesisGrid::new(5, 5, dna);

        // Clear grid
        for cell in &mut grid.cells {
            cell.state = false;
        }

        // Blinker pattern (Vertical line of 3) at center column
        let center = 2 * 5 + 2; // (2,2)
        grid.cells[center].state = true;
        grid.cells[center - 5].state = true; // (1,2)
        grid.cells[center + 5].state = true; // (3,2)

        // Update
        grid.update();

        assert!(grid.cells[center].state, "Center should survive");
        assert!(grid.cells[center - 1].state, "Left neighbor should be born");
        assert!(grid.cells[center + 1].state, "Right neighbor should be born");
        assert!(!grid.cells[center - 5].state, "Top should die");
        assert!(!grid.cells[center + 5].state, "Bottom should die");
    }
}

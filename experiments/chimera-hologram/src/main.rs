use anyhow::Result;
use chimera_lang::prelude::*;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, time::Duration};

mod hologram;
use hologram::HolographicMemory;

struct Agent {
    vm: ChimeraVM,
    x: usize,
    y: usize,
}

impl Agent {
    fn new(dna: Dna, x: usize, y: usize) -> Self {
        let mut vm = ChimeraVM::new(dna);
        // Seed the stack with an initial direction value
        vm.stack.push(Value::Int(0));
        Self {
            vm,
            x,
            y,
        }
    }

    fn update(&mut self, width: usize, height: usize) {
        // Step the VM
        self.vm.step();

        // Use top of stack as direction driver
        let dir_val = self.vm.stack.last().and_then(|v| match v {
            Value::Int(i) => Some(*i),
            _ => None,
        }).unwrap_or(0);

        // Map integer to direction (0=Up, 1=Right, 2=Down, 3=Left)
        // But modulate with some randomness
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let dx = match dir_val % 4 {
            1 => 1,
            3 => -1,
            _ => 0,
        };

        let dy = match dir_val % 4 {
            0 => -1,
            2 => 1,
            _ => 0,
        };

        // 20% chance of random brownian motion overriding DNA
        let (dx, dy) = if rng.gen_bool(0.2) {
             (rng.gen_range(-1..=1), rng.gen_range(-1..=1))
        } else {
            (dx, dy)
        };

        self.x = (self.x as isize + dx).rem_euclid(width as isize) as usize;
        self.y = (self.y as isize + dy).rem_euclid(height as isize) as usize;
    }
}

fn generate_dna() -> Dna {
    // A simple program: Loop incrementing the value on the stack.
    // [ Push(1), Add, Jump(0) ]
    // Assumes stack has an initial integer (seeded in Agent::new).
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::Add, args: vec![] }, // Consumes 1 and StackTop, pushes Result
        Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] }, // Jump to start of strand 0
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

    // Setup Simulation
    // Use a smaller grid for TUI visibility
    let width = 64;
    let height = 32;
    let mut hologram = HolographicMemory::new(width, height);

    let mut agents = Vec::new();
    for _ in 0..15 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        agents.push(Agent::new(
            generate_dna(),
            rng.gen_range(0..width),
            rng.gen_range(0..height),
        ));
    }

    let res = run_app(&mut terminal, &mut hologram, &mut agents, width, height);

    // Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    hologram: &mut HolographicMemory,
    agents: &mut Vec<Agent>,
    width: usize,
    height: usize,
) -> Result<()>
where
    B::Error: std::marker::Send + std::marker::Sync + 'static,
{
    let mut last_tick = std::time::Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3), // Title
                        Constraint::Min(10), // Hologram
                        Constraint::Length(3), // Info
                    ]
                    .as_ref(),
                )
                .split(f.area());

            // Title
            let title = Paragraph::new(Span::styled(
                "🧬 Chimera-Hologram: Distributed Existence",
                Style::default().fg(Color::Cyan),
            ))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            // Hologram Render
            let reconstruction = hologram.reconstruct();

            // Map reconstruction (float) to ascii characters
            // Use String buffer for performance
            let mut s = String::with_capacity(height * (width + 1));

            for y in 0..height {
                for x in 0..width {
                    // Check if agent is here (Overlay)
                    let agent_here = agents.iter().any(|a| a.x == x && a.y == y);

                    if agent_here {
                        s.push('🧬');
                    } else {
                        let idx = y * width + x;
                        let val = if idx < reconstruction.len() { reconstruction[idx] } else { 0.0 };

                        let c = if val > 0.8 { '█' }
                        else if val > 0.6 { '▓' }
                        else if val > 0.4 { '▒' }
                        else if val > 0.2 { '░' }
                        else if val > 0.05 { '.' }
                        else { ' ' };
                        s.push(c);
                    }
                }
                s.push('\n');
            }

            let holo_view = Paragraph::new(s)
                .block(Block::default().title("Reconstructed Field").borders(Borders::ALL));
            f.render_widget(holo_view, chunks[1]);

            // Info
            let total_energy: i64 = agents.iter().map(|a| a.vm.energy).sum();
            let info = Paragraph::new(format!("Agents: {} | Total Bio-Energy: {}", agents.len(), total_energy))
                 .block(Block::default().borders(Borders::ALL));
            f.render_widget(info, chunks[2]);

        })?;

        // Input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        // Update at fixed rate
        if last_tick.elapsed() >= tick_rate {
            last_tick = std::time::Instant::now();

            hologram.decay(0.90);

            // Spatial grid for recording
            let mut object_grid = vec![0.0; width * height];

            for agent in agents.iter_mut() {
                agent.update(width, height);

                // Agent emits light into the hologram
                let idx = agent.y * width + agent.x;
                if idx < object_grid.len() {
                    // Intensity based on energy
                    let intensity = (agent.vm.energy as f64 / 50.0).clamp(0.1, 5.0);
                    object_grid[idx] += intensity;
                }
            }

            hologram.record(&object_grid, 1.0);
        }
    }
}

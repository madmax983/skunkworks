mod agent;
mod physics;
mod platter;
mod universe;

use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Terminal,
};

use agent::Agent;
use chimera_lang::{
    ast::{Dna, Gene, Helix, Nucleotide, Strand},
    opcode::OpCode,
};
use physics::Vec2;
use rand::Rng;
use universe::Universe;

fn random_dna() -> Dna {
    let mut rng = rand::thread_rng();
    let mut genes = Vec::new();

    // Create a random strand of 20 genes
    for _ in 0..20 {
        let op_choice = rng.gen_range(0..10);
        let (op, args) = match op_choice {
            0 => (
                OpCode::Push,
                vec![Nucleotide::Number(rng.gen_range(0..100))],
            ),
            1 => (OpCode::Add, vec![]),
            2 => (OpCode::Sub, vec![]),
            3 => (OpCode::Mul, vec![]),
            4 => (OpCode::Div, vec![]),
            5 => (OpCode::Dup, vec![]),
            6 => (OpCode::Jump, vec![Nucleotide::Number(rng.gen_range(0..5))]), // Small jump
            7 => (OpCode::Brz, vec![Nucleotide::Number(rng.gen_range(0..5))]),
            8 => (OpCode::Consume, vec![]), // Gain energy
            _ => (OpCode::Nop, vec![]),
        };
        genes.push(Gene { op, args });
    }

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

fn main() -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Logic
    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> anyhow::Result<()>
where
    <B as Backend>::Error: Send + Sync + 'static,
{
    // 1. Universe
    // Width/Height 200 matches the bounds [-100, 100] offset by 100.
    let mut universe = Universe::new(200, 200);

    // 2. Spawn Agents
    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        let x = rng.gen_range(-80.0..80.0);
        let y = rng.gen_range(-80.0..80.0);
        let dna = random_dna();
        let agent = Agent::new(dna, x, y);
        universe.add_agent(agent);
    }

    // 3. Loop
    let mut zoom = 1.0;
    let mut pan = Vec2::ZERO;
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(size);

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(format!(
                    "Ferrous Chimera | Agents: {} | Magnetic Stigmergy Active",
                    universe.agents.len()
                )))
                .x_bounds([
                    pan.x as f64 - 100.0 * zoom as f64,
                    pan.x as f64 + 100.0 * zoom as f64,
                ])
                .y_bounds([
                    pan.y as f64 - 100.0 * zoom as f64,
                    pan.y as f64 + 100.0 * zoom as f64,
                ])
                .paint(|ctx| {
                    // Draw Platter (Magnetism)
                    let step = 4; // Faster render
                    for y in (0..universe.platter.height).step_by(step) {
                        for x in (0..universe.platter.width).step_by(step) {
                            let mag = universe.platter.get_magnetism(x, y);
                            if mag > 0.1 {
                                let px = x as f64 - 100.0;
                                let py = y as f64 - 100.0;

                                let color = match mag {
                                    m if m > 0.8 => Color::Red,
                                    m if m > 0.5 => Color::Magenta,
                                    m if m > 0.2 => Color::Blue,
                                    _ => Color::DarkGray,
                                };
                                ctx.print(px, py, Span::styled("·", Style::default().fg(color)));
                            }
                        }
                    }

                    // Agents
                    for agent in &universe.agents {
                        // Trail
                        for i in 0..agent.body.trail.len().saturating_sub(1) {
                            ctx.draw(&Line {
                                x1: agent.body.trail[i].x as f64,
                                y1: agent.body.trail[i].y as f64,
                                x2: agent.body.trail[i + 1].x as f64,
                                y2: agent.body.trail[i + 1].y as f64,
                                color: Color::Gray,
                            });
                        }

                        // Body
                        let symbol = if agent.vm.halted { "X" } else { "O" };
                        ctx.print(
                            agent.body.pos.x as f64,
                            agent.body.pos.y as f64,
                            Span::styled(symbol, Style::default().fg(agent.body.color)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let controls =
                Paragraph::new("Controls: [Q] Quit | [+/-] Zoom | [Arrows] Pan | [R] Reset")
                    .style(Style::default().fg(Color::White).bg(Color::DarkGray));
            f.render_widget(controls, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => {
                        zoom = 1.0;
                        pan = Vec2::ZERO;
                    }
                    KeyCode::Char('+') => zoom *= 0.9,
                    KeyCode::Char('-') => zoom *= 1.1,
                    KeyCode::Up => pan.y += 10.0 * zoom,
                    KeyCode::Down => pan.y -= 10.0 * zoom,
                    KeyCode::Left => pan.x -= 10.0 * zoom,
                    KeyCode::Right => pan.x += 10.0 * zoom,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            universe.step(0.05);
            last_tick = Instant::now();
        }
    }
}

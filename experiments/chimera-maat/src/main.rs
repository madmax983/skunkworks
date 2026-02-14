use anyhow::Result;
use chimera_lang::prelude::*;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use maat_engine::{ScalesOfMaat, Soul, to_hieroglyphs};
use num_bigint::BigUint;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Gauge},
    Terminal,
};
use std::io;
use std::time::{Duration, Instant};
use rand::Rng;

struct Agent {
    vm: ChimeraVM,
    id: u64,
    last_demand: String,
    last_status: String,
    allocated_ticks: usize,
    color: Color,
}

struct App {
    scales: ScalesOfMaat,
    agents: Vec<Agent>,
    tick_count: u64,
}

impl App {
    fn new() -> Self {
        Self {
            scales: ScalesOfMaat::new(100), // 100 blocks of time
            agents: Vec::new(),
            tick_count: 0,
        }
    }

    fn spawn_agent(&mut self, id: u64) {
        // DNA that pushes two numbers then loops
        // E.g., Push(1), Push(3), Jump(0) -> Demand 1/3
        let mut rng = rand::thread_rng();
        let numer = rng.gen_range(1..5);
        let denom = rng.gen_range(numer+1..20); // Proper fraction usually

        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(denom)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(numer)] },
            // Add some noise instructions to make it interesting
            Gene { op: OpCode::Nop, args: vec![] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
        ];

        let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
        let mut vm = ChimeraVM::new(dna);

        // Initial kickstart
        vm.step(); // execute push
        vm.step(); // execute push

        let color = Color::Rgb(rng.gen(), rng.gen(), rng.gen());

        self.agents.push(Agent {
            vm,
            id,
            last_demand: "Waiting...".to_string(),
            last_status: "Born".to_string(),
            allocated_ticks: 0,
            color,
        });
    }

    fn update(&mut self) {
        self.tick_count += 1;

        // Reset scales for this tick
        self.scales.timeline = vec![None; self.scales.capacity];

        // Process Agents
        let mut dead_indices = Vec::new();

        for (idx, agent) in self.agents.iter_mut().enumerate() {
            // 1. Determine Demand
            // Peek stack. We need top 2 values.
            // Stack is LIFO. Last pushed is Top (Numerator). Before that is Denominator.
            // Agent code: Push(Denom), Push(Num). Stack: [Denom, Num].
            // So pop() gives Num, pop() gives Denom.

            // We simulate "peeking" or consuming. Let's consume.
            // If stack empty, random demand.

            let (numer, denom) = if agent.vm.stack.len() >= 2 {
                let n_val = agent.vm.stack.pop().unwrap_or(Value::Int(1));
                let d_val = agent.vm.stack.pop().unwrap_or(Value::Int(10));

                let n = match n_val { Value::Int(i) => i.abs() as u64, _ => 1 };
                let d = match d_val { Value::Int(i) => i.abs() as u64, _ => 10 };
                (n, d)
            } else {
                let mut rng = rand::thread_rng();
                (rng.gen_range(1..3), rng.gen_range(3..10))
            };

            let safe_denom = if denom == 0 { 1 } else { denom };

            // 2. Weigh Heart
            let soul = Soul::new(agent.id, numer, safe_denom);
            agent.last_demand = format!("{}/{} ({})", numer, safe_denom, to_hieroglyphs(&BigUint::from(safe_denom))); // Simplified visual

            match self.scales.weigh_heart(&soul) {
                Ok(allocations) => {
                    // Success
                    let total_blocks: usize = allocations.iter().map(|(_, size)| size).sum();
                    agent.allocated_ticks = total_blocks; // 1 block = 1 tick execution?
                    agent.last_status = format!("GRANTED: {} blocks", total_blocks);

                    // Run VM
                    for _ in 0..total_blocks {
                        agent.vm.step();
                    }

                    // Reward energy for successful allocation?
                    agent.vm.energy += 1;
                }
                Err(msg) => {
                    // Failure
                    agent.allocated_ticks = 0;
                    agent.last_status = format!("DENIED: {}", msg);
                    agent.vm.energy -= 5; // Punishment

                    // Mutate out of desperation
                    agent.vm.mutate();
                }
            }

            // Check Life
            if agent.vm.energy <= 0 || agent.vm.halted {
                dead_indices.push(idx);
            }
        }

        // Reincarnation
        // Remove dead, spawn new
        // We do this in reverse to keep indices valid
        for idx in dead_indices.iter().rev() {
            self.agents.remove(*idx);
            self.spawn_agent(self.tick_count * 100 + *idx as u64);
        }

        // Maintain population
        while self.agents.len() < 8 {
            self.spawn_agent(self.tick_count * 100 + self.agents.len() as u64);
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    // Initial spawn
    for i in 0..8 {
        app.spawn_agent(i);
    }

    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(5), // Scales
            Constraint::Min(0),    // Agents
        ].as_ref())
        .split(f.area());

    // Title
    let title = Paragraph::new("CHIMERA x MAAT: Bureaucratic Survival")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Scales Timeline
    // We visualize the 100 slots.
    // If slot is Some(id), we check agent color.
    let mut spans = Vec::new();
    let mut last_id = None;
    let mut run_len = 0;

    for slot in &app.scales.timeline {
        if *slot == last_id {
            run_len += 1;
        } else {
            // Flush previous run
            if run_len > 0 {
                let color = if let Some(id) = last_id {
                    if let Some(agent) = app.agents.iter().find(|a| a.id == id) {
                        agent.color
                    } else {
                        Color::Gray
                    }
                } else {
                    Color::DarkGray
                };

                let s = "█".repeat(run_len);
                spans.push(Span::styled(s, Style::default().fg(color)));
            }
            last_id = *slot;
            run_len = 1;
        }
    }
    // Flush last
    if run_len > 0 {
        let color = if let Some(id) = last_id {
            if let Some(agent) = app.agents.iter().find(|a| a.id == id) {
                agent.color
            } else {
                Color::Gray
            }
        } else {
            Color::DarkGray
        };
        let s = "█".repeat(run_len);
        spans.push(Span::styled(s, Style::default().fg(color)));
    }

    let timeline = Paragraph::new(Line::from(spans))
        .block(Block::default().borders(Borders::ALL).title("The Scales of Maat (Timeline Allocation)"));
    f.render_widget(timeline, chunks[1]);

    // Agents List
    let items: Vec<ListItem> = app.agents.iter().map(|agent| {
        let demand_style = if agent.allocated_ticks > 0 {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Red)
        };

        let content = format!(
            "ID {:<4} | Energy {:<3} | Demand: {:<20} | Status: {}",
            agent.id % 1000,
            agent.vm.energy,
            agent.last_demand,
            agent.last_status
        );

        ListItem::new(Span::styled(content, Style::default().fg(agent.color)))
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Petitioners"));
    f.render_widget(list, chunks[2]);
}

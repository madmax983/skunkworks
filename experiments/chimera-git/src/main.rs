use anyhow::Result;
use chimera_lang::{
    ast::{Dna, Gene, Nucleotide},
    opcode::OpCode,
    prelude::ChimeraVM,
};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use git_associates::GitModel;
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct Agent {
    vm: ChimeraVM,
    x: f64,
    y: f64,
    target_commit: usize,
    energy: f64,
}

impl Agent {
    fn new(x: f64, y: f64, target_commit: usize) -> Self {
        let genes = vec![
            Gene::new(OpCode::Push, vec![Nucleotide::Number(1)]),
            Gene::new(OpCode::Add, vec![]),
        ];

        let dna = Dna::from_genes(genes);

        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        Self {
            vm,
            x,
            y,
            target_commit,
            energy: 100.0,
        }
    }

    fn step(&mut self, commits: &[git_associates::Commit], width: f64, height: f64) {
        if self.energy <= 0.0 {
            return;
        }

        self.vm.step();

        if self.target_commit < commits.len() {
            let commit = &commits[self.target_commit];
            let msg_len = commit.message.len() as f64;

            let hash_bytes = commit.hash.as_bytes();
            let target_x = (hash_bytes[0] as f64 / 255.0) * width;
            let target_y = (hash_bytes[1] as f64 / 255.0) * height;

            let dx = target_x - self.x;
            let dy = target_y - self.y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 5.0 {
                self.energy += msg_len * 0.1;
                self.target_commit += 1;
            } else {
                let speed = 2.0;
                self.x += (dx / dist) * speed;
                self.y += (dy / dist) * speed;
                self.energy -= 0.1;
            }
        }
    }
}

struct App {
    commits: Vec<git_associates::Commit>,
    agents: Vec<Agent>,
}

impl App {
    fn new() -> Result<Self> {
        let model = GitModel::open(".")?;
        let history = model.history(50)?;

        let mut agents = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            agents.push(Agent::new(
                rng.gen_range(0.0..100.0),
                rng.gen_range(0.0..100.0),
                0,
            ));
        }

        Ok(Self {
            commits: history,
            agents,
        })
    }

    fn step(&mut self) {
        let width = 100.0;
        let height = 100.0;
        for agent in &mut self.agents {
            agent.step(&self.commits, width, height);
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new()?;

    let tick_rate = Duration::from_millis(32);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && (key.code == KeyCode::Char('q') || key.code == KeyCode::Esc)
                {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.step();
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Chimera Git Foraging "),
        )
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(|ctx| {
            for commit in &app.commits {
                let hash_bytes = commit.hash.as_bytes();
                let x = (hash_bytes[0] as f64 / 255.0) * 100.0;
                let y = (hash_bytes[1] as f64 / 255.0) * 100.0;
                ctx.print(x, y, "C");
            }

            for agent in &app.agents {
                if agent.energy > 0.0 {
                    ctx.print(agent.x, agent.y, "A");
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    let info = Paragraph::new(vec![
        Line::from(format!("Total Commits: {}", app.commits.len())),
        Line::from(format!(
            "Active Agents: {}",
            app.agents.iter().filter(|a| a.energy > 0.0).count()
        )),
        Line::from(""),
        Line::from("Agents forage through commit history,"),
        Line::from("consuming diffs to gain energy."),
    ])
    .block(Block::default().borders(Borders::ALL).title(" Info "));

    f.render_widget(info, chunks[1]);
}

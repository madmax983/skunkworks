mod model;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use market_sim::Particle;
use model::{ProcessState, Simulation};
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Circle},
        Block, Borders, Cell, List, ListItem, Row, Table,
    },
};
use std::{io, time::Duration};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create Simulation
    // 4 Cores, 30 Processes
    let mut sim = Simulation::new(4, 30);

    let res = run_app(&mut terminal, &mut sim);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, sim: &mut Simulation) -> Result<()>
where
    B::Error: std::error::Error + Send + Sync + 'static,
{
    loop {
        terminal.draw(|f| ui(f, sim))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                }
            }
        }

        sim.update();
    }
}

fn ui(f: &mut Frame, sim: &Simulation) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(f.area());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(chunks[0]);

    // --- Process List ---
    let header = Row::new(vec!["PID", "State", "Wealth", "Progress"]);
    let rows: Vec<Row> = sim.processes.iter().map(|p| {
        let state_str = match p.state {
            ProcessState::Ready => "Ready",
            ProcessState::Running(c) => return Row::new(vec![
                Cell::from(p.id.to_string()),
                Cell::from(format!("Run({})", c)).style(Style::default().fg(Color::Green)),
                Cell::from(format!("{:.1}", p.wealth)),
                Cell::from(format!("{}/{}", p.work_done, p.work_total)),
            ]),
            ProcessState::Blocked => "Blocked",
            ProcessState::Finished => "Done",
        };

        let style = if p.state == ProcessState::Finished {
            Style::default().fg(Color::DarkGray)
        } else if p.wealth < 10.0 {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        };

        Row::new(vec![
            Cell::from(p.id.to_string()),
            Cell::from(state_str),
            Cell::from(format!("{:.1}", p.wealth)),
            Cell::from(format!("{}/{}", p.work_done, p.work_total)),
        ]).style(style)
    }).collect();

    let table = Table::new(rows, [Constraint::Length(5), Constraint::Length(10), Constraint::Length(10), Constraint::Length(10)])
        .header(header)
        .block(Block::default().title("Processes").borders(Borders::ALL));
    f.render_widget(table, top_chunks[0]);

    // --- CPU Cores ---
    let items: Vec<ListItem> = sim.cores.iter().map(|c| {
        let content = if let Some(pid) = c.current_process {
            format!("Core {}: PID {} ({} ticks)", c.id, pid, c.cycles_remaining)
        } else {
            format!("Core {}: IDLE", c.id)
        };
        ListItem::new(content)
    }).collect();

    let list = List::new(items)
        .block(Block::default().title("CPU Cores").borders(Borders::ALL));
    f.render_widget(list, top_chunks[1]);

    // --- Market Visualization ---
    let canvas = Canvas::default()
        .block(Block::default().title("Market (Double Auction)").borders(Borders::ALL))
        .x_bounds([0.0, sim.market.width as f64])
        .y_bounds([0.0, sim.market.height as f64])
        .paint(|ctx| {
            // Background grid lines? Maybe later.

            // Draw particles
            for y in 0..sim.market.height {
                for x in 0..sim.market.width {
                    match sim.market.get(x, y) {
                        Particle::Bid(pid) => {
                            let color = if pid < sim.processes.len() {
                                let (r, g, b) = sim.processes[pid].color;
                                Color::Rgb(r, g, b)
                            } else {
                                Color::Green
                            };
                            let display_y = (sim.market.height - 1 - y) as f64;
                            ctx.draw(&Circle {
                                x: x as f64 + 0.5,
                                y: display_y,
                                radius: 0.4,
                                color,
                            });
                        }
                        Particle::Ask(_) => {
                            // Red (Moving Down)
                            let display_y = (sim.market.height - 1 - y) as f64;
                            ctx.draw(&Circle {
                                x: x as f64 + 0.5,
                                y: display_y,
                                radius: 0.4,
                                color: Color::Red,
                            });
                        }
                        Particle::Trade { age } => {
                            // Explosion
                            let display_y = (sim.market.height - 1 - y) as f64;
                             ctx.draw(&Circle {
                                x: x as f64 + 0.5,
                                y: display_y,
                                radius: 0.6 + (age as f64 * 0.1),
                                color: Color::Yellow,
                            });
                        }
                        Particle::Empty => {}
                    }
                }
            }
        });

    f.render_widget(canvas, chunks[1]);
}

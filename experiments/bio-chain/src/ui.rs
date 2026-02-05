use crate::network::Network;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, List, ListItem, Paragraph},
};
use std::io;

pub struct App {
    pub network: Network,
    pub tick: usize,
    pub paused: bool,
    pub history: Vec<(usize, usize, usize)>, // (tick, honest_count, malicious_count)
    pub energy_history: Vec<(f64, f64)>,     // (tick, avg_energy)
}

impl App {
    pub fn new(network: Network) -> Self {
        App {
            network,
            tick: 0,
            paused: false,
            history: vec![],
            energy_history: vec![],
        }
    }

    pub fn update(&mut self) {
        if !self.paused {
            self.network.step_silent();
            self.tick = self.network.tick as usize;

            // Record history
            let honest = self
                .network
                .validators
                .iter()
                .filter(|v| !v.is_malicious)
                .count();
            let malicious = self
                .network
                .validators
                .iter()
                .filter(|v| v.is_malicious)
                .count();
            self.history.push((self.tick, honest, malicious));

            // Keep last 100 points
            if self.history.len() > 100 {
                self.history.remove(0);
            }

            // Energy history
            if !self.network.validators.is_empty() {
                let avg_energy =
                    self.network.total_stake() as f64 / self.network.validators.len() as f64;
                self.energy_history.push((self.tick as f64, avg_energy));

                if self.energy_history.len() > 100 {
                    self.energy_history.remove(0);
                }
            }
        }
    }
}

pub fn run_ui(mut app: App) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui(f, &app))?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => app.paused = !app.paused,
                    _ => {}
                }
            }
        }

        // Update
        app.update();

        // Stop if extinct
        if app.network.validators.is_empty() {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(f.area());

    // Title
    render_title(f, chunks[0], app);

    // Main stats + population chart
    let middle = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[1]);

    render_stats(f, middle[0], app);
    render_population_chart(f, middle[1], app);

    // Validator list + energy chart
    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    render_validators(f, bottom[0], app);
    render_energy_chart(f, bottom[1], app);

    // Controls
    render_controls(f, chunks[3], app);
}

fn render_title(f: &mut Frame, area: Rect, app: &App) {
    let status = if app.paused {
        "⏸ PAUSED"
    } else {
        "▶ RUNNING"
    };
    let title = Paragraph::new(format!(
        "🧬 BioCoin: Living Blockchain | Tick {} | {}",
        app.tick, status
    ))
    .style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
}

fn render_stats(f: &mut Frame, area: Rect, app: &App) {
    let honest = app
        .network
        .validators
        .iter()
        .filter(|v| !v.is_malicious)
        .count();
    let malicious = app
        .network
        .validators
        .iter()
        .filter(|v| v.is_malicious)
        .count();
    let total = app.network.validators.len();

    let items = vec![
        ListItem::new(format!("🟢 Honest: {} validators", honest))
            .style(Style::default().fg(Color::Green)),
        ListItem::new(format!("🔴 Malicious: {} validators", malicious))
            .style(Style::default().fg(Color::Red)),
        ListItem::new(format!("Total Population: {}", total)),
        ListItem::new(""),
        ListItem::new(format!(
            "⛓️  Blocks Finalized: {}",
            app.network.finalized_chain.len()
        )),
        ListItem::new(format!(
            "📦 Pending Blocks: {}",
            app.network.pending_blocks.len()
        )),
        ListItem::new(""),
        ListItem::new(format!("🧬 Births: {}", app.network.births)),
        ListItem::new(format!("💀 Deaths: {}", app.network.deaths)),
        ListItem::new(""),
        ListItem::new(format!("💰 Total Stake: {}", app.network.total_stake())),
        ListItem::new(format!(
            "🎯 Consensus Threshold: {}",
            (app.network.total_stake() as f64 * 0.67) as i64
        )),
    ];

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("📊 Statistics"),
    );
    f.render_widget(list, area);
}

fn render_population_chart(f: &mut Frame, area: Rect, app: &App) {
    if app.history.is_empty() {
        return;
    }

    let honest_data: Vec<(f64, f64)> = app
        .history
        .iter()
        .map(|(t, h, _)| (*t as f64, *h as f64))
        .collect();

    let malicious_data: Vec<(f64, f64)> = app
        .history
        .iter()
        .map(|(t, _, m)| (*t as f64, *m as f64))
        .collect();

    let max_pop = app
        .history
        .iter()
        .map(|(_, h, m)| h + m)
        .max()
        .unwrap_or(10) as f64;

    let datasets = vec![
        Dataset::default()
            .name("Honest")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Green))
            .data(&honest_data),
        Dataset::default()
            .name("Malicious")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Red))
            .data(&malicious_data),
    ];

    let min_tick = app.history.first().map(|(t, _, _)| *t).unwrap_or(0) as f64;
    let max_tick = app.history.last().map(|(t, _, _)| *t).unwrap_or(100) as f64;

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("📈 Population Evolution"),
        )
        .x_axis(
            Axis::default()
                .title("Tick")
                .bounds([min_tick, max_tick])
                .labels(vec![
                    Span::from(format!("{}", min_tick as usize)),
                    Span::from(format!("{}", max_tick as usize)),
                ]),
        )
        .y_axis(
            Axis::default()
                .title("Count")
                .bounds([0.0, max_pop + 1.0])
                .labels(vec![
                    Span::from("0"),
                    Span::from(format!("{}", (max_pop / 2.0) as usize)),
                    Span::from(format!("{}", max_pop as usize)),
                ]),
        );

    f.render_widget(chart, area);
}

fn render_validators(f: &mut Frame, area: Rect, app: &App) {
    let item_count = app.network.validators.len().min(15);
    let items: Vec<ListItem> = app
        .network
        .validators
        .iter()
        .take(15)
        .map(|v| {
            let marker = if v.is_malicious { "🔴" } else { "🟢" };
            let energy_bar = "█".repeat((v.energy / 100).min(10) as usize);
            ListItem::new(format!("{} V{}: {} {}", marker, v.id, energy_bar, v.energy)).style(
                if v.is_malicious {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default().fg(Color::Green)
                },
            )
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(format!(
        "🧬 Validators (showing {}/{})",
        item_count,
        app.network.validators.len()
    )));
    f.render_widget(list, area);
}

fn render_energy_chart(f: &mut Frame, area: Rect, app: &App) {
    if app.energy_history.is_empty() {
        return;
    }

    let datasets = vec![
        Dataset::default()
            .name("Avg Energy")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .data(&app.energy_history),
    ];

    let min_tick = app.energy_history.first().map(|(t, _)| *t).unwrap_or(0.0);
    let max_tick = app.energy_history.last().map(|(t, _)| *t).unwrap_or(100.0);
    let max_energy = app
        .energy_history
        .iter()
        .map(|(_, e)| *e)
        .fold(0.0, f64::max);

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("⚡ Average Energy"),
        )
        .x_axis(
            Axis::default()
                .title("Tick")
                .bounds([min_tick, max_tick])
                .labels(vec![
                    Span::from(format!("{}", min_tick as usize)),
                    Span::from(format!("{}", max_tick as usize)),
                ]),
        )
        .y_axis(
            Axis::default()
                .title("Energy")
                .bounds([0.0, max_energy + 100.0])
                .labels(vec![
                    Span::from("0"),
                    Span::from(format!("{}", max_energy as usize)),
                ]),
        );

    f.render_widget(chart, area);
}

fn render_controls(f: &mut Frame, area: Rect, app: &App) {
    let controls = Paragraph::new("Controls: [SPACE] Pause/Resume | [Q] Quit")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(controls, area);
}

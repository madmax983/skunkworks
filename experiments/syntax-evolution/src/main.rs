mod lang;
mod sim;

use crate::lang::{Meaning, WordOrder};
use crate::sim::{Agent, erode, Sentence};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    symbols,
    text::{Line, Span},
    widgets::{
        Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Row, Table, Wrap,
    },
    Frame, Terminal,
};
use std::{io, time::{Duration, Instant}};
use rand::Rng;

struct App {
    agents: Vec<Agent>,
    logs: Vec<LogEntry>,
    epoch: usize,
    stats_history: Vec<(f64, f64, f64)>, // (epoch, avg_morphology, avg_rigidity)
    paused: bool,
}

struct LogEntry {
    speaker: usize,
    listener: usize,
    original: String,
    eroded: String,
    outcome: String,
    success: bool,
}

impl App {
    fn new() -> Self {
        let mut agents = Vec::new();
        for _ in 0..2 {
            agents.push(Agent::new());
        }
        Self {
            agents,
            logs: Vec::new(),
            epoch: 0,
            stats_history: Vec::new(),
            paused: false,
        }
    }

    fn on_tick(&mut self) {
        if self.paused { return; }

        self.epoch += 1;
        let mut rng = rand::thread_rng();

        // 1. Pick Speaker and Listener
        let speaker_idx = rng.gen_range(0..self.agents.len());
        let listener_idx = (speaker_idx + 1) % self.agents.len();

        // 2. Formulate Meaning (Random concept)
        let actors = ["puer", "canis", "rex", "miles", "nauta"];
        let targets = ["puella", "domus", "gladius", "urbs", "cibus"];
        let actions = ["amat", "videt", "habet", "necat", "facit"];

        let meaning = Meaning::new(
            actors[rng.gen_range(0..actors.len())],
            actions[rng.gen_range(0..actions.len())],
            targets[rng.gen_range(0..targets.len())],
        );

        // 3. Speak & Erode
        let original_sentence = self.agents[speaker_idx].speak(&meaning);
        let eroded_sentence = erode(&original_sentence);

        // 4. Comprehend
        let result = self.agents[listener_idx].comprehend(&eroded_sentence);

        let success = match &result {
            Ok(m) => m == &meaning,
            Err(_) => false,
        };

        // 5. Learn
        // Both learn? Or just listener?
        // In this simulation, if communication fails, the listener might adjust.
        // Also the speaker might adjust if they realize they weren't understood (not implemented yet, assuming feedback).
        // Let's assume shared feedback.
        self.agents[listener_idx].learn(success);

        // Mutate Speaker: If success was high, maybe get lazy (reduce morphology)?
        // If success low, reinforce order?
        // Let's implement a simple drift:
        // Every tick, small chance to erode morphology strength naturally (laziness principle)
        if rng.gen::<f32>() < 0.05 {
             self.agents[speaker_idx].grammar.morphology_strength *= 0.99;
        }

        // Log
        let log = LogEntry {
            speaker: speaker_idx,
            listener: listener_idx,
            original: original_sentence.to_string(),
            eroded: eroded_sentence.to_string(),
            outcome: match &result {
                Ok(m) => m.to_string(),
                Err(e) => format!("Err: {}", e),
            },
            success,
        };
        self.logs.push(log);
        if self.logs.len() > 20 {
            self.logs.remove(0);
        }

        // Stats
        if self.epoch % 10 == 0 {
            let avg_morph = self.agents.iter().map(|a| a.grammar.morphology_strength).sum::<f32>() / self.agents.len() as f32;
            let avg_rigidity = self.agents.iter().map(|a| if a.grammar.word_order != WordOrder::Free { 1.0 } else { 0.0 }).sum::<f32>() / self.agents.len() as f32;

            self.stats_history.push((self.epoch as f64, avg_morph as f64, avg_rigidity as f64));
             if self.stats_history.len() > 100 {
                self.stats_history.remove(0);
            }
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()>
where
    <B as Backend>::Error: Send + Sync + 'static,
{
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => app.paused = !app.paused,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(f.area());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    draw_stats(f, app, top_chunks[0]);
    draw_grammar(f, app, top_chunks[1]);
    draw_logs(f, app, chunks[1]);
}

fn draw_stats(f: &mut Frame, app: &App, area: Rect) {
    let morph_data: Vec<(f64, f64)> = app.stats_history.iter().map(|(x, y, _)| (*x, *y)).collect();
    let rigidity_data: Vec<(f64, f64)> = app.stats_history.iter().map(|(x, _, z)| (*x, *z)).collect();

    let datasets = vec![
        Dataset::default()
            .name("Morphology Strength")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(&morph_data),
        Dataset::default()
            .name("Order Rigidity")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Red))
            .data(&rigidity_data),
    ];

    let x_min = app.stats_history.first().map(|(x, _, _)| *x).unwrap_or(0.0);
    let x_max = app.stats_history.last().map(|(x, _, _)| *x).unwrap_or(100.0).max(x_min + 1.0);

    let chart = Chart::new(datasets)
        .block(Block::default().title("Evolution Stats").borders(Borders::ALL))
        .x_axis(Axis::default().bounds([x_min, x_max]).labels(vec![] as Vec<Span>))
        .y_axis(Axis::default().bounds([0.0, 1.0]).labels(vec![
            Span::raw("0.0"),
            Span::raw("0.5"),
            Span::raw("1.0"),
        ]));
    f.render_widget(chart, area);
}

fn draw_grammar(f: &mut Frame, app: &App, area: Rect) {
    let mut rows = Vec::new();
    for (i, agent) in app.agents.iter().enumerate() {
        rows.push(Row::new(vec![
            format!("{}", i),
            format!("{:?}", agent.grammar.word_order),
            format!("{:.2}", agent.grammar.morphology_strength),
        ]));
    }

    let table = Table::new(
        rows,
        &[Constraint::Length(5), Constraint::Length(20), Constraint::Length(20)]
    )
    .header(Row::new(vec!["ID", "Word Order", "Morphology"]).style(Style::default().add_modifier(Modifier::BOLD)))
    .block(Block::default().title("Agents").borders(Borders::ALL));

    f.render_widget(table, area);
}

fn draw_logs(f: &mut Frame, app: &App, area: Rect) {
    let mut rows = Vec::new();
    for log in &app.logs {
        let style = if log.success {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Red)
        };

        rows.push(Row::new(vec![
            format!("{}->{}", log.speaker, log.listener),
            log.original.clone(),
            log.eroded.clone(),
            log.outcome.clone(),
        ]).style(style));
    }

    let table = Table::new(
        rows,
        &[
            Constraint::Length(10),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ]
    )
    .header(Row::new(vec!["Link", "Original", "Eroded (Heard)", "Result"]).style(Style::default().add_modifier(Modifier::BOLD)))
    .block(Block::default().title("Conversation Log").borders(Borders::ALL));

    f.render_widget(table, area);
}

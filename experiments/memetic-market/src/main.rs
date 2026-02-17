use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    symbols,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem},
};
use std::{
    io,
    time::{Duration, Instant},
};

use memetic_market::sim::{Agent, Market, Strategy, Topic};
use rand::Rng;

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup Simulation
    let topics = vec![
        Topic::new("RUST", 100.0),
        Topic::new("AI", 150.0),
        Topic::new("CRYPTO", 50.0),
        Topic::new("VR", 80.0),
        Topic::new("QUANTUM", 200.0),
        Topic::new("MEME", 10.0),
    ];

    let mut agents = Vec::new();
    let strategies = vec![
        Strategy::TrendFollower,
        Strategy::ValueInvestor,
        Strategy::HypeBeast,
        Strategy::Contrarian,
        Strategy::Random,
    ];

    let mut rng = rand::thread_rng();
    for i in 0..500 {
        let strategy = strategies[rng.gen_range(0..strategies.len())].clone();
        agents.push(Agent::new(&format!("Bot_{}", i), strategy, 1000.0));
    }

    let mut market = Market::new(topics, agents);

    // Run Loop
    let mut selected_topic_index = 0;
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(100);

    loop {
        terminal.draw(|f| ui(f, &market, selected_topic_index))?;

        // Input
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Down => {
                            if selected_topic_index < market.topics.len() - 1 {
                                selected_topic_index += 1;
                            } else {
                                selected_topic_index = 0;
                            }
                        }
                        KeyCode::Up => {
                            if selected_topic_index > 0 {
                                selected_topic_index -= 1;
                            } else {
                                selected_topic_index = market.topics.len() - 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Update Sim
        if last_tick.elapsed() >= tick_rate {
            market.update();
            last_tick = Instant::now();
        }
    }

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, market: &Market, selected_topic_index: usize) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.area());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    // 1. Topic List
    let items: Vec<ListItem> = market
        .topics
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let style = if i == selected_topic_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let change = if t.history.len() > 10 {
                let old = t.history[t.history.len() - 10];
                ((t.market_price - old) / old) * 100.0
            } else {
                0.0
            };

            let line = format!("{:<10} ${:.2} ({:+.1}%)", t.name, t.market_price, change);
            ListItem::new(line).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Topics (Up/Down to Select)"),
    );
    f.render_widget(list, left_chunks[0]);

    // 2. Agent Leaderboard
    // Sort agents by wealth (cash + portfolio value)
    // Calculating portfolio value requires current prices, which is expensive to do every frame.
    // For now, just show top agents by Cash (simplification).
    let mut rich_agents: Vec<&Agent> = market.agents.iter().collect();
    rich_agents.sort_by(|a, b| b.cash.total_cmp(&a.cash));

    let agent_items: Vec<ListItem> = rich_agents
        .iter()
        .take(10)
        .map(|a| ListItem::new(format!("{}: ${:.2} ({:?})", a.name, a.cash, a.strategy)))
        .collect();

    let agent_list = List::new(agent_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Top Agents (Cash)"),
    );
    f.render_widget(agent_list, left_chunks[1]);

    // 3. Chart
    if let Some(topic) = market.topics.get(selected_topic_index) {
        let data: Vec<(f64, f64)> = topic
            .history
            .iter()
            .enumerate()
            .map(|(i, &p)| (i as f64, p))
            .collect();

        let datasets = vec![
            Dataset::default()
                .name("Market Price")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Cyan))
                .data(&data),
            Dataset::default()
                .name("Intrinsic Value")
                .marker(symbols::Marker::Dot)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Green))
                // Just a horizontal line for intrinsic value for now, or we track it if we stored history
                .data(&[]),
        ];

        let min_price = topic
            .history
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min)
            .min(topic.intrinsic_value * 0.5);
        let max_price = topic
            .history
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max)
            .max(topic.intrinsic_value * 1.5);

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{} Hype Cycle", topic.name)),
            )
            .x_axis(Axis::default().title("Time").bounds([0.0, 100.0]))
            .y_axis(
                Axis::default()
                    .title("Price")
                    .bounds([min_price, max_price])
                    .labels(vec![
                        Span::raw(format!("{:.1}", min_price)),
                        Span::raw(format!("{:.1}", (min_price + max_price) / 2.0)),
                        Span::raw(format!("{:.1}", max_price)),
                    ]),
            );
        f.render_widget(chart, chunks[1]);
    }
}

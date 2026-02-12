use std::{error::Error, io, time::{Duration, Instant}};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use sched_market::model::{Scheduler, Thread};
use rand::Rng;

struct App {
    scheduler: Scheduler,
    paused: bool,
    tick_rate: Duration,
    last_tick: Instant,
}

impl App {
    fn new() -> Self {
        let mut sched = Scheduler::new();
        // Initial seed threads
        sched.add_thread(Thread::new(1, 50, 100, 5000.0, 10000.0, 0));
        sched.add_thread(Thread::new(2, 30, 80, 2000.0, 5000.0, 0));
        sched.add_thread(Thread::new(3, 10, 20, 1000.0, 5000.0, 0));

        Self {
            scheduler: sched,
            paused: false,
            tick_rate: Duration::from_millis(100),
            last_tick: Instant::now(),
        }
    }

    fn on_tick(&mut self) {
        if !self.paused {
            self.scheduler.tick();

            // Randomly spawn new threads
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.1) { // 10% chance per tick
                let id = self.scheduler.current_tick as usize + 100;
                let work = rng.gen_range(5..40);
                let deadline = self.scheduler.current_tick + work as u64 * rng.gen_range(1..5);
                let budget = rng.gen_range(100.0..2000.0);
                let value = budget * rng.gen_range(1.1..3.0);

                self.scheduler.add_thread(Thread::new(id, work, deadline, budget, value, self.scheduler.current_tick));
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create App
    let mut app = App::new();

    // Run Loop
    let res = run_app(&mut terminal, &mut app);

    // Restore Terminal
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app)).map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;

        let timeout = app.tick_rate
            .checked_sub(app.last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('p') => app.paused = !app.paused,
                    KeyCode::Char('r') => *app = App::new(),
                    _ => {}
                }
            }
        }

        if app.last_tick.elapsed() >= app.tick_rate {
            app.on_tick();
            app.last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Main View (Gantt + Stats)
            Constraint::Percentage(30), // Thread List
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new(format!("SCHED-MARKET: Tick {} | Paused: {}", app.scheduler.current_tick, app.paused))
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, chunks[0]);

    // Middle: Split into Price Chart and Execution Chart
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Execution (Thread ID vs Time)
            Constraint::Percentage(50), // Price (Price vs Time)
        ])
        .split(chunks[1]);

    draw_execution_chart(f, app, middle_chunks[0]);
    draw_price_chart(f, app, middle_chunks[1]);

    // Bottom: Thread List
    draw_thread_list(f, app, chunks[2]);
}

fn draw_price_chart(f: &mut Frame, app: &App, area: Rect) {
    let history = &app.scheduler.history;
    let window_size = 100;
    let start_idx = history.len().saturating_sub(window_size);
    let view_history = &history[start_idx..];

    let data: Vec<(f64, f64)> = view_history.iter().map(|r| {
        (r.tick as f64, r.price)
    }).collect();

    let datasets = vec![
        Dataset::default()
            .name("Price")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .data(&data)
    ];

    let x_min = view_history.first().map(|r| r.tick as f64).unwrap_or(0.0);
    let x_max = view_history.last().map(|r| r.tick as f64).unwrap_or(10.0).max(x_min + 10.0);

    let y_max = view_history.iter().map(|r| r.price).fold(0.0, f64::max).max(10.0) * 1.1;

    let chart = Chart::new(datasets)
        .block(Block::default().title("Market Price").borders(Borders::ALL))
        .x_axis(Axis::default()
            .title("Tick")
            .bounds([x_min, x_max])
            .labels(vec![
                Span::styled(format!("{:.0}", x_min), Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(format!("{:.0}", x_max), Style::default().add_modifier(Modifier::BOLD)),
            ]))
        .y_axis(Axis::default()
            .title("Price")
            .bounds([0.0, y_max])
            .labels(vec![
                Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(format!("{:.1}", y_max), Style::default().add_modifier(Modifier::BOLD)),
            ]));

    f.render_widget(chart, area);
}

fn draw_execution_chart(f: &mut Frame, app: &App, area: Rect) {
    let history = &app.scheduler.history;
    let window_size = 100;
    let start_idx = history.len().saturating_sub(window_size);
    let view_history = &history[start_idx..];

    // X: Tick, Y: Thread ID
    let data: Vec<(f64, f64)> = view_history.iter().map(|r| {
        (r.tick as f64, r.winner_id.unwrap_or(0) as f64)
    }).collect();

    let datasets = vec![
        Dataset::default()
            .name("Thread ID")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&data)
    ];

    let x_min = view_history.first().map(|r| r.tick as f64).unwrap_or(0.0);
    let x_max = view_history.last().map(|r| r.tick as f64).unwrap_or(10.0).max(x_min + 10.0);

    // Find max thread ID for Y axis
    let max_thread_id = view_history.iter()
        .filter_map(|r| r.winner_id)
        .max()
        .unwrap_or(0) as f64;
    let y_max = max_thread_id.max(5.0) * 1.1;

    let chart = Chart::new(datasets)
        .block(Block::default().title("Execution History (Tick vs Thread ID)").borders(Borders::ALL))
        .x_axis(Axis::default()
            .title("Tick")
            .bounds([x_min, x_max])
            .labels(vec![
                Span::styled(format!("{:.0}", x_min), Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(format!("{:.0}", x_max), Style::default().add_modifier(Modifier::BOLD)),
            ]))
        .y_axis(Axis::default()
            .title("Thread ID")
            .bounds([0.0, y_max])
            .labels(vec![
                Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(format!("{:.0}", y_max), Style::default().add_modifier(Modifier::BOLD)),
            ]));

    f.render_widget(chart, area);
}

fn draw_thread_list(f: &mut Frame, app: &App, area: Rect) {
    let header = Row::new(vec![
        "ID", "Work Left", "Deadline", "Budget", "Start Tick", "Status"
    ]).style(Style::default().fg(Color::Yellow));

    let mut rows = Vec::new();
    for t in &app.scheduler.threads {
        rows.push(Row::new(vec![
            format!("{}", t.id),
            format!("{}", t.work_needed),
            format!("{}", t.deadline),
            format!("{:.2}", t.budget),
            format!("{}", t.start_tick),
            "Active".to_string(),
        ]));
    }
    // Also show recently failed/completed?
    for t in app.scheduler.completed_threads.iter().rev().take(5) {
        rows.push(Row::new(vec![
            format!("{}", t.id),
            "0".to_string(),
            format!("{}", t.deadline),
            format!("{:.2}", t.budget),
            format!("{}", t.start_tick),
            "Done".to_string(),
        ]).style(Style::default().fg(Color::Green)));
    }

    for t in app.scheduler.failed_threads.iter().rev().take(5) {
         rows.push(Row::new(vec![
            format!("{}", t.id),
            format!("{}", t.work_needed),
            format!("{}", t.deadline),
            format!("{:.2}", t.budget),
            format!("{}", t.start_tick),
            "Failed".to_string(),
        ]).style(Style::default().fg(Color::Red)));
    }

    let table = Table::new(rows, [
        Constraint::Length(5),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(15),
        Constraint::Length(10),
        Constraint::Length(10),
    ])
    .header(header)
    .block(Block::default().title("Threads").borders(Borders::ALL));

    f.render_widget(table, area);
}

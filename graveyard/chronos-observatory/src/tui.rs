use crate::babylonian::BabylonianNumber;
use crate::forecaster::Forecaster;
use crate::mayan::MayanDate;
use chrono::{Duration, Utc};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Frame, Terminal,
};
use std::io;

pub struct App {
    pub forecaster: Forecaster,
    pub start_date: MayanDate,
    pub end_date: MayanDate,
    pub min_val: f64,
    pub max_val: f64,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            forecaster: Forecaster::new(),
            start_date: MayanDate::from(Utc::now()), // placeholder
            end_date: MayanDate::from(Utc::now()),   // placeholder
            min_val: 0.0,
            max_val: 100.0,
        };
        app.generate_data();
        app
    }

    pub fn generate_data(&mut self) {
        let mut rng = rand::thread_rng();
        let now = Utc::now();
        self.start_date = MayanDate::from(now - Duration::days(50));
        self.end_date = MayanDate::from(now + Duration::days(10)); // Forecast into future

        // Generate 50 points of history
        for i in 0..50 {
            let dt = now - Duration::days(50 - i);
            let mayan = MayanDate::from(dt);
            // Simulate a trend + noise
            let val = (i as f64) * 1.5 + rng.gen_range(-5.0..5.0) + 10.0;
            let bab = BabylonianNumber::from_f64(val.max(0.0));
            self.forecaster.add_point(mayan, bab);
        }

        // Update bounds
        self.min_val = 0.0;
        self.max_val = 100.0; // Fixed for now
    }
}

pub fn run_app() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app_loop(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app_loop<B: Backend>(terminal: &mut Terminal<B>, app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    // Prepare data for Chart
    let data_points: Vec<(f64, f64)> = app
        .forecaster
        .data
        .iter()
        .map(|(d, v)| (d.total_days() as f64, v.to_f64()))
        .collect();

    // Prepare regression line
    let mut regression_points = Vec::new();
    if let Some((m, b)) = app.forecaster.linear_regression() {
        let start_days = app.start_date.total_days() as f64;
        let end_days = app.end_date.total_days() as f64;

        // y = mx + b
        let y1 = m.to_f64() * start_days + b.to_f64();
        let y2 = m.to_f64() * end_days + b.to_f64();

        regression_points.push((start_days, y1));
        regression_points.push((end_days, y2));
    }

    // X-Axis Labels (Mayan)
    // We pick 5 labels distributed across the range
    let x_min = app.start_date.total_days() as f64;
    let x_max = app.end_date.total_days() as f64;
    let x_labels: Vec<Span> = (0..=4)
        .map(|i| {
            let days = x_min + (x_max - x_min) * (i as f64 / 4.0);
            // Reverse engineer days to MayanDate display?
            // I didn't implement From<u64> for MayanDate properly, only logic in From<DateTime>.
            // For simplicity, let's construct a simplified display or implement From<u64>.
            // Actually, MayanDate is just a struct, I can implement logic here or there.
            // Let's implement From<u64> for MayanDate in mayan.rs later, for now construct ad-hoc
            let d = days as u64;
            // logic copy from mayan.rs
            let mut rem = d;
            let baktun = rem / 144000;
            rem %= 144000;
            let katun = rem / 7200;
            rem %= 7200;
            let tun = rem / 360;
            rem %= 360;
            let uinal = rem / 20;
            rem %= 20;
            let kin = rem;
            Span::raw(format!("{}.{}.{}.{}.{}", baktun, katun, tun, uinal, kin))
        })
        .collect();

    // Y-Axis Labels (Babylonian)
    let y_labels: Vec<Span> = (0..=5)
        .map(|i| {
            let val = app.min_val + (app.max_val - app.min_val) * (i as f64 / 5.0);
            let bab = BabylonianNumber::from_f64(val);
            Span::raw(format!("{}", bab))
        })
        .collect();

    let datasets = vec![
        Dataset::default()
            .name("Observation")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .graph_type(GraphType::Scatter)
            .data(&data_points),
        Dataset::default()
            .name("Prophecy")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Red))
            .graph_type(GraphType::Line)
            .data(&regression_points),
    ];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "CHRONOS OBSERVATORY ⚛️🏺",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("Time (Mayan Long Count)")
                .style(Style::default().fg(Color::Gray))
                .bounds([x_min, x_max])
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .title("Value (Babylonian Sexagesimal)")
                .style(Style::default().fg(Color::Gray))
                .bounds([app.min_val, app.max_val])
                .labels(y_labels),
        );

    f.render_widget(chart, chunks[0]);
}

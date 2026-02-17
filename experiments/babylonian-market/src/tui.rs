use std::io::{self, Stdout};
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use crate::babylonian::BabylonianNumber;
use crate::market_wrapper::BabylonianMarket;
use market_sim::Particle;

pub fn run_app() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_loop(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    // 50 columns, 40 rows (prices)
    let mut market = BabylonianMarket::new(50, 40);

    loop {
        terminal.draw(|f| ui(f, &mut market))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }

        market.update();
    }
}

fn ui(f: &mut Frame, market: &mut BabylonianMarket) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header (Mayan Date)
            Constraint::Min(0),    // Market Grid
            Constraint::Length(3), // Footer (Stats)
        ])
        .split(f.size());

    // --- Header: Mayan Date ---
    let date_str = format!("Mayan Date: {}", market.current_date);
    let title = Paragraph::new(date_str)
        .block(Block::default().borders(Borders::ALL).title("Chronos Market"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, chunks[0]);

    // --- Body: Market Grid + Price Axis ---
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(15), // Y-Axis (Babylonian Prices)
            Constraint::Min(0),     // The Grid
        ])
        .split(chunks[1]);

    // Render Y-Axis
    let height = market.grid.height;
    let mut price_lines = Vec::new();
    // Render from Top (High Price) to Bottom (Low Price)
    // But terminal renders top-down.
    // Price 0 is Top. Price H-1 is Bottom.
    // Let's map Y to Price.
    // In market_sim: y=0 is High Price. y=H-1 is Low Price.
    // Let's say Price = Height - y.

    // We can't render every single line if the terminal is too small,
    // but let's assume it fits or let ratatui handle scrolling (Paragraph scrolls).
    // Actually Paragraph doesn't scroll automatically without offset.
    // We should fit the grid to the view.
    // For now, let's just render the lines corresponding to the grid rows.

    for y in 0..height {
        let price_val = (height - 1 - y) as i64;
        // Only show label every 5 lines to avoid clutter if needed,
        // but Babylonian numbers can be long.
        if y % 2 == 0 {
            let b_num = BabylonianNumber::from(price_val);
            price_lines.push(Line::from(vec![
                Span::raw(format!("{:>10}", b_num.to_string())),
            ]));
        } else {
             price_lines.push(Line::from(""));
        }
    }

    let y_axis = Paragraph::new(price_lines)
        .block(Block::default().borders(Borders::RIGHT).title("Price (Sexagesimal)"))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(y_axis, body_chunks[0]);

    // Render Grid
    // We need to render the particles.
    // We can use a Canvas or just a Paragraph of text.
    // Paragraph of text is easier for characters.

    let mut grid_lines = Vec::new();
    for y in 0..height {
        let mut line_spans = Vec::new();
        for x in 0..market.grid.width {
             let p = market.grid.get(x, y);
             let (char, color) = match p {
                 Particle::Empty => (' ', Color::Reset),
                 Particle::Bid(_) => ('^', Color::Green),
                 Particle::Ask(_) => ('v', Color::Red),
                 Particle::Trade { .. } => ('*', Color::White),
             };
             line_spans.push(Span::styled(char.to_string(), Style::default().fg(color)));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines)
        .block(Block::default().borders(Borders::ALL).title("Order Book Flow"));
    f.render_widget(grid_widget, body_chunks[1]);


    // --- Footer: Stats ---
    let stats = format!(
        "Bids: {} | Asks: {} | Trades: {} | CoM: {:.2}",
        market.grid.total_bids,
        market.grid.total_asks,
        market.grid.trade_count,
        market.grid.center_of_mass
    );
    let footer = Paragraph::new(stats)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));
    f.render_widget(footer, chunks[2]);
}

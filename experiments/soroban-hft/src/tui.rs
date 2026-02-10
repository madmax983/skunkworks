use crate::beads::Soroban;
use crate::engine::{OrderBook, Side, Trade};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Color, Style},
};
use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};
use rand::Rng;

#[allow(clippy::collapsible_if)]
pub fn run_tui() -> anyhow::Result<()> {
    // Setup terminal
    let mut terminal = ratatui::init();

    let mut app = App::new();
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }

    ratatui::restore();
    Ok(())
}

struct App {
    order_book: OrderBook,
    trades: Vec<Trade>,
    last_action: String,
}

impl App {
    fn new() -> Self {
        Self {
            order_book: OrderBook::new(),
            trades: Vec::new(),
            last_action: "Initializing...".to_string(),
        }
    }

    fn on_tick(&mut self) {
        // Randomly place an order
        let mut rng = rand::thread_rng();
        let side = if rng.gen_bool(0.5) { Side::Bid } else { Side::Ask };

        // Price roughly between 90 and 110
        let price_val = rng.gen_range(90..110);
        let price = Soroban::from_u64(price_val);

        // Quantity 1-5
        let qty_val = rng.gen_range(1..6);
        let qty = Soroban::from_u64(qty_val);

        self.order_book.place_limit_order(side, price.clone(), qty.clone());
        // For display, clean up formatting slightly
        self.last_action = format!("Placed {:?} Price: {} Qty: {}", side, price, qty);

        // Run matching
        let new_trades = self.order_book.match_orders();
        if !new_trades.is_empty() {
            let count = Soroban::from_u64(new_trades.len() as u64);
            self.last_action = format!("Executed {} trades", count);
            // Keep last 20 trades
            for t in new_trades {
                self.trades.insert(0, t);
            }
            if self.trades.len() > 20 {
                self.trades.truncate(20);
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Order Book
            Constraint::Percentage(30), // Trades
            Constraint::Percentage(20), // Status
        ])
        .split(f.area());

    // Order Book Split
    let book_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    // Bids
    let bids_text: String = app.order_book.bids.iter().take(10)
        .map(|o| format!("{} @ {}", o.quantity, o.price)) // Qty @ Price
        .collect::<Vec<_>>()
        .join("\n");

    let bids_block = Paragraph::new(bids_text)
        .block(Block::default().title("BIDS (Qty @ Price)").borders(Borders::ALL).style(Style::default().fg(Color::Green)));
    f.render_widget(bids_block, book_chunks[0]);

    // Asks
    let asks_text: String = app.order_book.asks.iter().take(10)
        .map(|o| format!("{} @ {}", o.quantity, o.price))
        .collect::<Vec<_>>()
        .join("\n");

    let asks_block = Paragraph::new(asks_text)
        .block(Block::default().title("ASKS (Qty @ Price)").borders(Borders::ALL).style(Style::default().fg(Color::Red)));
    f.render_widget(asks_block, book_chunks[1]);

    // Trades
    let trades_text: String = app.trades.iter()
        .map(|t| format!("Px: {} Qty: {}", t.price, t.quantity))
        .collect::<Vec<_>>()
        .join("\n");

    let trades_block = Paragraph::new(trades_text)
        .block(Block::default().title("Recent Trades").borders(Borders::ALL));
    f.render_widget(trades_block, chunks[1]);

    // Status
    let status_text = format!("Last Action: {}\nPress 'q' to quit.", app.last_action);
    let status_block = Paragraph::new(status_text)
        .block(Block::default().title("Status").borders(Borders::ALL));
    f.render_widget(status_block, chunks[2]);
}

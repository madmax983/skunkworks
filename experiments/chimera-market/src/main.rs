use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use pest::Parser;
use pest_derive::Parser;
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color},
    text::Line,
    widgets::{
        Block, Borders, Paragraph,
        canvas::{Canvas, Points},
    },
};
use std::time::Duration;
use tui_shared::Tui;

mod ast;
mod market;
mod vm;

use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::market::{Grid, Particle};
use crate::vm::{ChimeraVM, Value};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ChimeraParser;

struct Trader {
    id: usize,
    dna: Dna,
    vm: ChimeraVM,
    balance: f32,
    inventory: i32,
}

impl Trader {
    fn new(id: usize, dna: Dna) -> Self {
        Self {
            id,
            vm: ChimeraVM::new(dna.clone()),
            dna,
            balance: 1000.0,
            inventory: 0,
        }
    }

    fn reset_vm(&mut self, price: f32, trend: f32) {
        // Create new VM instance with same DNA but refreshed state
        self.vm = ChimeraVM::new(self.dna.clone());
        // Inject inputs: [Price, Trend, Balance, Inventory]
        // Note: Stack is LIFO. Last pushed is top.
        // We want Top to be Price maybe?
        // Let's push Inventory, Balance, Trend, Price.
        // Stack Top -> Price.
        self.vm.stack.push(Value::Int(self.inventory as i64));
        self.vm.stack.push(Value::Int(self.balance as i64));
        self.vm.stack.push(Value::Int((trend * 100.0) as i64));
        self.vm.stack.push(Value::Int(price as i64));
    }
}

fn generate_random_dna() -> Dna {
    let mut rng = rand::thread_rng();
    let enzymes = ["push", "add", "sub", "mul", "div", "dup", "swap", "brz", "jump", "drop"];
    let mut genes = Vec::new();
    for _ in 0..rng.gen_range(5..20) {
        let name = enzymes[rng.gen_range(0..enzymes.len())];
        let args = match name {
            "push" => vec![Nucleotide::Number(rng.gen_range(0..50))],
            "jump" | "brz" => vec![Nucleotide::Number(rng.gen_range(0..5))],
            _ => vec![],
        };
        genes.push(Gene {
            name: name.to_string(),
            args,
        });
    }
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut grid = Grid::new(60, 40);
    let mut traders: Vec<Trader> = (0..20)
        .map(|i| Trader::new(i, generate_random_dna()))
        .collect();

    // UI Buffers
    let mut bids_buf = Vec::new();
    let mut asks_buf = Vec::new();
    let mut trades_buf = Vec::new();
    // let mut price_history = Vec::new(); // Unused for now

    let mut tick = 0;
    let mut last_price = 20.0;

    loop {
        // 1. Update Market Physics
        let events = grid.update();

        // 2. Process Trades
        for event in events {
            if let Some(buyer) = traders.iter_mut().find(|t| t.id == event.buyer) {
                buyer.balance -= event.price;
                buyer.inventory += 1;
            }
            if let Some(seller) = traders.iter_mut().find(|t| t.id == event.seller) {
                seller.balance += event.price;
                seller.inventory -= 1;
            }
            last_price = event.price;
        }

        // 3. Trader Logic
        for trader in &mut traders {
            trader.reset_vm(last_price, 0.0);
            for _ in 0..50 {
                trader.vm.step();
                if trader.vm.halted { break; }
            }

            // Output: [Offset, Action] (Top is Action)
            // Or [Action, Offset] if we pushed Offset first.
            // Let's assume Top is Action.
            if let Some(Value::Int(action)) = trader.vm.stack.pop() {
                 let offset = if let Some(Value::Int(o)) = trader.vm.stack.pop() {
                     o.clamp(0, 10) as f32
                 } else { 0.0 };

                 match action {
                     1 => { // Buy
                        let price = (last_price - offset).clamp(0.0, grid.height as f32 - 1.0);
                        let y = (grid.height as f32 - 1.0 - price) as usize;
                        let x = rand::thread_rng().gen_range(0..grid.width);
                        grid.set(x, y, Particle::Bid(trader.id));
                     }
                     2 => { // Sell
                        let price = (last_price + offset).clamp(0.0, grid.height as f32 - 1.0);
                        let y = (grid.height as f32 - 1.0 - price) as usize;
                        let x = rand::thread_rng().gen_range(0..grid.width);
                        grid.set(x, y, Particle::Ask(trader.id));
                     }
                     _ => {}
                 }
            }
        }

        // 4. Evolution
        if tick % 200 == 0 && tick > 0 {
             traders.sort_by(|a, b| b.balance.partial_cmp(&a.balance).unwrap());
             let elite_count = traders.len() / 5;
             let elite_dna: Vec<Dna> = traders.iter().take(elite_count).map(|t| t.dna.clone()).collect();

             let len = traders.len();
             for i in (len - elite_count)..len {
                 let parent_dna = &elite_dna[i % elite_count];
                 let mut temp_vm = ChimeraVM::new(parent_dna.clone());
                 temp_vm.mutate(); // Mutate once
                 temp_vm.mutate(); // Mutate twice
                 traders[i] = Trader::new(i, temp_vm.dna);
                 traders[i].balance = 1000.0;
                 traders[i].inventory = 0;
             }
        }

        tick += 1;

        // Draw
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(f.area());

            // Market View
            bids_buf.clear();
            asks_buf.clear();
            trades_buf.clear();

            for y in 0..grid.height {
                for x in 0..grid.width {
                    match grid.get(x, y) {
                        Particle::Bid(_) => bids_buf.push((x as f64, (grid.height - 1 - y) as f64)),
                        Particle::Ask(_) => asks_buf.push((x as f64, (grid.height - 1 - y) as f64)),
                        Particle::Trade { .. } => trades_buf.push((x as f64, (grid.height - 1 - y) as f64)),
                        _ => {}
                    }
                }
            }

            let market_canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(" Chimera Market "))
                .x_bounds([0.0, grid.width as f64])
                .y_bounds([0.0, grid.height as f64])
                .paint(|ctx| {
                     ctx.draw(&Points { coords: &bids_buf, color: Color::Green });
                     ctx.draw(&Points { coords: &asks_buf, color: Color::Red });
                     ctx.draw(&Points { coords: &trades_buf, color: Color::White });
                });
            f.render_widget(market_canvas, chunks[0]);

            // Leaderboard
            let mut lines = vec![Line::from("Top Traders:")];
            let mut sorted_traders: Vec<&Trader> = traders.iter().collect();
            sorted_traders.sort_by(|a, b| b.balance.partial_cmp(&a.balance).unwrap());

            for (i, t) in sorted_traders.iter().take(15).enumerate() {
                lines.push(Line::from(format!(
                    "{}. ID:{} Bal:{:.0}",
                    i+1, t.id, t.balance
                )));
            }
             lines.push(Line::from(""));
             lines.push(Line::from(format!("Price: {:.2}", last_price)));
             lines.push(Line::from(format!("Tick: {}", tick)));

            let stats_block = Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL).title(" Stats "));
            f.render_widget(stats_block, chunks[1]);

        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && (key.code == KeyCode::Esc || key.code == KeyCode::Char('q')) {
                    break;
                }
            }
        }
    }
    Ok(())
}

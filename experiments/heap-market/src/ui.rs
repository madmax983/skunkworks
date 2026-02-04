use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Row, Table},
    Frame,
};
use crate::model::*;
use crate::market::Market;

pub fn draw_ui(f: &mut Frame, market: &Market, blocks: &[MemoryBlock]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Top: Charts
            Constraint::Percentage(50), // Bottom: Heap Map
        ])
        .split(f.area());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Order Book
            Constraint::Percentage(50), // Price History
        ])
        .split(chunks[0]);

    draw_order_book(f, top_chunks[0], market);
    draw_price_history(f, top_chunks[1], market);
    draw_heap_map(f, chunks[1], blocks);
}

fn draw_order_book(f: &mut Frame, area: Rect, market: &Market) {
    // Group bids and asks by price
    // This is a simplified visualization
    let mut bids_by_price: Vec<(u64, usize)> = Vec::new();
    for bid in &market.bids {
        if let Some(entry) = bids_by_price.iter_mut().find(|(p, _)| *p == bid.price) {
            entry.1 += bid.quantity;
        } else {
            bids_by_price.push((bid.price, bid.quantity));
        }
    }
    bids_by_price.sort_by(|a, b| b.0.cmp(&a.0)); // High to low

    let mut asks_by_price: Vec<(u64, usize)> = Vec::new();
    for ask in &market.asks {
        if let Some(entry) = asks_by_price.iter_mut().find(|(p, _)| *p == ask.price) {
            entry.1 += ask.quantity;
        } else {
            asks_by_price.push((ask.price, ask.quantity));
        }
    }
    asks_by_price.sort_by(|a, b| a.0.cmp(&b.0)); // Low to high

    // Render as two tables or lists?
    // Let's use a Table for now.

    let mut rows = Vec::new();
    let max_len = std::cmp::max(bids_by_price.len(), asks_by_price.len());

    for i in 0..max_len {
        let bid_str = if i < bids_by_price.len() {
            format!("{} @ ${}", bids_by_price[i].1, bids_by_price[i].0)
        } else {
            String::new()
        };

        let ask_str = if i < asks_by_price.len() {
            format!("${} @ {}", asks_by_price[i].0, asks_by_price[i].1)
        } else {
            String::new()
        };

        rows.push(Row::new(vec![bid_str, ask_str]));
    }

    let table = Table::new(rows, [Constraint::Percentage(50), Constraint::Percentage(50)])
        .header(Row::new(vec!["BIDS (Qty @ Price)", "ASKS (Price @ Qty)"])
            .style(Style::default().fg(Color::Yellow)))
        .block(Block::default().title("Order Book").borders(Borders::ALL));

    f.render_widget(table, area);
}

fn draw_price_history(f: &mut Frame, area: Rect, market: &Market) {
    let data: Vec<(f64, f64)> = market.history.iter().enumerate()
        .map(|(i, &p)| (i as f64, p as f64))
        .collect();

    let datasets = vec![
        Dataset::default()
            .name("Price")
            .marker(ratatui::symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(&data),
    ];

    let max_price = market.history.iter().max().copied().unwrap_or(100) as f64;
    let max_time = market.history.len() as f64;

    let chart = Chart::new(datasets)
        .block(Block::default().title("Clearing Price").borders(Borders::ALL))
        .x_axis(Axis::default()
            .title("Time")
            .bounds([0.0, max_time.max(10.0)]))
        .y_axis(Axis::default()
            .title("Credits")
            .bounds([0.0, max_price * 1.2])); // 20% buffer

    f.render_widget(chart, area);
}

fn draw_heap_map(f: &mut Frame, area: Rect, blocks: &[MemoryBlock]) {
    // Render memory blocks as a grid of characters
    // Assume blocks are linear.
    // Wrap based on area width.

    let inner_area = Block::default().borders(Borders::ALL).title("Heap Memory Map").inner(area);
    let width = inner_area.width as usize;
    if width == 0 { return; }

    let mut lines = Vec::new();
    let mut current_spans = Vec::new();

    for block in blocks {
        let color = match block.owner {
            Some(AgentId(id)) => Color::Indexed((id % 255) as u8 + 1), // Simple color hashing
            None => Color::DarkGray,
        };

        // Represent size with repeated chars
        let char_str = "█";

        for _ in 0..block.size {
             current_spans.push(Span::styled(char_str, Style::default().fg(color)));

             if current_spans.len() >= width {
                 lines.push(Line::from(current_spans.clone()));
                 current_spans.clear();
             }
        }
    }

    if !current_spans.is_empty() {
        lines.push(Line::from(current_spans));
    }

    let paragraph = Paragraph::new(lines)
        .block(Block::default().title("Heap Memory Map").borders(Borders::ALL));

    f.render_widget(paragraph, area);
}

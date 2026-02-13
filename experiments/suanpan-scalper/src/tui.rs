use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    text::{Span, Line},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Widget},
    Frame,
};

use crate::market::{Market, Scalper};
use crate::suanpan::Suanpan;

pub fn draw(f: &mut Frame, market: &Market, scalper: &Scalper) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Chart
            Constraint::Percentage(30), // Abacus
            Constraint::Percentage(20), // Logs/Stats
        ])
        .split(f.size());

    draw_chart(f, market, scalper, chunks[0]);

    let abacus_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    draw_abacus(f, &market.price, "Market Price 🧮", abacus_chunks[0]);
    draw_abacus(f, &scalper.sma, "Moving Average 🧮", abacus_chunks[1]);

    draw_stats(f, market, scalper, chunks[2]);
}

fn draw_chart(f: &mut Frame, market: &Market, _scalper: &Scalper, area: Rect) {
    let history_data: Vec<(f64, f64)> = market
        .history
        .iter()
        .enumerate()
        .map(|(i, &p)| (i as f64, p as f64))
        .collect();

    let datasets = vec![Dataset::default()
        .name("Price")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .graph_type(GraphType::Line)
        .data(&history_data)];

    let min_price = market.history.iter().min().unwrap_or(&0);
    let max_price = market.history.iter().max().unwrap_or(&100);

    let chart = Chart::new(datasets)
        .block(Block::default().title("Market Ticker").borders(Borders::ALL))
        .x_axis(Axis::default().title("Time").bounds([0.0, 100.0]))
        .y_axis(Axis::default().title("Price").bounds([*min_price as f64, *max_price as f64]));

    f.render_widget(chart, area);
}

fn draw_abacus(f: &mut Frame, suanpan: &Suanpan, title: &str, area: Rect) {
    let mut lines = Vec::new();

    // Top Border
    lines.push(Line::from(vec![Span::raw("┌───────────────────────────────────────────┐")]));

    let mut row_h_top = String::from("│ ");
    let mut row_h_bot = String::from("│ ");
    let mut row_beam  = String::from("│ ");
    let mut e_rows: Vec<String> = vec![String::from("│ "); 6];

    for rod in suanpan.rods.iter().rev() {
        // Heaven
        let (h_top, h_bot) = match rod.heaven {
            0 => ("88", "  "),
            1 => ("8 ", " 8"),
            2 => ("  ", "88"),
            _ => ("??", "??"),
        };
        row_h_top.push_str(h_top); row_h_top.push(' ');
        row_h_bot.push_str(h_bot); row_h_bot.push(' ');
        row_beam.push_str("══ ");

        // Earth
        let e = rod.earth;
        let mut col_chars = Vec::new();
        for _ in 0..e { col_chars.push('8'); }
        col_chars.push('|');
        for _ in 0..(5-e) { col_chars.push('8'); }

        for (i, c) in col_chars.iter().enumerate() {
            if i < e_rows.len() {
                e_rows[i].push(*c);
                e_rows[i].push_str("  ");
            }
        }
    }

    lines.push(Line::from(Span::raw(row_h_top)));
    lines.push(Line::from(Span::raw(row_h_bot)));
    lines.push(Line::from(Span::raw(row_beam)));
    for r in e_rows {
        lines.push(Line::from(Span::raw(r)));
    }
    lines.push(Line::from(Span::raw(format!("└ Value: {} ┘", suanpan.to_u64()))));

    let paragraph = Paragraph::new(lines)
        .block(Block::default().title(title).borders(Borders::ALL));
    f.render_widget(paragraph, area);
}

fn draw_stats(f: &mut Frame, market: &Market, scalper: &Scalper, area: Rect) {
    let text = vec![
        Line::from(vec![
            Span::styled("Scalper Balance: ", Style::default().fg(Color::Green)),
            Span::raw(format!("{} (Units)", scalper.balance.to_u64())),
        ]),
        Line::from(vec![
            Span::styled("Position: ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("{}", scalper.position)),
        ]),
        Line::from(vec![
            Span::styled("Trend: ", Style::default().fg(Color::Blue)),
            Span::raw(format!("{:.2}", market.trend)),
        ]),
    ];
    let block = Block::default().title("Trade Desk").borders(Borders::ALL);
    let p = Paragraph::new(text).block(block);
    f.render_widget(p, area);
}

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Line},
    widgets::{Block, Borders, Chart, Dataset, Axis, GraphType, Paragraph},
    Frame,
};
use crate::market::{Market, Heap};

pub fn draw_ui(f: &mut Frame, market: &Market) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(70), // Heap Map
            Constraint::Percentage(30), // Stats & Ticker
        ].as_ref())
        .split(f.area());

    draw_heap_map(f, &market.heap, chunks[0]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Ticker
            Constraint::Percentage(50), // Stats
        ].as_ref())
        .split(chunks[1]);

    draw_ticker(f, market, bottom_chunks[0]);
    draw_stats(f, market, bottom_chunks[1]);
}

fn draw_heap_map(f: &mut Frame, heap: &Heap, area: Rect) {
    let block_widget = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Heap Map (Capacity: {}, Blocks: {}) ", heap.capacity, heap.blocks.len()));

    f.render_widget(block_widget.clone(), area);

    let inner_area = block_widget.inner(area);
    let width = inner_area.width as usize;
    let height = inner_area.height as usize;
    let total_cells = width * height;

    if total_cells == 0 { return; }

    // Map heap address space to cells
    let scale = (heap.capacity as f64 / total_cells as f64).max(1.0);

    let mut cells = vec![Span::raw(" "); total_cells];

    for block in &heap.blocks {
        let start_cell = (block.start as f64 / scale).floor() as usize;
        let end_cell = ((block.start + block.size) as f64 / scale).ceil() as usize;

        let color = if let Some(owner) = block.owner {
            match owner % 6 {
                0 => Color::Red,
                1 => Color::Green,
                2 => Color::Yellow,
                3 => Color::Blue,
                4 => Color::Magenta,
                5 => Color::Cyan,
                _ => Color::White,
            }
        } else {
            Color::DarkGray
        };

        let ch = if block.owner.is_some() { "█" } else { "░" };

        for i in start_cell..end_cell.min(total_cells) {
             cells[i] = Span::styled(ch, Style::default().fg(color));
        }
    }

    let mut lines = Vec::new();
    for row in cells.chunks(width) {
        lines.push(Line::from(row.to_vec()));
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner_area);
}

fn draw_ticker(f: &mut Frame, market: &Market, area: Rect) {
    let history: Vec<(f64, f64)> = market.history
        .iter()
        .enumerate()
        .map(|(i, &p)| (i as f64, p))
        .collect();

    let max_price = market.history.iter().fold(0.0f64, |a, &b| a.max(b)).max(10.0);

    let datasets = vec![
        Dataset::default()
            .name("Price")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .graph_type(GraphType::Line)
            .data(&history),
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().title(" Market Price ").borders(Borders::ALL))
        .x_axis(Axis::default()
            .title("Time")
            .bounds([0.0, 100.0])
            .labels(vec![
                Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled("50", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled("100", Style::default().add_modifier(Modifier::BOLD)),
            ]))
        .y_axis(Axis::default()
            .title("Price")
            .bounds([0.0, max_price])
            .labels(vec![
                Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled("Max", Style::default().add_modifier(Modifier::BOLD)),
            ]));

    f.render_widget(chart, area);
}

fn draw_stats(f: &mut Frame, market: &Market, area: Rect) {
    let fragmentation = market.heap.fragmentation();
    let free_space: usize = market.heap.blocks.iter().filter(|b| b.owner.is_none()).map(|b| b.size).sum();
    let owned_space: usize = market.heap.blocks.iter().filter(|b| b.owner.is_some()).map(|b| b.size).sum();

    let text = vec![
        Line::from(vec![
            Span::raw("Ticks: "),
            Span::styled(format!("{}", market.tick_count), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw("Current Price: "),
            Span::styled(format!("{:.2}", market.current_price), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("Fragmentation Index: "),
            Span::styled(format!("{:.2}", fragmentation), Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::raw("Free Space: "),
            Span::styled(format!("{}", free_space), Style::default().fg(Color::Blue)),
        ]),
        Line::from(vec![
            Span::raw("Owned Space: "),
            Span::styled(format!("{}", owned_space), Style::default().fg(Color::Magenta)),
        ]),
        Line::from(vec![
            Span::raw("Agents: "),
            Span::styled(format!("{}", market.agents.len()), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::raw("Turnover: "),
            Span::styled(format!("{:.2}", market.last_turnover), Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::raw("Volume: "),
            Span::styled(format!("{}", market.last_volume), Style::default().fg(Color::Cyan)),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().title(" Market Stats ").borders(Borders::ALL));

    f.render_widget(paragraph, area);
}

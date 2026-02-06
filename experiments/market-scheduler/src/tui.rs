use crate::simulation::Simulation;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        Axis, Block, Borders, Cell, Chart, Dataset, GraphType, Paragraph, Row, Table,
    },
    Frame,
};

pub fn draw(f: &mut Frame, sim: &Simulation) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main Content
            Constraint::Length(10), // Price History
        ])
        .split(f.area());

    draw_header(f, sim, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Agents
            Constraint::Percentage(50), // Order Book
        ])
        .split(chunks[1]);

    draw_agents(f, sim, main_chunks[0]);
    draw_order_book(f, sim, main_chunks[1]);

    draw_price_history(f, sim, chunks[2]);
}

fn draw_header(f: &mut Frame, sim: &Simulation, area: Rect) {
    let stats = vec![
        Span::styled(" MARKET SCHEDULER ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(format!("Tick: {} | ", sim.tick_count)),
        Span::raw(format!("Price: {:.2} | ", sim.market.last_price)),
        Span::raw(format!("Agents: {} | ", sim.agents.len())),
        Span::raw(format!("Trades: {} ", sim.transactions.len())),
    ];

    let p = Paragraph::new(Line::from(stats))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(p, area);
}

fn draw_order_book(f: &mut Frame, sim: &Simulation, area: Rect) {
    // Visualize Bids (Green) and Asks (Red)
    let bids: Vec<(f64, f64)> = sim.market.bids.iter()
        .map(|o| (o.price, o.quantity as f64))
        .collect();

    let asks: Vec<(f64, f64)> = sim.market.asks.iter()
        .map(|o| (o.price, o.quantity as f64))
        .collect();

    let datasets = vec![
        Dataset::default()
            .name("Bids")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Green))
            .data(&bids),
        Dataset::default()
            .name("Asks")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Red))
            .data(&asks),
    ];

    // Dynamic X Bounds
    let min_price = 0.0;
    let max_price = sim.market.last_price * 2.0;

    let chart = Chart::new(datasets)
        .block(Block::default().title("Order Book (Price x Qty)").borders(Borders::ALL))
        .x_axis(Axis::default().title("Price").bounds([min_price, max_price]))
        .y_axis(Axis::default().title("Qty").bounds([0.0, 30.0]));

    f.render_widget(chart, area);
}

fn draw_agents(f: &mut Frame, sim: &Simulation, area: Rect) {
    let header_cells = ["Name", "Credits", "Battery", "Strategy", "State"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = sim.agents.iter().map(|agent| {
        let bat_style = if agent.battery < 20.0 {
            Style::default().fg(Color::Red)
        } else if agent.battery > 80.0 {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        };

        let state = if agent.is_alive { "ALIVE" } else { "DEAD" };
        let state_style = if !agent.is_alive { Style::default().fg(Color::Red) } else { Style::default() };

        let cells = vec![
            Cell::from(agent.name.clone()),
            Cell::from(format!("{:.1}", agent.credits)),
            Cell::from(format!("{:.1}%", agent.battery)).style(bat_style),
            Cell::from(format!("{:?}", agent.strategy)),
            Cell::from(state).style(state_style),
        ];
        Row::new(cells).height(1)
    });

    let table = Table::new(rows, [
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
    ])
    .header(header)
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED)) // Use row_highlight_style
    .block(Block::default().borders(Borders::ALL).title("Processes"));

    f.render_widget(table, area);
}

fn draw_price_history(f: &mut Frame, sim: &Simulation, area: Rect) {
    let data: Vec<(f64, f64)> = sim.transactions.iter()
        .map(|t| (t.timestamp as f64, t.price))
        .collect();

    let dataset = Dataset::default()
        .name("Price")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .data(&data);

    let min_x = sim.tick_count.saturating_sub(100) as f64;
    let max_x = sim.tick_count as f64;

    // Y Bounds
    let min_y = data.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min).min(0.0);
    let max_y = data.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max).max(20.0);

    let chart = Chart::new(vec![dataset])
        .block(Block::default().title("Price History").borders(Borders::ALL))
        .x_axis(Axis::default().bounds([min_x, max_x]))
        .y_axis(Axis::default().bounds([min_y, max_y + 5.0]));

    f.render_widget(chart, area);
}

use crate::model::{Agent, MarketState};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Cell, Paragraph, Row, Sparkline, Table,
        canvas::{Canvas, Rectangle},
    },
};

pub fn draw_ui(f: &mut Frame, state: &MarketState, agents: &[Agent], tick: u64) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(main_layout[0]);

    draw_memory_grid(f, content_layout[0], state, agents);
    draw_sidebar(f, content_layout[1], state, agents);
    draw_status_bar(f, main_layout[1], state, agents, tick);
}

fn draw_status_bar(f: &mut Frame, area: Rect, state: &MarketState, agents: &[Agent], tick: u64) {
    let total_wealth: f64 = agents.iter().map(|a| a.budget).sum();
    let occupancy = state.pages.iter().filter(|p| p.owner.is_some()).count();
    let total_pages = state.total_pages();
    let occupancy_pct = (occupancy as f64 / total_pages as f64) * 100.0;

    let text = vec![
        Line::from(vec![
            Span::styled(" Tick: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:<6} ", tick),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Price: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("${:<6.2} ", state.current_price),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(" Wealth: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("${:<8.0} ", total_wealth),
                Style::default().fg(Color::Green),
            ),
            Span::styled(" Usage: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:.1}% ", occupancy_pct),
                Style::default().fg(if occupancy_pct > 90.0 {
                    Color::Red
                } else {
                    Color::Blue
                }),
            ),
        ]),
        Line::from(vec![
            Span::styled(" Controls: ", Style::default().fg(Color::Yellow)),
            Span::raw("Press 'q' to quit"),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" System Status ")
        .style(Style::default());

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_memory_grid(f: &mut Frame, area: Rect, state: &MarketState, agents: &[Agent]) {
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" RAM Bazaar: Memory Map "),
        )
        .x_bounds([0.0, state.width as f64])
        .y_bounds([0.0, state.height as f64])
        .paint(|ctx| {
            for (i, page) in state.pages.iter().enumerate() {
                let x = (i % state.width) as f64;
                let y = (state.height - 1 - (i / state.width)) as f64; // Invert Y to draw top-down visually

                let color = if let Some(owner_id) = page.owner {
                    // Find owner strategy color
                    if let Some(agent) = agents.iter().find(|a| a.id == owner_id) {
                        agent.strategy.color()
                    } else {
                        Color::Gray
                    }
                } else {
                    Color::DarkGray
                };

                // Intensity based on rent relative to global price?
                // For now just solid blocks.
                ctx.draw(&Rectangle {
                    x,
                    y,
                    width: 1.0,
                    height: 1.0,
                    color,
                });
            }
        });

    f.render_widget(canvas, area);
}

fn draw_sidebar(f: &mut Frame, area: Rect, state: &MarketState, agents: &[Agent]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(10)])
        .split(area);

    // Price Chart
    let price_history: Vec<u64> = state.price_history.iter().map(|p| *p as u64).collect();
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title(" Memory Price Index ")
                .borders(Borders::ALL),
        )
        .data(&price_history)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(sparkline, chunks[0]);

    // Agent List
    let mut sorted_agents = agents.to_vec();
    sorted_agents.sort_by(|a, b| b.owned_pages.cmp(&a.owned_pages));

    let header = Row::new(vec![
        Cell::from("ID"),
        Cell::from("Strat"),
        Cell::from("Pages"),
        Cell::from("Budget"),
    ])
    .style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = sorted_agents
        .iter()
        .take(20)
        .map(|agent| {
            let style = Style::default().fg(agent.strategy.color());
            Row::new(vec![
                Cell::from(format!("{:02}", agent.id)),
                Cell::from(format!("{:?}", agent.strategy)),
                Cell::from(format!("{}", agent.owned_pages)),
                Cell::from(format!("{:.1}", agent.budget)),
            ])
            .style(style)
        })
        .collect();

    let widths = [
        Constraint::Length(4),
        Constraint::Length(8),
        Constraint::Length(6),
        Constraint::Length(8),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().title(" Top Agents ").borders(Borders::ALL));

    f.render_widget(table, chunks[1]);
}

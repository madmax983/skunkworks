use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders, Sparkline, Paragraph,
    },
    Frame,
};
use crate::model::{MarketState, Agent};

pub fn draw_ui(f: &mut Frame, state: &MarketState, agents: &[Agent]) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(f.area());

    draw_memory_grid(f, chunks[0], state, agents);
    draw_sidebar(f, chunks[1], state, agents);
}

fn draw_memory_grid(f: &mut Frame, area: Rect, state: &MarketState, agents: &[Agent]) {
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" RAM Bazaar: Memory Map "))
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
        .constraints([
            Constraint::Length(10),
            Constraint::Min(10),
        ])
        .split(area);

    // Price Chart
    let price_history: Vec<u64> = state.price_history.iter().map(|p| *p as u64).collect();
    let sparkline = Sparkline::default()
        .block(Block::default().title(" Memory Price Index ").borders(Borders::ALL))
        .data(&price_history)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(sparkline, chunks[0]);

    // Agent List
    let mut agent_lines = Vec::new();
    // Sort by owned pages
    let mut sorted_agents = agents.to_vec();
    sorted_agents.sort_by(|a, b| b.owned_pages.cmp(&a.owned_pages));

    for agent in sorted_agents.iter().take(20) {
        let style = Style::default().fg(agent.strategy.color());
        let line = Line::from(vec![
            Span::styled(format!("Agent {:02} [{:?}]", agent.id, agent.strategy), style),
            Span::raw(format!(": {} pages, ${:.1}", agent.owned_pages, agent.budget)),
        ]);
        agent_lines.push(line);
    }

    let paragraph = Paragraph::new(agent_lines)
        .block(Block::default().title(" Top Agents ").borders(Borders::ALL));

    f.render_widget(paragraph, chunks[1]);
}

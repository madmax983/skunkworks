use crate::model::{AgentState, Core, MarketState, ThreadAgent};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Row, Sparkline, Table},
    Frame,
};

pub fn draw_ui(f: &mut Frame, state: &MarketState, agents: &[ThreadAgent], cores: &[Core]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(10), // Market Graph
            Constraint::Min(0),     // Body
        ])
        .split(f.area());

    // Header
    let header_text = vec![
        Line::from(vec![
            Span::styled(
                " CPU CASINO ",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "| Tick: {} | Price: {:.2} | Active: {} | Killed: {} | Finished: {}",
                state.current_tick,
                state.current_price,
                state.active_thread_count,
                state.killed_count,
                state.finished_count
            )),
        ]),
        Line::from(vec![
            Span::styled(" Controls: ", Style::default().fg(Color::Yellow)),
            Span::raw("'q' to quit"),
        ]),
    ];
    let header = Paragraph::new(header_text).block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Market Graph
    let price_data: Vec<u64> = state
        .price_history
        .iter()
        .map(|&x| (x * 100.0) as u64)
        .collect();
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title(" Market Price (cents) ")
                .borders(Borders::ALL),
        )
        .data(&price_data)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(sparkline, chunks[1]);

    // Split Body into Cores and Agent List
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Agents
            Constraint::Percentage(50), // Cores
        ])
        .split(chunks[2]);

    // Agent List (Top 20 active)
    let header_cells = ["ID", "Strat", "Credit", "Work", "Urg", "State"];
    let header = Row::new(header_cells).style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = agents
        .iter()
        .filter(|a| a.state != AgentState::Finished && a.state != AgentState::Killed)
        .take(20) // Show top 20
        .map(|agent| {
            let ticks_remaining = agent.deadline.saturating_sub(state.current_tick);
            let urgency = if ticks_remaining == 0 {
                99.0
            } else {
                agent.work_remaining / ticks_remaining as f64
            };

            let color = match agent.state {
                AgentState::Running => Color::Green,
                AgentState::Killed => Color::Red,
                AgentState::Finished => Color::Blue,
                AgentState::Active => {
                    if urgency > 0.9 {
                        Color::Red
                    } else {
                        Color::White
                    }
                }
            };

            let cells = vec![
                format!("{:04}", agent.id),
                format!("{:?}", agent.strategy),
                format!("{:.1}", agent.credits),
                format!("{:.1}", agent.work_remaining),
                format!("{:.2}", urgency),
                format!("{:?}", agent.state),
            ];
            Row::new(cells).style(Style::default().fg(color))
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(6),
            Constraint::Length(10),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(6),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Active Threads ")
            .borders(Borders::ALL),
    );

    f.render_widget(table, body_chunks[0]);

    // Cores
    let core_list: Vec<ListItem> = cores
        .iter()
        .map(|core| {
            let content = if let Some(agent_id) = core.current_agent_id {
                format!(
                    " CORE {:02}: RUNNING Agent {:04} (Util: {:.1}%)",
                    core.id,
                    agent_id,
                    core.utilization * 100.0
                )
            } else {
                format!(
                    " CORE {:02}: IDLE (Util: {:.1}%)",
                    core.id,
                    core.utilization * 100.0
                )
            };

            let style = if core.current_agent_id.is_some() {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            ListItem::new(Line::from(content)).style(style)
        })
        .collect();

    let cores_widget =
        List::new(core_list).block(Block::default().title(" CPU Cores ").borders(Borders::ALL));

    f.render_widget(cores_widget, body_chunks[1]);
}

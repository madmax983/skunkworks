use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::model::{ProcessState, Scheduler};

pub fn draw_ui(f: &mut Frame, scheduler: &Scheduler) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Body
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    draw_header(f, scheduler, chunks[0]);
    draw_body(f, scheduler, chunks[1]);
    draw_footer(f, scheduler, chunks[2]);
}

fn draw_header(f: &mut Frame, scheduler: &Scheduler, area: Rect) {
    let price = scheduler.market.last_clearing_price;
    let text = vec![
        Line::from(vec![
            Span::raw("Quanta Market | Tick: "),
            Span::styled(format!("{}", scheduler.tick_count), Style::default().fg(Color::Yellow)),
            Span::raw(" | Clearing Price: "),
            Span::styled(format!("{:.2}", price), Style::default().fg(Color::Green)),
            Span::raw(" | Active Processes: "),
            Span::styled(format!("{}", scheduler.processes.len()), Style::default().fg(Color::Cyan)),
        ]),
    ];
    let block = Block::default().borders(Borders::ALL).title("Status");
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_body(f: &mut Frame, scheduler: &Scheduler, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Process List
            Constraint::Percentage(70), // Gantt / Details
        ])
        .split(area);

    draw_process_list(f, scheduler, chunks[0]);
    draw_gantt_chart(f, scheduler, chunks[1]);
}

fn draw_process_list(f: &mut Frame, scheduler: &Scheduler, area: Rect) {
    let items: Vec<ListItem> = scheduler.processes.iter()
        .filter(|p| p.state != ProcessState::Dead)
        .map(|p| {
            let color = match p.state {
                ProcessState::Running => Color::Green,
                ProcessState::Waiting => Color::Yellow,
                ProcessState::Zombie => Color::Red,
                ProcessState::Dead => Color::DarkGray,
            };

            let content = format!(
                "ID: {:03} | W: {:.1} | U: {:.2} | D: {}",
                p.id, p.wealth, p.urgency, p.deadline
            );

            ListItem::new(content).style(Style::default().fg(color))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Processes (Active)"));
    f.render_widget(list, area);
}

fn draw_gantt_chart(f: &mut Frame, scheduler: &Scheduler, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Timeline (increased height for visibility)
            Constraint::Min(0),    // Instructions
        ])
        .split(area);

    // History Timeline
    let mut spans = Vec::new();
    for (_tick, pid, color) in &scheduler.history {
        let char = if *pid == 0 { " " } else { "█" };
        spans.push(Span::styled(char, Style::default().fg(*color)));
    }

    let gantt = Paragraph::new(Line::from(spans))
        .block(Block::default().borders(Borders::ALL).title("CPU History (Gantt)"))
        .wrap(Wrap { trim: false });

    f.render_widget(gantt, layout[0]);

    let instructions_text = vec![
        Line::from(Span::styled("Controls:", Style::default().fg(Color::Yellow))),
        Line::from("  Space: Pause / Resume"),
        Line::from("  +    : Spawn Process"),
        Line::from("  -    : Kill Random Process"),
        Line::from("  b    : Burst (Spawn 10)"),
        Line::from("  q    : Quit"),
        Line::from(""),
        Line::from(Span::styled("Mechanism:", Style::default().fg(Color::Cyan))),
        Line::from("  Processes bid [Wealth * Urgency] for CPU time."),
        Line::from("  Winner pays 2nd highest price (Vickrey Auction)."),
        Line::from("  Running earns +10.0 Wealth."),
        Line::from("  Waiting costs -0.5 Wealth."),
        Line::from("  Zero Wealth -> Zombie."),
    ];

    let instructions = Paragraph::new(instructions_text)
        .block(Block::default().borders(Borders::ALL).title("Info"));
    f.render_widget(instructions, layout[1]);
}

fn draw_footer(f: &mut Frame, _scheduler: &Scheduler, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title("Log");
    let text = Paragraph::new("Events will appear here (Not implemented yet)").block(block);
    f.render_widget(text, area);
}

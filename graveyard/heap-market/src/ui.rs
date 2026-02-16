use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::simulation::Heap;

pub fn draw(f: &mut Frame, heap: &Heap) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(f.area());

    // 1. Render Heap Grid
    let mut lines = Vec::new();
    for y in 0..heap.height {
        let mut spans = Vec::new();
        for x in 0..heap.width {
            let idx = y * heap.width + x;
            if idx < heap.pages.len() {
                let page = &heap.pages[idx];
                if let Some(owner_id) = page.owner {
                    if let Some(agent) = heap.agents.get(&owner_id) {
                        // Deterministic color based on agent color_idx
                        let color = match agent.color_idx % 6 {
                            0 => Color::Red,
                            1 => Color::Green,
                            2 => Color::Yellow,
                            3 => Color::Blue,
                            4 => Color::Magenta,
                            5 => Color::Cyan,
                            _ => Color::White,
                        };
                        spans.push(Span::styled("█", Style::default().fg(color)));
                    } else {
                        // Zombie page (shouldn't happen)
                        spans.push(Span::raw("?"));
                    }
                } else {
                    // Empty Page. Visualize Heat/Price.
                    let char = if page.heat > 4.0 {
                        "@"
                    } else if page.heat > 2.0 {
                        "#"
                    } else if page.heat > 1.0 {
                        "%"
                    } else if page.heat > 0.5 {
                        "+"
                    } else if page.heat > 0.1 {
                        "-"
                    } else {
                        "."
                    };

                    // Heat color: Gray -> White -> Red
                    let fg = if page.heat > 3.0 {
                        Color::Red
                    } else if page.heat > 1.0 {
                        Color::Yellow
                    } else {
                        Color::DarkGray
                    };

                    spans.push(Span::styled(char, Style::default().fg(fg)));
                }
            }
        }
        lines.push(Line::from(spans));
    }

    let heap_block = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Heap Allocation Map "),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(heap_block, chunks[0]);

    // 2. Render Stats
    let total_pages = heap.pages.len();
    let occupied_pages = heap.pages.iter().filter(|p| p.owner.is_some()).count();
    let avg_rent: f32 = heap.pages.iter().map(|p| p.rent).sum::<f32>() / total_pages as f32;
    let max_heat: f32 = heap.pages.iter().map(|p| p.heat).fold(0.0, f32::max);

    let mut stats_lines = vec![
        Line::from(vec![
            Span::raw("Tick: "),
            Span::styled(
                format!("{}", heap.tick_count),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Total Memory: "),
            Span::styled(
                format!("{} pages", total_pages),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::raw("Used Memory:  "),
            Span::styled(
                format!("{} pages", occupied_pages),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::raw("Utilization:  "),
            Span::styled(
                format!(
                    "{:.1}%",
                    (occupied_pages as f32 / total_pages as f32) * 100.0
                ),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Avg Rent:     "),
            Span::styled(
                format!("{:.2}", avg_rent),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::raw("Max Heat:     "),
            Span::styled(format!("{:.2}", max_heat), Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::raw("OOM Kills:    "),
            Span::styled(
                format!("{}", heap.oom_count),
                Style::default().fg(Color::Red),
            ),
        ]),
        Line::from(""),
        Line::from("Active Processes:"),
    ];

    // List top 10 agents by balance
    let mut agents: Vec<_> = heap.agents.values().collect();
    agents.sort_by(|a, b| b.balance.partial_cmp(&a.balance).unwrap());

    for agent in agents.iter().take(15) {
        let color = match agent.color_idx % 6 {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Yellow,
            3 => Color::Blue,
            4 => Color::Magenta,
            5 => Color::Cyan,
            _ => Color::White,
        };
        stats_lines.push(Line::from(vec![
            Span::styled(format!("{:<8}", agent.name), Style::default().fg(color)),
            Span::raw(format!(" ${:<5.0} ({}p)", agent.balance, agent.size)),
        ]));
    }

    let stats_block = Paragraph::new(stats_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Kernel Monitor "),
    );

    f.render_widget(stats_block, chunks[1]);
}

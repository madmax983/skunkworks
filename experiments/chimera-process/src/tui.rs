use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};
use crate::world::World;

pub struct TuiApp {
    pub world: World,
    pub table_state: TableState,
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            table_state: TableState::default(),
        }
    }

    pub fn on_tick(&mut self) {
        self.world.update();
    }

    pub fn draw(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(f.area());

        self.draw_process_table(f, chunks[0]);
        self.draw_footer(f, chunks[1]);
    }

    fn draw_process_table(&mut self, f: &mut Frame, area: Rect) {
        let header_cells = ["PID", "Name", "CPU %", "Mem (MB)", "Energy", "Gene", "Last Op"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
        let header = Row::new(header_cells)
            .style(Style::default().bg(Color::Blue))
            .height(1)
            .bottom_margin(1);

        // Sort agents by CPU for interesting view
        let mut agents: Vec<_> = self.world.agents.values().collect();
        agents.sort_by(|a, b| b.last_stats.0.partial_cmp(&a.last_stats.0).unwrap());

        let rows = agents.iter().map(|agent| {
            let pid = agent.pid.to_string();
            let name = &agent.name;
            let cpu = format!("{:.1}", agent.last_stats.0);
            let mem = format!("{}", agent.last_stats.1 / 1024 / 1024);
            let energy = agent.vm.energy.to_string();
            let ip = format!("({},{})", agent.vm.ip.0, agent.vm.ip.1);

            // Get last op from DNA or history
            let op = if agent.vm.ip.0 < agent.vm.dna.helix.strands.len()
                     && agent.vm.ip.1 < agent.vm.dna.helix.strands[agent.vm.ip.0].genes.len() {
                 agent.vm.dna.helix.strands[agent.vm.ip.0].genes[agent.vm.ip.1].op.to_string()
            } else {
                 "HALT".to_string()
            };

            let style = if agent.last_stats.0 > 10.0 {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else if agent.last_stats.0 > 1.0 {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Gray)
            };

            let cells = vec![
                Cell::from(pid),
                Cell::from(name.as_str()),
                Cell::from(cpu),
                Cell::from(mem),
                Cell::from(energy),
                Cell::from(ip),
                Cell::from(op),
            ];
            Row::new(cells).style(style)
        });

        let widths = [
            Constraint::Length(8),
            Constraint::Length(20),
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Min(10),
        ];

        let t = Table::new(rows, widths)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Chimera Process Monitor"));

        f.render_stateful_widget(t, area, &mut self.table_state);
    }

    fn draw_footer(&mut self, f: &mut Frame, area: Rect) {
        let text = Line::from(vec![
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit"),
        ]);
        let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
        f.render_widget(p, area);
    }
}

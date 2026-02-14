use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use maat_engine::{ScalesOfMaat, Soul};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

struct App {
    scales: ScalesOfMaat,
    incoming_souls: Vec<Soul>,
    allocated_souls: Vec<Soul>,
    message: String,
    counter: u64,
}

impl App {
    fn new() -> Self {
        // Capacity 3600 (Highly composite number)
        Self {
            scales: ScalesOfMaat::new(3600),
            incoming_souls: Vec::new(),
            allocated_souls: Vec::new(),
            message: "Press <Space> to summon a Soul. <Enter> to Weigh Heart.".to_string(),
            counter: 1,
        }
    }

    fn summon_soul(&mut self) {
        let mut rng = rand::thread_rng();
        // Generate a random fraction < 1
        // Denom between 2 and 20
        let denom: u64 = rng.gen_range(2..21);
        // Numer between 1 and denom-1
        let numer: u64 = rng.gen_range(1..denom);

        let soul = Soul::new(self.counter, numer, denom);
        self.counter += 1;
        self.incoming_souls.push(soul);
        self.message = format!("Summoned Soul #{} with demand {}/{}", self.counter-1, numer, denom);
    }

    fn weigh_next_heart(&mut self) {
        if self.incoming_souls.is_empty() {
            self.message = "The Hall of Maat is empty.".to_string();
            return;
        }

        // Peek at the first soul
        let soul = self.incoming_souls[0].clone();

        match self.scales.weigh_heart(&soul) {
            Ok(_) => {
                self.message = format!("Soul #{} has been balanced.", soul.id);
                self.allocated_souls.push(soul);
                self.incoming_souls.remove(0);
            }
            Err(e) => {
                self.message = format!("Soul #{} is heavy: {}", soul.id, e);
                // Move to back of queue or discard?
                // Let's move to back to retry later
                let s = self.incoming_souls.remove(0);
                self.incoming_souls.push(s);
            }
        }
    }
}

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char(' ') => app.summon_soul(),
                    KeyCode::Enter => app.weigh_next_heart(),
                    _ => {}
                }
            }
        }
    }

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Body
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // Header
    let title = Paragraph::new("⚖️  THE MAAT ENGINE: Resource Allocation by Ancient Law ⚖️")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Body
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Queue
            Constraint::Percentage(70), // Scales
        ])
        .split(chunks[1]);

    // Queue
    let items: Vec<ListItem> = app.incoming_souls.iter().map(|s| {
        ListItem::new(format!("#{} - {}/{}", s.id, s.demand.numer(), s.demand.denom()))
    }).collect();

    let queue_block = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Souls Awaiting Judgment "))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(queue_block, body_chunks[0]);

    // Scales (Timeline Visualization)
    let scales_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Visualization
            Constraint::Percentage(50), // Details
        ])
        .split(body_chunks[1]);

    // Timeline Rendering
    render_timeline(f, scales_chunks[0], app);

    // Allocated List
    let allocated_items: Vec<ListItem> = app.allocated_souls.iter().rev().take(10).map(|s| {
        // Show Egyptian decomposition
        let ef = maat_engine::EgyptianFraction::from(s.demand.clone());
        ListItem::new(format!("#{} : {}", s.id, ef))
    }).collect();

    let allocated_block = List::new(allocated_items)
        .block(Block::default().borders(Borders::ALL).title(" The Field of Reeds (Allocated) "));
    f.render_widget(allocated_block, scales_chunks[1]);

    // Footer
    let footer = Paragraph::new(app.message.as_str())
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn render_timeline(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default().borders(Borders::ALL).title(" The Feather of Truth (Resource Timeline) ");
    f.render_widget(block.clone(), area);

    let inner_area = block.inner(area);
    let width = inner_area.width as usize;
    let height = inner_area.height as usize;

    if width == 0 || height == 0 { return; }

    let total_cells = width * height;
    let slots_per_cell = (app.scales.capacity as f64 / total_cells as f64).ceil() as usize;

    let mut spans = Vec::new();
    let mut current_line = Vec::new();

    for i in 0..total_cells {
        let start_slot = i * slots_per_cell;
        let end_slot = std::cmp::min((i + 1) * slots_per_cell, app.scales.capacity);

        if start_slot >= app.scales.capacity { break; }

        let mut occupied_count = 0;
        let mut last_soul_id = None;

        for slot_idx in start_slot..end_slot {
            if let Some(id) = app.scales.timeline[slot_idx] {
                occupied_count += 1;
                last_soul_id = Some(id);
            }
        }

        let char = if occupied_count == 0 {
            "·"
        } else if occupied_count >= (end_slot - start_slot) {
            "█"
        } else {
            "▒"
        };

        let color = if let Some(id) = last_soul_id {
            match id % 6 {
                0 => Color::Red,
                1 => Color::Green,
                2 => Color::Blue,
                3 => Color::Yellow,
                4 => Color::Magenta,
                5 => Color::Cyan,
                _ => Color::White,
            }
        } else {
            Color::DarkGray
        };

        current_line.push(Span::styled(char, Style::default().fg(color)));

        if current_line.len() >= width {
            spans.push(Line::from(current_line));
            current_line = Vec::new();
        }
    }
    if !current_line.is_empty() {
        spans.push(Line::from(current_line));
    }

    let p = Paragraph::new(spans).block(Block::default().borders(Borders::NONE));
    f.render_widget(p, inner_area);
}

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use nile_scheduler::Scheduler;
use num_rational::Ratio;
use ratatui::{
    prelude::*,
    widgets::*,
};
use std::{io, time::Duration};
use rand::Rng;

struct App {
    scheduler: Scheduler,
    should_quit: bool,
    input_mode: bool,
    input_buffer: String,
    message: String,
}

impl App {
    fn new() -> Self {
        Self {
            scheduler: Scheduler::new(),
            should_quit: false,
            input_mode: false,
            input_buffer: String::new(),
            message: "Welcome to the Nile Scheduler. Press 'a' to add a process, 'r' for random, 'q' to quit.".to_string(),
        }
    }

    fn on_key(&mut self, key: KeyEvent) {
        if self.input_mode {
            match key.code {
                KeyCode::Enter => {
                    self.add_process();
                    self.input_mode = false;
                }
                KeyCode::Esc => {
                    self.input_mode = false;
                    self.input_buffer.clear();
                    self.message = "Cancelled.".to_string();
                }
                KeyCode::Char(c) => {
                    self.input_buffer.push(c);
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Char('q') => self.should_quit = true,
                KeyCode::Char('a') => {
                    self.input_mode = true;
                    self.input_buffer.clear();
                    self.message = "Enter demand fraction (e.g. 1/3): ".to_string();
                }
                KeyCode::Char('r') => {
                    let mut rng = rand::thread_rng();
                    let den = rng.gen_range(2..10);
                    let num = 1;
                    let demand = Ratio::new(num, den);
                    match self.scheduler.allocate(demand) {
                        Ok(frac) => self.message = format!("Allocated {} ({})", demand, frac),
                        Err(e) => self.message = format!("Error: {}", e),
                    }
                }
                _ => {}
            }
        }
    }

    fn add_process(&mut self) {
        let parts: Vec<&str> = self.input_buffer.split('/').collect();
        let demand = if parts.len() == 2 {
            let n: u64 = parts[0].trim().parse().unwrap_or(0);
            let d: u64 = parts[1].trim().parse().unwrap_or(1);
            if d == 0 { Ratio::new(0, 1) } else { Ratio::new(n, d) }
        } else {
             let n: u64 = self.input_buffer.trim().parse().unwrap_or(0);
             Ratio::new(n, 1)
        };

        if *demand.numer() == 0 {
             self.message = "Invalid input. Use 'n/d'.".to_string();
             return;
        }

        match self.scheduler.allocate(demand) {
            Ok(frac) => self.message = format!("Allocated {} -> {}", demand, frac),
            Err(e) => self.message = format!("Allocation failed: {}", e),
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.on_key(key);
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // The Nile (Bar)
            Constraint::Min(0),    // List
            Constraint::Length(3), // Status
        ])
        .split(f.area());

    render_header(f, chunks[0]);
    render_nile_bar(f, chunks[1], &app.scheduler);
    render_process_list(f, chunks[2], &app.scheduler);
    render_status(f, chunks[3], app);
}

fn render_header(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("𓀀 THE NILE SCHEDULER 𓀀")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
}

fn render_nile_bar(f: &mut Frame, area: Rect, scheduler: &Scheduler) {
    // The Nile Bar Visualization
    let width = area.width as u64;

    let mut spans = Vec::new();

    for p in &scheduler.allocated {
        for &d in &p.allocated.parts {
            let fraction = 1.0 / (d as f64);
            let block_len_f = (width as f64 - 2.0) * fraction;
            let block_len = block_len_f.round() as usize;

            let color = match p.id % 6 {
                0 => Color::Red,
                1 => Color::Green,
                2 => Color::Blue,
                3 => Color::Magenta,
                4 => Color::Cyan,
                _ => Color::White,
            };

            let s = "█".repeat(block_len.max(1));
            spans.push(Span::styled(s, Style::default().fg(color)));
        }
    }

    // Remaining free space
    let free_ratio = scheduler.free_space.to_ratio();
    if *free_ratio.numer() > 0 {
        let free_frac = (*free_ratio.numer() as f64) / (*free_ratio.denom() as f64);
        let free_len_f = (width as f64 - 2.0) * free_frac;
        let free_len = free_len_f.round() as usize;
        let s = "░".repeat(free_len);
        spans.push(Span::styled(s, Style::default().fg(Color::DarkGray)));
    }

    let nile = Paragraph::new(Line::from(spans))
        .block(Block::default().title("Resource Allocation (The Nile)").borders(Borders::ALL));
    f.render_widget(nile, area);
}

fn render_process_list(f: &mut Frame, area: Rect, scheduler: &Scheduler) {
    let items: Vec<ListItem> = scheduler
        .allocated
        .iter()
        .rev()
        .take(10)
        .map(|p| {
            let content = format!("Proc #{}: {} -> {}", p.id, p.demand, p.allocated);
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Scribe's Log").borders(Borders::ALL));
    f.render_widget(list, area);
}

fn render_status(f: &mut Frame, area: Rect, app: &App) {
    let status_text = if app.input_mode {
        format!("Input (n/d): {}_", app.input_buffer)
    } else {
        app.message.clone()
    };
    let status = Paragraph::new(status_text)
        .style(if app.input_mode {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        })
        .block(Block::default().title("Status").borders(Borders::ALL));
    f.render_widget(status, area);
}

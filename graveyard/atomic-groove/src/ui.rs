use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
    Frame,
};
use std::{error::Error, io, sync::mpsc::Receiver, time::{Duration, Instant}};
use crate::simulation::SimulationEvent;

pub struct App {
    pub thread_states: Vec<ThreadState>,
    pub events_rx: Receiver<SimulationEvent>,
    pub should_quit: bool,
}

#[derive(Clone)]
pub struct ThreadState {
    pub id: usize,
    pub status: ThreadStatus,
    pub last_event_time: Instant,
    pub label: String,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ThreadStatus {
    Running,
    Waiting,
    Locked,
    Releasing,
}

impl App {
    pub fn new(num_threads: usize, events_rx: Receiver<SimulationEvent>) -> Self {
        let mut thread_states = Vec::new();
        for i in 0..num_threads {
            thread_states.push(ThreadState {
                id: i,
                status: ThreadStatus::Running,
                last_event_time: Instant::now(),
                label: format!("Thread {}", i),
            });
        }
        Self {
            thread_states,
            events_rx,
            should_quit: false,
        }
    }

    pub fn on_tick(&mut self) {
        // Process all pending events
        while let Ok(event) = self.events_rx.try_recv() {
            match event {
                SimulationEvent::LockAcquired(id) => {
                    if let Some(t) = self.thread_states.iter_mut().find(|t| t.id == id) {
                        t.status = ThreadStatus::Locked;
                        t.last_event_time = Instant::now();
                    }
                },
                SimulationEvent::LockReleased(id) => {
                    if let Some(t) = self.thread_states.iter_mut().find(|t| t.id == id) {
                        t.status = ThreadStatus::Releasing;
                        t.last_event_time = Instant::now();
                    }
                },
                SimulationEvent::Waiting(id) => {
                     if let Some(t) = self.thread_states.iter_mut().find(|t| t.id == id) {
                        t.status = ThreadStatus::Waiting;
                        t.last_event_time = Instant::now();
                    }
                },
            }
        }

        // Auto-decay status back to Running after a short while
        for t in &mut self.thread_states {
            if t.status == ThreadStatus::Releasing && t.last_event_time.elapsed() > Duration::from_millis(100) {
                t.status = ThreadStatus::Running;
            }
            // Also decay "Locked" to "Releasing" automatically if we missed the event? No, trust the events.
            // But what if "Waiting" gets stuck?
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|f| ui(f, &self))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        self.should_quit = true;
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }

            if self.should_quit {
                break;
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ].as_ref())
        .split(f.size());

    let title = Paragraph::new("ATOMIC GROOVE: Thread Synchronization Sonifier")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let thread_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            std::iter::repeat(Constraint::Length(3))
                .take(app.thread_states.len())
                .collect::<Vec<_>>()
        )
        .split(chunks[1]);

    for (i, thread) in app.thread_states.iter().enumerate() {
        if i >= thread_chunks.len() { break; }

        let (color, status_text, ratio) = match thread.status {
            ThreadStatus::Running => (Color::Blue, "SLEEPING", 0.1),
            ThreadStatus::Waiting => (Color::Red, "WAITING (BLOCKED)", 0.5),
            ThreadStatus::Locked => (Color::Green, "LOCKED (PLAYING)", 1.0),
            ThreadStatus::Releasing => (Color::Yellow, "RELEASING", 0.8),
        };

        let gauge = Gauge::default()
            .block(Block::default().title(thread.label.as_str()).borders(Borders::ALL))
            .gauge_style(Style::default().fg(color))
            .ratio(ratio)
            .label(Span::styled(status_text, Style::default().fg(Color::White)));

        f.render_widget(gauge, thread_chunks[i]);
    }

    let footer = Paragraph::new("Press 'q' to stop recording and exit.")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(footer, chunks[2]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn test_app_logic() {
        let (tx, rx) = mpsc::channel();
        let mut app = App::new(2, rx);

        // Initial state
        assert_eq!(app.thread_states[0].status, ThreadStatus::Running);

        // Send event
        tx.send(SimulationEvent::Waiting(0)).unwrap();
        app.on_tick();
        assert_eq!(app.thread_states[0].status, ThreadStatus::Waiting);

        tx.send(SimulationEvent::LockAcquired(0)).unwrap();
        app.on_tick();
        assert_eq!(app.thread_states[0].status, ThreadStatus::Locked);
    }
}

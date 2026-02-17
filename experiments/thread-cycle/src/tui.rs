use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Gauge},
    Frame,
};
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use crate::audio::AudioEvent;

pub struct ThreadState {
    pub id: usize,
    pub phase: ThreadPhase,
    pub last_update: Instant,
    pub cycle_duration: Duration,
    pub work_duration: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThreadPhase {
    Resting,
    Waiting,
    Blocked,
    Working,
}

pub struct App {
    pub threads: Vec<ThreadState>,
    pub logs: VecDeque<String>,
    pub start_time: Instant,
}

impl App {
    pub fn new(thread_configs: Vec<(usize, Duration, Duration)>) -> Self {
        let now = Instant::now();
        let threads = thread_configs.into_iter().map(|(id, cycle, work)| {
            ThreadState {
                id,
                phase: ThreadPhase::Resting,
                last_update: now,
                cycle_duration: cycle,
                work_duration: work,
            }
        }).collect();

        Self {
            threads,
            logs: VecDeque::with_capacity(20),
            start_time: now,
        }
    }

    pub fn on_event(&mut self, event: AudioEvent) {
        let now = Instant::now();
        let (id, phase, msg) = match event {
            AudioEvent::LockAttempt(id) => (id, ThreadPhase::Waiting, "Attempting Lock"),
            AudioEvent::Blocked(id) => (id, ThreadPhase::Blocked, "Blocked!"),
            AudioEvent::LockAcquired(id) => (id, ThreadPhase::Working, "Acquired Lock"),
            AudioEvent::LockReleased(id) => (id, ThreadPhase::Resting, "Released Lock"),
            AudioEvent::WorkTick(id) => (id, ThreadPhase::Working, "Working..."),
        };

        if let Some(thread) = self.threads.iter_mut().find(|t| t.id == id) {
            thread.phase = phase;
            thread.last_update = now;
        }

        self.logs.push_front(format!("{:0.2}s: Thread {} - {}", now.duration_since(self.start_time).as_secs_f32(), id, msg));
        if self.logs.len() > 20 {
            self.logs.pop_back();
        }
    }
}

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.area());

    // Draw Threads
    let thread_constraints: Vec<Constraint> = app.threads.iter().map(|_| Constraint::Ratio(1, app.threads.len() as u32)).collect();
    let thread_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(thread_constraints)
        .split(chunks[0]);

    for (i, thread) in app.threads.iter().enumerate() {
        let (color, label, _ratio) = match thread.phase {
            ThreadPhase::Resting => (Color::Gray, "Resting", 0.0),
            ThreadPhase::Waiting => (Color::Yellow, "Waiting", 0.5),
            ThreadPhase::Blocked => (Color::Red, "BLOCKED", 1.0),
            ThreadPhase::Working => (Color::Green, "WORKING", 1.0),
        };

        // Calculate progress for "Resting" phase based on time
        let elapsed = thread.last_update.elapsed();
        let display_ratio = if thread.phase == ThreadPhase::Resting {
            let total_rest = thread.cycle_duration.saturating_sub(thread.work_duration);
            if total_rest.as_millis() > 0 {
                (elapsed.as_millis() as f64 / total_rest.as_millis() as f64).min(1.0)
            } else {
                0.0
            }
        } else if thread.phase == ThreadPhase::Working {
            (elapsed.as_millis() as f64 / thread.work_duration.as_millis() as f64).min(1.0)
        } else {
             // Pulse effect for Waiting/Blocked
             (elapsed.as_millis() as f64 / 100.0).sin().abs()
        };

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(format!("Thread {} ({}ms / {}ms)", thread.id, thread.cycle_duration.as_millis(), thread.work_duration.as_millis())))
            .gauge_style(Style::default().fg(color).add_modifier(Modifier::BOLD))
            .ratio(display_ratio)
            .label(format!("{} {:.1}s", label, elapsed.as_secs_f32()));

        f.render_widget(gauge, thread_chunks[i]);
    }

    // Draw Logs
    let logs: Vec<ListItem> = app.logs.iter().map(|s| ListItem::new(Line::from(s.as_str()))).collect();
    let log_list = List::new(logs)
        .block(Block::default().borders(Borders::ALL).title("Events"));
    f.render_widget(log_list, chunks[1]);
}

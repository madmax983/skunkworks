use crate::parser::MusicalEvent;
use std::time::Instant;

pub struct App {
    pub events: Vec<MusicalEvent>,
    pub event_start_times: Vec<f32>,
    pub start_time: Option<Instant>,
    pub current_event_index: usize,
    pub is_playing: bool,
    pub total_duration: f32,
    pub elapsed: f32,
}

impl App {
    pub fn new(events: Vec<MusicalEvent>) -> Self {
        let mut total_duration = 0.0;
        let mut event_start_times = Vec::new();

        for event in &events {
            event_start_times.push(total_duration);
            total_duration += event.duration;
        }

        Self {
            events,
            event_start_times,
            start_time: None,
            current_event_index: 0,
            is_playing: false,
            total_duration,
            elapsed: 0.0,
        }
    }

    pub fn start(&mut self) {
        self.is_playing = true;
        self.start_time = Some(Instant::now());
    }

    pub fn update(&mut self) {
        if !self.is_playing {
            return;
        }

        if let Some(start) = self.start_time {
             let now = Instant::now();
             let duration = now.duration_since(start);
             self.elapsed = duration.as_secs_f32();

             // Update index
             if self.elapsed >= self.total_duration {
                 self.is_playing = false;
                 self.current_event_index = self.events.len();
             } else {
                 // Binary search or linear scan
                 // Linear is fine for small N, but binary is better.
                 // `event_start_times` is sorted.
                 match self.event_start_times.binary_search_by(|t| t.partial_cmp(&self.elapsed).unwrap()) {
                     Ok(idx) => self.current_event_index = idx,
                     Err(idx) => self.current_event_index = idx.saturating_sub(1),
                 }
             }
        }
    }

    pub fn tick(&mut self, dt: f32) {
        if !self.is_playing {
            return;
        }
        self.elapsed += dt;

        if self.elapsed >= self.total_duration {
            self.is_playing = false;
            self.current_event_index = self.events.len();
        } else {
             match self.event_start_times.binary_search_by(|t| t.partial_cmp(&self.elapsed).unwrap()) {
                 Ok(idx) => self.current_event_index = idx,
                 Err(idx) => self.current_event_index = idx.saturating_sub(1),
             }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Timbre;

    #[test]
    fn test_app_logic() {
        let events = vec![
            MusicalEvent {
                frequency: 440.0,
                duration: 1.0,
                timbre: Timbre::Sine,
                description: "A".to_string(),
            },
            MusicalEvent {
                frequency: 880.0,
                duration: 1.0,
                timbre: Timbre::Sine,
                description: "B".to_string(),
            },
        ];

        let mut app = App::new(events);
        app.start();

        assert!(app.is_playing);
        assert_eq!(app.current_event_index, 0);

        app.tick(0.5);
        assert_eq!(app.current_event_index, 0);

        app.tick(1.0);
        assert_eq!(app.current_event_index, 1);

        app.tick(1.0);
        assert!(!app.is_playing);
    }
}

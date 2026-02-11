use anyhow::Result;
use crossterm::event::Event;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{Duration, Instant};

/// A single event stamped with the relative time since recording started.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedEvent {
    /// Time elapsed since the start of recording.
    pub time: Duration,
    /// The event that occurred.
    pub event: Event,
}

#[derive(Serialize, Deserialize)]
pub struct FlightData {
    pub seed: u64,
    pub events: Vec<RecordedEvent>,
}

impl FlightData {
    pub fn save(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &str) -> Result<Self> {
        let json = fs::read_to_string(path)?;
        let data = serde_json::from_str(&json)?;
        Ok(data)
    }
}

/// Records a sequence of input events with timestamps.
///
/// Use this to capture a user's session.
#[derive(Default)]
pub struct Recorder {
    start_time: Option<Instant>,
    /// The list of recorded events.
    pub events: Vec<RecordedEvent>,
}

impl Recorder {
    /// Creates a new, empty recorder.
    pub fn new() -> Self {
        Self {
            start_time: None,
            events: Vec::new(),
        }
    }

    /// Starts the recording timer and clears any previous events.
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.events.clear();
    }

    /// Records an event with the current timestamp relative to start time.
    ///
    /// If `start()` has not been called, this does nothing.
    pub fn record(&mut self, event: Event) {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            self.events.push(RecordedEvent {
                time: elapsed,
                event,
            });
        }
    }
}

/// Replays a sequence of recorded events with correct timing.
///
/// Use this to feed recorded inputs back into an application.
pub struct Replayer {
    start_time: Option<Instant>,
    events: Vec<RecordedEvent>,
    cursor: usize,
}

impl Replayer {
    /// Creates a replayer from a list of recorded events.
    pub fn new(events: Vec<RecordedEvent>) -> Self {
        Self {
            start_time: None,
            events,
            cursor: 0,
        }
    }

    /// Starts the replay timer.
    ///
    /// This resets the cursor to the beginning.
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.cursor = 0;
    }

    /// Checks if the next event is ready to be fired.
    ///
    /// Returns `Some(Event)` if the time elapsed since `start()` is greater than or equal to
    /// the timestamp of the next event in the queue. Returns `None` if it's not time yet,
    /// or if all events have been replayed.
    pub fn poll(&mut self) -> Option<Event> {
        let start = self.start_time?;

        if self.cursor >= self.events.len() {
            return None;
        }

        let next_event = &self.events[self.cursor];
        if start.elapsed() >= next_event.time {
            self.cursor += 1;
            Some(next_event.event.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_flight_data_serialization() {
        let event = RecordedEvent {
            time: Duration::from_secs(1),
            event: Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)),
        };
        let data = FlightData {
            seed: 12345,
            events: vec![event],
        };

        let json = serde_json::to_string(&data).unwrap();
        let decoded: FlightData = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded.seed, 12345);
        assert_eq!(decoded.events.len(), 1);
        match &decoded.events[0].event {
            Event::Key(k) => match k.code {
                KeyCode::Char(c) => assert_eq!(c, 'a'),
                _ => panic!("Wrong key code"),
            },
            _ => panic!("Wrong event type"),
        }
    }
}

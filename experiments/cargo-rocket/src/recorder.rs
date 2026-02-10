use crate::ghost::RecordedEvent;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ghost::{GhostEvent, GhostKeyCode, GhostKeyEvent, GhostKeyEventKind};
    use std::time::Duration;

    #[test]
    fn test_flight_data_serialization() {
        let event = RecordedEvent {
            time: Duration::from_secs(1),
            event: GhostEvent::Key(GhostKeyEvent {
                code: GhostKeyCode::Char('a'),
                modifiers: 0,
                kind: GhostKeyEventKind::Press,
                state: 0,
            }),
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
            GhostEvent::Key(k) => match k.code {
                GhostKeyCode::Char(c) => assert_eq!(c, 'a'),
                _ => panic!("Wrong key code"),
            },
            _ => panic!("Wrong event type"),
        }
    }
}

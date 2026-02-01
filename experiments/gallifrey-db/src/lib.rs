use std::collections::HashMap;
use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GallifreyError {
    #[error("unknown error")]
    Unknown,
}

#[derive(Debug, Clone)]
struct Entry {
    value: String,
    timestamp: DateTime<Utc>,
}

pub struct GallifreyStore {
    data: HashMap<String, Vec<Entry>>,
}

impl GallifreyStore {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Appends a new version of the key.
    pub fn put(&mut self, key: String, value: String) {
        let entry = Entry {
            value,
            timestamp: Utc::now(),
        };
        self.data.entry(key).or_default().push(entry);
    }

    /// Returns the latest value for the key.
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).and_then(|history| {
            history.last().map(|entry| entry.value.clone())
        })
    }

    /// Returns the value of the key effective at the given timestamp.
    /// This finds the latest entry where entry.timestamp <= timestamp.
    pub fn get_at(&self, key: &str, timestamp: DateTime<Utc>) -> Option<String> {
        let history = self.data.get(key)?;

        // Binary search could be used here for performance, but linear scan reverse is fine for MVP
        for entry in history.iter().rev() {
            if entry.timestamp <= timestamp {
                return Some(entry.value.clone());
            }
        }
        None
    }

    /// Returns the full history of a key
    pub fn history(&self, key: &str) -> Option<Vec<(DateTime<Utc>, String)>> {
        self.data.get(key).map(|history| {
            history.iter().map(|e| (e.timestamp, e.value.clone())).collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_basic_put_get() {
        let mut store = GallifreyStore::new();
        store.put("user:1".to_string(), "Alice".to_string());
        assert_eq!(store.get("user:1"), Some("Alice".to_string()));
    }

    #[test]
    fn test_time_travel() {
        let mut store = GallifreyStore::new();

        store.put("stock".to_string(), "100".to_string());
        let t1 = Utc::now();

        // Sleep to ensure timestamp difference
        thread::sleep(Duration::from_millis(10));

        store.put("stock".to_string(), "105".to_string());
        let t2 = Utc::now();

        thread::sleep(Duration::from_millis(10));

        store.put("stock".to_string(), "110".to_string());

        // Latest
        assert_eq!(store.get("stock"), Some("110".to_string()));

        // Time Travel
        assert_eq!(store.get_at("stock", t1), Some("100".to_string()));
        assert_eq!(store.get_at("stock", t2), Some("105".to_string()));
    }
}

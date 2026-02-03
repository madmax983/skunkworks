use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Mood {
    Happy,
    Neutral,
    Sad,
    Dead,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pet {
    pub name: String,
    pub xp: u64,
    pub hunger: f64, // 0.0 to 100.0 (100 = starving)
    pub mood: Mood,
    pub last_fed: i64, // Unix timestamp
    pub birth_date: i64,
    #[serde(skip)]
    pub activity_today: usize,
}

impl Default for Pet {
    fn default() -> Self {
        let now = Utc::now().timestamp();
        Self {
            name: "Git-Gotchi".to_string(),
            xp: 0,
            hunger: 0.0,
            mood: Mood::Happy,
            last_fed: now,
            birth_date: now,
            activity_today: 0,
        }
    }
}

impl Pet {
    #[allow(dead_code)]
    pub fn new(name: String) -> Self {
        let mut pet = Pet::default();
        pet.name = name;
        pet
    }

    pub fn load() -> Result<Self> {
        let path = Path::new(".git-gotchi.json");
        if path.exists() {
            let content = fs::read_to_string(path).context("Failed to read pet file")?;
            let pet: Pet = serde_json::from_str(&content).context("Failed to parse pet file")?;
            Ok(pet)
        } else {
            Ok(Pet::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(".git-gotchi.json", content)?;
        Ok(())
    }

    pub fn update(&mut self, last_commit_time: i64, activity_today: usize) {
        let now = Utc::now().timestamp();
        self.activity_today = activity_today;

        // Calculate time delta since we last saw the pet (or last saved)
        // We actually want to calculate hunger based on time since *last fed*.

        // If there was a commit *after* the last time we fed it:
        if last_commit_time > self.last_fed {
            // Feeding time!
            let commits_count = 1; // Simplified, we just know there's at least one new one.
            self.feed(commits_count);
            self.last_fed = last_commit_time;
        }

        // Decay logic
        // Hunger increases by 10 every hour (3600 seconds)
        let time_since_fed = (now - self.last_fed).max(0);
        let hours_since_fed = time_since_fed as f64 / 3600.0;

        self.hunger = (hours_since_fed * 5.0).min(100.0);

        // Mood logic
        self.mood = if self.hunger < 20.0 {
            Mood::Happy
        } else if self.hunger < 60.0 {
            Mood::Neutral
        } else if self.hunger < 90.0 {
            Mood::Sad
        } else {
            Mood::Dead
        };
    }

    pub fn feed(&mut self, amount: u64) {
        self.hunger = (self.hunger - (amount as f64 * 10.0)).max(0.0);
        self.xp += amount * 10;
        // Resetting last_fed is handled in update usually, but if manually fed:
        // self.last_fed = Utc::now().timestamp();
        // But here we rely on git timestamps.
    }

    pub fn get_level(&self) -> u64 {
        (self.xp as f64).sqrt() as u64 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pet() {
        let pet = Pet::new("TestBot".to_string());
        assert_eq!(pet.name, "TestBot");
        assert_eq!(pet.hunger, 0.0);
    }

    #[test]
    fn test_update_hunger() {
        let mut pet = Pet::new("TestBot".to_string());
        pet.last_fed = Utc::now().timestamp() - 7200; // 2 hours ago

        // Update with no new commits
        pet.update(pet.last_fed - 100, 0);

        // Hunger should be around 10 (5 per hour * 2 hours)
        assert!(pet.hunger >= 9.9 && pet.hunger <= 10.1);
    }

    #[test]
    fn test_feeding() {
        let mut pet = Pet::new("TestBot".to_string());
        pet.hunger = 50.0;
        pet.feed(2); // Feed 2 "units"

        assert_eq!(pet.hunger, 30.0);
        assert_eq!(pet.xp, 20);
    }
}

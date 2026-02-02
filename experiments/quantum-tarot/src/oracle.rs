use crate::cards::{Card, CARDS};
use rand::seq::SliceRandom;
use std::process::Command;
use std::fs;

pub struct Reading {
    pub past: Card,
    pub present: Card,
    pub future: Card,
}

pub struct Oracle;

impl Oracle {
    pub fn consult() -> Reading {
        let mut rng = rand::thread_rng();
        Reading {
            past: Self::draw_past(&mut rng),
            present: Self::draw_present(&mut rng),
            future: Self::draw_future(&mut rng),
        }
    }

    fn draw_past(rng: &mut impl rand::Rng) -> Card {
        // Attempt to determine "karma" from git
        // If many commits in last 24h -> The Merge
        // If error/no git -> Random
        if let Ok(output) = Command::new("git")
            .args(["log", "--since=24.hours", "--oneline"])
            .output()
        {
            let count = String::from_utf8_lossy(&output.stdout).lines().count();
            if count > 10 {
                return Self::find_card("The Merge");
            } else if count == 0 {
                return Self::find_card("The Legacy");
            }
        }

        CARDS.choose(rng).unwrap().clone()
    }

    fn draw_present(rng: &mut impl rand::Rng) -> Card {
        // Count files in src
        if let Ok(entries) = fs::read_dir("src") {
            let count = entries.count();
            if count > 20 {
                 return Self::find_card("The Architect");
            } else if count < 3 {
                 return Self::find_card("The Green Build");
            }
        }

        CARDS.choose(rng).unwrap().clone()
    }

    fn draw_future(rng: &mut impl rand::Rng) -> Card {
        // Pure entropy
        CARDS.choose(rng).unwrap().clone()
    }

    fn find_card(name: &str) -> Card {
        CARDS.iter().find(|c| c.name == name).unwrap_or(&CARDS[0]).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consult_returns_three_cards() {
        let reading = Oracle::consult();
        assert!(!reading.past.name.is_empty());
        assert!(!reading.present.name.is_empty());
        assert!(!reading.future.name.is_empty());
    }

    #[test]
    fn test_find_card() {
        let card = Oracle::find_card("The Bug");
        assert_eq!(card.name, "The Bug");
    }
}

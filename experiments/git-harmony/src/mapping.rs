use resonance_audio::audio::AudioCommand;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct GitEvent {
    pub file_path: String,
    pub insertions: usize,
    pub deletions: usize,
    pub author: String,
    pub timestamp: i64,
}

pub fn map_event_to_command(event: &GitEvent, grid_w: usize, grid_h: usize) -> AudioCommand {
    let mut hasher = DefaultHasher::new();
    event.file_path.hash(&mut hasher);
    let h1 = hasher.finish();

    // Salt for Y coordinate to avoid diagonal clustering
    hasher.write_u8(0xFF);
    let h2 = hasher.finish();

    let x = (h1 as usize) % grid_w;
    let y = (h2 as usize) % grid_h;

    // Calculate strength based on churn
    let churn = event.insertions + event.deletions;
    let strength = (churn as f32 / 50.0).min(1.0).max(0.1);

    // If deletions dominate significantly, maybe create a "Void" or a very low frequency tone
    if event.deletions > event.insertions + 10 {
        // Big deletion
        // Let's make it a strong Pluck but negative? Or maybe add a Wall temporarily?
        // Let's stick to Pluck for simplicity in first pass, but maybe negative strength?
        // PhysicsGrid adds strength, so negative might create a "dip".
        AudioCommand::Pluck {
            x,
            y,
            strength: -strength,
        }
    } else {
        AudioCommand::Pluck {
            x,
            y,
            strength,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping_basic() {
        let event = GitEvent {
            file_path: "src/main.rs".to_string(),
            insertions: 10,
            deletions: 2,
            author: "Genesis".to_string(),
            timestamp: 1234567890,
        };

        let cmd = map_event_to_command(&event, 100, 100);

        match cmd {
            AudioCommand::Pluck { x, y, strength } => {
                // Verify deterministic coordinates
                assert!(x < 100);
                assert!(y < 100);
                assert!(strength > 0.0);
            }
            _ => panic!("Expected Pluck command"),
        }
    }

    #[test]
    fn test_mapping_deletions() {
        let event = GitEvent {
            file_path: "src/old_code.rs".to_string(),
            insertions: 0,
            deletions: 100,
            author: "Genesis".to_string(),
            timestamp: 1234567890,
        };

        let cmd = map_event_to_command(&event, 100, 100);

        match cmd {
            AudioCommand::Pluck { strength, .. } => {
                // Expect negative strength or high magnitude
                assert!(strength < 0.0);
            }
            _ => {},
        }
    }
}

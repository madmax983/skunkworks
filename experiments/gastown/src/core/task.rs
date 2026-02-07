use serde::{Deserialize, Serialize};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub status: String,
}

pub fn generate_id(prefix: &str) -> String {
    let rng = thread_rng();
    let suffix: String = rng
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();
    format!("{}-{}", prefix, suffix).to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let id = generate_id("test");
        assert!(id.starts_with("test-"));
        assert_eq!(id.len(), 4 + 1 + 5); // prefix + - + 5 chars
    }
}

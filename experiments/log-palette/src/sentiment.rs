#[derive(Default)]
pub struct SentimentAnalyzer;

impl SentimentAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn score(&self, text: &str) -> f32 {
        let lower = text.to_lowercase();
        let mut score: f32 = 0.0;

        // Simple bag-of-words approach
        // We could use a HashSet for lookup, but for a small list, matching is fine.

        for word in lower.split_whitespace() {
            // Cleanup punctuation
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());

            let word_score = match clean_word {
                // Negative
                "error" | "err" | "fail" | "failed" | "failure" | "fatal" | "panic"
                | "critical" => -1.0,
                "exception" | "denied" | "timeout" | "unreachable" | "broken" | "crash" => -0.8,
                "warn" | "warning" | "bad" | "wrong" | "invalid" | "missing" => -0.5,
                "slow" | "lag" | "latency" | "retry" => -0.3,

                // Positive
                "success" | "succeeded" | "ok" | "okay" | "completed" | "verified" => 1.0,
                "connected" | "restored" | "fixed" | "stable" | "ready" => 0.8,
                "good" | "valid" | "passed" | "allow" | "allowed" => 0.5,
                "start" | "started" | "new" | "create" | "created" => 0.2,

                // Neutral / Structural
                "info" | "debug" | "trace" => 0.0,

                _ => 0.0,
            };

            if word_score != 0.0 {
                score += word_score;
            }
        }

        // Normalize?
        // If we have "error error error", score is -3.0. We should clamp or average.
        // Averaging might dilute a single error in a long message.
        // Summing might overblow.
        // Let's use a soft clamp (tanh-like) or just hard clamp.

        score.clamp(-1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_sentiment() {
        let analyzer = SentimentAnalyzer::new();
        assert!(analyzer.score("Critical error in system") < -0.5);
        assert!(analyzer.score("Connection timeout") < 0.0);
    }

    #[test]
    fn test_positive_sentiment() {
        let analyzer = SentimentAnalyzer::new();
        assert!(analyzer.score("Operation completed successfully") > 0.5);
        assert!(analyzer.score("System is stable") > 0.0);
    }

    #[test]
    fn test_neutral_sentiment() {
        let analyzer = SentimentAnalyzer::new();
        assert_eq!(analyzer.score("Just some info log"), 0.0);
    }

    #[test]
    fn test_mixed_sentiment() {
        let analyzer = SentimentAnalyzer::new();
        // "failed" (-1.0) + "success" (1.0) -> 0.0
        assert_eq!(analyzer.score("Task failed but then success"), 0.0);
    }
}

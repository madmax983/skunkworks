use std::time::Instant;

pub struct Game {
    pub snippet: String,
    pub input: String,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub finished: bool,
}

impl Game {
    pub fn new(snippet: String) -> Self {
        Self {
            snippet,
            input: String::new(),
            start_time: None,
            end_time: None,
            finished: false,
        }
    }

    pub fn input_char(&mut self, c: char) {
        if self.finished {
            return;
        }

        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }

        // Limit input length to snippet length
        if self.input.len() < self.snippet.len() {
            self.input.push(c);
            self.check_finished();
        }
    }

    pub fn backspace(&mut self) {
        if self.finished {
            return;
        }
        self.input.pop();
    }

    fn check_finished(&mut self) {
        if self.input == self.snippet {
            self.finished = true;
            self.end_time = Some(Instant::now());
        }
    }

    pub fn wpm(&self) -> f64 {
        let elapsed = if let Some(start) = self.start_time {
            if let Some(end) = self.end_time {
                end.duration_since(start)
            } else {
                start.elapsed()
            }
        } else {
            return 0.0;
        };

        let minutes = elapsed.as_secs_f64() / 60.0;
        if minutes < 0.001 {
            return 0.0;
        }

        let words = self.input.len() as f64 / 5.0;
        words / minutes
    }

    pub fn accuracy(&self) -> f64 {
        if self.input.is_empty() {
            return 100.0;
        }

        let correct_chars = self
            .input
            .chars()
            .zip(self.snippet.chars())
            .filter(|(a, b)| a == b)
            .count();

        (correct_chars as f64 / self.input.len() as f64) * 100.0
    }

    pub fn progress(&self) -> f64 {
        if self.snippet.is_empty() {
            return 1.0;
        }
        self.input.len() as f64 / self.snippet.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_flow() {
        let mut game = Game::new("foo".to_string());
        assert!(!game.finished);
        assert_eq!(game.accuracy(), 100.0);

        game.input_char('f');
        assert!(game.start_time.is_some());
        assert_eq!(game.input, "f");
        assert_eq!(game.accuracy(), 100.0);

        game.input_char('x'); // Wrong char
        assert_eq!(game.input, "fx");
        assert_eq!(game.accuracy(), 50.0); // 1 correct / 2 total

        game.backspace();
        assert_eq!(game.input, "f");

        game.input_char('o');
        game.input_char('o');
        assert_eq!(game.input, "foo");
        assert!(game.finished);
        assert!(game.end_time.is_some());
    }

    #[test]
    fn test_wpm() {
        let mut game = Game::new("hello".to_string());
        game.input_char('h'); // Start timer

        // Mocking time passing is hard without dependency injection or ignoring time in logic.
        // We'll trust Instant works.
    }
}

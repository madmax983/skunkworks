use rand::Rng;

#[derive(Debug, Clone)]
pub struct Word {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub speed: f64,
}

pub struct Game {
    pub words: Vec<Word>,
    pub input: String,
    pub score: u32,
    pub game_over: bool,
    pub width: u16,
    pub height: u16,
}

impl Game {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            words: Vec::new(),
            input: String::new(),
            score: 0,
            game_over: false,
            width,
            height,
        }
    }

    pub fn spawn_word(&mut self, text: String) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..(self.width as f64 - text.len() as f64).max(0.0));
        let speed = rng.gen_range(0.1..0.5); // Slow fall

        self.words.push(Word {
            text,
            x,
            y: 0.0,
            speed,
        });
    }

    /// For testing, explicit spawn
    pub fn spawn_word_at(&mut self, text: String, x: f64, speed: f64) {
        self.words.push(Word {
            text,
            x,
            y: 0.0,
            speed,
        });
    }

    pub fn tick(&mut self) {
        if self.game_over {
            return;
        }

        for word in &mut self.words {
            word.y += word.speed;
        }

        // Check for game over (word hit bottom)
        if self.words.iter().any(|w| w.y >= self.height as f64) {
            self.game_over = true;
        }
    }

    pub fn input_char(&mut self, c: char) {
        if self.game_over {
            return;
        }
        self.input.push(c);
        self.check_input();
    }

    pub fn backspace(&mut self) {
        if self.game_over {
            return;
        }
        self.input.pop();
    }

    fn check_input(&mut self) {
        if let Some(index) = self.words.iter().position(|w| w.text == self.input) {
            // Match found!
            self.score += self.input.len() as u32;
            self.input.clear();
            self.words.remove(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_and_tick() {
        let mut game = Game::new(100, 20);
        game.spawn_word_at("test".to_string(), 10.0, 1.0);

        assert_eq!(game.words.len(), 1);
        assert_eq!(game.words[0].y, 0.0);

        game.tick();
        assert_eq!(game.words[0].y, 1.0);
    }

    #[test]
    fn test_input_match() {
        let mut game = Game::new(100, 20);
        game.spawn_word_at("nova".to_string(), 10.0, 1.0);

        game.input_char('n');
        game.input_char('o');
        game.input_char('v');
        assert_eq!(game.words.len(), 1); // Not done yet

        game.input_char('a');
        assert_eq!(game.words.len(), 0); // Matched and removed
        assert_eq!(game.score, 4);
        assert_eq!(game.input.len(), 0); // Input cleared
    }

    #[test]
    fn test_game_over() {
        let mut game = Game::new(100, 10);
        game.spawn_word_at("drop".to_string(), 10.0, 10.0); // Fast drop

        game.tick(); // y becomes 10.0 >= height 10
        assert!(game.game_over);
    }
}

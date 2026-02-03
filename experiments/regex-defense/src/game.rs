use rand::Rng;
use regex::Regex;
use tui_shared::math::Vec2;

#[derive(Debug, Clone)]
pub struct Enemy {
    pub text: String,
    pub pos: Vec2,
    pub speed: f64,
    pub matched: bool,
}

pub struct Game {
    pub enemies: Vec<Enemy>,
    pub input: String,
    pub score: u32,
    pub game_over: bool,
    pub width: f64,
    pub height: f64,
    spawn_timer: f64,
    pub last_regex_error: Option<String>,
}

impl Game {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            enemies: Vec::new(),
            input: String::new(),
            score: 0,
            game_over: false,
            width,
            height,
            spawn_timer: 0.0,
            last_regex_error: None,
        }
    }

    pub fn update(&mut self, dt: f64) {
        if self.game_over {
            return;
        }

        // Spawn
        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 {
            self.spawn_enemy();
            // Spawning gets faster as score increases, capped at 0.5s
            let spawn_rate = (2.0 - (self.score as f64 * 0.05)).max(0.5);
            self.spawn_timer = spawn_rate;
        }

        // Move
        for enemy in &mut self.enemies {
            enemy.pos.y += enemy.speed * dt;
        }

        // Check fail
        for enemy in &self.enemies {
            if enemy.pos.y >= self.height {
                self.game_over = true;
            }
        }

        // Remove matched
        self.enemies.retain(|e| !e.matched);
    }

    fn spawn_enemy(&mut self) {
        let mut rng = rand::thread_rng();
        let words = vec![
            "user@example.com",
            "admin@corp.net",
            "info@site.org",
            "192.168.1.1",
            "10.0.0.5",
            "127.0.0.1",
            "https://rust-lang.org",
            "http://google.com",
            "ftp://files.com",
            "2023-10-27",
            "1999-01-01",
            "error: 404",
            "Warning: 500",
            "DEBUG: trace",
            "TODO: fixme",
            "123-456-7890",
            "(555) 123-4567",
            "#FFAA00",
            "#000000",
            "#FFFFFF",
            "foo",
            "bar",
            "baz",
            "qux",
            "Apple",
            "Banana",
            "Cherry",
        ];
        let text = words[rng.gen_range(0..words.len())].to_string();

        // Ensure within bounds
        let max_x = (self.width - text.len() as f64).max(0.0);
        let x = rng.gen_range(0.0..=max_x);

        self.enemies.push(Enemy {
            text,
            pos: Vec2::new(x, 0.0),
            speed: 2.0 + (self.score as f64 * 0.1), // Increase speed with score
            matched: false,
        });
    }

    pub fn input_char(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn input_backspace(&mut self) {
        self.input.pop();
    }

    pub fn submit_regex(&mut self) {
        if self.input.is_empty() {
            return;
        }

        match Regex::new(&self.input) {
            Ok(re) => {
                self.last_regex_error = None;
                let mut hit = false;
                for enemy in &mut self.enemies {
                    if re.is_match(&enemy.text) {
                        enemy.matched = true;
                        self.score += 10 + enemy.text.len() as u32;
                        hit = true;
                    }
                }
                if hit {
                    self.input.clear();
                }
            }
            Err(e) => {
                // Simplified error message
                self.last_regex_error = Some(format!("Invalid Regex: {}", e));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_matching() {
        let mut game = Game::new(100.0, 100.0);

        // Manually add an enemy
        game.enemies.push(Enemy {
            text: "test@example.com".to_string(),
            pos: Vec2::new(10.0, 10.0),
            speed: 1.0,
            matched: false,
        });

        // Test matching regex
        game.input = r".+@.+\..+".to_string();
        game.submit_regex();

        assert!(game.enemies[0].matched);
        assert!(game.last_regex_error.is_none());
        assert!(game.score > 0);
    }

    #[test]
    fn test_regex_failure() {
        let mut game = Game::new(100.0, 100.0);

        game.enemies.push(Enemy {
            text: "hello".to_string(),
            pos: Vec2::new(10.0, 10.0),
            speed: 1.0,
            matched: false,
        });

        // Test invalid regex
        game.input = r"[unclosed class".to_string();
        game.submit_regex();

        assert!(!game.enemies[0].matched);
        assert!(game.last_regex_error.is_some());
        assert_eq!(game.score, 0);
    }
}

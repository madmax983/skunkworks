use crate::diff::{DiffLine, get_diff};
use anyhow::Result;
use rand::Rng;

pub struct Bullet {
    pub x: f64,
    pub y: f64,
}

pub struct Enemy {
    pub x: f64,
    pub y: f64,
    pub line: DiffLine,
    pub width: usize,
    pub max_width: usize,
}

pub struct Game {
    pub player_x: f64,
    pub bullets: Vec<Bullet>,
    pub enemies: Vec<Enemy>,
    pub pending_lines: Vec<DiffLine>,
    pub score: usize,
    pub game_over: bool,
    pub width: f64,
    pub height: f64,
    pub tick_count: usize,
}

impl Game {
    pub fn new(width: f64, height: f64) -> Result<Self> {
        let lines = get_diff()?;
        Ok(Self::new_with_lines(width, height, lines))
    }

    pub fn new_with_lines(width: f64, height: f64, lines: Vec<DiffLine>) -> Self {
        Self {
            player_x: width / 2.0,
            bullets: Vec::new(),
            enemies: Vec::new(),
            pending_lines: lines,
            score: 0,
            game_over: false,
            width,
            height,
            tick_count: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.game_over { return; }
        self.tick_count += 1;

        // Spawn enemies
        // Spawn every 40 ticks (~0.6s)
        if self.tick_count % 40 == 0 && !self.pending_lines.is_empty() {
            let line = self.pending_lines.remove(0);
            let content_len = line.content.len();
            // Max width to fit on screen, wrap or truncate?
            // Truncate for now.
            let max_w = (self.width as usize).saturating_sub(4).max(1);
            let width = content_len.min(max_w);

            // Random x pos
            let mut rng = rand::thread_rng();
            // Ensure it fits
            let max_x = (self.width - width as f64).max(0.0);
            let x = rng.gen_range(0.0..=max_x);

            self.enemies.push(Enemy {
                x,
                y: 0.0,
                line,
                width,
                max_width: max_w,
            });
        }

        // Move bullets
        for b in &mut self.bullets {
            b.y -= 1.0;
        }
        self.bullets.retain(|b| b.y > 0.0);

        // Move enemies
        // Speed up as score increases?
        let move_freq = if self.score > 1000 { 5 } else { 10 };

        if self.tick_count % move_freq == 0 {
             for e in &mut self.enemies {
                 e.y += 1.0;
                 if e.y >= self.height - 2.0 {
                     self.game_over = true;
                 }
             }
        }

        // Collisions
        let mut dead_bullets = Vec::new();
        let mut dead_enemies = Vec::new();

        for (bi, b) in self.bullets.iter().enumerate() {
            for (ei, e) in self.enemies.iter().enumerate() {
                // Simple AABB
                // Enemy height is 1
                if b.y.round() == e.y.round() {
                     if b.x >= e.x && b.x <= e.x + e.width as f64 {
                         dead_bullets.push(bi);
                         dead_enemies.push(ei);
                     }
                }
            }
        }

        // Cleanup
        dead_bullets.sort_unstable_by(|a, b| b.cmp(a));
        dead_bullets.dedup();
        for idx in dead_bullets {
            if idx < self.bullets.len() {
                self.bullets.remove(idx);
            }
        }

        dead_enemies.sort_unstable_by(|a, b| b.cmp(a));
        dead_enemies.dedup();
        for idx in dead_enemies {
             if idx < self.enemies.len() {
                 let e = self.enemies.remove(idx);
                 // Score depends on length
                 self.score += e.width * 10;
             }
        }
    }

    pub fn move_player(&mut self, dx: f64) {
        // Player is width 3: /^\
        let p_width = 3.0;
        self.player_x = (self.player_x + dx).clamp(0.0, self.width - p_width);
    }

    pub fn shoot(&mut self) {
        // Center of player
        self.bullets.push(Bullet { x: self.player_x + 1.0, y: self.height - 3.0 });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff::LineType;

    #[test]
    fn test_game_spawn_enemy() {
        let lines = vec![DiffLine {
            content: "test line".to_string(),
            line_type: LineType::Addition,
        }];
        let mut game = Game::new_with_lines(100.0, 100.0, lines);

        // Initial state
        assert!(game.enemies.is_empty());
        assert_eq!(game.pending_lines.len(), 1);

        // Tick 40 times to trigger spawn
        for _ in 0..40 {
            game.tick();
        }

        assert_eq!(game.enemies.len(), 1);
        assert_eq!(game.pending_lines.len(), 0);
        assert_eq!(game.enemies[0].line.content, "test line");
    }

    #[test]
    fn test_shooting() {
        let mut game = Game::new_with_lines(100.0, 100.0, vec![]);
        // Spawn an enemy manually
        game.enemies.push(Enemy {
            x: 50.0,
            y: 50.0,
            line: DiffLine { content: "target".to_string(), line_type: LineType::Addition },
            width: 6,
            max_width: 10,
        });

        // Place player under enemy
        game.player_x = 49.0; // Shoot from 50.0
        // height is 100. Player is at 97. Bullet spawns at 97.
        // Enemy is at 50.

        game.shoot();
        assert_eq!(game.bullets.len(), 1);

        // Move bullets up (y decreases)
        // Bullet y = 97. Enemy y = 50.
        // Needs 47 ticks to hit.
        for _ in 0..50 {
            game.tick();
        }

        // Enemy should be dead
        assert!(game.enemies.is_empty());
        assert!(game.score > 0);
    }
}

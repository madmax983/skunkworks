use std::collections::VecDeque;

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 24;

#[derive(Debug, Clone, Copy)]
pub struct LogisticMap {
    pub x: f64,
    pub r: f64,
}

impl LogisticMap {
    pub fn new(x: f64, r: f64) -> Self {
        Self { x, r }
    }

    pub fn next(&mut self) -> f64 {
        self.x = self.r * self.x * (1.0 - self.x);
        self.x
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub pos: Position,
    pub hp: f64,
    pub id: usize,
    pub chaos_val: f64,
}

#[derive(Debug, Clone)]
pub struct Tower {
    pub pos: Position,
    pub cooldown: usize,
}

#[derive(Debug, Clone)]
pub struct Nest {
    pub pos: Position,
    pub chaos: LogisticMap,
    pub spawn_timer: f64,
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub enemies: Vec<Enemy>,
    pub towers: Vec<Tower>,
    pub nests: Vec<Nest>,
    pub global_r: f64,
    pub resources: f64,
    pub ticks: usize,
    pub history: VecDeque<f64>,
}

impl World {
    pub fn new() -> Self {
        let nests = vec![
            Nest {
                pos: Position { x: 5.0, y: 5.0 },
                chaos: LogisticMap::new(0.5, 3.0),
                spawn_timer: 0.0,
            },
            Nest {
                pos: Position { x: 75.0, y: 20.0 },
                chaos: LogisticMap::new(0.6, 3.2),
                spawn_timer: 0.0,
            },
            Nest {
                pos: Position { x: 40.0, y: 2.0 },
                chaos: LogisticMap::new(0.1, 3.9),
                spawn_timer: 0.0,
            },
        ];

        Self {
            width: WIDTH,
            height: HEIGHT,
            enemies: Vec::new(),
            towers: Vec::new(),
            nests,
            global_r: 3.0,
            resources: 100.0,
            ticks: 0,
            history: VecDeque::with_capacity(200),
        }
    }

    pub fn add_tower(&mut self, x: f64, y: f64) {
        if self.resources >= 20.0 {
            self.resources -= 20.0;
            self.towers.push(Tower {
                pos: Position { x, y },
                cooldown: 0,
            });
            // Adding a tower increases industrialization -> higher r
            self.global_r += 0.05;
        }
    }

    pub fn update(&mut self) {
        self.ticks += 1;

        // Influence nests with global_r
        for (i, nest) in self.nests.iter_mut().enumerate() {
            let drift = (self.global_r - nest.chaos.r) * 0.005;
            nest.chaos.r += drift;
            nest.chaos.r = nest.chaos.r.clamp(0.0, 4.0);

            // Tick chaos
            nest.chaos.next();

            // Record history from the first nest
            if i == 0 {
                self.history.push_back(nest.chaos.x);
                if self.history.len() > 200 {
                    self.history.pop_front();
                }
            }

            // Spawn logic
            nest.spawn_timer += nest.chaos.x;
            if nest.spawn_timer > 5.0 {
                nest.spawn_timer = 0.0;
                self.enemies.push(Enemy {
                    pos: nest.pos,
                    hp: 10.0,
                    id: self.ticks,
                    chaos_val: nest.chaos.x,
                });
            }
        }

        // Move enemies
        let center = Position {
            x: (WIDTH / 2) as f64,
            y: (HEIGHT / 2) as f64,
        };
        for enemy in &mut self.enemies {
            let dx = center.x - enemy.pos.x;
            let dy = center.y - enemy.pos.y;
            let dist = (dx * dx + dy * dy).sqrt();

            // Deterministic jitter
            enemy.chaos_val = 3.9 * enemy.chaos_val * (1.0 - enemy.chaos_val);
            let angle_offset = (enemy.chaos_val - 0.5) * 1.0;

            if dist > 0.5 {
                let speed = 0.3;
                let base_angle = dy.atan2(dx);
                let final_angle = base_angle + angle_offset;

                enemy.pos.x += final_angle.cos() * speed;
                enemy.pos.y += final_angle.sin() * speed;
            } else {
                self.resources -= 5.0;
                enemy.hp = 0.0;
            }
        }

        self.enemies.retain(|e| e.hp > 0.0);

        // Towers
        for tower in &mut self.towers {
            if tower.cooldown > 0 {
                tower.cooldown -= 1;
                continue;
            }

            let mut best_target = None;
            let mut min_dist = 15.0;

            for (i, enemy) in self.enemies.iter().enumerate() {
                let dx = enemy.pos.x - tower.pos.x;
                let dy = enemy.pos.y - tower.pos.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < min_dist {
                    min_dist = dist;
                    best_target = Some(i);
                }
            }

            if let Some(idx) = best_target {
                self.enemies[idx].hp -= 5.0;
                if self.enemies[idx].hp <= 0.0 {
                    self.resources += 2.0;
                }
                tower.cooldown = 5;
            }
        }

        if self.resources < 0.0 {
            self.resources = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extinction() {
        let mut map = LogisticMap::new(0.5, 0.5);
        for _ in 0..100 {
            map.next();
        }
        assert!(
            map.x < 0.001,
            "Population should die out with r=0.5. Got {}",
            map.x
        );
    }

    #[test]
    fn test_stability() {
        let mut map = LogisticMap::new(0.1, 2.5);
        for _ in 0..100 {
            map.next();
        }
        // Converges to (r-1)/r = 1.5/2.5 = 0.6
        assert!(
            (map.x - 0.6).abs() < 0.001,
            "Population should stabilize at 0.6 with r=2.5. Got {}",
            map.x
        );
    }

    #[test]
    fn test_period_2() {
        let mut map = LogisticMap::new(0.1, 3.2);
        // Burn in
        for _ in 0..100 {
            map.next();
        }
        let x1 = map.x;
        let x2 = map.next();
        let x3 = map.next();

        assert!(
            (x1 - x3).abs() < 0.001,
            "Population should oscillate with period 2. x1={} x3={}",
            x1,
            x3
        );
        assert!(
            (x1 - x2).abs() > 0.01,
            "Values should differ in period 2. x1={} x2={}",
            x1,
            x2
        );
    }
}

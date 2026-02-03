use crate::parser::Fighter;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct CombatLog {
    pub messages: Vec<String>,
}

impl CombatLog {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn add(&mut self, msg: String) {
        self.messages.push(msg);
        if self.messages.len() > 50 {
            self.messages.remove(0);
        }
    }
}

pub struct BattleState {
    pub fighter_a: Fighter,
    pub fighter_b: Fighter,
    pub log: CombatLog,
    pub turn: usize,
    pub winner: Option<String>,
}

impl BattleState {
    pub fn new(a: Fighter, b: Fighter) -> Self {
        Self {
            fighter_a: a,
            fighter_b: b,
            log: CombatLog::new(),
            turn: 0,
            winner: None,
        }
    }

    pub fn step(&mut self) {
        if self.winner.is_some() {
            return;
        }

        self.turn += 1;
        let mut rng = rand::thread_rng();

        // Speed contest
        let speed_a = self.fighter_a.speed + rng.gen_range(0..20);
        let speed_b = self.fighter_b.speed + rng.gen_range(0..20);

        // We need to borrow mutably, but we can't borrow both fields of self if we use a method.
        // We'll just clone indices or something?
        // Actually, let's just use a boolean flag for who attacks.

        let a_attacks = speed_a >= speed_b;

        let (attacker, defender) = if a_attacks {
            (&self.fighter_a, &mut self.fighter_b)
        } else {
            (&self.fighter_b, &mut self.fighter_a)
        };

        // Attack Roll
        let attack_roll = attacker.attack + rng.gen_range(1..20);
        let defense_roll = defender.defense + rng.gen_range(1..10);

        // We need to capture names before mutable borrow ends or just use them carefully.
        // Actually we can't borrow self.fighter_a immutable and self.fighter_b mutable easily if we put them in a tuple.
        // Rust borrow checker might complain.

        // Let's resolve values first.
        let damage = if attack_roll > defense_roll {
            (attack_roll - defense_roll).max(1)
        } else {
            0
        };

        let attacker_name = attacker.name.clone();
        let defender_name = defender.name.clone();

        if damage > 0 {
            defender.hp -= damage;
            self.log.add(format!(
                "Turn {}: {} attacks {}! ({} vs {}) -> {} DMG",
                self.turn, attacker_name, defender_name, attack_roll, defense_roll, damage
            ));

            if defender.hp <= 0 {
                defender.hp = 0;
                self.winner = Some(attacker_name.clone());
                self.log.add(format!(
                    "Segfault! {} has crashed. {} Wins!",
                    defender_name, attacker_name
                ));
            }
        } else {
            self.log.add(format!(
                "Turn {}: {} attacks but {} dodges! ({} vs {})",
                self.turn, attacker_name, defender_name, attack_roll, defense_roll
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Fighter;

    fn make_dummy_fighter(name: &str) -> Fighter {
        Fighter {
            name: name.to_string(),
            hp: 100,
            max_hp: 100,
            attack: 10,
            defense: 5,
            speed: 10,
            signature: "fn test()".to_string(),
            file_path: "test.rs".to_string(),
        }
    }

    #[test]
    fn test_battle_step() {
        let f1 = make_dummy_fighter("F1");
        let f2 = make_dummy_fighter("F2");
        let mut battle = BattleState::new(f1, f2);

        battle.step();

        assert_eq!(battle.turn, 1);
        assert!(!battle.log.messages.is_empty());
    }

    #[test]
    fn test_win_condition() {
        let mut f1 = make_dummy_fighter("Winner");
        f1.attack = 1000; // One hit kill
        let f2 = make_dummy_fighter("Loser");

        let mut battle = BattleState::new(f1, f2);

        // Step until win
        for _ in 0..10 {
            battle.step();
            if battle.winner.is_some() {
                break;
            }
        }

        assert!(battle.winner.is_some());
    }
}

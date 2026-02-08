#[cfg(feature = "nova")]
use crate::ast::Strand;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::nova_bestiary::{self, CombatStats};
#[cfg(feature = "nova")]
use std::collections::VecDeque;

#[cfg(feature = "nova")]
#[derive(Debug, Clone)]
pub struct Gladiator {
    pub name: String,
    pub strand: Strand,
    pub stats: CombatStats,
    pub energy: i64,
    pub ip: usize,
    pub traits: Vec<String>,
}

#[cfg(feature = "nova")]
#[derive(Debug, Clone)]
pub struct ArenaState {
    pub combatants: Vec<Gladiator>,
    pub logs: VecDeque<String>,
    pub turn: usize,
    pub active: bool,
    pub winner: Option<String>,
}

#[cfg(feature = "nova")]
impl ArenaState {
    pub fn new() -> Self {
        Self {
            combatants: Vec::new(),
            logs: VecDeque::new(),
            turn: 0,
            active: false,
            winner: None,
        }
    }

    pub fn add_gladiator(&mut self, strand: Strand, seed: u64) {
        let traits = nova_bestiary::analyze_traits(&strand);
        let name = nova_bestiary::generate_name(seed, &traits);
        let stats = nova_bestiary::generate_combat_stats(&traits);

        let gladiator = Gladiator {
            name: name.clone(),
            strand,
            stats,
            energy: 50,
            ip: 0,
            traits,
        };

        self.logs.push_back(format!("{} enters the arena!", name));
        if self.logs.len() > 20 {
            self.logs.pop_front();
        }
        self.combatants.push(gladiator);
    }

    pub fn start(&mut self) {
        if self.combatants.len() >= 2 {
            self.active = true;
            self.turn = 0;
            self.winner = None;
            self.logs.push_back("FIGHT STARTED!".to_string());
        } else {
            self.logs.push_back("Not enough combatants!".to_string());
        }
    }

    pub fn reset(&mut self) {
        self.combatants.clear();
        self.logs.clear();
        self.active = false;
        self.winner = None;
        self.turn = 0;
    }

    pub fn tick(&mut self) {
        if !self.active || self.combatants.len() < 2 {
            return;
        }

        self.turn += 1;

        let count = self.combatants.len();
        let mut damage_events = Vec::new();

        for i in 0..count {
            let (attacker_name, damage, heal, action) = {
                let gladiator = &mut self.combatants[i];
                if gladiator.stats.hp <= 0 {
                    continue;
                }

                let mut action_desc = String::new();
                let mut dmg_out: i64 = 0;
                let mut heal_out: i64 = 0;

                let exec_limit = 5 + (gladiator.stats.speed / 5) as usize;

                for _ in 0..exec_limit {
                    if gladiator.strand.genes.is_empty() {
                        break;
                    }
                    if gladiator.ip >= gladiator.strand.genes.len() {
                        gladiator.ip = 0;
                    }

                    let op = &gladiator.strand.genes[gladiator.ip].op;
                    match op {
                        OpCode::Fire | OpCode::Virus | OpCode::Irradiate => {
                            dmg_out += 5 + (gladiator.stats.attack / 2);
                            action_desc = format!("uses {}", op);
                        }
                        OpCode::Consume | OpCode::Photosynthesize | OpCode::Banquet | OpCode::Savor => {
                            heal_out += 2;
                            action_desc = format!("uses {}", op);
                        }
                        _ => {}
                    }
                    gladiator.ip += 1;
                }

                // Base attack if no special move
                if dmg_out == 0 && heal_out == 0 {
                    dmg_out = gladiator.stats.attack.max(1);
                    action_desc = "attacks".to_string();
                }

                (gladiator.name.clone(), dmg_out, heal_out, action_desc)
            };

            // Apply self heal
            if heal > 0 {
                 let gladiator = &mut self.combatants[i];
                 gladiator.stats.hp = (gladiator.stats.hp + heal).min(gladiator.stats.max_hp);
                 self.logs.push_back(format!("{} heals for {}.", attacker_name, heal));
            }

            // Deal damage to opponent (assuming 1v1 for now, targeting index 1-i)
            if damage > 0 {
                let target_idx = (i + 1) % count;
                damage_events.push((target_idx, damage, attacker_name, action));
            }
        }

        // Apply damage
        for (target_idx, raw_damage, attacker, action) in damage_events {
            let target = &mut self.combatants[target_idx];
            let defense = target.stats.defense;
            let actual_damage = (raw_damage - (defense / 2)).max(1);

            target.stats.hp -= actual_damage;
            self.logs.push_back(format!("{} {}! {} takes {} dmg.", attacker, action, target.name, actual_damage));

            if target.stats.hp <= 0 {
                self.logs.push_back(format!("{} has been defeated!", target.name));
            }
        }

        if self.logs.len() > 20 {
            self.logs.pop_front();
        }

        // Check win condition
        let alive: Vec<&Gladiator> = self.combatants.iter().filter(|g| g.stats.hp > 0).collect();
        if alive.len() == 1 {
            self.active = false;
            self.winner = Some(alive[0].name.clone());
            self.logs.push_back(format!("WINNER: {}!", alive[0].name));
        } else if alive.is_empty() {
             self.active = false;
             self.winner = None;
             self.logs.push_back("DRAW! Everyone died.".to_string());
        }
    }
}

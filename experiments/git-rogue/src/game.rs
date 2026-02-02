use crate::crawler::Node;
use rand::Rng;
use std::collections::HashMap;

pub struct Game {
    pub nodes: HashMap<String, Node>,
    pub current_hash: String,
    pub hp: i32,
    pub max_hp: i32,
    pub xp: i32,
    #[allow(dead_code)]
    pub inventory: Vec<String>,
    pub log: Vec<String>,
    pub game_over: bool,
}

impl Game {
    pub fn new(node_list: Vec<Node>) -> Self {
        if node_list.is_empty() {
            panic!("Cannot start game with empty node list");
        }

        let mut nodes = HashMap::new();
        // Assuming the first node in the list is HEAD (because of revwalk order)
        let start_hash = node_list[0].hash.clone();

        for node in node_list {
            nodes.insert(node.hash.clone(), node);
        }

        Self {
            nodes,
            current_hash: start_hash,
            hp: 100,
            max_hp: 100,
            xp: 0,
            inventory: Vec::new(),
            log: vec!["Welcome to Git Rogue. Find the Initial Commit!".to_string()],
            game_over: false,
        }
    }

    pub fn current_node(&self) -> &Node {
        self.nodes
            .get(&self.current_hash)
            .expect("Current node must exist")
    }

    pub fn move_to(&mut self, hash: String) {
        if self.game_over {
            return;
        }

        if self.nodes.contains_key(&hash) {
            self.current_hash = hash;
            self.encounter();
        }
    }

    fn encounter(&mut self) {
        // Clone message to avoid borrow issues
        let msg = self.current_node().message.to_lowercase();
        let short_hash = self.current_node().short_hash.clone();

        self.log.push(format!("Entered Commit {}", short_hash));

        let mut rng = rand::thread_rng();

        // Check keywords
        if msg.contains("fix")
            || msg.contains("bug")
            || msg.contains("panic")
            || msg.contains("error")
        {
            // Battle!
            let damage = rng.gen_range(5..15);
            self.hp -= damage;
            self.log
                .push(format!("A BUG attacks! You take {} damage.", damage));
        } else if msg.contains("feat") || msg.contains("add") || msg.contains("new") {
            // Loot
            let gain = rng.gen_range(10..20);
            self.xp += gain;
            self.log
                .push(format!("You discovered a Feature! Gained {} XP.", gain));

            // Small heal
            if self.hp < self.max_hp {
                let heal = 5;
                self.hp = (self.hp + heal).min(self.max_hp);
                self.log.push(format!("You feel refreshed (+{} HP).", heal));
            }
        } else if msg.contains("merge") {
            // Boss?
            let damage = rng.gen_range(10..25);
            self.hp -= damage;
            self.log.push(format!(
                "Merge Conflict Dragon! You take {} damage.",
                damage
            ));
        } else {
            // Nothing
            self.log.push("The code here seems stable.".to_string());
        }

        if self.hp <= 0 {
            self.hp = 0;
            self.game_over = true;
            self.log.push("SYSTEM FAILURE (Game Over).".to_string());
        }

        // Trim log
        if self.log.len() > 10 {
            self.log.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mock_node(hash: &str, msg: &str) -> Node {
        Node {
            hash: hash.to_string(),
            short_hash: hash[..7.min(hash.len())].to_string(),
            message: msg.to_string(),
            author: "Tester".to_string(),
            parents: vec![],
            children: vec![],
        }
    }

    #[test]
    fn test_game_mechanics() {
        let n1 = create_mock_node("1111111", "Initial commit");
        let n2 = create_mock_node("2222222", "fix: deep bug"); // Should cause damage

        let mut game = Game::new(vec![n1.clone(), n2.clone()]);

        assert_eq!(game.hp, 100);
        assert_eq!(game.current_hash, "1111111");

        game.move_to("2222222".to_string());
        assert_eq!(game.current_hash, "2222222");
        assert!(game.hp < 100, "Should take damage from bug");

        // Log should contain damage info
        assert!(game.log.iter().any(|l| l.contains("damage")));
    }
}

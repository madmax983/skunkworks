use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct LSystem {
    pub axiom: String,
    pub rules: HashMap<char, String>,
}

impl LSystem {
    pub fn new(axiom: &str, rules: Vec<(char, &str)>) -> Self {
        let mut rule_map = HashMap::new();
        for (k, v) in rules {
            rule_map.insert(k, v.to_string());
        }
        Self {
            axiom: axiom.to_string(),
            rules: rule_map,
        }
    }

    pub fn expand(&self, iterations: u32) -> String {
        let mut current = self.axiom.clone();
        for _ in 0..iterations {
            let mut next = String::with_capacity(current.len() * 2);
            for c in current.chars() {
                if let Some(replacement) = self.rules.get(&c) {
                    next.push_str(replacement);
                } else {
                    next.push(c);
                }
            }
            current = next;
        }
        current
    }
}

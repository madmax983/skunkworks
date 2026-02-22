use std::collections::HashMap;

pub struct LSystem {
    pub axiom: String,
    pub rules: HashMap<char, String>,
}

impl LSystem {
    pub fn new(axiom: &str, rules: Vec<(char, &str)>) -> Self {
        let mut rules_map = HashMap::new();
        for (k, v) in rules {
            rules_map.insert(k, v.to_string());
        }
        Self {
            axiom: axiom.to_string(),
            rules: rules_map,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algae() {
        let rules = vec![('A', "AB"), ('B', "A")];
        let sys = LSystem::new("A", rules);
        assert_eq!(sys.expand(0), "A");
        assert_eq!(sys.expand(1), "AB");
        assert_eq!(sys.expand(2), "ABA");
        assert_eq!(sys.expand(3), "ABAAB");
        assert_eq!(sys.expand(4), "ABAABABA");
    }
}

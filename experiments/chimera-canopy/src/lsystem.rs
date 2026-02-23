use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Grammar {
    pub axiom: String,
    pub rules: HashMap<char, String>,
}

impl Grammar {
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
            let mut next = String::new();
            for c in current.chars() {
                match self.rules.get(&c) {
                    Some(replacement) => next.push_str(replacement),
                    None => next.push(c),
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
    fn test_lsystem_expansion() {
        let grammar = Grammar::new("A", vec![('A', "AB"), ('B', "A")]);
        // n=0: A
        // n=1: AB
        // n=2: ABA
        // n=3: ABAAB
        assert_eq!(grammar.expand(0), "A");
        assert_eq!(grammar.expand(1), "AB");
        assert_eq!(grammar.expand(2), "ABA");
        assert_eq!(grammar.expand(3), "ABAAB");
    }
}

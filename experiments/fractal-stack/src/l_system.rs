#[derive(Debug, Clone)]
pub struct Rule {
    pub predecessor: char,
    pub successor: String,
}

impl Rule {
    pub fn new(predecessor: char, successor: &str) -> Self {
        Self {
            predecessor,
            successor: successor.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LSystem {
    pub axiom: String,
    pub rules: Vec<Rule>,
}

impl LSystem {
    pub fn new(axiom: &str, rules: Vec<Rule>) -> Self {
        Self {
            axiom: axiom.to_string(),
            rules,
        }
    }

    pub fn expand(&self, generations: usize) -> String {
        let mut current = self.axiom.clone();
        for _ in 0..generations {
            let mut next = String::new();
            for c in current.chars() {
                let mut found = false;
                for rule in &self.rules {
                    if rule.predecessor == c {
                        next.push_str(&rule.successor);
                        found = true;
                        break;
                    }
                }
                if !found {
                    next.push(c);
                }
            }
            current = next;
        }
        current
    }
}

// Presets
pub mod presets {
    use super::*;

    pub fn binary_tree() -> LSystem {
        // A -> A[+A]A[-A]A might be too dense.
        // Simple tree: 0 -> 1[0]0
        // 1 -> 11
        // Let's use standard F encoding.
        // F -> FF
        // X -> F[+X]F[-X]+X
        LSystem::new(
            "X",
            vec![Rule::new('X', "F[+X]F[-X]+X"), Rule::new('F', "FF")],
        )
    }

    pub fn sierpinski_arrowhead() -> LSystem {
        // A -> B-A-B
        // B -> A+B+A
        // Angle 60
        LSystem::new("A", vec![Rule::new('A', "B-A-B"), Rule::new('B', "A+B+A")])
    }

    pub fn fractal_plant() -> LSystem {
        // F -> FF+[+F-F-F]-[-F+F+F] (Barnsley-ish)
        // Or generic: X -> F[+X]F[-X]+X
        LSystem::new(
            "X",
            vec![Rule::new('X', "F+[[X]-X]-F[-FX]+X"), Rule::new('F', "FF")],
        )
    }

    // Call Stack specific example: Fibonacci Call Tree
    // Fib(n) calls Fib(n-1) and Fib(n-2)
    // We want to visualize the stack depth.
    // S -> A
    // A -> B[A]A  (Incorrect. Fib is additive)
    // Let's model the recursion structure:
    // F(n) -> F(n-1) + F(n-2)
    // This is a tree where each node splits into two.
    // L-system: A -> B[+A]A (Asymmetric?)
    // Let's try: A -> B[+A][-A]
    pub fn recursive_tree() -> LSystem {
        LSystem::new("A", vec![Rule::new('A', "F[+A][-A]"), Rule::new('F', "FF")])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expansion() {
        let rules = vec![Rule::new('A', "AB"), Rule::new('B', "A")];
        let sys = LSystem::new("A", rules);
        assert_eq!(sys.expand(0), "A");
        assert_eq!(sys.expand(1), "AB");
        assert_eq!(sys.expand(2), "ABA");
        assert_eq!(sys.expand(3), "ABAAB");
    }
}

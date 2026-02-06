use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    DrawForward(f32), // Draw and move
    MoveForward(f32), // Move without drawing
    Turn(f32, f32, f32), // Rotate (yaw, pitch, roll) in radians
    PushStack,
    PopStack,
    ScaleWidth(f32),
    // CallStack specific visualizations
    EnterFrame(String), // Enter a function frame
    ExitFrame,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub predecessor: char,
    pub successor: String,
    pub probability: f32, // For stochastic L-systems
}

pub struct LSystem {
    pub axiom: String,
    pub rules: HashMap<char, Vec<Rule>>,
    pub default_angle: f32,
    pub default_distance: f32,
}

impl LSystem {
    pub fn new(axiom: &str, angle: f32, distance: f32) -> Self {
        Self {
            axiom: axiom.to_string(),
            rules: HashMap::new(),
            default_angle: angle,
            default_distance: distance,
        }
    }

    pub fn add_rule(&mut self, predecessor: char, successor: &str) {
        self.rules.entry(predecessor).or_default().push(Rule {
            predecessor,
            successor: successor.to_string(),
            probability: 1.0,
        });
    }

    pub fn expand(&self, iterations: u32) -> Vec<Operation> {
        let mut current_string = self.axiom.clone();

        for _ in 0..iterations {
            let mut next_string = String::new();
            for c in current_string.chars() {
                if let Some(rules) = self.rules.get(&c) {
                    // Simple deterministic selection for now, take the first one
                    // TODO: Implement stochastic selection
                    if let Some(rule) = rules.first() {
                        next_string.push_str(&rule.successor);
                    } else {
                        next_string.push(c);
                    }
                } else {
                    next_string.push(c);
                }
            }
            current_string = next_string;
        }

        self.parse_string(&current_string)
    }

    fn parse_string(&self, s: &str) -> Vec<Operation> {
        let mut ops = Vec::new();
        let angle = self.default_angle;
        let dist = self.default_distance;

        for c in s.chars() {
            match c {
                'F' => ops.push(Operation::DrawForward(dist)),
                'f' => ops.push(Operation::MoveForward(dist)),
                '+' => ops.push(Operation::Turn(angle, 0.0, 0.0)), // Yaw +
                '-' => ops.push(Operation::Turn(-angle, 0.0, 0.0)), // Yaw -
                '&' => ops.push(Operation::Turn(0.0, angle, 0.0)), // Pitch +
                '^' => ops.push(Operation::Turn(0.0, -angle, 0.0)), // Pitch -
                '\\' => ops.push(Operation::Turn(0.0, 0.0, angle)), // Roll +
                '/' => ops.push(Operation::Turn(0.0, 0.0, -angle)), // Roll -
                '[' => ops.push(Operation::PushStack),
                ']' => ops.push(Operation::PopStack),
                // Custom symbols for "Call Stack" logic
                '{' => ops.push(Operation::EnterFrame("Scope".to_string())),
                '}' => ops.push(Operation::ExitFrame),
                _ => {} // Ignore unknown symbols (variables)
            }
        }
        ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_expansion() {
        let mut lsys = LSystem::new("F", 1.57, 10.0);
        lsys.add_rule('F', "F+F");

        // Iteration 0: F
        let ops0 = lsys.expand(0);
        assert_eq!(ops0.len(), 1);
        match ops0[0] {
            Operation::DrawForward(_) => {},
            _ => panic!("Expected DrawForward"),
        }

        // Iteration 1: F+F
        let ops1 = lsys.expand(1);
        // F, +, F
        assert_eq!(ops1.len(), 3);
    }

    #[test]
    fn test_bracket_parsing() {
        let lsys = LSystem::new("F[+F]F", 1.57, 10.0);
        let ops = lsys.expand(0);
        // F, [, +, F, ], F
        assert_eq!(ops.len(), 6);
        assert!(matches!(ops[1], Operation::PushStack));
        assert!(matches!(ops[4], Operation::PopStack));
    }
}

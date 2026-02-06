use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};
use std::collections::HashMap;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[derive(Clone, Debug)]
pub struct LSystem {
    pub axiom: String,
    pub rules: HashMap<char, String>,
}

impl LSystem {
    pub fn new(axiom: &str, rules: HashMap<char, String>) -> Self {
        Self {
            axiom: axiom.to_string(),
            rules,
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

#[derive(Clone, Copy)]
struct State {
    x: f64,
    y: f64,
    angle: f64, // radians
}

pub struct Turtle {
    state: State,
    stack: Vec<State>,
    step_size: f64,
    angle_increment: f64,
}

impl Turtle {
    pub fn new(x: f64, y: f64, angle: f64, step_size: f64, angle_increment: f64) -> Self {
        Self {
            state: State { x, y, angle },
            stack: Vec::new(),
            step_size,
            angle_increment,
        }
    }

    pub fn interpret(&mut self, instructions: &str) -> Vec<Line> {
        let mut lines = Vec::new();

        for c in instructions.chars() {
            match c {
                'F' | 'G' => {
                    let new_x = self.state.x + self.step_size * self.state.angle.cos();
                    let new_y = self.state.y + self.step_size * self.state.angle.sin();

                    lines.push(Line {
                        x1: self.state.x,
                        y1: self.state.y,
                        x2: new_x,
                        y2: new_y,
                    });

                    self.state.x = new_x;
                    self.state.y = new_y;
                }
                '+' => {
                    self.state.angle += self.angle_increment;
                }
                '-' => {
                    self.state.angle -= self.angle_increment;
                }
                '[' => {
                    self.stack.push(self.state);
                }
                ']' => {
                    if let Some(saved_state) = self.stack.pop() {
                        self.state = saved_state;
                    }
                }
                _ => {}
            }
        }
        lines
    }
}

#[cfg(feature = "nova")]
pub fn exec_fractal_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::LSystem => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(axiom) = val {
                    vm.fractal_axiom = axiom.clone();
                    vm.output
                        .push(format!("L-SYSTEM: Axiom set to '{}'", axiom));
                } else {
                    vm.output
                        .push("Error: LSystem requires a string axiom".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for LSystem".to_string());
            }
        }
        OpCode::Fractal => {
            // Stack: [rule_char, expansion_string]
            if vm.stack.len() >= 2 {
                let expansion_val = vm.stack.pop().unwrap();
                let char_val = vm.stack.pop().unwrap();

                let rule_key = if let Value::Int(c) = char_val {
                    Some((c as u8) as char)
                } else if let Value::Str(s) = char_val {
                    s.chars().next()
                } else {
                    None
                };

                if let Some(key) = rule_key {
                    if let Value::Str(exp) = expansion_val {
                        vm.fractal_rules.insert(key, exp.clone());
                        vm.output.push(format!("FRACTAL: Rule {} -> {}", key, exp));
                    } else {
                        vm.output
                            .push("Error: Expansion must be a string".to_string());
                    }
                } else {
                    vm.output.push("Error: Invalid rule key".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for Fractal".to_string());
            }
        }
        OpCode::Grow => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(iter) = val {
                    let iterations = iter.clamp(0, 6) as u32; // Limit iterations

                    let system = LSystem::new(&vm.fractal_axiom, vm.fractal_rules.clone());
                    let expanded = system.expand(iterations);

                    // Simple angle heuristic: If rule contains X or Y (Dragon/Plant), use 25-90 deg.
                    // Default to 90 (PI/2)
                    // If rules contain 'X', maybe 25 deg?
                    // Let's stick to 90 for now, or maybe check for 60.
                    // This is "Dreaming", it doesn't have to be perfect.
                    let angle = if vm.fractal_rules.contains_key(&'X') {
                        25.0f64.to_radians()
                    } else if vm.fractal_rules.contains_key(&'A') {
                        60.0f64.to_radians()
                    } else {
                        PI / 2.0
                    };

                    let mut turtle = Turtle::new(0.0, 0.0, -PI / 2.0, 10.0, angle);

                    // Adaptive step size
                    turtle.step_size = 80.0 / (iterations as f64 + 1.0).powf(1.2);

                    vm.fractal_lines = turtle.interpret(&expanded);

                    vm.energy = vm
                        .energy
                        .saturating_sub((vm.fractal_lines.len() / 5) as i64);
                    vm.output
                        .push(format!("GROW: Generated {} lines", vm.fractal_lines.len()));
                } else {
                    vm.output
                        .push("Error: Grow requires iteration count".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for Grow".to_string());
            }
        }
        _ => {}
    }
    None
}

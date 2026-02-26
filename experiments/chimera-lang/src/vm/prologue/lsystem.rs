use super::{normalize_coords, PrologueAgent};
use crate::vm::Value;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct LSystemState {
    pub axiom: String,
    pub rules: HashMap<char, String>,
    pub iterations: usize,
    pub current_string: String,
    pub current_iteration: usize,
    pub pc: usize, // Index into current_string for turtle execution
    pub dir: usize, // 0=N, 1=E, 2=S, 3=W
    pub stack: Vec<(usize, usize, usize)>, // (x, y, dir) for '[' and ']'
}

impl Default for LSystemState {
    fn default() -> Self {
        Self {
            axiom: "F".to_string(),
            rules: HashMap::new(),
            iterations: 0,
            current_string: "F".to_string(),
            current_iteration: 0,
            pc: 0,
            dir: 0, // North
            stack: Vec::new(),
        }
    }
}

impl LSystemState {
    pub fn to_value(&self) -> Value {
        // Serialize rules as "Key=Val,Key=Val"
        let rules_str: Vec<String> = self
            .rules
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        let rules_joined = rules_str.join(",");

        // Serialize stack as "x,y,d|x,y,d"
        let stack_str: Vec<String> = self.stack.iter()
            .map(|(x, y, d)| format!("{},{},{}", x, y, d))
            .collect();
        let stack_joined = stack_str.join("|");

        // Format: 🌲:Axiom:Rules:Iter:PC:Dir:CurrString:Stack
        Value::Str(format!(
            "🌲:{}:{}:{}:{}:{}:{}:{}",
            self.axiom,
            rules_joined,
            self.iterations,
            self.pc,
            self.dir,
            self.current_string,
            stack_joined
        ))
    }
}

impl FromStr for LSystemState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        // 🌲:Axiom:Rules:Iter OR 🌲:Axiom:Rules:Iter:PC:Dir:CurrString:Stack (Restored)
        if parts.len() >= 4 && parts[0] == "🌲" {
            let axiom = parts[1].to_string();
            let rules_part = parts[2];
            let iterations = parts[3].parse().unwrap_or(3);

            let mut rules = HashMap::new();
            for rule_pair in rules_part.split(',') {
                let kv: Vec<&str> = rule_pair.split('=').collect();
                if kv.len() == 2 {
                    if let Some(key) = kv[0].chars().next() {
                        rules.insert(key, kv[1].to_string());
                    }
                }
            }

            if parts.len() >= 7 {
                // Restored state
                let pc = parts[4].parse().unwrap_or(0);
                let dir = parts[5].parse().unwrap_or(0);
                let current_string = parts[6].to_string();

                // Parse Stack if present (part 7)
                let mut stack = Vec::new();
                if parts.len() >= 8 && !parts[7].is_empty() {
                    for frame in parts[7].split('|') {
                        let coords: Vec<&str> = frame.split(',').collect();
                        if coords.len() == 3 {
                            if let (Ok(x), Ok(y), Ok(d)) = (coords[0].parse(), coords[1].parse(), coords[2].parse()) {
                                stack.push((x, y, d));
                            }
                        }
                    }
                }

                // Infer current_iteration roughly or just trust state
                let current_iteration = if current_string == axiom { 0 } else { iterations };

                Ok(Self {
                    axiom,
                    rules,
                    iterations,
                    current_string,
                    current_iteration,
                    pc,
                    dir,
                    stack,
                })
            } else {
                // New state
                Ok(Self {
                    axiom: axiom.clone(),
                    rules,
                    iterations,
                    current_string: axiom,
                    current_iteration: 0,
                    pc: 0,
                    dir: 0,
                    stack: Vec::new(),
                })
            }
        } else {
            // Default plant
            let mut rules = HashMap::new();
            rules.insert('F', "F[+F]F[-F]F".to_string());
            Ok(Self {
                axiom: "F".to_string(),
                rules,
                iterations: 3,
                current_string: "F".to_string(),
                current_iteration: 0,
                pc: 0,
                dir: 0,
                stack: Vec::new(),
            })
        }
    }
}

pub fn process_lsystem_agent(
    vm: &mut crate::vm::ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let mut state = if let Value::Str(s) = &agent.state {
        s.parse::<LSystemState>().unwrap_or(LSystemState::default())
    } else {
        LSystemState::default()
    };

    // 1. Growth Phase
    if state.current_iteration < state.iterations {
        let mut next_string = String::new();
        for c in state.current_string.chars() {
            if let Some(replacement) = state.rules.get(&c) {
                next_string.push_str(replacement);
            } else {
                next_string.push(c);
            }
        }
        state.current_string = next_string;
        state.current_iteration += 1;

        let mut updated = agent.clone();
        updated.state = state.to_value();
        return Some((updated, None)); // Grow in place
    }

    // 2. Execution Phase (Turtle)
    if state.pc >= state.current_string.len() {
        // Done executing
        return Some((agent.clone(), None));
    }

    // Optimization: Process multiple non-movement commands per tick
    // to speed up drawing. Stop on Move or Draw.
    let mut instructions = state.current_string.chars().skip(state.pc);
    let mut move_target = None;
    let mut current_pos = (agent.y, agent.x);
    let _moved = false;

    // Limit steps per tick to avoid infinite loops if string is only rotation
    let max_steps = 10;
    let mut steps = 0;

    while let Some(cmd) = instructions.next() {
        state.pc += 1;
        steps += 1;

        match cmd {
            'F' | 'G' => {
                // Move Forward
                let (dy, dx) = match state.dir {
                    0 => (-1, 0), // N
                    1 => (0, 1),  // E
                    2 => (1, 0),  // S
                    3 => (0, -1), // W
                    _ => (0, 0),
                };

                if let Some((ny, nx)) = normalize_coords(current_pos.0 as i64 + dy, current_pos.1 as i64 + dx) {
                    // Check if blocked in snapshot
                    if matches!(grid_snapshot[ny][nx], Value::Int(0)) {
                        move_target = Some((ny, nx));

                        // Draw trail at OLD position
                        // If we move, we leave a mark behind? Or draw at new?
                        // Typically 'F' draws line. In grid, we leave a trail.
                        vm.grid[current_pos.0][current_pos.1] = Value::Str("*".to_string());

                        current_pos = (ny, nx);
                        let _moved = true;
                        break; // Stop to execute movement
                    } else {
                        // Blocked -> Stop
                        break;
                    }
                } else {
                    // Out of bounds -> Stop
                    break;
                }
            }
            '+' => {
                state.dir = (state.dir + 1) % 4; // Turn Right
            }
            '-' => {
                state.dir = (state.dir + 3) % 4; // Turn Left
            }
            '[' => {
                // Push State
                state.stack.push((current_pos.1, current_pos.0, state.dir));
            }
            ']' => {
                // Pop State
                if let Some((x, y, dir)) = state.stack.pop() {
                    // Jump to popped position
                    if x != current_pos.1 || y != current_pos.0 {
                        // Teleport back
                        move_target = Some((y, x));
                        state.dir = dir;
                        let _moved = true;
                        break;
                    } else {
                        state.dir = dir;
                    }
                }
            }
            _ => {} // Ignore unknown
        }

        if steps >= max_steps {
            break;
        }
    }

    let mut updated = agent.clone();
    updated.state = state.to_value();

    Some((updated, move_target))
}

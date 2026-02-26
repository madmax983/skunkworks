use super::normalize_coords;
use crate::vm::Value;
use std::fmt;
use std::str::FromStr;

/// Represents the state of an Automaton Agent (🤖).
///
/// Format: "A:PC:Dir:Memory:Program"
/// Example: "A:0:1:0:^>v<"
#[derive(Debug, Clone)]
pub struct AutomatonState {
    pub pc: usize,      // Program Counter
    pub dir: usize,     // 0=N, 1=E, 2=S, 3=W
    pub memory: i64,    // Internal Register/Clipboard
    pub program: String,// The Code
}

impl AutomatonState {
    pub fn new(pc: usize, dir: usize, memory: i64, program: String) -> Self {
        Self {
            pc,
            dir,
            memory,
            program,
        }
    }

    pub fn default() -> Self {
        Self {
            pc: 0,
            dir: 1, // East
            memory: 0,
            program: "W>".to_string(), // Default: Write 0, Move East
        }
    }

    pub fn to_value(&self) -> Value {
        Value::Str(self.to_string())
    }
}

impl fmt::Display for AutomatonState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "A:{}:{}:{}:{}",
            self.pc, self.dir, self.memory, self.program
        )
    }
}

impl FromStr for AutomatonState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(5, ':').collect();
        // A:PC:Dir:Mem:Program
        if parts.len() >= 5 && parts[0] == "A" {
            let pc = parts[1].parse().unwrap_or(0);
            let dir = parts[2].parse().unwrap_or(0);
            let memory = parts[3].parse().unwrap_or(0);
            let program = parts[4].to_string();
            Ok(Self {
                pc,
                dir,
                memory,
                program,
            })
        } else {
            // Fallback for simple program string initialization
            // If the string doesn't start with A:, treat it as a raw program
            Ok(Self {
                pc: 0,
                dir: 1,
                memory: 0,
                program: s.to_string(),
            })
        }
    }
}

/// Executes one step of the Automaton's program.
///
/// Returns the updated state and an optional movement target.
pub fn process_automaton_agent(
    vm: &mut crate::vm::ChimeraVM,
    agent: &super::PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(super::PrologueAgent, Option<(usize, usize)>)> {
    let (y, x) = (agent.y, agent.x);
    let mut state = if let Value::Str(s) = &agent.state {
        s.parse::<AutomatonState>().unwrap_or(AutomatonState::default())
    } else {
        AutomatonState::default()
    };

    if state.program.is_empty() {
        return Some((agent.clone(), None));
    }

    // Optimization: Avoid collecting Vec<char> (O(N) alloc) every tick.
    // Instead, iterate to find PC (O(PC)). Since we execute one instruction per tick,
    // this avoids the massive allocation overhead for large programs.

    let program_char_count = state.program.chars().count(); // O(N), but zero-alloc.

    // Ensure PC is within bounds (wrapping logic from original implementation)
    // Note: We update state.pc here for the execution context
    state.pc %= program_char_count.max(1);

    // 1. Find byte offset and char at current PC
    // Using `char_indices().nth(pc)` is O(PC), but strictly cheaper than O(N) alloc.
    let (current_byte_offset, instruction) = state
        .program
        .char_indices()
        .nth(state.pc)
        .map(|(i, c)| (i, c))
        .unwrap_or((0, ' '));

    // Advance PC (default behavior)
    let mut next_pc = (state.pc + 1) % program_char_count.max(1);

    let mut move_target = None;
    let mut updated_agent = agent.clone();

    match instruction {
        '^' => {
            // Move North (Absolute)
            if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                move_target = Some((ny, nx));
            }
        }
        'v' => {
            // Move South (Absolute)
            if let Some((ny, nx)) = normalize_coords(y as i64 + 1, x as i64) {
                move_target = Some((ny, nx));
            }
        }
        '<' => {
            // Move West (Absolute)
            if let Some((ny, nx)) = normalize_coords(y as i64, x as i64 - 1) {
                move_target = Some((ny, nx));
            }
        }
        '>' => {
            // Move East (Absolute)
            if let Some((ny, nx)) = normalize_coords(y as i64, x as i64 + 1) {
                move_target = Some((ny, nx));
            }
        }
        'F' => {
            // Forward (Relative)
            let (dy, dx) = match state.dir {
                0 => (-1, 0),
                1 => (0, 1),
                2 => (1, 0),
                3 => (0, -1),
                _ => (0, 0),
            };
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                move_target = Some((ny, nx));
            }
        }
        'B' => {
            // Backward (Relative)
            let (dy, dx) = match state.dir {
                0 => (1, 0),
                1 => (0, -1),
                2 => (-1, 0),
                3 => (0, 1),
                _ => (0, 0),
            };
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                move_target = Some((ny, nx));
            }
        }
        'L' => {
            // Turn Left
            state.dir = (state.dir + 3) % 4;
        }
        'R' => {
            // Turn Right
            state.dir = (state.dir + 1) % 4;
        }
        'W' => {
            // Write Memory to Grid (Underfoot)
            // Wait, usually writing underfoot is tricky if we move away.
            // But we have mutable access to VM via `vm`.
            // However, `process_agents` handles movement.
            // If we write to (y, x), we overwrite the agent?
            // No, the agent is on top.
            // But if we write, we change the cell value.
            // Let's write to the *registers* if it's an agent, or grid if empty?
            // Actually, usually "Write" means "Deposit Value".
            // If we move, we leave the agent char behind? No, `process_agents` cleans up.
            // So if we write, we should update the grid *at current location*?
            // But the agent occupies current location.
            // Let's write to the *Forward* cell.
            let (dy, dx) = match state.dir {
                0 => (-1, 0),
                1 => (0, 1),
                2 => (1, 0),
                3 => (0, -1),
                _ => (0, 0),
            };
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                // Check if target is empty or overwritable
                let target_val = &grid_snapshot[ny][nx];
                let safe = matches!(target_val, Value::Int(0) | Value::Str(_));
                if safe {
                    vm.grid[ny][nx] = Value::Int(state.memory);
                }
            }
        }
        'G' => {
            // Grab/Get (Read from Forward cell)
            let (dy, dx) = match state.dir {
                0 => (-1, 0),
                1 => (0, 1),
                2 => (1, 0),
                3 => (0, -1),
                _ => (0, 0),
            };
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                match &grid_snapshot[ny][nx] {
                    Value::Int(n) => state.memory = *n,
                    Value::Str(s) => {
                        // Try to parse int, or hash string?
                        if let Ok(n) = s.parse::<i64>() {
                            state.memory = n;
                        } else {
                            state.memory = s.len() as i64;
                        }
                    }
                    _ => {}
                }
            }
        }
        '+' => {
            state.memory = state.memory.wrapping_add(1);
        }
        '-' => {
            state.memory = state.memory.wrapping_sub(1);
        }
        '[' => {
            // Jump Forward if Zero
            if state.memory == 0 {
                let mut depth = 1;
                // Scan forward from current position
                // Note: current_byte_offset points to '[', so we skip it first?
                // Iterating from byte_offset includes current char.
                // We want to find matching ']'.
                // If we use char_indices from current offset:
                let rest = &state.program[current_byte_offset..];
                let mut chars = rest.char_indices();
                chars.next(); // Skip the current '['

                let mut steps = 0;
                for (_, c) in chars {
                    steps += 1;
                    if c == '[' {
                        depth += 1;
                    } else if c == ']' {
                        depth -= 1;
                    }
                    if depth == 0 {
                        break;
                    }
                }
                // Update PC
                next_pc = (state.pc + steps + 1) % program_char_count.max(1);
            }
        }
        ']' => {
            // Jump Back if Non-Zero
            if state.memory != 0 {
                let mut depth = 1;
                // Scan backward from current position
                // We need chars BEFORE current_byte_offset
                let prefix = &state.program[..current_byte_offset];
                let mut steps = 0;
                // Iterate in reverse
                for c in prefix.chars().rev() {
                    steps += 1;
                    if c == ']' {
                        depth += 1;
                    } else if c == '[' {
                        depth -= 1;
                    }
                    if depth == 0 {
                        // Found matching [
                        // Calculate new PC
                        // If we stepped back 'steps' chars, new pc is current - steps
                        next_pc = state.pc.saturating_sub(steps);
                        break;
                    }
                }
                if depth > 0 && state.pc > 0 {
                    // Loop wrapped or not found? Original logic aborts if hitting 0.
                    // If not found, we just continue (fall through to next instruction).
                    // Or we assume wrap around?
                    // Original code: `if scan_pc == 0 { break }`.
                    // So if we don't find it, we don't jump.
                }
            }
        }
        '!' => {
             vm.output.push(format!("AUTOMATON: {}", state.memory));
        }
        _ => {}
    }

    // Check collision for movement
    if let Some((ny, nx)) = move_target {
         let dest_val = &grid_snapshot[ny][nx];
         if !matches!(dest_val, Value::Int(0)) {
             // Blocked
             move_target = None;
         }
    }

    // Commit PC update
    state.pc = next_pc;

    updated_agent.state = state.to_value();
    Some((updated_agent, move_target))
}

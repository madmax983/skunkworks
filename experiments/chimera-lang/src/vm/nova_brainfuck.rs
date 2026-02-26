use super::{ChimeraVM, Value};
use std::collections::{HashMap, VecDeque};

/// Executes a Brainfuck program string with input.
///
/// **OpCode:** `Brainfuck`
/// **Stack:** `[ ..., bf_code, input ] -> [ ..., output ]`
pub fn exec_brainfuck(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: bf_code_string, input_string (top)
    let input = vm.pop_str("brainfuck")?;
    let code = vm.pop_str("brainfuck")?;

    let code_chars: Vec<char> = code.chars().collect();
    let mut input_chars: VecDeque<u8> = input.bytes().collect::<VecDeque<_>>();
    let mut output_bytes: Vec<u8> = Vec::new();

    let mut tape = vec![0u8; 30000];
    let mut ptr = 0;
    let mut pc = 0;
    let mut cycles = 0;
    let max_cycles = 10000; // Safety limit

    // Precompute jump targets
    let mut jumps = HashMap::new();
    let mut loop_stack = Vec::new();
    for (i, &c) in code_chars.iter().enumerate() {
        if c == '[' {
            loop_stack.push(i);
        } else if c == ']' {
            if let Some(start) = loop_stack.pop() {
                jumps.insert(start, i);
                jumps.insert(i, start);
            }
        }
    }

    while pc < code_chars.len() && cycles < max_cycles {
        match code_chars[pc] {
            '>' => {
                if ptr < tape.len() - 1 {
                    ptr += 1;
                } else {
                    ptr = 0;
                } // Wrap
            }
            '<' => {
                if ptr > 0 {
                    ptr -= 1;
                } else {
                    ptr = tape.len() - 1;
                } // Wrap
            }
            '+' => tape[ptr] = tape[ptr].wrapping_add(1),
            '-' => tape[ptr] = tape[ptr].wrapping_sub(1),
            '.' => {
                if output_bytes.len() < crate::vm::MAX_BRAINFUCK_OUTPUT {
                    output_bytes.push(tape[ptr]);
                }
            }
            ',' => {
                tape[ptr] = input_chars.pop_front().unwrap_or(0);
            }
            '[' => {
                if tape[ptr] == 0 {
                    if let Some(&target) = jumps.get(&pc) {
                        pc = target;
                    }
                }
            }
            ']' => {
                if tape[ptr] != 0 {
                    if let Some(&target) = jumps.get(&pc) {
                        pc = target;
                    }
                }
            }
            _ => {} // Ignore non-BF chars
        }
        pc += 1;
        cycles += 1;
    }

    let output_str = String::from_utf8_lossy(&output_bytes).to_string();
    vm.stack.push(Value::Str(output_str));
    vm.energy = vm.energy.saturating_sub((cycles / 100) as i64);
    vm.output.push(format!("BRAINFUCK: Ran {} cycles", cycles));

    None
}

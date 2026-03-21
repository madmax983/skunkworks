use super::{ChimeraVM, Value};
use std::collections::HashMap;

/// Executes a Brainfuck program string with input.
///
/// **OpCode:** `Brainfuck`
/// **Stack:** `[ ..., bf_code, input ] -> [ ..., output ]`
pub fn exec_brainfuck(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: bf_code_string, input_string (top)
    let input = vm.pop_str("brainfuck")?;
    let code = vm.pop_str("brainfuck")?;

    let code_bytes = code.as_bytes();
    let mut input_bytes = input.bytes();
    let mut output_bytes: Vec<u8> = Vec::new();

    let mut tape = vec![0u8; 30000];
    let mut ptr = 0;
    let mut pc = 0;
    let mut cycles = 0;
    let max_cycles = 10000; // Safety limit

    // Precompute jump targets
    let mut jumps = HashMap::new();
    let mut loop_stack = Vec::new();
    for (i, &c) in code_bytes.iter().enumerate() {
        if c == b'[' {
            loop_stack.push(i);
        } else if c == b']' {
            if let Some(start) = loop_stack.pop() {
                jumps.insert(start, i);
                jumps.insert(i, start);
            }
        }
    }

    while pc < code_bytes.len() && cycles < max_cycles {
        match code_bytes[pc] {
            b'>' => {
                if ptr < tape.len() - 1 {
                    ptr += 1;
                } else {
                    ptr = 0;
                } // Wrap
            }
            b'<' => {
                if ptr > 0 {
                    ptr -= 1;
                } else {
                    ptr = tape.len() - 1;
                } // Wrap
            }
            b'+' => tape[ptr] = tape[ptr].wrapping_add(1),
            b'-' => tape[ptr] = tape[ptr].wrapping_sub(1),
            b'.' => {
                if output_bytes.len() < crate::vm::MAX_BRAINFUCK_OUTPUT {
                    output_bytes.push(tape[ptr]);
                }
            }
            b',' => {
                tape[ptr] = input_bytes.next().unwrap_or(0);
            }
            b'[' => {
                if tape[ptr] == 0 {
                    if let Some(&target) = jumps.get(&pc) {
                        pc = target;
                    }
                }
            }
            b']' => {
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

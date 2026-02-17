use std::collections::HashMap;
use crate::vm::OpCode;

pub fn assemble(source: &str) -> Result<Vec<u8>, String> {
    let mut bytecode = Vec::new();
    let mut labels = HashMap::new();
    let mut jump_patches = Vec::new(); // (bytecode_index, label_name)

    // First pass: Generate bytecode and record labels/patches
    for line in source.lines() {
        let line = line.trim();
        // Remove comments
        let line = if let Some(idx) = line.find(';') {
            &line[..idx].trim()
        } else {
            line
        };

        if line.is_empty() {
            continue;
        }

        // Check for label
        if line.ends_with(':') {
            let label = &line[..line.len() - 1];
            if labels.contains_key(label) {
                return Err(format!("Duplicate label: {}", label));
            }
            labels.insert(label.to_string(), bytecode.len());
            continue;
        }

        // Parse instruction
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mnemonic = parts[0].to_uppercase();

        match mnemonic.as_str() {
            "PUSH" => {
                bytecode.push(OpCode::Push as u8);
                if parts.len() < 2 { return Err("PUSH requires an argument".to_string()); }
                let val: i32 = parts[1].parse().map_err(|_| "Invalid number for PUSH".to_string())?;
                bytecode.extend_from_slice(&val.to_le_bytes());
            }
            "POP" => bytecode.push(OpCode::Pop as u8),
            "ADD" => bytecode.push(OpCode::Add as u8),
            "SUB" => bytecode.push(OpCode::Sub as u8),
            "MUL" => bytecode.push(OpCode::Mul as u8),
            "DIV" => bytecode.push(OpCode::Div as u8),
            "MOD" => bytecode.push(OpCode::Mod as u8),
            "JMP" => {
                bytecode.push(OpCode::Jmp as u8);
                if parts.len() < 2 { return Err("JMP requires a label".to_string()); }
                jump_patches.push((bytecode.len(), parts[1].to_string()));
                bytecode.extend_from_slice(&0u32.to_le_bytes()); // Placeholder
            }
            "JZ" => {
                bytecode.push(OpCode::Jz as u8);
                if parts.len() < 2 { return Err("JZ requires a label".to_string()); }
                jump_patches.push((bytecode.len(), parts[1].to_string()));
                bytecode.extend_from_slice(&0u32.to_le_bytes()); // Placeholder
            }
            "JNZ" => {
                bytecode.push(OpCode::Jnz as u8);
                if parts.len() < 2 { return Err("JNZ requires a label".to_string()); }
                jump_patches.push((bytecode.len(), parts[1].to_string()));
                bytecode.extend_from_slice(&0u32.to_le_bytes()); // Placeholder
            }
            "LOAD" => bytecode.push(OpCode::Load as u8),
            "STORE" => bytecode.push(OpCode::Store as u8),
            "PLOT" => bytecode.push(OpCode::Plot as u8),
            "CLS" => bytecode.push(OpCode::Cls as u8),
            "RND" => bytecode.push(OpCode::Rnd as u8),
            "WAIT" => bytecode.push(OpCode::Wait as u8),
            "DUP" => bytecode.push(OpCode::Dup as u8),
            "HALT" => bytecode.push(OpCode::Halt as u8),
            _ => return Err(format!("Unknown instruction: {}", mnemonic)),
        }
    }

    // Second pass: Patch jumps
    for (offset, label) in jump_patches {
        let target_addr = labels.get(&label).ok_or(format!("Undefined label: {}", label))?;
        let addr_bytes = (*target_addr as u32).to_le_bytes();
        bytecode[offset..offset+4].copy_from_slice(&addr_bytes);
    }

    Ok(bytecode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble_simple() {
        let src = "PUSH 10\nPUSH 20\nADD\nHALT";
        let bc = assemble(src).expect("Failed to assemble");
        // PUSH(1) + 4 bytes + PUSH(1) + 4 bytes + ADD(1) + HALT(1) = 12 bytes
        assert_eq!(bc.len(), 12);
        assert_eq!(bc[0], OpCode::Push as u8);
        assert_eq!(bc[5], OpCode::Push as u8);
        assert_eq!(bc[10], OpCode::Add as u8);
        assert_eq!(bc[11], OpCode::Halt as u8);
    }

    #[test]
    fn test_assemble_labels() {
        let src = "
        PUSH 10
        Loop:
        PUSH 1
        SUB
        DUP
        JNZ Loop
        HALT
        ";
        let bc = assemble(src).expect("Failed to assemble");
        // Check patches
        // PUSH 10 (5 bytes)
        // Loop: offset 5
        // PUSH 1 (5 bytes) -> 10
        // SUB (1 byte) -> 11
        // DUP (1 byte) -> 12
        // JNZ (5 bytes) -> 17. Target should be 5.
        // HALT (1 byte) -> 18

        // Check JNZ target
        let jnz_opcode_idx = 12;
        assert_eq!(bc[jnz_opcode_idx], OpCode::Jnz as u8);
        let target_bytes = &bc[jnz_opcode_idx+1..jnz_opcode_idx+5];
        let target = u32::from_le_bytes(target_bytes.try_into().unwrap());
        assert_eq!(target, 5);
    }
}

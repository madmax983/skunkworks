use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    End,
    Fwd,       // Moves by current step_size
    Rot(i8),   // Rotates by N degrees
    Pen(bool), // true = down, false = up
    Color(u8, u8, u8),
    SetStep(u8),
    AddStep(i8),
    Rep(u8, u8), // count, instruction_count
}

impl Instruction {
    #[allow(dead_code)]
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Instruction::End => vec![0x00],
            Instruction::Fwd => vec![0x01],
            Instruction::Rot(deg) => vec![0x02, *deg as u8],
            Instruction::Pen(down) => vec![0x03, if *down { 1 } else { 0 }],
            Instruction::Color(r, g, b) => vec![0x04, *r, *g, *b],
            Instruction::SetStep(val) => vec![0x05, *val],
            Instruction::AddStep(val) => vec![0x06, *val as u8],
            Instruction::Rep(count, len) => vec![0x07, *count, *len],
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        match self {
            Instruction::End => 1,
            Instruction::Fwd => 1,
            Instruction::Rot(_) => 2,
            Instruction::Pen(_) => 2,
            Instruction::Color(_, _, _) => 4,
            Instruction::SetStep(_) => 2,
            Instruction::AddStep(_) => 2,
            Instruction::Rep(_, _) => 3,
        }
    }
}

pub struct Assembler;

impl Assembler {
    pub fn parse(input: &str) -> Result<Vec<u8>> {
        let mut bytecode = Vec::new();
        let tokens: Vec<&str> = input.split_whitespace().collect();
        let mut i = 0;

        while i < tokens.len() {
            let token = tokens[i].to_uppercase();
            i += 1;

            match token.as_str() {
                "END" => bytecode.push(0x00),
                "FWD" => bytecode.push(0x01),
                "ROT" => {
                    if i >= tokens.len() {
                        return Err(anyhow!("Missing arg for ROT"));
                    }
                    let val = tokens[i].parse::<i8>()?;
                    bytecode.push(0x02);
                    bytecode.push(val as u8);
                    i += 1;
                }
                "PEN" => {
                    if i >= tokens.len() {
                        return Err(anyhow!("Missing arg for PEN"));
                    }
                    let val = tokens[i].parse::<u8>()?;
                    bytecode.push(0x03);
                    bytecode.push(val);
                    i += 1;
                }
                "COL" => {
                    if i + 2 >= tokens.len() {
                        return Err(anyhow!("Missing args for COL"));
                    }
                    let r = tokens[i].parse::<u8>()?;
                    let g = tokens[i + 1].parse::<u8>()?;
                    let b = tokens[i + 2].parse::<u8>()?;
                    bytecode.push(0x04);
                    bytecode.push(r);
                    bytecode.push(g);
                    bytecode.push(b);
                    i += 3;
                }
                "SET_STEP" => {
                    if i >= tokens.len() {
                        return Err(anyhow!("Missing arg for SET_STEP"));
                    }
                    let val = tokens[i].parse::<u8>()?;
                    bytecode.push(0x05);
                    bytecode.push(val);
                    i += 1;
                }
                "ADD_STEP" => {
                    if i >= tokens.len() {
                        return Err(anyhow!("Missing arg for ADD_STEP"));
                    }
                    let val = tokens[i].parse::<i8>()?;
                    bytecode.push(0x06);
                    bytecode.push(val as u8);
                    i += 1;
                }
                "REP" => {
                    if i + 1 >= tokens.len() {
                        return Err(anyhow!("Missing args for REP"));
                    }
                    let count = tokens[i].parse::<u8>()?;
                    let len = tokens[i + 1].parse::<u8>()?;
                    bytecode.push(0x07);
                    bytecode.push(count);
                    bytecode.push(len);
                    i += 2;
                }
                // Comments
                s if s.starts_with('#') || s.starts_with("//") => {
                    // Skip comments
                    // Since we parse token by token, we might be inside a line comment.
                    // But split_whitespace breaks lines.
                    // This simple parser only handles comments if the token itself starts with #.
                    // It won't skip the rest of the line.
                    // For a robust comment system, we need line-based parsing.
                    // For now, let's just ignore tokens starting with #
                    continue;
                }
                _ => return Err(anyhow!("Unknown token: {}", token)),
            }
        }

        Ok(bytecode)
    }

    pub fn disassemble(bytes: &[u8]) -> Result<Vec<Instruction>> {
        let mut instructions = Vec::new();
        let mut i = 0;
        while i < bytes.len() {
            let op = bytes[i];
            i += 1;
            match op {
                0x00 => instructions.push(Instruction::End),
                0x01 => instructions.push(Instruction::Fwd),
                0x02 => {
                    if i >= bytes.len() {
                        break;
                    }
                    instructions.push(Instruction::Rot(bytes[i] as i8));
                    i += 1;
                }
                0x03 => {
                    if i >= bytes.len() {
                        break;
                    }
                    instructions.push(Instruction::Pen(bytes[i] != 0));
                    i += 1;
                }
                0x04 => {
                    if i + 2 >= bytes.len() {
                        break;
                    }
                    instructions.push(Instruction::Color(bytes[i], bytes[i + 1], bytes[i + 2]));
                    i += 3;
                }
                0x05 => {
                    if i >= bytes.len() {
                        break;
                    }
                    instructions.push(Instruction::SetStep(bytes[i]));
                    i += 1;
                }
                0x06 => {
                    if i >= bytes.len() {
                        break;
                    }
                    instructions.push(Instruction::AddStep(bytes[i] as i8));
                    i += 1;
                }
                0x07 => {
                    if i + 1 >= bytes.len() {
                        break;
                    }
                    instructions.push(Instruction::Rep(bytes[i], bytes[i + 1]));
                    i += 2;
                }
                _ => return Err(anyhow!("Unknown opcode: {}", op)),
            }
        }
        Ok(instructions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asm_disasm() {
        let asm = "FWD ROT 90 PEN 0 COL 255 0 0 REP 2 1 FWD";
        // Wait, Rep 2 1 means repeat next 1 instruction.
        // Instructions: Fwd (1), Rot (2), Pen (2), Col (4), Rep (3), Fwd (1)
        // Bytecode len: 1+2+2+4+3+1 = 13.
        let bytes = Assembler::parse(asm).unwrap();
        let instrs = Assembler::disassemble(&bytes).unwrap();

        assert_eq!(instrs.len(), 6);
        assert_eq!(instrs[0], Instruction::Fwd);
        assert_eq!(instrs[1], Instruction::Rot(90));
        assert_eq!(instrs[4], Instruction::Rep(2, 1));
        assert_eq!(instrs[5], Instruction::Fwd);
    }
}

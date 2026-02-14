use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shape {
    Circle = 0,
    Square = 1,
    Triangle = 2,
    Diamond = 3,
    Hexagon = 4,
    Star = 5,
    Pentagram = 6,
    Octagon = 7,
}

impl Shape {
    pub fn from_u8(n: u8) -> Option<Self> {
        match n {
            0 => Some(Shape::Circle),
            1 => Some(Shape::Square),
            2 => Some(Shape::Triangle),
            3 => Some(Shape::Diamond),
            4 => Some(Shape::Hexagon),
            5 => Some(Shape::Star),
            6 => Some(Shape::Pentagram),
            7 => Some(Shape::Octagon),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
    Purple = 4,
    Cyan = 5,
    White = 6,
    Black = 7,
}

impl Color {
    pub fn from_u8(n: u8) -> Option<Self> {
        match n {
            0 => Some(Color::Red),
            1 => Some(Color::Green),
            2 => Some(Color::Blue),
            3 => Some(Color::Yellow),
            4 => Some(Color::Purple),
            5 => Some(Color::Cyan),
            6 => Some(Color::White),
            7 => Some(Color::Black),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Jewel {
    pub shape: Shape,
    pub color: Color,
}

impl Jewel {
    pub fn new(shape: Shape, color: Color) -> Self {
        Self { shape, color }
    }

    /// Encodes a 4-bit nibble into a Jewel.
    /// Uses randomization to allow multiple visual representations for the same value.
    /// Formula: (Shape_ID * 8 + Color_ID) % 16 == nibble
    pub fn from_nibble(nibble: u8) -> Self {
        let nibble = nibble & 0xF;
        let mut rng = rand::thread_rng();

        // We need to find (s, c) such that (s * 8 + c) % 16 == nibble
        // (s * 8 + c) % 16 = (s % 2) * 8 + c (since c < 8)
        // So nibble = (s % 2) * 8 + c
        // c = nibble % 8
        // s % 2 = nibble / 8

        let c_idx = nibble % 8;
        let s_mod_2 = nibble / 8; // 0 or 1

        // s can be any shape index such that s % 2 == s_mod_2
        // Shapes are 0..7.
        // If s_mod_2 is 0, s can be 0, 2, 4, 6.
        // If s_mod_2 is 1, s can be 1, 3, 5, 7.

        let valid_shapes = if s_mod_2 == 0 {
            vec![0, 2, 4, 6]
        } else {
            vec![1, 3, 5, 7]
        };

        let s_idx = valid_shapes[rng.gen_range(0..valid_shapes.len())];

        Jewel {
            shape: Shape::from_u8(s_idx).unwrap(),
            color: Color::from_u8(c_idx).unwrap(),
        }
    }

    pub fn to_nibble(&self) -> u8 {
        let s = self.shape as u8;
        let c = self.color as u8;
        // Formula reverse:
        // nibble = (s % 2) * 8 + c
        (s % 2) * 8 + c
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    HALT = 0x00,
    PUSH = 0x01,
    ADD = 0x02,
    SUB = 0x03,
    MUL = 0x04,
    DIV = 0x05,
    MOD = 0x06,
    EQ = 0x07,
    LT = 0x08,
    JMP = 0x09,
    JZ = 0x0A,
    DUP = 0x0B,
    SWAP = 0x0C,
    PRINT = 0x0D,
    // Extensions
    POP = 0x0E,
}

impl OpCode {
    pub fn from_u8(n: u8) -> Option<Self> {
        match n {
            0x00 => Some(OpCode::HALT),
            0x01 => Some(OpCode::PUSH),
            0x02 => Some(OpCode::ADD),
            0x03 => Some(OpCode::SUB),
            0x04 => Some(OpCode::MUL),
            0x05 => Some(OpCode::DIV),
            0x06 => Some(OpCode::MOD),
            0x07 => Some(OpCode::EQ),
            0x08 => Some(OpCode::LT),
            0x09 => Some(OpCode::JMP),
            0x0A => Some(OpCode::JZ),
            0x0B => Some(OpCode::DUP),
            0x0C => Some(OpCode::SWAP),
            0x0D => Some(OpCode::PRINT),
            0x0E => Some(OpCode::POP),
            _ => None,
        }
    }

    pub fn has_operand(&self) -> bool {
        match self {
            OpCode::PUSH | OpCode::JMP | OpCode::JZ => true,
            _ => false,
        }
    }
}

pub fn encode(data: &[u8]) -> Vec<Jewel> {
    let mut jewels = Vec::new();
    for &byte in data {
        let low = byte & 0xF;
        let high = (byte >> 4) & 0xF;
        jewels.push(Jewel::from_nibble(high)); // High nibble first? Or low?
        // Standard is usually High then Low for readability if printed hex.
        // Let's do High first.
        jewels.push(Jewel::from_nibble(low));
    }
    jewels
}

pub fn decode(jewels: &[Jewel]) -> Vec<u8> {
    let mut data = Vec::new();
    let mut iter = jewels.iter();

    while let Some(high_jewel) = iter.next() {
        if let Some(low_jewel) = iter.next() {
            let high = high_jewel.to_nibble();
            let low = low_jewel.to_nibble();
            let byte = (high << 4) | low;
            data.push(byte);
        } else {
            // Odd number of jewels? Ignore the last one or error?
            // Ignore for now.
            break;
        }
    }
    data
}

pub fn compile(source: &str) -> Vec<u8> {
    let mut bytecode = Vec::new();
    let tokens: Vec<&str> = source.split_whitespace().collect();
    let mut iter = tokens.iter();

    while let Some(&token) = iter.next() {
        let op = match token {
            "HALT" => Some(OpCode::HALT),
            "PUSH" => Some(OpCode::PUSH),
            "ADD" => Some(OpCode::ADD),
            "SUB" => Some(OpCode::SUB),
            "MUL" => Some(OpCode::MUL),
            "DIV" => Some(OpCode::DIV),
            "MOD" => Some(OpCode::MOD),
            "EQ" => Some(OpCode::EQ),
            "LT" => Some(OpCode::LT),
            "JMP" => Some(OpCode::JMP),
            "JZ" => Some(OpCode::JZ),
            "DUP" => Some(OpCode::DUP),
            "SWAP" => Some(OpCode::SWAP),
            "PRINT" => Some(OpCode::PRINT),
            "POP" => Some(OpCode::POP),
            _ => None,
        };

        if let Some(op_code) = op {
            bytecode.push(op_code as u8);
            if op_code.has_operand() {
                if let Some(arg_str) = iter.next() {
                    if let Ok(arg) = arg_str.parse::<u8>() {
                        bytecode.push(arg);
                    } else {
                        // Error handling? Push 0
                        bytecode.push(0);
                    }
                } else {
                     bytecode.push(0);
                }
            }
        } else {
            // Ignore unknown tokens or comments?
            // Could be a label? Not implementing labels yet.
        }
    }

    bytecode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let original_data: Vec<u8> = vec![0x12, 0x34, 0xAB, 0xCD, 0xFF, 0x00];
        let jewels = encode(&original_data);
        let decoded_data = decode(&jewels);
        assert_eq!(original_data, decoded_data);
    }

    #[test]
    fn test_compile() {
        let source = "PUSH 10 ADD";
        // PUSH = 0x01, 10 = 0x0A, ADD = 0x02
        let expected = vec![0x01, 0x0A, 0x02];
        let compiled = compile(source);
        assert_eq!(compiled, expected);
    }
}

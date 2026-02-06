use crate::opcode::OpCode;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Knot {
    Simple,          // NOP
    FigureEight,     // Terminator
    Op(OpCode),      // Instruction
}

impl fmt::Display for Knot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Knot::Simple => write!(f, "●"),
            Knot::FigureEight => write!(f, "∞"),
            Knot::Op(op) => write!(f, "[{}]", op),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Cord {
    pub id: usize,
    pub knots: Vec<Knot>,
}

impl Cord {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            knots: Vec::new(),
        }
    }

    pub fn push(&mut self, knot: Knot) {
        self.knots.push(knot);
    }

    pub fn push_op(&mut self, op: OpCode) {
        self.knots.push(Knot::Op(op));
    }
}

#[derive(Debug, Clone, Default)]
pub struct Quipu {
    pub cords: Vec<Cord>,
}

impl Quipu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_cord(&mut self, cord: Cord) {
        self.cords.push(cord);
    }

    pub fn get_cord(&self, id: usize) -> Option<&Cord> {
        self.cords.get(id)
    }
}

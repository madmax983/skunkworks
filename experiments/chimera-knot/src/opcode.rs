use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    // Stack Ops
    Push(i64),
    Dup,
    Swap,
    Drop,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // Control Flow
    Jump(usize), // Jump to a specific Cord index
    Call(usize), // Call a cord as a subroutine
    Brz(usize),  // Branch if Zero to Cord index
    Ret,         // Return to parent cord

    // IO
    Print, // Print top of stack

    // Magic
    Tangle, // Randomize stack top
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Push(v) => write!(f, "PUSH {}", v),
            OpCode::Dup => write!(f, "DUP"),
            OpCode::Swap => write!(f, "SWAP"),
            OpCode::Drop => write!(f, "DROP"),
            OpCode::Add => write!(f, "ADD"),
            OpCode::Sub => write!(f, "SUB"),
            OpCode::Mul => write!(f, "MUL"),
            OpCode::Div => write!(f, "DIV"),
            OpCode::Jump(c) => write!(f, "JUMP #{}", c),
            OpCode::Call(c) => write!(f, "CALL #{}", c),
            OpCode::Brz(c) => write!(f, "BRZ #{}", c),
            OpCode::Ret => write!(f, "RET"),
            OpCode::Print => write!(f, "PRINT"),
            OpCode::Tangle => write!(f, "TANGLE"),
        }
    }
}

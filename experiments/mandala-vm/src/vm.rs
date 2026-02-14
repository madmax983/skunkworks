use crate::cipher::OpCode;

pub struct VM {
    pub stack: Vec<i32>,
    pub pc: usize,
    pub output: String,
    pub halted: bool,
    pub cycle: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            pc: 0,
            output: String::new(),
            halted: false,
            cycle: 0,
        }
    }

    pub fn step(&mut self, bytecode: &[u8]) {
        if self.halted {
            return;
        }

        if self.pc >= bytecode.len() {
            self.halted = true;
            return;
        }

        let op_byte = bytecode[self.pc];
        if let Some(op) = OpCode::from_u8(op_byte) {
            match op {
                OpCode::HALT => {
                    self.halted = true;
                    self.pc += 1;
                }
                OpCode::PUSH => {
                    if self.pc + 1 < bytecode.len() {
                        let val = bytecode[self.pc + 1] as i32;
                        self.stack.push(val);
                        self.pc += 2;
                    } else {
                        self.halted = true; // Error: Unexpected EOF
                    }
                }
                OpCode::ADD => {
                    if self.stack.len() >= 2 {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a + b);
                    }
                    self.pc += 1;
                }
                OpCode::SUB => {
                    if self.stack.len() >= 2 {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a - b);
                    }
                    self.pc += 1;
                }
                OpCode::MUL => {
                    if self.stack.len() >= 2 {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a * b);
                    }
                    self.pc += 1;
                }
                OpCode::DIV => {
                    if self.stack.len() >= 2 {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        if b != 0 {
                            self.stack.push(a / b);
                        } else {
                            self.halted = true; // Error: Div by zero
                            self.output.push_str("Error: Div by Zero\n");
                        }
                    }
                    self.pc += 1;
                }
                OpCode::MOD => {
                    if self.stack.len() >= 2 {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        if b != 0 {
                            self.stack.push(a % b);
                        } else {
                             self.halted = true;
                        }
                    }
                    self.pc += 1;
                }
                OpCode::EQ => {
                    if self.stack.len() >= 2 {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(if a == b { 1 } else { 0 });
                    }
                    self.pc += 1;
                }
                OpCode::LT => {
                    if self.stack.len() >= 2 {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(if a < b { 1 } else { 0 });
                    }
                    self.pc += 1;
                }
                OpCode::JMP => {
                     if self.pc + 1 < bytecode.len() {
                        let target = bytecode[self.pc + 1] as usize;
                        self.pc = target;
                    } else {
                        self.halted = true;
                    }
                }
                OpCode::JZ => {
                    if self.pc + 1 < bytecode.len() {
                        let target = bytecode[self.pc + 1] as usize;
                        if let Some(val) = self.stack.pop() {
                            if val == 0 {
                                self.pc = target;
                            } else {
                                self.pc += 2;
                            }
                        } else {
                             self.pc += 2; // Error handling?
                        }
                    } else {
                        self.halted = true;
                    }
                }
                OpCode::DUP => {
                    if let Some(&val) = self.stack.last() {
                        self.stack.push(val);
                    }
                    self.pc += 1;
                }
                OpCode::SWAP => {
                    let len = self.stack.len();
                    if len >= 2 {
                        self.stack.swap(len - 1, len - 2);
                    }
                    self.pc += 1;
                }
                OpCode::PRINT => {
                    if let Some(val) = self.stack.pop() {
                        // Print as char if ASCII, else number
                        if val >= 32 && val <= 126 {
                             self.output.push(val as u8 as char);
                        } else {
                             self.output.push_str(&format!("{}", val));
                        }
                    }
                    self.pc += 1;
                }
                 OpCode::POP => {
                    self.stack.pop();
                    self.pc += 1;
                }
            }
        } else {
            // Unknown OpCode, skip
            self.pc += 1;
        }
        self.cycle += 1;
    }

    pub fn execute(&mut self, bytecode: &[u8]) {
        while !self.halted && self.pc < bytecode.len() {
            self.step(bytecode);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cipher::compile;

    #[test]
    fn test_add() {
        let mut vm = VM::new();
        let code = compile("PUSH 10 PUSH 20 ADD");
        vm.execute(&code);
        assert_eq!(vm.stack.last(), Some(&30));
    }

    #[test]
    fn test_print() {
        let mut vm = VM::new();
        // 65 = 'A'
        let code = compile("PUSH 65 PRINT");
        vm.execute(&code);
        assert_eq!(vm.output, "A");
    }

    #[test]
    fn test_loop() {
        // PUSH 5 (Counter)
        // DUP
        // PUSH 0
        // EQ
        // JZ 9 (Jump to Halt if 0) -> No, Jump to End
        // ...
        // Too complex to hand write offsets.
        // Let's test basic jumps.
        let mut vm = VM::new();
        // PUSH 0 JZ 4 PUSH 1 HALT PUSH 2 HALT
        // 0: PUSH 0 (01 00) -> PC=2. Stack=[0]
        // 2: JZ 6   (0A 06) -> Pop 0. PC=6.
        // 4: PUSH 1 (01 01) -> Skipped
        // 6: PUSH 2 (01 02) -> Executed. Stack=[2]
        // 8: HALT

        let code = vec![
            0x01, 0x00, // PUSH 0
            0x0A, 0x06, // JZ 6
            0x01, 0x01, // PUSH 1
            0x01, 0x02, // PUSH 2 (Offset 6)
            0x00        // HALT
        ];

        vm.execute(&code);
        assert_eq!(vm.stack.last(), Some(&2));
    }
}

use rand::Rng;

pub const SCREEN_WIDTH: usize = 128;
pub const SCREEN_HEIGHT: usize = 128;
pub const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
pub const MEMORY_SIZE: usize = 65536;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    Push = 0x01,
    Pop = 0x02,
    Add = 0x03,
    Sub = 0x04,
    Mul = 0x05,
    Div = 0x06,
    Mod = 0x07,
    Jmp = 0x08,
    Jz = 0x09,
    Jnz = 0x0A,
    Load = 0x0B,
    Store = 0x0C,
    Plot = 0x0D,
    Cls = 0x0E,
    Rnd = 0x0F,
    Wait = 0x10,
    Dup = 0x11,
    Halt = 0xFF,
}

impl OpCode {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0x01 => Some(OpCode::Push),
            0x02 => Some(OpCode::Pop),
            0x03 => Some(OpCode::Add),
            0x04 => Some(OpCode::Sub),
            0x05 => Some(OpCode::Mul),
            0x06 => Some(OpCode::Div),
            0x07 => Some(OpCode::Mod),
            0x08 => Some(OpCode::Jmp),
            0x09 => Some(OpCode::Jz),
            0x0A => Some(OpCode::Jnz),
            0x0B => Some(OpCode::Load),
            0x0C => Some(OpCode::Store),
            0x0D => Some(OpCode::Plot),
            0x0E => Some(OpCode::Cls),
            0x0F => Some(OpCode::Rnd),
            0x10 => Some(OpCode::Wait),
            0x11 => Some(OpCode::Dup),
            0xFF => Some(OpCode::Halt),
            _ => None,
        }
    }
}

pub struct VM {
    pub memory: [u8; MEMORY_SIZE],
    pub stack: Vec<i32>,
    pub pc: usize, // Program Counter
    pub screen: [u8; SCREEN_SIZE],
    pub halted: bool,
    pub waiting: bool,
}

impl VM {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            stack: Vec::with_capacity(1024),
            pc: 0,
            screen: [0; SCREEN_SIZE],
            halted: false,
            waiting: false,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        // Clear memory
        self.memory.fill(0);
        // Load program into memory at 0
        if program.len() > MEMORY_SIZE {
            panic!("Program too large for memory");
        }
        self.memory[..program.len()].copy_from_slice(program);
        self.pc = 0;
        self.halted = false;
        self.waiting = false;
        self.stack.clear();
    }

    pub fn step(&mut self) {
        if self.halted || self.waiting {
            return;
        }

        if self.pc >= MEMORY_SIZE {
            self.halted = true;
            return;
        }

        let opcode_byte = self.memory[self.pc];
        self.pc += 1;

        match OpCode::from_u8(opcode_byte) {
            Some(OpCode::Push) => {
                // Read next 4 bytes as i32 (little endian)
                if self.pc + 4 > MEMORY_SIZE {
                    self.halted = true;
                    return;
                }
                let bytes = &self.memory[self.pc..self.pc + 4];
                let val = i32::from_le_bytes(bytes.try_into().unwrap());
                self.stack.push(val);
                self.pc += 4;
            }
            Some(OpCode::Pop) => {
                self.stack.pop();
            }
            Some(OpCode::Add) => {
                let b = self.stack.pop().unwrap_or(0);
                let a = self.stack.pop().unwrap_or(0);
                self.stack.push(a.wrapping_add(b));
            }
            Some(OpCode::Sub) => {
                let b = self.stack.pop().unwrap_or(0);
                let a = self.stack.pop().unwrap_or(0);
                self.stack.push(a.wrapping_sub(b));
            }
            Some(OpCode::Mul) => {
                let b = self.stack.pop().unwrap_or(0);
                let a = self.stack.pop().unwrap_or(0);
                self.stack.push(a.wrapping_mul(b));
            }
            Some(OpCode::Div) => {
                let b = self.stack.pop().unwrap_or(1); // Avoid div by zero
                let a = self.stack.pop().unwrap_or(0);
                if b == 0 {
                    self.stack.push(0);
                } else {
                    self.stack.push(a.wrapping_div(b));
                }
            }
            Some(OpCode::Mod) => {
                let b = self.stack.pop().unwrap_or(1);
                let a = self.stack.pop().unwrap_or(0);
                if b == 0 {
                    self.stack.push(0);
                } else {
                    self.stack.push(a.wrapping_rem(b));
                }
            }
            Some(OpCode::Jmp) => {
                if self.pc + 4 > MEMORY_SIZE {
                    self.halted = true;
                    return;
                }
                let bytes = &self.memory[self.pc..self.pc + 4];
                let addr = u32::from_le_bytes(bytes.try_into().unwrap()) as usize;
                self.pc = addr;
            }
            Some(OpCode::Jz) => {
                if self.pc + 4 > MEMORY_SIZE {
                    self.halted = true;
                    return;
                }
                let bytes = &self.memory[self.pc..self.pc + 4];
                let addr = u32::from_le_bytes(bytes.try_into().unwrap()) as usize;
                let val = self.stack.pop().unwrap_or(0);
                self.pc += 4; // Advance past addr
                if val == 0 {
                    self.pc = addr;
                }
            }
            Some(OpCode::Jnz) => {
                if self.pc + 4 > MEMORY_SIZE {
                    self.halted = true;
                    return;
                }
                let bytes = &self.memory[self.pc..self.pc + 4];
                let addr = u32::from_le_bytes(bytes.try_into().unwrap()) as usize;
                let val = self.stack.pop().unwrap_or(0);
                self.pc += 4; // Advance past addr
                if val != 0 {
                    self.pc = addr;
                }
            }
            Some(OpCode::Load) => {
                let addr = self.stack.pop().unwrap_or(0) as usize;
                if addr < MEMORY_SIZE {
                    self.stack.push(self.memory[addr] as i32);
                } else {
                    self.stack.push(0);
                }
            }
            Some(OpCode::Store) => {
                let val = self.stack.pop().unwrap_or(0) as u8;
                let addr = self.stack.pop().unwrap_or(0) as usize;
                if addr < MEMORY_SIZE {
                    self.memory[addr] = val;
                }
            }
            Some(OpCode::Plot) => {
                let color = self.stack.pop().unwrap_or(0) as u8;
                let y = self.stack.pop().unwrap_or(0) as usize;
                let x = self.stack.pop().unwrap_or(0) as usize;
                if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
                    self.screen[y * SCREEN_WIDTH + x] = color;
                }
            }
            Some(OpCode::Cls) => {
                let color = self.stack.pop().unwrap_or(0) as u8;
                self.screen.fill(color);
            }
            Some(OpCode::Rnd) => {
                let max = self.stack.pop().unwrap_or(0);
                if max <= 0 {
                    self.stack.push(0);
                } else {
                    let val = rand::thread_rng().gen_range(0..max);
                    self.stack.push(val);
                }
            }
            Some(OpCode::Wait) => {
                self.waiting = true;
            }
            Some(OpCode::Dup) => {
                if let Some(&val) = self.stack.last() {
                    self.stack.push(val);
                }
            }
            Some(OpCode::Halt) => {
                self.halted = true;
            }
            None => {
                // Unknown opcode, halt
                // println!("Unknown Opcode: 0x{:02X} at {}", opcode_byte, self.pc - 1);
                self.halted = true;
            }
        }
    }

    pub fn run_frame(&mut self, instructions_per_frame: usize) {
        if self.halted {
            return;
        }
        self.waiting = false;
        for _ in 0..instructions_per_frame {
            self.step();
            if self.waiting || self.halted {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut vm = VM::new();
        // PUSH 10, PUSH 20, ADD, HALT
        let mut prog = Vec::new();
        prog.push(OpCode::Push as u8);
        prog.extend_from_slice(&10i32.to_le_bytes());
        prog.push(OpCode::Push as u8);
        prog.extend_from_slice(&20i32.to_le_bytes());
        prog.push(OpCode::Add as u8);
        prog.push(OpCode::Halt as u8);

        vm.load_program(&prog);
        vm.run_frame(100);

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], 30);
    }

    #[test]
    fn test_loop() {
        let mut vm = VM::new();
        // i = 10
        // Loop:
        // i = i - 1
        // if i != 0 goto Loop
        // Halt

        // PUSH 10
        // Loop_Label: (PC = 5)
        // PUSH 1
        // SUB
        // JNZ Loop_Label (needs dup? No JNZ pops)
        // Wait, JNZ pops. So we need to DUP if we want to check and keep.
        // My VM doesn't have DUP. I should add it or just re-push.
        // Let's just do: PUSH 10. Loop: PUSH 1, SUB. PUSH (new val), PUSH (new val), JNZ... messy.
        // Let's add DUP opcode?
        // Or just re-implement test logic simpler.

        // PUSH 5
        // SUB 1
        // (stack is 4)
        // HALT

        // PUSH 5, PUSH 1, SUB, HALT
        let mut prog = Vec::new();
        prog.push(OpCode::Push as u8);
        prog.extend_from_slice(&5i32.to_le_bytes());
        prog.push(OpCode::Push as u8);
        prog.extend_from_slice(&1i32.to_le_bytes());
        prog.push(OpCode::Sub as u8);
        prog.push(OpCode::Halt as u8);

        vm.load_program(&prog);
        vm.run_frame(100);

        assert_eq!(vm.stack[0], 4);
    }
}

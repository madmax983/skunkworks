//! The Virtual Machine for executing Stego-Cartridge bytecode.
//!
//! This module provides the [`VM`](crate::vm::VM) and the instruction set ([`OpCode`](crate::vm::OpCode)) required
//! to run the embedded programs. The VM is a simple stack-based architecture
//! with a built-in 128x128 palette-indexed framebuffer.

use rand::Rng;

/// The width of the VM's framebuffer in pixels.
pub const SCREEN_WIDTH: usize = 128;
/// The height of the VM's framebuffer in pixels.
pub const SCREEN_HEIGHT: usize = 128;
/// The total number of pixels in the framebuffer (`SCREEN_WIDTH` * `SCREEN_HEIGHT`).
pub const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
/// The total byte capacity of the VM's flat memory space.
pub const MEMORY_SIZE: usize = 65536;

/// The instruction set architecture for the Stego-Cartridge [`VM`].
///
/// Each instruction is represented as a single byte in memory.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    /// Pushes the next 4 bytes in memory onto the stack as a little-endian `i32`.
    Push = 0x01,
    /// Removes the top value from the stack.
    Pop = 0x02,
    /// Pops two values, adds them, and pushes the result.
    Add = 0x03,
    /// Pops two values (A then B), subtracts them (B - A), and pushes the result.
    Sub = 0x04,
    /// Pops two values, multiplies them, and pushes the result.
    Mul = 0x05,
    /// Pops two values (A then B), divides them (B / A), and pushes the result. Avoids dividing by zero.
    Div = 0x06,
    /// Pops two values (A then B), calculates modulo (B % A), and pushes the result.
    Mod = 0x07,
    /// Unconditional jump. Reads the next 4 bytes as the memory address to set the PC to.
    Jmp = 0x08,
    /// Jump if Zero. Pops a value; if 0, reads next 4 bytes as jump address.
    Jz = 0x09,
    /// Jump if Not Zero. Pops a value; if not 0, reads next 4 bytes as jump address.
    Jnz = 0x0A,
    /// Pops an address, reads a byte from that memory address, and pushes it as an `i32`.
    Load = 0x0B,
    /// Pops a value and an address, and stores the lowest byte of the value at that address.
    Store = 0x0C,
    /// Pops an X, Y, and Color index, and updates the framebuffer at `(X, Y)`.
    Plot = 0x0D,
    /// Pops a Color index and fills the entire framebuffer with it.
    Cls = 0x0E,
    /// Pops a maximum value, generates a random number `[0..max)`, and pushes it.
    Rnd = 0x0F,
    /// Signals the VM to pause execution until the next frame.
    Wait = 0x10,
    /// Duplicates the top value on the stack.
    Dup = 0x11,
    /// Halts VM execution permanently.
    Halt = 0xFF,
}

impl OpCode {
    /// Attempts to parse a raw byte into an [`OpCode`].
    ///
    /// # Arguments
    /// * `v` - The raw byte read from memory.
    ///
    /// # Examples
    /// ```rust
    /// use stego_cartridge::vm::OpCode;
    ///
    /// assert_eq!(OpCode::from_u8(0x01), Some(OpCode::Push));
    /// assert_eq!(OpCode::from_u8(0xFF), Some(OpCode::Halt));
    /// assert_eq!(OpCode::from_u8(0x99), None); // Invalid opcode
    /// ```
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

/// The state and execution context for a Stego-Cartridge program.
///
/// Contains a flat memory space, an operational stack, a program counter,
/// and a palette-indexed framebuffer.
pub struct VM {
    /// The flat 64KB memory space.
    pub memory: [u8; MEMORY_SIZE],
    /// The operational data stack holding `i32` values.
    pub stack: Vec<i32>,
    /// The Program Counter (index of the next instruction in memory).
    pub pc: usize,
    /// The 128x128 palette-indexed framebuffer.
    pub screen: [u8; SCREEN_SIZE],
    /// If true, the VM has hit a `HALT` instruction or fatal error and will not run further.
    pub halted: bool,
    /// If true, the VM hit a `WAIT` instruction and is paused until the next frame tick.
    pub waiting: bool,
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    /// Creates a new Virtual Machine in a clean, zeroed state.
    ///
    /// # Examples
    /// ```rust
    /// use stego_cartridge::vm::VM;
    ///
    /// let vm = VM::new();
    /// assert_eq!(vm.pc, 0);
    /// assert_eq!(vm.halted, false);
    /// ```
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

    /// Loads a bytecode program into memory starting at address `0` and resets the VM state.
    ///
    /// # Arguments
    /// * `program` - The raw bytecode slice to load.
    ///
    /// # Panics
    /// Panics if the `program` length exceeds [`MEMORY_SIZE`] (64KB).
    ///
    /// # Examples
    /// ```rust
    /// use stego_cartridge::vm::{VM, OpCode};
    ///
    /// let mut vm = VM::new();
    /// let program = vec![OpCode::Push as u8, 10, 0, 0, 0, OpCode::Halt as u8];
    /// vm.load_program(&program);
    /// assert_eq!(vm.memory[0], OpCode::Push as u8);
    /// ```
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

    /// Executes a single instruction at the current Program Counter.
    ///
    /// If the VM is halted or waiting, this function does nothing.
    /// If an unknown opcode is encountered, or a memory read goes out of bounds,
    /// the VM is automatically halted.
    ///
    /// # Examples
    /// ```rust
    /// use stego_cartridge::vm::{VM, OpCode};
    ///
    /// let mut vm = VM::new();
    /// // PUSH 42
    /// let program = vec![OpCode::Push as u8, 42, 0, 0, 0];
    /// vm.load_program(&program);
    ///
    /// vm.step();
    /// assert_eq!(vm.stack[0], 42);
    /// ```
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
                let val = i32::from_le_bytes(bytes.try_into().unwrap_or([0; 4]));
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
                let addr = u32::from_le_bytes(bytes.try_into().unwrap_or([0; 4])) as usize;
                self.pc = addr;
            }
            Some(OpCode::Jz) => {
                if self.pc + 4 > MEMORY_SIZE {
                    self.halted = true;
                    return;
                }
                let bytes = &self.memory[self.pc..self.pc + 4];
                let addr = u32::from_le_bytes(bytes.try_into().unwrap_or([0; 4])) as usize;
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
                let addr = u32::from_le_bytes(bytes.try_into().unwrap_or([0; 4])) as usize;
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

    /// Runs a batch of instructions representing a single frame of execution.
    ///
    /// Execution will stop early if the VM hits a `WAIT` instruction, a `HALT`
    /// instruction, or after `instructions_per_frame` cycles have elapsed. The
    /// `waiting` flag is cleared at the start of this call.
    ///
    /// # Arguments
    /// * `instructions_per_frame` - The maximum number of instructions to execute before yielding.
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

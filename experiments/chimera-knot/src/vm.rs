use crate::opcode::OpCode;
use crate::quipu::{Knot, Quipu};

pub struct VM {
    pub quipu: Quipu,
    pub stack: Vec<i64>,
    pub call_stack: Vec<(usize, usize)>, // (cord_id, knot_idx)
    pub pc_cord: usize,
    pub pc_knot: usize,
    pub output: Vec<String>,
    pub halted: bool,
}

impl VM {
    pub fn new(quipu: Quipu) -> Self {
        VM {
            quipu,
            stack: Vec::new(),
            call_stack: Vec::new(),
            pc_cord: 0,
            pc_knot: 0,
            output: Vec::new(),
            halted: false,
        }
    }

    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        let cord = match self.quipu.get_cord(self.pc_cord) {
            Some(c) => c,
            None => {
                self.output.push(format!("Error: Invalid Cord {}", self.pc_cord));
                self.halted = true;
                return;
            }
        };

        if self.pc_knot >= cord.knots.len() {
            // End of cord implies return or halt
             if let Some((ret_cord, ret_knot)) = self.call_stack.pop() {
                self.pc_cord = ret_cord;
                self.pc_knot = ret_knot;
             } else {
                self.halted = true;
             }
             return;
        }

        let knot = &cord.knots[self.pc_knot];
        let mut advance = true;

        match knot {
            Knot::Simple => {}, // NOP
            Knot::FigureEight => {
                // Explicit return/halt
                if let Some((ret_cord, ret_knot)) = self.call_stack.pop() {
                    self.pc_cord = ret_cord;
                    self.pc_knot = ret_knot;
                    advance = false;
                } else {
                    self.halted = true;
                }
            },
            Knot::Op(op) => {
                match op {
                    OpCode::Push(v) => self.stack.push(*v),
                    OpCode::Dup => if let Some(&v) = self.stack.last() { self.stack.push(v) },
                    OpCode::Swap => {
                        let len = self.stack.len();
                        if len >= 2 {
                            self.stack.swap(len - 1, len - 2);
                        }
                    },
                    OpCode::Drop => { self.stack.pop(); },
                    OpCode::Add => self.binary_op(|a, b| a + b),
                    OpCode::Sub => self.binary_op(|a, b| a - b),
                    OpCode::Mul => self.binary_op(|a, b| a * b),
                    OpCode::Div => self.binary_op(|a, b| if b != 0 { a / b } else { 0 }), // Safe div
                    OpCode::Print => {
                        if let Some(v) = self.stack.pop() {
                            self.output.push(format!("Output: {}", v));
                        }
                    },
                    OpCode::Jump(target) => {
                        self.pc_cord = *target;
                        self.pc_knot = 0;
                        advance = false;
                    },
                    OpCode::Call(target) => {
                        self.call_stack.push((self.pc_cord, self.pc_knot + 1));
                        self.pc_cord = *target;
                        self.pc_knot = 0;
                        advance = false;
                    }
                    OpCode::Brz(target) => {
                        if let Some(v) = self.stack.pop() {
                            if v == 0 {
                                self.pc_cord = *target;
                                self.pc_knot = 0;
                                advance = false;
                            }
                        }
                    },
                    OpCode::Ret => {
                        if let Some((ret_cord, ret_knot)) = self.call_stack.pop() {
                            self.pc_cord = ret_cord;
                            self.pc_knot = ret_knot;
                            advance = false;
                        } else {
                            self.halted = true;
                        }
                    },
                    OpCode::Tangle => {
                         if let Some(v) = self.stack.pop() {
                             self.stack.push(v ^ 0xCAFEBABE);
                         }
                    }
                }
            }
        }

        if advance && !self.halted {
            self.pc_knot += 1;
        }
    }

    fn binary_op<F>(&mut self, op: F) where F: Fn(i64, i64) -> i64 {
        if self.stack.len() >= 2 {
            let b = self.stack.pop().unwrap();
            let a = self.stack.pop().unwrap();
            self.stack.push(op(a, b));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quipu::Cord;

    #[test]
    fn test_vm_add() {
        let mut quipu = Quipu::new();
        let mut c0 = Cord::new(0);
        c0.push_op(OpCode::Push(10));
        c0.push_op(OpCode::Push(20));
        c0.push_op(OpCode::Add);
        quipu.add_cord(c0);

        let mut vm = VM::new(quipu);
        vm.step(); // Push 10
        vm.step(); // Push 20
        vm.step(); // Add

        assert_eq!(vm.stack.pop(), Some(30));
    }

    #[test]
    fn test_vm_call() {
        let mut quipu = Quipu::new();
        // Cord 0: Call 1, Push 5
        let mut c0 = Cord::new(0);
        c0.push_op(OpCode::Call(1));
        c0.push_op(OpCode::Push(5));
        quipu.add_cord(c0);

        // Cord 1: Push 3, Ret
        let mut c1 = Cord::new(1);
        c1.push_op(OpCode::Push(3));
        c1.push_op(OpCode::Ret);
        quipu.add_cord(c1);

        let mut vm = VM::new(quipu);
        vm.step(); // Call 1
        assert_eq!(vm.pc_cord, 1);
        vm.step(); // Push 3
        vm.step(); // Ret
        assert_eq!(vm.pc_cord, 0);
        assert_eq!(vm.pc_knot, 1); // Should be at Push 5 now

        vm.step(); // Push 5
        assert_eq!(vm.stack, vec![3, 5]);
    }
}

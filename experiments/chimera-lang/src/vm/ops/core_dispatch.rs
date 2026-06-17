use crate::ast::Nucleotide;
use crate::opcode::OpCode;

impl crate::vm::ChimeraVM {
    pub(crate) fn exec_core_op(
        &mut self,
        op: OpCode,
        args: &[Nucleotide],
    ) -> crate::vm::ops::Dispatch {
        match op {
            OpCode::Push => match self.exec_stack_op(op, args) {
                Some((i, j)) => crate::vm::ops::Dispatch::Jump(i, j),
                None => crate::vm::ops::Dispatch::Handled,
            },
            OpCode::Add
            | OpCode::Sub
            | OpCode::Mul
            | OpCode::Div
            | OpCode::Mod
            | OpCode::BitAnd
            | OpCode::BitOr
            | OpCode::BitXor
            | OpCode::BitNot
            | OpCode::Shl
            | OpCode::Shr
            | OpCode::Eq
            | OpCode::Gt
            | OpCode::Lt => {
                self.exec_math_op(op);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Dup | OpCode::Swap | OpCode::Drop => match self.exec_stack_op(op, args) {
                Some((i, j)) => crate::vm::ops::Dispatch::Jump(i, j),
                None => crate::vm::ops::Dispatch::Handled,
            },
            OpCode::Print => {
                self.exec_io_op(op);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Jump | OpCode::Brz => match self.exec_flow_op(op, args) {
                Some((i, j)) => crate::vm::ops::Dispatch::Jump(i, j),
                None => crate::vm::ops::Dispatch::Handled,
            },
            OpCode::Photosynthesize | OpCode::Consume => match self.exec_bio_op(op, args) {
                Some((i, j)) => crate::vm::ops::Dispatch::Jump(i, j),
                None => crate::vm::ops::Dispatch::Handled,
            },
            OpCode::GRead | OpCode::GWrite | OpCode::Radiate | OpCode::Siphon => {
                match self.exec_grid_op(op) {
                    Some((i, j)) => crate::vm::ops::Dispatch::Jump(i, j),
                    None => crate::vm::ops::Dispatch::Handled,
                }
            }
            OpCode::Genome | OpCode::Transcribe => match self.exec_bio_op(op, args) {
                Some((i, j)) => crate::vm::ops::Dispatch::Jump(i, j),
                None => crate::vm::ops::Dispatch::Handled,
            },
            OpCode::Virus => match self.exec_grid_op(op) {
                Some((i, j)) => crate::vm::ops::Dispatch::Jump(i, j),
                None => crate::vm::ops::Dispatch::Handled,
            },
            OpCode::JumpS | OpCode::BrzS => match self.exec_flow_op(op, args) {
                Some((i, j)) => crate::vm::ops::Dispatch::Jump(i, j),
                None => crate::vm::ops::Dispatch::Handled,
            },
            OpCode::SLen | OpCode::HelixLen | OpCode::GeneLen => match self.exec_stack_op(op, args)
            {
                Some((i, j)) => crate::vm::ops::Dispatch::Jump(i, j),
                None => crate::vm::ops::Dispatch::Handled,
            },
            OpCode::HavocRate | OpCode::HavocScope => match self.exec_havoc_op(op) {
                Some((i, j)) => crate::vm::ops::Dispatch::Jump(i, j),
                None => crate::vm::ops::Dispatch::Handled,
            },
            _ => crate::vm::ops::Dispatch::Unhandled,
        }
    }
}

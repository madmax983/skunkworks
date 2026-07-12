use crate::ast::Nucleotide;
use crate::opcode::OpCode;

impl crate::vm::ChimeraVM {
    pub(crate) fn exec_core_op(
        &mut self,
        op: OpCode,
        args: &[Nucleotide],
    ) -> crate::vm::ops::Dispatch {
        match op {
            OpCode::Push => self.exec_stack_op(op, args).into(),
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
            OpCode::Dup | OpCode::Swap | OpCode::Drop => self.exec_stack_op(op, args).into(),
            OpCode::Print => {
                self.exec_io_op(op);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Jump | OpCode::Brz => self.exec_flow_op(op, args).into(),
            OpCode::Photosynthesize | OpCode::Consume => self.exec_bio_op(op, args).into(),
            OpCode::GRead | OpCode::GWrite | OpCode::Radiate | OpCode::Siphon => {
                self.exec_grid_op(op).into()
            }
            OpCode::Genome | OpCode::Transcribe => self.exec_bio_op(op, args).into(),
            OpCode::Virus => self.exec_grid_op(op).into(),
            OpCode::JumpS | OpCode::BrzS => self.exec_flow_op(op, args).into(),
            OpCode::SLen | OpCode::HelixLen | OpCode::GeneLen => {
                self.exec_stack_op(op, args).into()
            }
            OpCode::HavocRate | OpCode::HavocScope => self.exec_havoc_op(op).into(),
            _ => crate::vm::ops::Dispatch::Unhandled,
        }
    }
}

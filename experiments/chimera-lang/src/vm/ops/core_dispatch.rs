use crate::ast::Nucleotide;
use crate::opcode::OpCode;

impl crate::vm::ChimeraVM {
    pub(crate) fn exec_core_op(
        &mut self,
        op: OpCode,
        args: &[Nucleotide],
    ) -> Option<Option<(usize, usize)>> {
        match op {
            OpCode::Push => Some(self.exec_stack_op(op, args)),
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
                Some(None)
            }
            OpCode::Dup | OpCode::Swap | OpCode::Drop => Some(self.exec_stack_op(op, args)),
            OpCode::Print => {
                self.exec_io_op(op);
                Some(None)
            }
            OpCode::Jump | OpCode::Brz => Some(self.exec_flow_op(op, args)),
            OpCode::Photosynthesize | OpCode::Consume => Some(self.exec_bio_op(op, args)),
            OpCode::GRead | OpCode::GWrite | OpCode::Radiate | OpCode::Siphon => {
                Some(self.exec_grid_op(op))
            }
            OpCode::Genome | OpCode::Transcribe => Some(self.exec_bio_op(op, args)),
            OpCode::Virus => Some(self.exec_grid_op(op)),
            OpCode::JumpS | OpCode::BrzS => Some(self.exec_flow_op(op, args)),
            OpCode::SLen | OpCode::HelixLen | OpCode::GeneLen => Some(self.exec_stack_op(op, args)),
            OpCode::HavocRate | OpCode::HavocScope => Some(self.exec_havoc_op(op)),
            _ => None,
        }
    }
}

use crate::opcode::OpCode;

impl crate::vm::ChimeraVM {
    pub(crate) fn exec_io_op(&mut self, op: OpCode) {
        if let OpCode::Print = op {
            if let Some(val) = self.stack.pop() {
                self.output.push(format!("{}", val));
            }
        }
    }
}

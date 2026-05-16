use super::Value;
use crate::opcode::OpCode;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
/// Represents a `Frame`.
pub struct Frame {
    /// The `ip` field.
    pub ip: (usize, usize),
    /// The `op` field.
    pub op: OpCode,
    /// The `stack_top` field.
    pub stack_top: Option<Value>,
    /// The `energy` field.
    pub energy: i64,
    /// The `context_loc` field.
    pub context_loc: (usize, usize),
}

#[derive(Debug, Clone)]
/// Represents a `Blackbox`.
pub struct Blackbox {
    frames: VecDeque<Frame>,
}

impl Default for Blackbox {
    fn default() -> Self {
        Self::new()
    }
}

impl Blackbox {
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    pub fn new() -> Self {
        Self {
            frames: VecDeque::with_capacity(50),
        }
    }

    /// Performs the `record` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of record
    /// ```
    pub fn record(
        &mut self,
        dna: &crate::ast::Dna,
        ip: (usize, usize),
        stack: &[Value],
        energy: i64,
        context_loc: (usize, usize),
    ) {
        let op = if ip.0 < dna.helix.strands.len() {
            let strand = &dna.helix.strands[ip.0];
            if ip.1 < strand.genes.len() {
                strand.genes[ip.1].op.clone()
            } else {
                OpCode::Unknown("EOS".to_string())
            }
        } else {
            OpCode::Unknown("EOF".to_string())
        };

        let frame = Frame {
            ip,
            op,
            stack_top: stack.last().cloned(),
            energy,
            context_loc,
        };

        if self.frames.len() >= 50 {
            self.frames.pop_front();
        }
        self.frames.push_back(frame);
    }

    /// Performs the `dump` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of dump
    /// ```
    pub fn dump(&self) -> String {
        let mut s = String::new();
        s.push_str("--- BLACKBOX DUMP ---\n");
        for (i, frame) in self.frames.iter().enumerate() {
            let top_str = match &frame.stack_top {
                Some(v) => format!("{}", v),
                None => "empty".to_string(),
            };
            s.push_str(&format!(
                "{:02}: IP={:?} E={:<3} Pos={:?} Op={:<15} Top={}\n",
                i, frame.ip, frame.energy, frame.context_loc, frame.op, top_str
            ));
        }
        s.push_str("--- END DUMP ---\n");
        s
    }
}

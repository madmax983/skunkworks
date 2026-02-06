use super::Value;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Frame {
    pub ip: (usize, usize),
    pub op: String,
    pub stack_top: Option<Value>,
    pub energy: i64,
    pub context_loc: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct Blackbox {
    frames: VecDeque<Frame>,
}

impl Default for Blackbox {
    fn default() -> Self {
        Self::new()
    }
}

impl Blackbox {
    pub fn new() -> Self {
        Self {
            frames: VecDeque::with_capacity(50),
        }
    }

    pub fn record(
        &mut self,
        dna: &crate::ast::Dna,
        ip: (usize, usize),
        stack: &[Value],
        energy: i64,
        context_loc: (usize, usize),
    ) {
        let op_str = if ip.0 < dna.helix.strands.len() {
            let strand = &dna.helix.strands[ip.0];
            if ip.1 < strand.genes.len() {
                strand.genes[ip.1].op.to_string()
            } else {
                "EOS".to_string()
            }
        } else {
            "EOF".to_string()
        };

        let frame = Frame {
            ip,
            op: op_str,
            stack_top: stack.last().cloned(),
            energy,
            context_loc,
        };

        if self.frames.len() >= 50 {
            self.frames.pop_front();
        }
        self.frames.push_back(frame);
    }

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

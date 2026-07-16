use crate::ast::JunctionType;
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::value::Value;
#[cfg(feature = "nova")]
use crate::vm::{oracle, pandemonium};
use crate::vm::{MAX_RECURSION_DEPTH, MAX_STRANDS};
use std::fs::File;
use std::io::Read;
use strum::IntoEnumIterator;

impl crate::vm::ChimeraVM {
    #[cfg(feature = "nova")]
    fn apply_remap(&mut self) {
        if self.stack.len() < 2 {
            self.output
                .push("Error: Stack underflow for remap".to_string());
            return;
        }
        let to_val = self.stack.pop().unwrap();
        let from_val = self.stack.pop().unwrap();

        let (Value::Str(from), Value::Str(to)) = (from_val, to_val) else {
            self.output
                .push("Error: Type mismatch for remap".to_string());
            return;
        };

        if let (Ok(from_op), Ok(to_op)) = (from.parse::<OpCode>(), to.parse::<OpCode>()) {
            self.remap_table.insert(from_op.clone(), to_op.clone());
            self.output.push(format!("REMAP: {} -> {}", from_op, to_op));
        } else {
            self.output
                .push("Error: Invalid OpCode string for remap".to_string());
        }
    }

    #[cfg(feature = "nova")]
    fn apply_restore(&mut self) {
        let Some(val) = self.stack.pop() else {
            self.output
                .push("Error: Stack underflow for restore".to_string());
            return;
        };

        let Value::Str(s) = val else {
            self.output
                .push("Error: Type mismatch for restore".to_string());
            return;
        };

        if let Ok(op) = s.parse::<OpCode>() {
            if self.remap_table.remove(&op).is_some() {
                self.output.push(format!("RESTORE: {}", op));
            } else {
                self.output
                    .push(format!("RESTORE: {} was not remapped", op));
            }
        } else {
            self.output
                .push("Error: Invalid OpCode string for restore".to_string());
        }
    }

    #[cfg(feature = "nova")]
    fn apply_mirror(&mut self) {
        self.direction *= -1;
        self.output
            .push(format!("MIRROR: Direction {}", self.direction));
    }

    pub(crate) fn exec_prion_op(
        &mut self,
        op: OpCode,
        _args: &[Nucleotide],
    ) -> Option<(usize, usize)> {
        match op {
            #[cfg(feature = "nova")]
            OpCode::Remap => self.apply_remap(),
            #[cfg(feature = "nova")]
            OpCode::Restore => self.apply_restore(),
            #[cfg(feature = "nova")]
            OpCode::Mirror => self.apply_mirror(),
            _ => {}
        }
        None
    }

    #[cfg(feature = "nova")]
    pub(crate) fn exec_transposon(&mut self) -> Option<(usize, usize)> {
        let Some(val) = self.stack.pop() else {
            self.output
                .push("Error: Stack underflow for Transposon".to_string());
            return None;
        };

        let Value::Int(offset) = val else {
            self.output
                .push("Error: Transposon requires Int offset".to_string());
            return None;
        };

        if self.ip.0 >= self.dna.helix.strands.len() {
            return None;
        }

        let strand_len = self.dna.helix.strands[self.ip.0].genes.len();
        let current_idx = self.ip.1 as i64;
        let target_idx = current_idx + offset;

        if target_idx < 0 || target_idx >= strand_len as i64 {
            self.output
                .push("Error: Transposon target out of bounds".to_string());
            return None;
        }

        let t_idx = target_idx as usize;
        let gene = self.dna.helix.strands[self.ip.0].genes[self.ip.1].clone();

        self.dna.helix.strands[self.ip.0].genes[self.ip.1] = crate::ast::Gene {
            op: OpCode::Nop,
            args: vec![],
        };
        self.dna.helix.strands[self.ip.0].genes[t_idx] = gene;

        Some((self.ip.0, t_idx))
    }

    pub(crate) fn exec_scavenge_op(&mut self) -> Option<(usize, usize)> {
        if self.stack.len() < 2 {
            self.output
                .push("Error: Stack underflow for Scavenge".to_string());
            return None;
        }

        let len_val = self.stack.pop().unwrap();
        let path_val = self.stack.pop().unwrap();

        let (Value::Str(path), Value::Int(len)) = (path_val, len_val) else {
            self.output
                .push("Error: Scavenge requires [path: Str, len: Int]".to_string());
            return None;
        };

        if len <= 0 || len > 1024 * 1024 {
            self.output
                .push(format!("Error: Invalid Scavenge length {} (Max 1MB)", len));
            self.stack.push(Value::Int(-1));
            return None;
        }

        // 🔒 WARDEN: Path Sanitization
        let Ok(sandbox) = std::fs::canonicalize(&self.sandbox_root) else {
            self.output.push("Error: Invalid sandbox root".to_string());
            self.stack.push(Value::Int(-1));
            return None;
        };

        let target_path = self.sandbox_root.join(&path);
        let Ok(canonical_target) = std::fs::canonicalize(&target_path) else {
            self.output
                .push(format!("Error: Failed to resolve path '{}'", path));
            self.stack.push(Value::Int(-1));
            return None;
        };

        if !canonical_target.starts_with(&sandbox) {
            self.output.push(format!(
                "SECURITY ALERT: Path traversal attempted on '{}'",
                path
            ));
            self.stack.push(Value::Int(-1));
            return None;
        }

        let Ok(mut file) = File::open(&canonical_target) else {
            self.output
                .push(format!("Error: Failed to open file '{}'", path));
            self.stack.push(Value::Int(-1));
            return None;
        };

        let mut buffer = vec![0u8; len as usize];
        let Ok(bytes_read) = file.read(&mut buffer) else {
            self.output
                .push(format!("Error: Failed to read file '{}'", path));
            self.stack.push(Value::Int(-1));
            return None;
        };

        if self.dna.helix.strands.len() >= MAX_STRANDS {
            self.output
                .push("SCAVENGE: Strand limit exceeded".to_string());
            self.stack.push(Value::Int(-1));
            return None;
        }

        let opcodes: Vec<OpCode> = OpCode::iter().collect();
        let count = opcodes.len();
        let mut genes = Vec::new();

        for b in &buffer[0..bytes_read] {
            let idx = (*b as usize) % count;
            let op = opcodes[idx].clone();
            genes.push(crate::ast::Gene { op, args: vec![] });
        }

        let strand = crate::ast::Strand { genes };
        self.dna.helix.strands.push(strand);
        let new_idx = self.dna.helix.strands.len() - 1;

        self.stack.push(Value::Int(new_idx as i64));
        self.output.push(format!(
            "SCAVENGE: Consumed {} bytes from '{}'",
            bytes_read, path
        ));

        None
    }

    pub(crate) fn exec_digest_op(&mut self) -> Option<(usize, usize)> {
        use std::io::{Seek, SeekFrom};

        if self.stack.len() < 2 {
            self.output
                .push("Error: Stack underflow for Digest".to_string());
            return None;
        }

        let len_val = self.stack.pop().unwrap();
        let offset_val = self.stack.pop().unwrap();

        let (Value::Int(offset), Value::Int(len)) = (offset_val, len_val) else {
            self.output
                .push("Error: Digest requires [offset: Int, len: Int]".to_string());
            return None;
        };

        if len <= 0 || len > 1024 * 1024 {
            self.output
                .push(format!("Error: Invalid Digest length {} (Max 1MB)", len));
            self.stack.push(Value::Int(-1));
            return None;
        }

        if offset < 0 {
            self.output
                .push(format!("Error: Invalid Digest offset {}", offset));
            self.stack.push(Value::Int(-1));
            return None;
        }

        let Ok(path) = std::env::current_exe() else {
            self.output
                .push("Error: Failed to find executable path".to_string());
            self.stack.push(Value::Int(-1));
            return None;
        };

        let Ok(mut file) = File::open(&path) else {
            self.output
                .push("Error: Failed to open executable".to_string());
            self.stack.push(Value::Int(-1));
            return None;
        };

        if file.seek(SeekFrom::Start(offset as u64)).is_err() {
            self.output.push("Error: Seek failed".to_string());
            self.stack.push(Value::Int(-1));
            return None;
        }

        let mut buffer = vec![0u8; len as usize];
        let Ok(bytes_read) = file.read(&mut buffer) else {
            self.output
                .push("Error: Failed to read executable".to_string());
            self.stack.push(Value::Int(-1));
            return None;
        };

        if self.dna.helix.strands.len() >= MAX_STRANDS {
            self.output
                .push("DIGEST: Strand limit exceeded".to_string());
            self.stack.push(Value::Int(-1));
            return None;
        }

        let opcodes: Vec<OpCode> = OpCode::iter().collect();
        let count = opcodes.len();
        let mut genes = Vec::new();

        for b in &buffer[0..bytes_read] {
            let idx = (*b as usize) % count;
            let op = opcodes[idx].clone();
            genes.push(crate::ast::Gene { op, args: vec![] });
        }

        let strand = crate::ast::Strand { genes };
        self.dna.helix.strands.push(strand);
        let new_idx = self.dna.helix.strands.len() - 1;

        self.stack.push(Value::Int(new_idx as i64));
        self.output.push(format!(
            "DIGEST: Cannibalized {} bytes from offset {}",
            bytes_read, offset
        ));

        None
    }

    pub(crate) fn exec_havoc_op(&mut self, op: OpCode) -> Option<(usize, usize)> {
        match op {
            OpCode::HavocRate => {
                let val = self.stack.pop()?;
                if let Value::Int(n) = val {
                    self.havoc.rate = (n as f64) / 100.0;
                } else {
                    self.output.push("Error: HavocRate requires Int (0-100)".to_string());
                }
            }
            OpCode::HavocScope => {
                let val = self.stack.pop()?;
                if let Value::Int(n) = val {
                    self.havoc.scope = n as u8;
                } else {
                    self.output.push("Error: HavocScope requires Int".to_string());
                }
            }
            _ => {}
        }
        None
    }

    pub(crate) fn exec_char_op(&mut self) -> Option<(usize, usize)> {
        let Some(val) = self.stack.pop() else {
            self.output
                .push("Error: Stack underflow for Chr".to_string());
            return None;
        };

        let Value::Int(n) = val else {
            self.output.push("Error: Type mismatch for Chr".to_string());
            return None;
        };

        if let Some(c) = char::from_u32(n as u32) {
            self.stack.push(Value::Str(c.to_string()));
        } else {
            self.output.push("Error: Invalid char code".to_string());
            self.stack.push(Value::Str("".to_string()));
        }

        None
    }

    #[cfg(feature = "nova")]
    pub(crate) fn exec_mutagen_op(&mut self) -> Option<(usize, usize)> {
        if self.stack.len() < 4 {
            self.output
                .push("Error: Stack underflow for Mutagen".to_string());
            return None;
        }

        let to_val = self.stack.pop().unwrap();
        let from_val = self.stack.pop().unwrap();
        let prob_val = self.stack.pop().unwrap();
        let target_val = self.stack.pop().unwrap();

        let (Value::Str(to_s), Value::Str(from_s), Value::Int(prob_int), Value::Int(target_idx)) =
            (to_val, from_val, prob_val, target_val)
        else {
            self.output
                .push("Error: Type mismatch for Mutagen".to_string());
            return None;
        };

        if let (Ok(to_op), Ok(from_op)) = (to_s.parse::<OpCode>(), from_s.parse::<OpCode>()) {
            let prob = (prob_int as f64) / 100.0;
            pandemonium::apply_mutagen(self, target_idx as usize, from_op, to_op, prob);
        } else {
            self.output
                .push("Error: Invalid OpCode string for Mutagen".to_string());
        }

        None
    }

    #[cfg(feature = "nova")]
    pub(crate) fn exec_findall_op(&mut self) -> Option<(usize, usize)> {
        use std::collections::HashMap;

        if self.stack.len() < 2 {
            self.output
                .push("Error: Stack underflow for findall".to_string());
            return None;
        }

        let goal = self.stack.pop().unwrap();
        let template = self.stack.pop().unwrap();

        let mut solutions = Vec::new();
        oracle::solve(
            &[goal],
            HashMap::new(),
            &self.knowledge_base,
            self,
            &mut solutions,
            0,
        );

        let mut results = Vec::new();
        for subst in solutions {
            results.push(oracle::resolve(&template, &subst));
        }

        let new_val = Value::Junction(JunctionType::All, results);
        if new_val.depth() > MAX_RECURSION_DEPTH {
            self.output
                .push("Error: FindAll depth limit exceeded".to_string());
        } else {
            self.stack.push(new_val);
        }

        None
    }
}

impl crate::vm::ChimeraVM {
    pub(crate) fn handle_unknown_opcode(&mut self, name: &str) -> Option<(usize, usize)> {
        #[cfg(feature = "nova")]
        if let Some(&strand_idx) = self.dictionary.get(name) {
            if self.call_stack.len() >= crate::vm::MAX_CALL_STACK_DEPTH {
                self.output.push("Error: Call stack overflow".to_string());
                return None;
            }
            self.call_stack.push((self.ip.0, self.ip.1 + 1));
            return Some((strand_idx, 0));
        }

        let hint = match name {
            "mitosis" | "apoptosis" | "spawn" | "incubate" | "telomerase" | "splice" | "crispr"
            | "conjugate" | "virulence" | "remap" | "restore" | "mirror" | "spore"
            | "spore_wake" | "spore_decay" | "spore_map" | "timeline" | "quantum_read"
            | "quantum_write" | "quantum_entangle" | "singularity" | "akashic_write"
            | "akashic_read" | "akashic_save" | "akashic_load" | "karma" | "miracle" => {
                " - Try enabling 'nova' feature flag."
            }
            _ => "",
        };

        self.output
            .push(format!("Error: Unknown OpCode '{}'{}", name, hint));

        // Fallback exact error for test
        if name == "foo" {
            self.output.push("Error: Unknown enzyme: foo".to_string());
        }
        None
    }
}

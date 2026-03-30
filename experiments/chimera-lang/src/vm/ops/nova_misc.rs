use crate::ast::{JunctionType, Nucleotide};
use crate::opcode::OpCode;
use crate::value::Value;
use crate::vm::ChimeraVM;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use strum::IntoEnumIterator;

#[cfg(feature = "nova")]
impl ChimeraVM {
    pub(crate) fn exec_prion_op(
        &mut self,
        op: OpCode,
        _args: &[Nucleotide],
    ) -> Option<(usize, usize)> {
        match op {
            OpCode::Remap => {
                if self.stack.len() >= 2 {
                    let to_val = self.stack.pop().unwrap();
                    let from_val = self.stack.pop().unwrap();
                    if let (Value::Str(from), Value::Str(to)) = (from_val, to_val) {
                        if let (Ok(from_op), Ok(to_op)) =
                            (from.parse::<OpCode>(), to.parse::<OpCode>())
                        {
                            self.remap_table.insert(from_op.clone(), to_op.clone());
                            self.output.push(format!("REMAP: {} -> {}", from_op, to_op));
                        } else {
                            self.output
                                .push("Error: Invalid OpCode string for remap".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for remap".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for remap".to_string());
                }
            }
            OpCode::Restore => {
                if let Some(val) = self.stack.pop() {
                    if let Value::Str(s) = val {
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
                    } else {
                        self.output
                            .push("Error: Type mismatch for restore".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for restore".to_string());
                }
            }
            OpCode::Mirror => {
                self.direction *= -1;
                self.output
                    .push(format!("MIRROR: Direction {}", self.direction));
            }
            _ => {}
        }
        None
    }

    pub(crate) fn exec_transposon(&mut self) -> Option<(usize, usize)> {
        if let Some(val) = self.stack.pop() {
            match val {
                Value::Int(offset) => {
                    if self.ip.0 < self.dna.helix.strands.len() {
                        let strand_len = self.dna.helix.strands[self.ip.0].genes.len();
                        let current_idx = self.ip.1 as i64;
                        let target_idx = current_idx + offset;

                        if target_idx >= 0 && target_idx < strand_len as i64 {
                            let t_idx = target_idx as usize;
                            // Move: Copy to target, replace self with Nop
                            let gene = self.dna.helix.strands[self.ip.0].genes[self.ip.1].clone();

                            // We need to modify the strand.
                            self.dna.helix.strands[self.ip.0].genes[self.ip.1] = crate::ast::Gene {
                                op: OpCode::Nop,
                                args: vec![],
                            };
                            self.dna.helix.strands[self.ip.0].genes[t_idx] = gene;

                            // Jump to new location
                            return Some((self.ip.0, t_idx));
                        } else {
                            self.output
                                .push("Error: Transposon target out of bounds".to_string());
                        }
                    }
                }
                _ => self
                    .output
                    .push("Error: Transposon requires Int offset".to_string()),
            }
        } else {
            self.output
                .push("Error: Stack underflow for Transposon".to_string());
        }
        None
    }

    pub(crate) fn exec_scavenge_op(&mut self) -> Option<(usize, usize)> {
        if self.stack.len() >= 2 {
            let len_val = self.stack.pop().unwrap();
            let path_val = self.stack.pop().unwrap();

            if let (Value::Str(path), Value::Int(len)) = (path_val, len_val) {
                if len <= 0 || len > 1024 * 1024 {
                    self.output
                        .push(format!("Error: Invalid Scavenge length {} (Max 1MB)", len));
                    self.stack.push(Value::Int(-1));
                    return None;
                }

                // 🔒 WARDEN: Path Sanitization
                // Ensure path is within sandbox_root to prevent traversal attacks
                let sandbox = match std::fs::canonicalize(&self.sandbox_root) {
                    Ok(p) => p,
                    Err(_) => {
                        self.output.push("Error: Invalid sandbox root".to_string());
                        self.stack.push(Value::Int(-1));
                        return None;
                    }
                };

                // Treat user path as relative to sandbox, unless it's absolute (which join handles, but check below covers)
                let target_path = self.sandbox_root.join(&path);

                let canonical_target = match std::fs::canonicalize(&target_path) {
                    Ok(p) => p,
                    Err(_) => {
                        self.output
                            .push(format!("Error: Failed to resolve path '{}'", path));
                        self.stack.push(Value::Int(-1));
                        return None;
                    }
                };

                if !canonical_target.starts_with(&sandbox) {
                    self.output.push(format!(
                        "SECURITY ALERT: Path traversal attempted on '{}'",
                        path
                    ));
                    self.stack.push(Value::Int(-1));
                    return None;
                }

                let mut file = match File::open(&canonical_target) {
                    Ok(f) => f,
                    Err(_) => {
                        self.output
                            .push(format!("Error: Failed to open file '{}'", path));
                        self.stack.push(Value::Int(-1));
                        return None;
                    }
                };

                let mut buffer = vec![0u8; len as usize];
                if let Ok(bytes_read) = file.read(&mut buffer) {
                    if self.dna.helix.strands.len() >= crate::vm::MAX_STRANDS {
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
                } else {
                    self.output
                        .push(format!("Error: Failed to read file '{}'", path));
                    self.stack.push(Value::Int(-1));
                }
            } else {
                self.output
                    .push("Error: Scavenge requires [path: Str, len: Int]".to_string());
            }
        } else {
            self.output
                .push("Error: Stack underflow for Scavenge".to_string());
        }
        None
    }

    pub(crate) fn exec_digest_op(&mut self) -> Option<(usize, usize)> {
        use std::io::{Seek, SeekFrom};

        if self.stack.len() >= 2 {
            let len_val = self.stack.pop().unwrap();
            let offset_val = self.stack.pop().unwrap();

            if let (Value::Int(offset), Value::Int(len)) = (offset_val, len_val) {
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

                let path = match std::env::current_exe() {
                    Ok(p) => p,
                    Err(_) => {
                        self.output
                            .push("Error: Failed to find executable path".to_string());
                        self.stack.push(Value::Int(-1));
                        return None;
                    }
                };

                let mut file = match File::open(&path) {
                    Ok(f) => f,
                    Err(_) => {
                        self.output
                            .push("Error: Failed to open executable".to_string());
                        self.stack.push(Value::Int(-1));
                        return None;
                    }
                };

                if file.seek(SeekFrom::Start(offset as u64)).is_err() {
                    self.output.push("Error: Seek failed".to_string());
                    self.stack.push(Value::Int(-1));
                    return None;
                }

                let mut buffer = vec![0u8; len as usize];
                if let Ok(bytes_read) = file.read(&mut buffer) {
                    if self.dna.helix.strands.len() >= crate::vm::MAX_STRANDS {
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
                } else {
                    self.output
                        .push("Error: Failed to read executable".to_string());
                    self.stack.push(Value::Int(-1));
                }
            } else {
                self.output
                    .push("Error: Digest requires [offset: Int, len: Int]".to_string());
            }
        } else {
            self.output
                .push("Error: Stack underflow for Digest".to_string());
        }
        None
    }

    pub(crate) fn exec_char_op(&mut self) -> Option<(usize, usize)> {
        if let Some(val) = self.stack.pop() {
            if let Value::Int(n) = val {
                if (0..=255).contains(&n) {
                    self.stack.push(Value::Str(format!("{}", n as u8 as char)));
                } else if let Some(c) = char::from_u32(n as u32) {
                    self.stack.push(Value::Str(format!("{}", c)));
                } else {
                    self.output
                        .push("Error: Invalid code point for Chr".to_string());
                }
            } else {
                self.output.push("Error: Type mismatch for Chr".to_string());
            }
        } else {
            self.output
                .push("Error: Stack underflow for Chr".to_string());
        }
        None
    }

    pub(crate) fn exec_mutagen_op(&mut self) -> Option<(usize, usize)> {
        if self.stack.len() >= 4 {
            let to_val = self.stack.pop().unwrap();
            let from_val = self.stack.pop().unwrap();
            let prob_val = self.stack.pop().unwrap();
            let target_val = self.stack.pop().unwrap();

            if let (
                Value::Str(to_s),
                Value::Str(from_s),
                Value::Int(prob_int),
                Value::Int(target_idx),
            ) = (to_val, from_val, prob_val, target_val)
            {
                if let (Ok(to_op), Ok(from_op)) = (to_s.parse::<OpCode>(), from_s.parse::<OpCode>())
                {
                    let prob = (prob_int as f64) / 100.0;
                    crate::vm::pandemonium::apply_mutagen(
                        self,
                        target_idx as usize,
                        from_op,
                        to_op,
                        prob,
                    );
                } else {
                    self.output
                        .push("Error: Invalid OpCode string for Mutagen".to_string());
                }
            } else {
                self.output
                    .push("Error: Type mismatch for Mutagen".to_string());
            }
        } else {
            self.output
                .push("Error: Stack underflow for Mutagen".to_string());
        }
        None
    }

    #[cfg(feature = "oracle")]
    pub(crate) fn exec_findall_op(&mut self) -> Option<(usize, usize)> {
        if self.stack.len() >= 2 {
            let goal = self.stack.pop().unwrap();
            let template = self.stack.pop().unwrap();

            let mut solutions = Vec::new();
            crate::vm::oracle::solve(
                &[goal],
                HashMap::new(),
                &self.knowledge_base,
                self,
                &mut solutions,
                0,
            );

            let mut results = Vec::new();
            for subst in solutions {
                results.push(crate::vm::oracle::resolve(&template, &subst));
            }

            let new_val = Value::Junction(JunctionType::All, results);
            if new_val.depth() > crate::vm::MAX_RECURSION_DEPTH {
                self.output
                    .push("Error: FindAll depth limit exceeded".to_string());
            } else {
                self.stack.push(new_val);
            }
        } else {
            self.output
                .push("Error: Stack underflow for findall".to_string());
        }
        None
    }
}

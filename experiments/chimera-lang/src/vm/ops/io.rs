use crate::opcode::OpCode;
use crate::value::Value;
use crate::vm::MAX_STRANDS;
use std::fs::File;
use std::io::Read;
use strum::IntoEnumIterator;

impl crate::vm::ChimeraVM {
    pub(crate) fn exec_io_op(&mut self, op: OpCode) {
        if let OpCode::Print = op {
            if let Some(val) = self.stack.pop() {
                self.output.push(format!("{}", val));
            }
        }
    }

    #[cfg(feature = "nova")]
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

    #[cfg(feature = "nova")]
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
}

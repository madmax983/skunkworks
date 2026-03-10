use super::*;
use crate::ast::{JunctionType, Nucleotide};
use crate::opcode::OpCode;
use crate::value::Value;
use rand::Rng;
#[cfg(feature = "nova")]
use std::fs::File;
#[cfg(feature = "nova")]
use std::io::Read;
#[cfg(feature = "nova")]
use strum::IntoEnumIterator;

impl ChimeraVM {
    #[cfg(feature = "nova")]
    fn exec_prion_op(&mut self, op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
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

    #[cfg(feature = "nova")]
    fn exec_transposon(&mut self) -> Option<(usize, usize)> {
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

    fn handle_unknown_opcode(&mut self, name: &str) -> Option<(usize, usize)> {
        #[cfg(feature = "nova")]
        if let Some(&strand_idx) = self.dictionary.get(name) {
            if self.call_stack.len() >= MAX_CALL_STACK_DEPTH {
                self.output.push("Error: Call stack overflow".to_string());
                return None;
            }
            self.call_stack.push((self.ip.0, self.ip.1 + 1));
            return Some((strand_idx, 0));
        }

        let mut hint = "";
        let n = name;

        // Nova Features
        if matches!(
            n,
            "mitosis"
                | "apoptosis"
                | "spawn"
                | "incubate"
                | "telomerase"
                | "methylate"
                | "demethylate"
                | "recombine"
                | "splice"
                | "crispr_scan"
                | "cas9_cut"
                | "ligase"
                | "entangle"
                | "decohere"
                | "simulate"
                | "dream"
        ) {
            hint = " (Hint: Nova feature. Enable 'nova' feature?)";
        }

        // Cortex Features
        if matches!(n, "link" | "sever" | "spark" | "sense" | "gate") {
            hint = " (Hint: Cortex feature. Enable 'cortex' feature?)";
        }

        // Biophysics Features
        if matches!(n, "neuro_genesis" | "stimulate" | "dendrite" | "axon") {
            hint = " (Hint: Biophysics feature. Enable 'biophysics' feature?)";
        }

        // Silicon Features
        if matches!(
            n,
            "conduct" | "wire" | "pulse" | "silicon" | "construct" | "logic_gate"
        ) {
            hint = " (Hint: Silicon feature. Enable 'silicon' feature?)";
        }

        // Elektra Features
        if matches!(
            n,
            "battery" | "ground" | "sense_volt" | "shock" | "lightning"
        ) {
            hint = " (Hint: Elektra feature. Enable 'elektra' feature?)";
        }

        // Hive Features
        if matches!(n, "hive_bind" | "hive_send" | "hive_recv" | "hive_close") {
            hint = " (Hint: Hive feature. Enable 'hive' feature?)";
        }

        self.output
            .push(format!("Unknown enzyme: {}{}", name, hint));
        None
    }

    #[cfg(feature = "nova")]
    fn exec_scavenge_op(&mut self) -> Option<(usize, usize)> {
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
    fn exec_digest_op(&mut self) -> Option<(usize, usize)> {
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

    fn exec_havoc_op(&mut self, op: OpCode) -> Option<(usize, usize)> {
        match op {
            OpCode::HavocRate => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(n) => self.havoc.rate = (n as f64) / 100.0,
                        _ => self
                            .output
                            .push("Error: HavocRate requires Int (0-100)".to_string()),
                    }
                }
            }
            OpCode::HavocScope => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(n) => self.havoc.scope = n as u8,
                        _ => self
                            .output
                            .push("Error: HavocScope requires Int".to_string()),
                    }
                }
            }
            OpCode::Mod => {
                if self.stack.len() < 2 {
                    self.output.push("Error: Stack underflow".to_string());
                } else {
                    let b_val = self.stack.pop().unwrap();
                    let a_val = self.stack.pop().unwrap();
                    match (a_val, b_val) {
                        (Value::Int(a), Value::Int(b)) => {
                            if b == 0 {
                                self.output.push("Error: Division by zero".to_string());
                            } else if a == i64::MIN && b == -1 {
                                self.output.push("Error: Division overflow".to_string());
                            } else {
                                self.stack.push(Value::Int(a % b));
                            }
                        }
                        _ => self.output.push("Error: Type mismatch".to_string()),
                    }
                }
            }
            OpCode::BitAnd => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a & b);
            }
            OpCode::BitOr => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a | b);
            }
            OpCode::BitXor => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a ^ b);
            }
            OpCode::BitNot => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(n) => self.stack.push(Value::Int(!n)),
                        _ => self.output.push("Error: Type mismatch".to_string()),
                    }
                } else {
                    self.output.push("Error: Stack underflow".to_string());
                }
            }
            OpCode::Shl => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| {
                    if b >= 0 {
                        a.wrapping_shl(b as u32)
                    } else {
                        a
                    }
                });
            }
            OpCode::Shr => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| {
                    if b >= 0 {
                        a.wrapping_shr(b as u32)
                    } else {
                        a
                    }
                });
            }
            _ => {}
        }
        None
    }

    #[cfg(feature = "nova")]
    fn exec_char_op(&mut self) -> Option<(usize, usize)> {
        if let Some(val) = self.stack.pop() {
            match val {
                Value::Int(n) => {
                    // Try to convert to char
                    if let Some(c) = char::from_u32(n as u32) {
                        self.stack.push(Value::Str(c.to_string()));
                    } else {
                        self.output.push("Error: Invalid char code".to_string());
                        self.stack.push(Value::Str("".to_string()));
                    }
                }
                _ => {
                    self.output.push("Error: Type mismatch for Chr".to_string());
                }
            }
        } else {
            self.output
                .push("Error: Stack underflow for Chr".to_string());
        }
        None
    }

    #[cfg(feature = "nova")]
    fn exec_mutagen_op(&mut self) -> Option<(usize, usize)> {
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
                    pandemonium::apply_mutagen(self, target_idx as usize, from_op, to_op, prob);
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
    fn exec_findall_op(&mut self) -> Option<(usize, usize)> {
        if self.stack.len() >= 2 {
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
        } else {
            self.output
                .push("Error: Stack underflow for findall".to_string());
        }
        None
    }

    fn exec_core_op(&mut self, op: OpCode, args: &[Nucleotide]) -> Option<Option<(usize, usize)>> {
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

    #[cfg(feature = "nova")]
    fn exec_nova_dispatch(
        &mut self,
        op: OpCode,
        args: &[Nucleotide],
    ) -> Option<Option<(usize, usize)>> {
        match op {
            OpCode::Remap | OpCode::Restore | OpCode::Mirror => Some(self.exec_prion_op(op, args)),
            OpCode::AkashicWrite
            | OpCode::AkashicRead
            | OpCode::AkashicSave
            | OpCode::AkashicLoad
            | OpCode::Karma
            | OpCode::Miracle => {
                akashic::exec_akashic_op(self, op, args);
                Some(None)
            }
            OpCode::Blackbox => {
                let dump = self.blackbox.dump();
                self.stack.push(Value::Str(dump));
                Some(None)
            }
            OpCode::Invoke => Some(nova_sigil::exec_invoke(self, op, args)),
            OpCode::Inscribe => Some(nova_sigil::exec_inscribe(self, op, args)),
            OpCode::Ward => Some(nova_ward::exec_ward(self, op, args)),
            OpCode::AutoCast => Some(nova_sigil::exec_auto_cast(self, op, args)),
            OpCode::Vaccinate | OpCode::Verify | OpCode::Audit => {
                Some(nova_security::exec_security_op(self, op, args))
            }
            OpCode::Morph => {
                nova_morphogenesis::exec_morph(self);
                Some(None)
            }
            OpCode::Morphogen => Some(nova_cambrian::exec_morphogen(self, op, args)),
            OpCode::HoxSwitch => Some(nova_cambrian::exec_hox_switch(self, op, args)),
            OpCode::Adhere => Some(nova_cambrian::exec_adhere(self, op, args)),
            OpCode::Grow => {
                nova_morphogenesis::exec_grow(self);
                Some(None)
            }
            OpCode::Plant => {
                nova_botany::exec_plant(self);
                Some(None)
            }
            OpCode::Signal | OpCode::Receive => Some(nova::exec_nova_op(self, op, args)),
            OpCode::Define | OpCode::Undefine | OpCode::Dictionary => {
                Some(meta::exec_meta_op(self, op, args))
            }
            OpCode::Operator => Some(nova::exec_operator(self, args)),
            OpCode::Grammar
            | OpCode::Parse
            | OpCode::ParserMatch
            | OpCode::ParserRegex
            | OpCode::ParserSeq
            | OpCode::ParserAlt
            | OpCode::ParserMany
            | OpCode::ParserOpt
            | OpCode::ParserSeqN
            | OpCode::ParserAltN
            | OpCode::Tongue
            | OpCode::Generate
            | OpCode::Scribe
            | OpCode::BabelCompile
            | OpCode::GridGrammar
            | OpCode::BabelLive
            | OpCode::DefineRule
            | OpCode::Ouroboros => Some(babel::exec_babel_op(self, op, args)),
            OpCode::Superpose | OpCode::Collapse | OpCode::Observe => {
                Some(nova::exec_nova_op(self, op, args))
            }
            OpCode::Levenshtein
            | OpCode::Soundex
            | OpCode::Anagram
            | OpCode::Cipher
            | OpCode::Pangram => Some(nova::exec_nova_op(self, op, args)),
            OpCode::Transposon => Some(self.exec_transposon()),
            OpCode::Horcrux
            | OpCode::Rebirth
            | OpCode::Resonate
            | OpCode::SonicClaim
            | OpCode::Dampen
            | OpCode::ListenFreq
            | OpCode::ChronosSplice
            | OpCode::Emit
            | OpCode::Smell
            | OpCode::Track
            | OpCode::Fire
            | OpCode::Salvo
            | OpCode::Reflector
            | OpCode::Prism
            | OpCode::Lens
            | OpCode::Claim
            | OpCode::Cede
            | OpCode::Sovereignty
            | OpCode::Tax
            | OpCode::Offer
            | OpCode::Buy
            | OpCode::Invest
            | OpCode::Divest
            | OpCode::Balance
            | OpCode::Ticker
            | OpCode::Splice
            | OpCode::Frankenstein
            | OpCode::Relativity
            | OpCode::Graviton
            | OpCode::EventHorizon
            | OpCode::Aeolus
            | OpCode::Storm
            | OpCode::SenseWind
            | OpCode::SenseMoisture
            | OpCode::QuantumJump
            | OpCode::Isomerize
            | OpCode::Spirit
            | OpCode::Alchemy
            | OpCode::Meme
            | OpCode::Conceive
            | OpCode::Propagate
            | OpCode::Forget
            | OpCode::Shibboleth
            | OpCode::Infect
            | OpCode::Outbreak
            | OpCode::Sanitize
            | OpCode::Drift
            | OpCode::Poly
            | OpCode::Chronostasis
            | OpCode::Prophecy
            | OpCode::Sing
            | OpCode::Listen
            | OpCode::Hyphae
            | OpCode::Connect
            | OpCode::Transport
            | OpCode::SporeCloud
            | OpCode::Brainfuck
            | OpCode::Irradiate
            | OpCode::SenseMutagen
            | OpCode::Devour
            | OpCode::Evolve
            | OpCode::Glitch
            | OpCode::Scramble
            | OpCode::Metamorphosis
            | OpCode::Pigment
            | OpCode::Glyph
            | OpCode::SensePigment
            | OpCode::SenseGlyph
            | OpCode::Rift
            | OpCode::Seal
            | OpCode::Shape
            | OpCode::Void
            | OpCode::Supernova
            | OpCode::Singularity
            | OpCode::Simulate
            | OpCode::Dream
            | OpCode::Lucid
            | OpCode::Chemotaxis
            | OpCode::Identity
            | OpCode::Differentiate
            | OpCode::Sporulate
            | OpCode::TimeLoop
            | OpCode::Germinate
            | OpCode::Paradox
            | OpCode::Spawn
            | OpCode::Incubate
            | OpCode::Methylate
            | OpCode::Demethylate
            | OpCode::Telomerase
            | OpCode::TLen
            | OpCode::Recombine
            | OpCode::SIndex
            | OpCode::CrisprScan
            | OpCode::Cas9Cut
            | OpCode::Ligase
            | OpCode::Mitosis
            | OpCode::Apoptosis
            | OpCode::Integrase
            | OpCode::Excision
            | OpCode::Secrete
            | OpCode::Detect
            | OpCode::Absorb
            | OpCode::Migrate
            | OpCode::Detox
            | OpCode::WRead
            | OpCode::Call
            | OpCode::Exec
            | OpCode::Ret
            | OpCode::Bind
            | OpCode::Unbind
            | OpCode::Entangle
            | OpCode::Decohere
            | OpCode::Conjugate
            | OpCode::Gravitate
            | OpCode::Lumine
            | OpCode::SenseLight
            | OpCode::Broadcast
            | OpCode::Tune
            | OpCode::PhaseShift
            | OpCode::Membrane
            | OpCode::Osmosis
            | OpCode::Symbiosis
            | OpCode::Lysis
            | OpCode::Reflex
            | OpCode::Compile
            | OpCode::Decompile
            | OpCode::Sonar
            | OpCode::Eval
            | OpCode::LispEval
            | OpCode::Map
            | OpCode::Fold
            | OpCode::Filter
            | OpCode::Zip
            | OpCode::Match
            | OpCode::Bury
            | OpCode::Exhume
            | OpCode::Seance
            | OpCode::Mourn
            | OpCode::TimeWarp
            | OpCode::Chronos
            | OpCode::Retroscope
            | OpCode::Reincarnate
            | OpCode::Piet
            | OpCode::Terraform
            | OpCode::SenseBiome
            | OpCode::RetinaDraw
            | OpCode::RetinaClear
            | OpCode::RetinaSize
            | OpCode::Scanline
            | OpCode::Rasterize
            | OpCode::EgregoreLink
            | OpCode::EgregoreTithe
            | OpCode::EgregoreChannel
            | OpCode::EgregoreDictate
            | OpCode::EgregoreQuery
            | OpCode::EgregoreSummon
            | OpCode::Sacrifice
            | OpCode::Entropy
            | OpCode::Stabilize
            | OpCode::Disintegrate
            | OpCode::Tsunami
            | OpCode::Dry
            | OpCode::Harmonize
            | OpCode::Choir
            | OpCode::Mix
            | OpCode::Brew
            | OpCode::Splash
            | OpCode::Fossilize
            | OpCode::Unearth
            | OpCode::CarbonDate
            | OpCode::Logistics
            | OpCode::Sow
            | OpCode::Harvest
            | OpCode::Draw
            | OpCode::Fate
            | OpCode::Shuffle
            | OpCode::Knot
            | OpCode::Unknot
            | OpCode::Cord
            | OpCode::ReadCord
            | OpCode::Tangle
            | OpCode::Pray
            | OpCode::Genesis
            | OpCode::Retrograde
            | OpCode::Synthesize
            | OpCode::Catalyze
            | OpCode::VoidRift
            | OpCode::VoidCast
            | OpCode::Chaos
            | OpCode::TuiMod
            | OpCode::Cambrian
            | OpCode::Prologue
            | OpCode::Rune
            | OpCode::BioHack
            | OpCode::SelfReplicate
            | OpCode::Forge
            | OpCode::Speak
            | OpCode::Etymology => Some(nova::exec_nova_op(self, op, args)),
            OpCode::EvoPopSize
            | OpCode::EvoLoad
            | OpCode::EvoStore
            | OpCode::EvoScore
            | OpCode::EvoBreed
            | OpCode::EvoMutate
            | OpCode::EvoReplace
            | OpCode::EvoClear
            | OpCode::EvoSave => Some(evolution::exec_evo_op(self, op, args)),
            #[cfg(feature = "nova")]
            OpCode::Codex => {
                if let Some(Value::Int(id)) = self.stack.pop() {
                    if let Some(spell) = self.codex.get_spell(id as usize) {
                        codex::exec_spell(self, &spell);
                    } else {
                        self.output
                            .push(format!("Error: Invalid Codex spell ID {}", id));
                    }
                } else {
                    self.output
                        .push("Error: Codex requires spell ID (Int)".to_string());
                }
                Some(None)
            }
            OpCode::Weave | OpCode::Unravel => Some(nova_weaver::exec_weave_op(self, op, args)),
            OpCode::Mutagen => Some(self.exec_mutagen_op()),
            OpCode::Scavenge => Some(self.exec_scavenge_op()),
            OpCode::Digest => Some(self.exec_digest_op()),
            OpCode::EntropySurge => {
                nova_flux::exec_entropy_surge(self);
                Some(None)
            }
            OpCode::QuantumTunnel => Some(nova_flux::exec_quantum_tunnel(self)),
            OpCode::Chain | OpCode::Curry | OpCode::Quote => {
                Some(nova_functional::exec_functional_op(self, op, args))
            }
            OpCode::Crossover => Some(nova_genetics::exec_crossover(self)),
            OpCode::Orca => {
                self.orca_mode = !self.orca_mode;
                let status = if self.orca_mode { "ON" } else { "OFF" };
                self.output
                    .push(format!("ORCA: Signal Processing {}", status));
                Some(None)
            }
            OpCode::Glossolalia | OpCode::Clarify | OpCode::Confuse => {
                babel_chaos::exec_babel_chaos_op(self, op, args);
                Some(None)
            }
            OpCode::Crucible => {
                alchemy::exec_crucible_op(self, op, args);
                Some(None)
            }
            OpCode::Chr => Some(self.exec_char_op()),
            OpCode::Guild => Some(nova_guild::exec_guild(self)),
            OpCode::Charter => Some(nova_guild::exec_charter(self)),
            OpCode::Logos => {
                self.logos_mode = !self.logos_mode;
                let status = if self.logos_mode { "ON" } else { "OFF" };
                self.output
                    .push(format!("LOGOS: Logic Chemistry {}", status));
                Some(None)
            }
            OpCode::Note | OpCode::Rest | OpCode::Tempo | OpCode::Perform | OpCode::Compose => {
                bard::exec_bard_op(self, op, args);
                Some(None)
            }
            OpCode::Scan | OpCode::Locate | OpCode::Chart | OpCode::Atlas => {
                nova_cartography::exec_cartography_op(self, op, args);
                Some(None)
            }
            OpCode::Pocket | OpCode::Unpocket => Some(nova::exec_nova_op(self, op, args)),
            OpCode::Quake
            | OpCode::Erode
            | OpCode::Sediment
            | OpCode::Tectonics
            | OpCode::Volcano => {
                nova_geology::exec_geology_op(self, op, args);
                Some(None)
            }
            OpCode::LeySense | OpCode::LeyTap | OpCode::LeyWarp | OpCode::LeyShift => {
                Some(nova_ley::exec_ley_op(self, op, args))
            }
            OpCode::Nucleate | OpCode::Accrete | OpCode::Shatter | OpCode::Anneal => {
                nova_crystal::exec_crystal_op(self, op, args);
                Some(None)
            }
            OpCode::Dimension | OpCode::DRead | OpCode::DWrite | OpCode::DMerge | OpCode::DView => {
                nova_planes::exec_planes_op(self, op, args);
                Some(None)
            }
            OpCode::StringNew | OpCode::StringPluck | OpCode::StringTune | OpCode::StringListen => {
                Some(nova_strings::exec_string_op(self, op, args))
            }
            OpCode::Bond => Some(nova_metazoa::exec_bond(self, op, args)),
            OpCode::Unbond => Some(nova_metazoa::exec_unbond(self, op, args)),
            OpCode::Signify => Some(nova_metazoa::exec_signify(self, op, args)),
            OpCode::Tissue => Some(nova_metazoa::exec_tissue(self, op, args)),
            OpCode::MeshNet
            | OpCode::MeshGrow
            | OpCode::MeshPrune
            | OpCode::MeshSend
            | OpCode::MeshRecv => {
                crate::vm::nova::exec_nova_op(self, op, args);
                Some(None)
            }
            OpCode::Reactor | OpCode::Reaction => {
                crate::vm::nova::exec_nova_op(self, op, args);
                Some(None)
            }
            OpCode::AbsorbGeometry => {
                Some(nova_alchemy_prime::exec_absorb_geometry(self, op, args))
            }
            OpCode::ProjectGeometry => {
                Some(nova_alchemy_prime::exec_project_geometry(self, op, args))
            }
            OpCode::HyperAdd
            | OpCode::HyperSub
            | OpCode::HyperMul
            | OpCode::HyperDiv
            | OpCode::Reduce
            | OpCode::Cross
            | OpCode::ZipWith => {
                nova_raku::exec_raku_op(self, op, args);
                Some(None)
            }
            #[cfg(feature = "oracle")]
            OpCode::Divergence => Some(nova::exec_nova_op(self, op, args)),
            _ => None,
        }
    }

    pub(crate) fn execute_gene_inner(
        &mut self,
        op: OpCode,
        args: &[Nucleotide],
    ) -> Option<(usize, usize)> {
        if let Some(res) = self.exec_core_op(op.clone(), args) {
            return res;
        }

        #[cfg(feature = "cortex")]
        if matches!(
            op,
            OpCode::Link | OpCode::Sever | OpCode::Spark | OpCode::Sense | OpCode::Gate
        ) {
            cortex::exec_cortex_op(self, op, args);
            return None;
        }

        #[cfg(feature = "nova")]
        if let Some(res) = self.exec_nova_dispatch(op.clone(), args) {
            return res;
        }

        #[cfg(feature = "oracle")]
        match op {
            OpCode::FindAll => return self.exec_findall_op(),
            OpCode::Assert
            | OpCode::Rule
            | OpCode::Retract
            | OpCode::Query
            | OpCode::Augury
            | OpCode::Divinate
            | OpCode::Seek
            | OpCode::Manifest
            | OpCode::Unify
            | OpCode::PrologCall
            | OpCode::Censor => return oracle::exec_oracle_op(self, op, args),
            _ => {}
        }

        #[cfg(feature = "resonance")]
        if matches!(
            op,
            OpCode::Pluck | OpCode::Oscillate | OpCode::Hear | OpCode::Scream
        ) {
            resonance::exec_resonance_op(self, op, args);
            return None;
        }

        #[cfg(all(feature = "nova", feature = "resonance"))]
        match op {
            OpCode::Sift => {
                nova_cymatics::exec_sift(self, op, args);
                return None;
            }
            OpCode::Reshape => {
                nova_cymatics::exec_reshape(self, op, args);
                return None;
            }
            _ => {}
        }

        #[cfg(feature = "biophysics")]
        if matches!(
            op,
            OpCode::NeuroGenesis
                | OpCode::Stimulate
                | OpCode::Dendrite
                | OpCode::Axon
                | OpCode::Receptor
                | OpCode::NeuroCoupling
                | OpCode::NeuroSynapse
        ) {
            neuron::exec_biophysics_op(self, op, args);
            return None;
        }

        #[cfg(feature = "silicon")]
        if matches!(
            op,
            OpCode::Conduct
                | OpCode::Wire
                | OpCode::Pulse
                | OpCode::Silicon
                | OpCode::Construct
                | OpCode::LogicGate
                | OpCode::PinIn
                | OpCode::PinOut
                | OpCode::Emitter
                | OpCode::Receiver
                | OpCode::Latch
                | OpCode::DAC
                | OpCode::ADC
                | OpCode::Trace
                | OpCode::Fabricate
        ) {
            silicon::exec_silicon_op(self, op, args);
            return None;
        }

        #[cfg(feature = "elektra")]
        if matches!(
            op,
            OpCode::Electrogenesis
                | OpCode::Induction
                | OpCode::WireGrowth
                | OpCode::CircuitBreaker
                | OpCode::Battery
                | OpCode::Ground
                | OpCode::SenseVolt
                | OpCode::Shock
                | OpCode::TeslaCoil
                | OpCode::Diode
                | OpCode::Transistor
                | OpCode::Muscle
                | OpCode::Sensor
                | OpCode::Patch
                | OpCode::Electrophoresis
                | OpCode::Modulate
                | OpCode::Lightning
        ) {
            return elektra::exec_elektra_op(self, op, args);
        }

        #[cfg(all(feature = "elektra", feature = "nova"))]
        if matches!(op, OpCode::Galvanize | OpCode::Railgun) {
            return elektra::exec_elektra_op(self, op, args);
        }

        #[cfg(feature = "hive")]
        if matches!(
            op,
            OpCode::HiveBind | OpCode::HiveSend | OpCode::HiveRecv | OpCode::HiveClose
        ) {
            hive::exec_hive_op(self, op, args);
            return None;
        }

        #[cfg(feature = "git")]
        if matches!(op, OpCode::Ancestry | OpCode::Excavate | OpCode::Evolution) {
            return git::exec_git_op(self, op, args);
        }

        #[cfg(feature = "phylogeny")]
        if matches!(
            op,
            OpCode::Crawl
                | OpCode::Sequencing
                | OpCode::PhyloSynthesize
                | OpCode::PhyloInfect
                | OpCode::Shell
        ) {
            phylogeny::exec_phylogeny_op(self, op, args);
            return None;
        }

        if let OpCode::Nop = op {
            return None;
        }

        if let OpCode::Unknown(name) = op {
            return self.handle_unknown_opcode(&name);
        }

        self.output
            .push(format!("Error: Unimplemented OpCode {}", op));
        None
    }

    fn binary_op<F>(stack: &mut Vec<Value>, output: &mut Vec<String>, op: F)
    where
        F: Fn(i64, i64) -> i64 + Copy,
    {
        if stack.len() < 2 {
            output.push("Error: Stack underflow".to_string());
            return;
        }
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();

        if let Some(res) = a.apply_binary_op(b, op, MAX_RECURSION_DEPTH, MAX_JUNCTION_SIZE) {
            stack.push(res);
        } else {
            output.push("Error: Type mismatch or complexity limit".to_string());
        }
    }

    fn exec_stack_op(&mut self, op: OpCode, args: &[Nucleotide]) -> Option<(usize, usize)> {
        match op {
            OpCode::Push => {
                if let Some(arg) = args.first() {
                    fn nuc_to_val(n: &Nucleotide, depth: usize) -> Option<Value> {
                        if depth > MAX_RECURSION_DEPTH {
                            return None;
                        }
                        match n {
                            Nucleotide::Number(v) => Some(Value::Int(*v)),
                            Nucleotide::String(s) => Some(Value::Str(s.clone())),
                            Nucleotide::Identifier(s) => Some(Value::Str(s.clone())),
                            Nucleotide::Junction(t, list) => {
                                let mut vals = Vec::new();
                                for item in list {
                                    if let Some(v) = nuc_to_val(item, depth + 1) {
                                        vals.push(v);
                                    } else {
                                        return None;
                                    }
                                }
                                Some(Value::Junction(*t, vals))
                            }
                        }
                    }

                    if let Some(val) = nuc_to_val(arg, 0) {
                        self.stack.push(val);
                    } else {
                        self.output
                            .push(format!("Error: Invalid arg for push: {:?}", arg));
                    }
                }
            }
            OpCode::Dup => {
                if let Some(val) = self.stack.last() {
                    let v: Value = Clone::clone(val);
                    self.stack.push(v);
                }
            }
            OpCode::Swap => {
                let len = self.stack.len();
                if len >= 2 {
                    self.stack.swap(len - 1, len - 2);
                } else {
                    self.output
                        .push("Error: Stack underflow for swap".to_string());
                }
            }
            OpCode::Drop => {
                self.stack.pop();
            }
            OpCode::SLen => {
                self.stack.push(Value::Int(self.stack.len() as i64));
            }
            OpCode::HelixLen => {
                self.stack
                    .push(Value::Int(self.dna.helix.strands.len() as i64));
            }
            OpCode::GeneLen => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(idx) => {
                            if idx >= 0 && (idx as usize) < self.dna.helix.strands.len() {
                                let len = self.dna.helix.strands[idx as usize].genes.len();
                                self.stack.push(Value::Int(len as i64));
                            } else {
                                self.output.push(
                                    "Error: Strand index out of bounds for gene_len".to_string(),
                                );
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for gene_len".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for gene_len".to_string());
                }
            }
            _ => {}
        }
        None
    }

    fn exec_math_op(&mut self, op: OpCode) {
        #[cfg(feature = "nova")]
        let effective_op = if self.chirality == Chirality::Right {
            match op {
                OpCode::Add => OpCode::Sub,
                OpCode::Sub => OpCode::Add,
                OpCode::Mul => OpCode::Div,
                OpCode::Div => OpCode::Mul,
                OpCode::Gt => OpCode::Lt,
                OpCode::Lt => OpCode::Gt,
                _ => op,
            }
        } else {
            op
        };
        #[cfg(not(feature = "nova"))]
        let effective_op = op;

        match effective_op {
            OpCode::Eq => {
                if self.stack.len() >= 2 {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Int(if a == b { 1 } else { 0 }));
                } else {
                    self.output.push("Error: Stack underflow".to_string());
                }
            }
            OpCode::Gt | OpCode::Lt => {
                if self.stack.len() >= 2 {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Int(ia), Value::Int(ib)) => {
                            let res = match effective_op {
                                OpCode::Gt => ia > ib,
                                OpCode::Lt => ia < ib,
                                _ => false,
                            };
                            self.stack.push(Value::Int(if res { 1 } else { 0 }));
                        }
                        _ => self.output.push("Error: Type mismatch".to_string()),
                    }
                } else {
                    self.output.push("Error: Stack underflow".to_string());
                }
            }
            OpCode::Add => {
                // Check for string concatenation
                if self.stack.len() >= 2 {
                    let b_is_str = matches!(self.stack.last(), Some(Value::Str(_)));
                    let a_is_str = matches!(
                        self.stack.get(self.stack.len().saturating_sub(2)),
                        Some(Value::Str(_))
                    );

                    if a_is_str && b_is_str {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        if let (Value::Str(s1), Value::Str(s2)) = (a, b) {
                            self.stack.push(Value::Str(s1 + &s2));
                            return;
                        }
                    }
                }
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a.wrapping_add(b));
            }
            OpCode::Sub => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a.wrapping_sub(b));
            }
            OpCode::Mul => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a.wrapping_mul(b));
            }
            OpCode::Div => {
                if self.stack.len() < 2 {
                    self.output.push("Error: Stack underflow".to_string());
                } else {
                    let b_val = self.stack.pop().unwrap();
                    let a_val = self.stack.pop().unwrap();
                    match (a_val, b_val) {
                        (Value::Int(a), Value::Int(b)) => {
                            if b == 0 {
                                self.output.push("Error: Division by zero".to_string());
                            } else if a == i64::MIN && b == -1 {
                                self.output.push("Error: Division overflow".to_string());
                            } else {
                                self.stack.push(Value::Int(a / b));
                            }
                        }
                        _ => self.output.push("Error: Type mismatch".to_string()),
                    }
                }
            }
            _ => {}
        }
    }

    fn exec_io_op(&mut self, op: OpCode) {
        if let OpCode::Print = op {
            if let Some(val) = self.stack.pop() {
                self.output.push(format!("{}", val));
            }
        }
    }

    fn exec_flow_op(&mut self, op: OpCode, args: &[Nucleotide]) -> Option<(usize, usize)> {
        match op {
            OpCode::Jump => {
                if let Some(Nucleotide::Number(n)) = args.first() {
                    Some((*n as usize, 0))
                } else {
                    self.output.push("Error: Invalid arg for jump".to_string());
                    None
                }
            }
            OpCode::Brz => {
                if let Some(Nucleotide::Number(n)) = args.first() {
                    if let Some(val) = self.stack.pop() {
                        fn check_zero(v: &Value) -> bool {
                            match v {
                                Value::Int(i) => *i == 0,
                                Value::Junction(t, vals) => match t {
                                    JunctionType::Any => vals.iter().any(check_zero),
                                    JunctionType::All | JunctionType::Dish => {
                                        vals.iter().all(check_zero)
                                    }
                                },
                                Value::Superposition(states) => {
                                    states.iter().any(|(v, _)| check_zero(v))
                                }
                                _ => false,
                            }
                        }

                        let is_zero = check_zero(&val);

                        #[cfg(feature = "nova")]
                        let condition = if self.chirality == Chirality::Right {
                            !is_zero
                        } else {
                            is_zero
                        };
                        #[cfg(not(feature = "nova"))]
                        let condition = is_zero;

                        if condition {
                            return Some((*n as usize, 0));
                        }
                    } else {
                        self.output
                            .push("Error: Stack underflow for brz".to_string());
                    }
                } else {
                    self.output.push("Error: Invalid arg for brz".to_string());
                }
                None
            }
            OpCode::JumpS => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(target) => {
                            if target >= 0 {
                                return Some((target as usize, 0));
                            } else {
                                self.output.push("Error: Negative jump target".to_string());
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for jump_s".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for jump_s".to_string());
                }
                None
            }
            OpCode::BrzS => {
                if self.stack.len() >= 2 {
                    let target_val = self.stack.pop().unwrap();
                    let cond_val = self.stack.pop().unwrap();

                    match (target_val, cond_val) {
                        (Value::Int(target), Value::Int(cond)) => {
                            if cond == 0 {
                                if target >= 0 {
                                    return Some((target as usize, 0));
                                } else {
                                    self.output.push("Error: Negative jump target".to_string());
                                }
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for brz_s".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for brz_s".to_string());
                }
                None
            }
            _ => None,
        }
    }

    /// Executes a spatial grid operation.
    ///
    /// These enzymes allow the organism to interact with its Petri Dish memory.
    ///
    /// - `GRead`/`GWrite`: Direct cell access.
    /// - `Radiate`: Writes a value to a circular area (Area of Effect).
    /// - `Siphon`: Consumes values from a circular area, summing them up.
    /// - `Virus`: Executes code found in the grid (Code injection).
    fn exec_grid_op(&mut self, op: OpCode) -> Option<(usize, usize)> {
        match op {
            OpCode::GRead => {
                if self.stack.len() >= 2 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                        if self.is_valid_coord(y, x) {
                            self.stack.push(self.grid[y as usize][x as usize].clone());
                        } else {
                            self.output
                                .push("Error: Grid index out of bounds".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for g_read".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for g_read".to_string());
                }
            }
            OpCode::GWrite => {
                #[cfg(feature = "nova")]
                if self.phase == nova::Phase::Ethereal {
                    self.output
                        .push("Error: Ethereal phase prevents GWrite".to_string());
                    return None;
                }

                if self.stack.len() >= 3 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    let val = self.stack.pop().unwrap();
                    if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                        if self.is_valid_coord(y, x) {
                            self.grid[y as usize][x as usize] = val;
                        } else {
                            self.output
                                .push("Error: Grid index out of bounds".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for g_write".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for g_write".to_string());
                }
            }
            OpCode::Radiate => {
                if self.stack.len() >= 4 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    let r_val = self.stack.pop().unwrap();
                    let val = self.stack.pop().unwrap();

                    if let (Value::Int(x), Value::Int(y), Value::Int(r)) = (x_val, y_val, r_val) {
                        let mut count = 0;
                        iterate_circle(
                            #[cfg(feature = "nova")]
                            self.topology,
                            x,
                            y,
                            r,
                            |cx, cy| {
                                self.grid[cy][cx] = val.clone();
                                count += 1;
                            },
                        );
                        self.energy = self.energy.saturating_sub((count / 2) as i64);
                        self.output.push(format!(
                            "RADIATE: Affected {} cells at {},{} r={}",
                            count, x, y, r
                        ));
                    } else {
                        self.output
                            .push("Error: Type mismatch for radiate".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for radiate".to_string());
                }
            }
            OpCode::Siphon => {
                if self.stack.len() >= 3 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    let r_val = self.stack.pop().unwrap();

                    if let (Value::Int(x), Value::Int(y), Value::Int(r)) = (x_val, y_val, r_val) {
                        let mut count = 0;
                        let mut sum: i64 = 0;
                        iterate_circle(
                            #[cfg(feature = "nova")]
                            self.topology,
                            x,
                            y,
                            r,
                            |cx, cy| {
                                if let Value::Int(n) = self.grid[cy][cx] {
                                    sum = sum.saturating_add(n);
                                }
                                self.grid[cy][cx] = Value::Int(0);
                                count += 1;
                            },
                        );
                        self.stack.push(Value::Int(sum));
                        self.energy = self.energy.saturating_sub(5);
                        self.output
                            .push(format!("SIPHON: Absorbed {} from {} cells", sum, count));
                    } else {
                        self.output
                            .push("Error: Type mismatch for siphon".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for siphon".to_string());
                }
            }
            OpCode::Virus => {
                if self.stack.len() >= 2 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();

                    let coords = if let (Value::Int(y), Value::Int(x)) = (&y_val, &x_val) {
                        if self.is_valid_coord(*y, *x) {
                            Some((y, x))
                        } else {
                            self.output
                                .push("Error: Grid index out of bounds".to_string());
                            None
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for virus".to_string());
                        None
                    };

                    if let Some((y, x)) = coords {
                        let val = self.grid[*y as usize][*x as usize].clone();
                        match val {
                            Value::Int(n) => self.stack.push(Value::Int(n)),
                            Value::Str(s) => {
                                let old_loc = self.context_loc;
                                self.context_loc = (*y as usize, *x as usize);
                                let op = s.parse().unwrap_or(OpCode::Unknown(s.clone()));
                                let result = self.execute_gene(op, &[]);
                                self.context_loc = old_loc;
                                return result;
                            }
                            Value::Junction(_, _) => {
                                self.output
                                    .push("Error: Virus cannot execute junction".to_string());
                            }
                            Value::Superposition(_) => {
                                self.output
                                    .push("Error: Virus cannot execute superposition".to_string());
                            }
                            Value::Symbol(_) => {
                                self.output
                                    .push("Error: Virus cannot execute symbol".to_string());
                            }
                            Value::Color(_, _, _) => {
                                self.output
                                    .push("Error: Virus cannot execute color".to_string());
                            }
                        }
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for virus".to_string());
                }
            }
            _ => {}
        }
        None
    }

    /// Executes a biological function enzyme.
    ///
    /// These operations define the organism's metabolism and self-modification capabilities.
    ///
    /// - `Photosynthesize`: Generates small amounts of Energy from nothing (Sunlight).
    /// - `Consume`: Converts top-of-stack Data into Energy.
    /// - `Genome`: Introspection (pushes current DNA to stack).
    /// - `Transcribe`: Epigenetic modification (rewrites arguments of genes at runtime).
    fn exec_bio_op(&mut self, op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
        match op {
            OpCode::Photosynthesize => {
                self.energy = self.energy.saturating_add(5);
            }
            OpCode::Consume => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(n) => self.energy = self.energy.saturating_add(n),
                        Value::Str(s) => self.energy = self.energy.saturating_add(s.len() as i64),
                        Value::Junction(_, _) => {
                            self.output
                                .push("Error: Cannot consume junction".to_string());
                        }
                        Value::Superposition(states) => {
                            let mut total = 0.0;
                            for (v, p) in states {
                                match v {
                                    Value::Int(n) => total += (n as f64) * p,
                                    Value::Str(s) => total += (s.len() as f64) * p,
                                    _ => {}
                                }
                            }
                            self.energy = self.energy.saturating_add(total as i64);
                        }
                        Value::Symbol(_) => {
                            self.output.push("Error: Cannot consume symbol".to_string());
                        }
                        Value::Color(r, g, b) => {
                            let e = (r as i64 + g as i64 + b as i64) / 3;
                            self.energy = self.energy.saturating_add(e);
                        }
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for consume".to_string());
                }
            }
            OpCode::Genome => {
                if self.ip.0 < self.dna.helix.strands.len() {
                    let strand = &self.dna.helix.strands[self.ip.0];
                    self.stack.push(Value::Int(strand.genes.len() as i64));
                    for gene in &strand.genes {
                        self.stack.push(Value::Str(gene.op.to_string()));
                    }
                }
            }
            OpCode::Transcribe => {
                if self.stack.len() < 4 {
                    self.output
                        .push("Error: Stack underflow for transcribe".to_string());
                    return None;
                }
                let val = self.stack.pop().unwrap();
                let arg_idx_val = self.stack.pop().unwrap();
                let gene_idx_val = self.stack.pop().unwrap();
                let strand_idx_val = self.stack.pop().unwrap();

                match (val, arg_idx_val, gene_idx_val, strand_idx_val) {
                    (Value::Int(v), Value::Int(ai), Value::Int(gi), Value::Int(si)) => {
                        let si_idx = si as usize;
                        #[cfg(feature = "nova")]
                        let mut success = false;
                        if si >= 0 && si_idx < self.dna.helix.strands.len() {
                            let strand = &mut self.dna.helix.strands[si_idx];
                            if gi >= 0 && (gi as usize) < strand.genes.len() {
                                let gene = &mut strand.genes[gi as usize];
                                if ai >= 0 && (ai as usize) < gene.args.len() {
                                    gene.args[ai as usize] = Nucleotide::Number(v);
                                    self.output.push(format!(
                                        "TRANSCRIBE: strand {} gene {} arg {} -> {}",
                                        si, gi, ai, v
                                    ));
                                    #[cfg(feature = "nova")]
                                    {
                                        success = true;
                                    }
                                } else {
                                    self.output
                                        .push("Error: Arg index out of bounds".to_string());
                                }
                            } else {
                                self.output
                                    .push("Error: Gene index out of bounds".to_string());
                            }
                        } else {
                            self.output
                                .push("Error: Strand index out of bounds".to_string());
                        }

                        #[cfg(feature = "nova")]
                        if success {
                            if let Some(&partner_idx) = self.entangled_pairs.get(&si_idx) {
                                if partner_idx < self.dna.helix.strands.len() {
                                    let p_strand = &mut self.dna.helix.strands[partner_idx];
                                    if (gi as usize) < p_strand.genes.len() {
                                        let p_gene = &mut p_strand.genes[gi as usize];
                                        if (ai as usize) < p_gene.args.len() {
                                            p_gene.args[ai as usize] = Nucleotide::Number(v);
                                            self.output.push(format!(
                                                "ENTANGLEMENT: Transcribed partner {} gene {} arg {}",
                                                partner_idx, gi, ai
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => self
                        .output
                        .push("Error: Type mismatch for transcribe args".to_string()),
                }
            }
            _ => {}
        }
        None
    }

    pub fn mutate(&mut self) {
        #[cfg(feature = "nova")]
        if self.phase == nova::Phase::Crystalline {
            return;
        }

        #[cfg(feature = "nova")]
        if self.hologram_mode {
            nova_hologram::mutate_hologram(self, 0.5);
            let new_genes = nova_hologram::refract_genes(self);
            if !new_genes.is_empty() {
                let helix_len = self.dna.helix.strands.len();
                if helix_len > 0 {
                    let mut rng = rand::thread_rng();
                    let target_idx = rng.gen_range(0..helix_len);
                    self.dna.helix.strands[target_idx] = crate::ast::Strand { genes: new_genes };
                    self.output.push(format!(
                        "HOLOGRAPHIC MUTATION: Refracted strand {}",
                        target_idx
                    ));
                }
            }
            return;
        }

        let mut rng = rand::thread_rng();
        let helix_len = self.dna.helix.strands.len();
        if helix_len == 0 {
            return;
        }

        let strand_idx = rng.gen_range(0..helix_len);
        let gene_count = self.dna.helix.strands[strand_idx].genes.len();
        if gene_count == 0 {
            return;
        }

        let gene_idx = rng.gen_range(0..gene_count);

        // 50% chance to change name, 50% to change arg
        if rng.gen_bool(0.5) {
            let enzymes = [
                OpCode::Push,
                OpCode::Add,
                OpCode::Sub,
                OpCode::Mul,
                OpCode::Div,
                OpCode::Dup,
                OpCode::Print,
                OpCode::Swap,
                OpCode::Drop,
                OpCode::Jump,
                OpCode::Brz,
                OpCode::Photosynthesize,
                OpCode::Consume,
                OpCode::Transcribe,
                OpCode::SLen,
                OpCode::HelixLen,
                OpCode::GeneLen,
                OpCode::GRead,
                OpCode::GWrite,
                OpCode::Radiate,
                OpCode::Siphon,
                OpCode::Genome,
                #[cfg(feature = "nova")]
                OpCode::Telomerase,
                #[cfg(feature = "nova")]
                OpCode::TLen,
                #[cfg(feature = "nova")]
                OpCode::SIndex,
                #[cfg(feature = "nova")]
                OpCode::Mitosis,
                #[cfg(feature = "nova")]
                OpCode::Apoptosis,
                #[cfg(feature = "nova")]
                OpCode::CrisprScan,
                #[cfg(feature = "nova")]
                OpCode::Cas9Cut,
                #[cfg(feature = "nova")]
                OpCode::Ligase,
                #[cfg(feature = "nova")]
                OpCode::Entangle,
                #[cfg(feature = "nova")]
                OpCode::Decohere,
            ];
            let new_op = enzymes[rng.gen_range(0..enzymes.len())].clone();

            // Apply to primary
            let old_op = self.dna.helix.strands[strand_idx].genes[gene_idx]
                .op
                .clone();
            self.dna.helix.strands[strand_idx].genes[gene_idx].op = new_op.clone();
            self.output
                .push(format!("MUTATION: {} -> {}", old_op, new_op));

            #[cfg(feature = "nova")]
            {
                self.cladistics.mutate_strand(strand_idx);
                self.trigger_reflex(1); // Event 1: Mutation
            }

            #[cfg(feature = "nova")]
            if let Some(&partner_idx) = self.entangled_pairs.get(&strand_idx) {
                if partner_idx < self.dna.helix.strands.len()
                    && gene_idx < self.dna.helix.strands[partner_idx].genes.len()
                {
                    self.dna.helix.strands[partner_idx].genes[gene_idx].op = new_op;
                    self.output.push(format!(
                        "ENTANGLEMENT: Mutated partner {} gene {} op",
                        partner_idx, gene_idx
                    ));
                }
            }
        } else {
            let has_args = !self.dna.helix.strands[strand_idx].genes[gene_idx]
                .args
                .is_empty();
            if has_args {
                let old_n = match &self.dna.helix.strands[strand_idx].genes[gene_idx].args[0] {
                    Nucleotide::Number(n) => *n,
                    _ => return, // Skip non-number args for simplicity
                };
                let new_n = rng.gen_range(0..100);

                // Apply
                self.dna.helix.strands[strand_idx].genes[gene_idx].args[0] =
                    Nucleotide::Number(new_n);
                self.output
                    .push(format!("MUTATION: arg {} -> {}", old_n, new_n));

                #[cfg(feature = "nova")]
                {
                    self.cladistics.mutate_strand(strand_idx);
                    self.trigger_reflex(1); // Event 1: Mutation
                }

                #[cfg(feature = "nova")]
                if let Some(&partner_idx) = self.entangled_pairs.get(&strand_idx) {
                    if partner_idx < self.dna.helix.strands.len()
                        && gene_idx < self.dna.helix.strands[partner_idx].genes.len()
                        && !self.dna.helix.strands[partner_idx].genes[gene_idx]
                            .args
                            .is_empty()
                    {
                        self.dna.helix.strands[partner_idx].genes[gene_idx].args[0] =
                            Nucleotide::Number(new_n);
                        self.output.push(format!(
                            "ENTANGLEMENT: Mutated partner {} gene {} arg",
                            partner_idx, gene_idx
                        ));
                    }
                }
            }
        }
    }
}

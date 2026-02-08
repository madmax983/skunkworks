#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};
use crate::ast::{Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::{ChimeraParser, Rule};
use pest::Parser;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Simple Linear Congruential Generator for stream cipher
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_byte(&mut self) -> u8 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.state >> 56) as u8
    }
}

fn strand_to_string(strand: &Strand) -> String {
    fn format_nucleotide(n: &Nucleotide, depth: usize) -> String {
        if depth > 10 {
            return "...".to_string();
        }
        match n {
            Nucleotide::Number(i) => i.to_string(),
            Nucleotide::String(s) => format!("\"{}\"", s),
            Nucleotide::Identifier(s) => s.clone(),
            Nucleotide::Junction(t, args) => {
                let t_str = match t {
                    crate::ast::JunctionType::Any => "any",
                    crate::ast::JunctionType::All => "all",
                };
                let args_str: Vec<String> = args
                    .iter()
                    .map(|arg| format_nucleotide(arg, depth + 1))
                    .collect();
                format!("{}({})", t_str, args_str.join(" "))
            }
        }
    }

    let mut s = String::from("[ ");
    for gene in &strand.genes {
        s.push_str(gene.op.as_ref());
        s.push('(');
        for (i, arg) in gene.args.iter().enumerate() {
            if i > 0 {
                s.push(' ');
            }
            s.push_str(&format_nucleotide(arg, 0));
        }
        s.push(')');
        s.push(' ');
    }
    s.push(']');
    s
}

pub fn exec_encrypt(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: strand_idx, key (top)
    if vm.stack.len() >= 2 {
        let key_val = vm.stack.pop().unwrap();
        let s_val = vm.stack.pop().unwrap();

        if let (Value::Int(key), Value::Int(idx)) = (key_val, s_val) {
            let s_idx = idx as usize;
            if s_idx < vm.dna.helix.strands.len() {
                let strand = &vm.dna.helix.strands[s_idx];
                let plaintext = strand_to_string(strand);
                let bytes = plaintext.as_bytes();

                let mut lcg = Lcg::new(key as u64);
                let mut ciphertext_hex = String::new();

                for b in bytes {
                    let cipher_byte = b ^ lcg.next_byte();
                    ciphertext_hex.push_str(&format!("{:02x}", cipher_byte));
                }

                vm.stack.push(Value::Str(ciphertext_hex));
                vm.energy = vm.energy.saturating_sub(10 + (bytes.len() as i64 / 10));
                vm.output.push(format!("ENCRYPT: Encrypted strand {}", s_idx));
            } else {
                vm.output.push("Error: Strand index out of bounds".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for encrypt".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for encrypt".to_string());
    }
    None
}

pub fn exec_decrypt(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: key, ciphertext (top)
    if vm.stack.len() >= 2 {
        let cipher_val = vm.stack.pop().unwrap();
        let key_val = vm.stack.pop().unwrap();

        if let (Value::Int(key), Value::Str(hex_str)) = (key_val, cipher_val) {
            let mut lcg = Lcg::new(key as u64);
            let mut plaintext_bytes = Vec::new();
            let mut error = false;

            // Decode hex and decrypt
            for i in (0..hex_str.len()).step_by(2) {
                if i + 2 <= hex_str.len() {
                    if let Ok(byte) = u8::from_str_radix(&hex_str[i..i + 2], 16) {
                        plaintext_bytes.push(byte ^ lcg.next_byte());
                    } else {
                        error = true;
                        break;
                    }
                }
            }

            if !error {
                if let Ok(plaintext) = String::from_utf8(plaintext_bytes) {
                    // Try to compile
                    match ChimeraParser::parse(Rule::strand, &plaintext) {
                        Ok(mut pairs) => {
                            let pair = pairs.next().unwrap();
                            match Strand::try_from_pair(pair) {
                                Ok(strand) => {
                                    vm.dna.helix.strands.push(strand);
                                    #[cfg(feature = "nova")]
                                    {
                                        vm.telomeres.push(50);
                                    }
                                    #[cfg(feature = "cortex")]
                                    {
                                        vm.activation_levels.push(0);
                                        vm.synapse_map.push(Vec::new());
                                    }

                                    let new_idx = vm.dna.helix.strands.len() - 1;

                                    #[cfg(feature = "nova")]
                                    vm.cladistics.register_strand(
                                        new_idx,
                                        Some(vm.ip.0),
                                        vm.tick_counter,
                                        "Decrypt".to_string(),
                                    );

                                    vm.stack.push(Value::Int(new_idx as i64));
                                    vm.energy = vm.energy.saturating_sub(20);
                                    vm.output.push(format!("DECRYPT: Success -> Strand {}", new_idx));
                                }
                                Err(e) => {
                                    vm.output.push(format!("DECRYPT ERROR: Compile failed: {}", e));
                                    vm.stack.push(Value::Int(-1));
                                }
                            }
                        }
                        Err(e) => {
                            vm.output.push(format!("DECRYPT ERROR: Parse failed: {}", e));
                            vm.stack.push(Value::Int(-1));
                        }
                    }
                } else {
                    vm.output.push("DECRYPT ERROR: Invalid UTF-8".to_string());
                    vm.stack.push(Value::Int(-1));
                }
            } else {
                vm.output.push("DECRYPT ERROR: Invalid Hex".to_string());
                vm.stack.push(Value::Int(-1));
            }
        } else {
            vm.output.push("Error: Type mismatch for decrypt".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for decrypt".to_string());
    }
    None
}

pub fn exec_sign(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: strand_idx, key (top)
    if vm.stack.len() >= 2 {
        let key_val = vm.stack.pop().unwrap();
        let s_val = vm.stack.pop().unwrap();

        if let (Value::Int(key), Value::Int(idx)) = (key_val, s_val) {
            let s_idx = idx as usize;
            if s_idx < vm.dna.helix.strands.len() {
                let strand = &vm.dna.helix.strands[s_idx];
                let mut hasher = DefaultHasher::new();
                strand.hash(&mut hasher);
                key.hash(&mut hasher);
                let signature = hasher.finish();

                vm.stack.push(Value::Int(signature as i64));
                vm.energy = vm.energy.saturating_sub(5);
                vm.output.push(format!("SIGN: Signed strand {}", s_idx));
            } else {
                vm.output.push("Error: Strand index out of bounds".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for sign".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for sign".to_string());
    }
    None
}

pub fn exec_verify_sig(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: strand_idx, signature, key (top)
    if vm.stack.len() >= 3 {
        let key_val = vm.stack.pop().unwrap();
        let sig_val = vm.stack.pop().unwrap();
        let s_val = vm.stack.pop().unwrap();

        if let (Value::Int(key), Value::Int(sig), Value::Int(idx)) = (key_val, sig_val, s_val) {
            let s_idx = idx as usize;
            if s_idx < vm.dna.helix.strands.len() {
                let strand = &vm.dna.helix.strands[s_idx];
                let mut hasher = DefaultHasher::new();
                strand.hash(&mut hasher);
                key.hash(&mut hasher);
                let expected = hasher.finish();

                let valid = (expected as i64) == sig;
                vm.stack.push(Value::Int(if valid { 1 } else { 0 }));
                vm.energy = vm.energy.saturating_sub(5);
                if valid {
                    vm.output.push("VERIFY: Signature Valid".to_string());
                } else {
                    vm.output.push("VERIFY: Signature Invalid".to_string());
                }
            } else {
                vm.output.push("Error: Strand index out of bounds".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for verify_sig".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for verify_sig".to_string());
    }
    None
}

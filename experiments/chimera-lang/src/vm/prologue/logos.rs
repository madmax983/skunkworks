use super::normalize_coords;
use crate::ast::{Dna, Nucleotide};
use crate::opcode::OpCode;
use crate::vm::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrammarRule {
    Literal(String),
    Sequence(Vec<GrammarRule>),
    Choice(Vec<GrammarRule>),
    Reference(String),
    WeightedChoice(Vec<(u32, GrammarRule)>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogosEngine {
    pub rules: HashMap<String, GrammarRule>,
}

impl LogosEngine {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn define_rule(&mut self, name: &str, definition: &str) {
        if let Ok(rule) = self.parse_rule_def(definition) {
            self.rules.insert(name.to_string(), rule);
        }
    }

    pub fn define_rule_from_dna(&mut self, name: &str, dna: &Dna, strand_idx: usize) {
        if let Ok(rule) = self.dna_to_grammar(dna, strand_idx) {
            self.rules.insert(name.to_string(), rule);
        }
    }

    fn dna_to_grammar(&self, dna: &Dna, strand_idx: usize) -> Result<GrammarRule, String> {
        if strand_idx >= dna.helix.strands.len() {
            return Err("Invalid strand index".to_string());
        }

        let strand = &dna.helix.strands[strand_idx];
        let mut rules = Vec::new();

        for gene in &strand.genes {
            let rule = match &gene.op {
                OpCode::Push => {
                    if let Some(arg) = gene.args.first() {
                        match arg {
                            Nucleotide::String(s) => GrammarRule::Literal(s.clone()),
                            Nucleotide::Number(n) => GrammarRule::Literal(n.to_string()),
                            _ => GrammarRule::Literal("".to_string()),
                        }
                    } else {
                        GrammarRule::Literal("".to_string())
                    }
                }
                OpCode::Call | OpCode::Exec => {
                    if let Some(Nucleotide::Number(idx)) = gene.args.first() {
                        // Naming convention: "strand_N"
                        GrammarRule::Reference(format!("strand_{}", idx))
                    } else {
                        GrammarRule::Literal("?".to_string())
                    }
                }
                OpCode::Adhere => {
                    // Placeholder for future sequence separator logic
                    GrammarRule::Literal("".to_string())
                }
                OpCode::Divergence => {
                    // Placeholder for future choice logic
                    GrammarRule::Literal(gene.op.to_string())
                }
                _ => GrammarRule::Literal(gene.op.to_string()),
            };

            // Filter out empty literals if any
            if let GrammarRule::Literal(s) = &rule {
                if s.is_empty() { continue; }
            }
            rules.push(rule);
        }

        if rules.is_empty() {
            Ok(GrammarRule::Literal("".to_string()))
        } else if rules.len() == 1 {
            Ok(rules[0].clone())
        } else {
            Ok(GrammarRule::Sequence(rules))
        }
    }

    fn parse_rule_def(&self, def: &str) -> Result<GrammarRule, String> {
        // Simple weighted check: "10:A | 1:B"
        if def.contains('|') {
            let parts: Vec<&str> = def.split('|').map(|s| s.trim()).collect();
            let mut weighted = false;
            let mut weights = Vec::new();
            let mut choices = Vec::new();

            for p in &parts {
                if let Some(idx) = p.find(':') {
                    if let Ok(w) = p[..idx].trim().parse::<u32>() {
                        weighted = true;
                        weights.push(w);
                        choices.push(p[idx+1..].trim());
                    } else {
                        choices.push(*p);
                        weights.push(1);
                    }
                } else {
                    choices.push(*p);
                    weights.push(1);
                }
            }

            if weighted {
                let mut rules = Vec::new();
                for (w, c) in weights.into_iter().zip(choices.into_iter()) {
                    rules.push((w, self.parse_rule_def(c)?));
                }
                return Ok(GrammarRule::WeightedChoice(rules));
            } else {
                let mut rules = Vec::new();
                for c in choices {
                    rules.push(self.parse_rule_def(c)?);
                }
                return Ok(GrammarRule::Choice(rules));
            }
        }

        let seq: Vec<&str> = def.split_whitespace().collect();
        if seq.len() > 1 {
            let mut rules = Vec::new();
            for s in seq {
                rules.push(self.parse_single_token(s)?);
            }
            return Ok(GrammarRule::Sequence(rules));
        }

        if !def.is_empty() {
            self.parse_single_token(def)
        } else {
             Err("Empty rule definition".to_string())
        }
    }

    fn parse_single_token(&self, token: &str) -> Result<GrammarRule, String> {
        if token.starts_with('"') && token.ends_with('"') && token.len() >= 2 {
            Ok(GrammarRule::Literal(token[1..token.len()-1].to_string()))
        } else {
            Ok(GrammarRule::Reference(token.to_string()))
        }
    }

    pub fn generate(&self, rule_name: &str) -> Result<String, String> {
        if let Some(rule) = self.rules.get(rule_name) {
            self.generate_from_rule(rule, 0)
        } else {
            Err(format!("Rule '{}' not found", rule_name))
        }
    }

    fn generate_from_rule(&self, rule: &GrammarRule, depth: usize) -> Result<String, String> {
        if depth > 100 {
            return Err("Recursion limit exceeded".to_string());
        }

        match rule {
            GrammarRule::Literal(s) => Ok(s.clone()),
            GrammarRule::Sequence(rules) => {
                let mut result = String::new();
                for r in rules {
                    let s = self.generate_from_rule(r, depth + 1)?;
                    if !s.is_empty() {
                        if !result.is_empty() && !result.ends_with(' ') { result.push(' '); }
                        result.push_str(&s);
                    }
                }
                Ok(result)
            }
            GrammarRule::Choice(rules) => {
                use rand::seq::SliceRandom;
                let mut rng = rand::thread_rng();
                if let Some(r) = rules.choose(&mut rng) {
                    self.generate_from_rule(r, depth + 1)
                } else {
                    Err("Empty choice".to_string())
                }
            }
            GrammarRule::WeightedChoice(choices) => {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let total_weight: u32 = choices.iter().map(|(w, _)| w).sum();
                if total_weight == 0 { return Err("Total weight is 0".to_string()); }

                let mut pick = rng.gen_range(0..total_weight);
                for (w, r) in choices {
                    if pick < *w {
                        return self.generate_from_rule(r, depth + 1);
                    }
                    pick -= w;
                }
                // Should not reach here
                self.generate_from_rule(&choices.last().unwrap().1, depth + 1)
            }
            GrammarRule::Reference(name) => {
                if let Some(r) = self.rules.get(name) {
                    self.generate_from_rule(r, depth + 1)
                } else {
                    Err(format!("Rule '{}' not found", name))
                }
            }
        }
    }

    pub fn parse_input(&self, rule_name: &str, input: &str) -> Result<Value, String> {
        if let Some(rule) = self.rules.get(rule_name) {
            let tokens: Vec<&str> = input.split_whitespace().collect();
            // We need a helper that takes tokens and position
            let (val, consumed) = self.parse_from_rule(rule, &tokens, 0, 0)?;
            if consumed == tokens.len() {
                Ok(val)
            } else {
                Err(format!("Incomplete parse. Consumed {} of {} tokens.", consumed, tokens.len()))
            }
        } else {
             Err(format!("Rule '{}' not found", rule_name))
        }
    }

    fn parse_from_rule(&self, rule: &GrammarRule, tokens: &[&str], pos: usize, depth: usize) -> Result<(Value, usize), String> {
        if depth > 100 {
            return Err("Recursion limit exceeded during parse".to_string());
        }
        match rule {
            GrammarRule::Literal(s) => {
                if pos < tokens.len() && tokens[pos] == s {
                    Ok((Value::Str(s.clone()), pos + 1))
                } else {
                    Err(format!("Expected '{}', found '{:?}'", s, tokens.get(pos)))
                }
            }
            GrammarRule::Sequence(rules) => {
                let mut current_pos = pos;
                let mut results = Vec::new();
                for r in rules {
                    let (val, next_pos) = self.parse_from_rule(r, tokens, current_pos, depth + 1)?;
                    results.push(val);
                    current_pos = next_pos;
                }
                Ok((Value::Junction(crate::ast::JunctionType::All, results), current_pos))
            }
            GrammarRule::Choice(rules) => {
                for r in rules {
                    if let Ok((val, next_pos)) = self.parse_from_rule(r, tokens, pos, depth + 1) {
                        return Ok((val, next_pos));
                    }
                }
                Err("No choice matched".to_string())
            }
            GrammarRule::WeightedChoice(choices) => {
                // For parsing, ignore weights, try all
                for (_, r) in choices {
                    if let Ok((val, next_pos)) = self.parse_from_rule(r, tokens, pos, depth + 1) {
                        return Ok((val, next_pos));
                    }
                }
                Err("No choice matched".to_string())
            }
            GrammarRule::Reference(name) => {
                if let Some(r) = self.rules.get(name) {
                    self.parse_from_rule(r, tokens, pos, depth + 1)
                } else {
                    Err(format!("Rule '{}' not found", name))
                }
            }
        }
    }
}

pub fn apply_logos_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    logos_engine: &mut LogosEngine,
    dna: &Dna,
) -> bool {
    let mut changes = false;

    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].as_ref()
    } else {
        None
    };

    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        current_signals[ny][nx].as_ref()
    } else {
        None
    };

    match rune {
        "Γ" => {
            // Gamma: Define Rule
            // West: Name (String)
            // North: Definition (String) OR Strand Index (Int)
            if let Some(Value::Str(name)) = w_sig {
                if let Some(val) = n_sig {
                    match val {
                        Value::Str(def) => {
                            logos_engine.define_rule(name, def);
                            if next_signals[y][x].is_none() {
                                next_signals[y][x] = Some(Value::Int(1));
                                changes = true;
                            }
                        },
                        Value::Int(idx) => {
                            if *idx >= 0 {
                                logos_engine.define_rule_from_dna(name, dna, *idx as usize);
                                if next_signals[y][x].is_none() {
                                    next_signals[y][x] = Some(Value::Int(1));
                                    changes = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        "»" => {
            // Speak / Generate
            // West: Rule Name
            // North: Mode ("GRID") -> Used to signal intent, output is still string.
            // Output: Self (Generated String)
            if let Some(Value::Str(name)) = w_sig {
                if let Ok(gen) = logos_engine.generate(name) {

                    // Note: If North is "GRID", the intent is to write to the grid.
                    // However, we are in the propagation phase and cannot modify the grid directly.
                    // We output the string, and a downstream Sink (like `$` or specialized Logic)
                    // would need to consume it. Or the user can pipe it.
                    // For now, we just respect the signal generation.

                    let res = Value::Str(gen);
                    if next_signals[y][x] != Some(res.clone()) {
                        next_signals[y][x] = Some(res);
                        changes = true;
                    }
                }
            }
        }
        "«" => {
            // Parse / Perceive
            // West: Input String
            // North: Rule Name
            // Output: Self (AST / Junction)
            if let (Some(Value::Str(input)), Some(Value::Str(name))) = (w_sig, n_sig) {
                if let Ok(ast) = logos_engine.parse_input(name, input) {
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(ast);
                        changes = true;
                    }
                }
            }
        }
        _ => {}
    }

    changes
}

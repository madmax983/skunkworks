use super::normalize_coords;
use crate::vm::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrammarRule {
    Literal(String),
    Sequence(Vec<GrammarRule>),
    Choice(Vec<GrammarRule>),
    Reference(String),
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

    fn parse_rule_def(&self, def: &str) -> Result<GrammarRule, String> {
        let choices: Vec<&str> = def.split('|').map(|s| s.trim()).collect();
        if choices.len() > 1 {
            let mut rules = Vec::new();
            for c in choices {
                rules.push(self.parse_rule_def(c)?);
            }
            return Ok(GrammarRule::Choice(rules));
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
                for (i, r) in rules.iter().enumerate() {
                    if i > 0 { result.push(' '); }
                    result.push_str(&self.generate_from_rule(r, depth + 1)?);
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
            // North: Definition (String)
            if let (Some(Value::Str(name)), Some(Value::Str(def))) = (w_sig, n_sig) {
                logos_engine.define_rule(name, def);
                // Ack
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            }
        }
        "»" => {
            // Speak / Generate
            // West: Rule Name
            // Output: Self (Generated String)
            if let Some(Value::Str(name)) = w_sig {
                // If North has Seed/Arg?
                if let Ok(gen) = logos_engine.generate(name) {
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
                    if next_signals[y][x] != Some(ast.clone()) {
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

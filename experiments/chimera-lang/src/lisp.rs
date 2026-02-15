use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use anyhow::{anyhow, Result};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum SExpr {
    Atom(String),
    List(Vec<SExpr>),
}

pub fn parse(input: &str) -> Result<Vec<SExpr>> {
    let tokens = tokenize(input);
    let mut exprs = Vec::new();
    let mut idx = 0;
    while idx < tokens.len() {
        let (expr, next_idx) = parse_expr(&tokens, idx)?;
        exprs.push(expr);
        idx = next_idx;
    }
    Ok(exprs)
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if in_string {
            if c == '"' {
                in_string = false;
                current.push(c);
                tokens.push(current.clone());
                current.clear();
            } else {
                current.push(c);
            }
        } else {
            match c {
                '(' | ')' => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    tokens.push(c.to_string());
                }
                '"' => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    in_string = true;
                    current.push(c);
                }
                ';' => {
                    // Comment until newline
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    while let Some(&next) = chars.peek() {
                        if next == '\n' {
                            break;
                        }
                        chars.next();
                    }
                }
                c if c.is_whitespace() => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                }
                _ => current.push(c),
            }
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

fn parse_expr(tokens: &[String], start: usize) -> Result<(SExpr, usize)> {
    if start >= tokens.len() {
        return Err(anyhow!("Unexpected EOF"));
    }
    let token = &tokens[start];
    if token == "(" {
        let mut list = Vec::new();
        let mut idx = start + 1;
        while idx < tokens.len() && tokens[idx] != ")" {
            let (expr, next_idx) = parse_expr(tokens, idx)?;
            list.push(expr);
            idx = next_idx;
        }
        if idx >= tokens.len() {
            return Err(anyhow!("Unclosed list"));
        }
        Ok((SExpr::List(list), idx + 1))
    } else if token == ")" {
        Err(anyhow!("Unexpected )"))
    } else {
        Ok((SExpr::Atom(token.clone()), start + 1))
    }
}

pub fn compile(source: &str) -> Result<Dna> {
    let exprs = parse(source)?;
    let mut strands = Vec::new();

    for expr in exprs {
        match expr {
            SExpr::List(items) => {
                if let Some(SExpr::Atom(head)) = items.first() {
                    if head == "strand" {
                        // (strand name body...)
                        if items.len() < 2 {
                            return Err(anyhow!("Invalid strand definition"));
                        }
                        // name is items[1] (not used in DNA struct directly, but maybe for labels?)
                        // Currently DNA just has a list of strands.
                        // The compiler tracks names.
                        // Here we just append.
                        let mut genes = Vec::new();
                        for item in items.iter().skip(2) {
                            genes.extend(compile_expr(item)?);
                        }
                        strands.push(Strand { genes });
                    } else {
                        // Assume implicit main if only one strand?
                        // Or allow top-level genes to form a "main" strand?
                        // Let's enforce (strand ...) for now.
                        return Err(anyhow!("Top level must be (strand ...)"));
                    }
                }
            }
            _ => return Err(anyhow!("Top level must be a list")),
        }
    }

    Ok(Dna {
        helix: Helix { strands },
    })
}

fn map_op(s: &str) -> Option<OpCode> {
    match s {
        "+" => Some(OpCode::Add),
        "-" => Some(OpCode::Sub),
        "*" => Some(OpCode::Mul),
        "/" => Some(OpCode::Div),
        "%" => None, // OpCode::Mod?, no mod?
        "print" => Some(OpCode::Print),
        "dup" => Some(OpCode::Dup),
        "drop" => Some(OpCode::Drop),
        "swap" => Some(OpCode::Swap),
        "push" => Some(OpCode::Push),
        "jump" => Some(OpCode::Jump),
        "brz" => Some(OpCode::Brz),
        "call" => Some(OpCode::Call),
        "spawn" => Some(OpCode::Spawn),
        "consume" => Some(OpCode::Consume),
        "photosynthesize" => Some(OpCode::Photosynthesize),
        "mutate" => Some(OpCode::HavocRate), // Hack? No, HavocRate sets rate.
        _ => OpCode::from_str(s).ok().or_else(|| {
            // Case insensitive?
            if let Some(first) = s.chars().next() {
                let title = first.to_uppercase().to_string() + &s[1..];
                OpCode::from_str(&title).ok()
            } else {
                None
            }
        }),
    }
}

fn is_immediate(op: &OpCode) -> bool {
    matches!(op,
        OpCode::Push | OpCode::Jump | OpCode::Brz | OpCode::Call |
        OpCode::Spawn | OpCode::Radiate | OpCode::Siphon | OpCode::Virus |
        OpCode::Lumine | OpCode::Transcribe | OpCode::HavocRate | OpCode::HavocScope
    )
}

fn compile_expr(expr: &SExpr) -> Result<Vec<Gene>> {
    match expr {
        SExpr::Atom(s) => {
            if let Ok(n) = s.parse::<i64>() {
                Ok(vec![Gene { op: OpCode::Push, args: vec![Nucleotide::Number(n)] }])
            } else if s.starts_with('"') && s.ends_with('"') {
                let content = &s[1..s.len()-1];
                Ok(vec![Gene { op: OpCode::Push, args: vec![Nucleotide::String(content.to_string())] }])
            } else {
                // Identifier
                // If it maps to an OpCode, it's a bare OpCode (no args, or stack args)
                if let Some(op) = map_op(s) {
                    Ok(vec![Gene { op, args: vec![] }])
                } else {
                    // Treat as string push (Identifier)
                    Ok(vec![Gene { op: OpCode::Push, args: vec![Nucleotide::String(s.clone())] }])
                }
            }
        }
        SExpr::List(items) => {
            if items.is_empty() { return Ok(vec![]); }

            if let SExpr::Atom(head) = &items[0] {
                if let Some(op) = map_op(head) {
                    if is_immediate(&op) {
                        // (op arg1 arg2) -> Gene { op, args: [arg1, arg2] }
                        let mut args = Vec::new();
                        for item in items.iter().skip(1) {
                            match item {
                                SExpr::Atom(s) => {
                                    if let Ok(n) = s.parse::<i64>() {
                                        args.push(Nucleotide::Number(n));
                                    } else if s.starts_with('"') {
                                         args.push(Nucleotide::String(s[1..s.len()-1].to_string()));
                                    } else {
                                         // Identifier in arg position -> String/Identifier
                                         args.push(Nucleotide::Identifier(s.clone()));
                                    }
                                }
                                SExpr::List(_) => return Err(anyhow!("Nested expression in immediate op args not supported")),
                            }
                        }
                        return Ok(vec![Gene { op, args }]);
                    } else {
                        // Stack Op: (add 1 2) -> 1 2 add
                        let mut genes = Vec::new();
                        for arg in items.iter().skip(1) {
                            genes.extend(compile_expr(arg)?);
                        }
                        genes.push(Gene { op, args: vec![] });
                        return Ok(genes);
                    }
                }
            }

            // Just a sequence: ((push 1) (print))
            let mut genes = Vec::new();
            for item in items {
                genes.extend(compile_expr(item)?);
            }
            Ok(genes)
        }
    }
}

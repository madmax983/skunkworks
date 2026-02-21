use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::Value;
use anyhow::{anyhow, Result};
use std::str::FromStr;

const MAX_LISP_DEPTH: usize = 256;

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
        let (expr, next_idx) = parse_expr(&tokens, idx, 0)?;
        exprs.push(expr);
        idx = next_idx;
    }
    Ok(exprs)
}

fn sexpr_to_value_inner(expr: &SExpr, depth: usize) -> Result<Value> {
    if depth > MAX_LISP_DEPTH {
        return Err(anyhow!("Recursion limit exceeded"));
    }
    match expr {
        SExpr::Atom(s) => {
            if let Ok(n) = s.parse::<i64>() {
                Ok(Value::Int(n))
            } else if s.starts_with('"') && s.ends_with('"') {
                Ok(Value::Str(s[1..s.len() - 1].to_string()))
            } else {
                Ok(Value::Str(s.clone()))
            }
        }
        SExpr::List(items) => {
            let mut vals = Vec::new();
            for item in items {
                vals.push(sexpr_to_value_inner(item, depth + 1)?);
            }
            Ok(Value::Junction(JunctionType::Any, vals))
        }
    }
}

pub fn sexpr_to_value(expr: &SExpr) -> Result<Value> {
    sexpr_to_value_inner(expr, 0)
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

fn parse_expr(tokens: &[String], start: usize, depth: usize) -> Result<(SExpr, usize)> {
    if depth > MAX_LISP_DEPTH {
        return Err(anyhow!("Recursion limit exceeded"));
    }
    if start >= tokens.len() {
        return Err(anyhow!("Unexpected EOF"));
    }
    let token = &tokens[start];
    if token == "(" {
        let mut list = Vec::new();
        let mut idx = start + 1;
        while idx < tokens.len() && tokens[idx] != ")" {
            let (expr, next_idx) = parse_expr(tokens, idx, depth + 1)?;
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
                        // name is items[1]
                        let mut genes = Vec::new();
                        for item in items.iter().skip(2) {
                            genes.extend(compile_expr(item, 0)?);
                        }
                        strands.push(Strand { genes });
                    } else {
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

pub fn compile_fragment(source: &str) -> Result<Vec<Gene>> {
    let exprs = parse(source)?;
    let mut genes = Vec::new();
    for expr in exprs {
        genes.extend(compile_expr(&expr, 0)?);
    }
    Ok(genes)
}

fn map_op(s: &str) -> Option<OpCode> {
    match s {
        "+" => Some(OpCode::Add),
        "-" => Some(OpCode::Sub),
        "*" => Some(OpCode::Mul),
        "/" => Some(OpCode::Div),
        "print" => Some(OpCode::Print),
        "dup" => Some(OpCode::Dup),
        "drop" => Some(OpCode::Drop),
        "swap" => Some(OpCode::Swap),
        "push" => Some(OpCode::Push),
        "jump" => Some(OpCode::Jump),
        "brz" => Some(OpCode::Brz),
        #[cfg(feature = "nova")]
        "call" => Some(OpCode::Call),
        #[cfg(feature = "nova")]
        "spawn" => Some(OpCode::Spawn),
        "consume" => Some(OpCode::Consume),
        "photosynthesize" => Some(OpCode::Photosynthesize),
        "mutate" => Some(OpCode::HavocRate),
        // Nova Mappings
        #[cfg(feature = "nova")]
        "entangle" => Some(OpCode::Entangle),
        #[cfg(feature = "nova")]
        "decohere" => Some(OpCode::Decohere),
        #[cfg(feature = "nova")]
        "time-warp" => Some(OpCode::TimeWarp),
        #[cfg(feature = "nova")]
        "glitch" => Some(OpCode::Glitch),
        #[cfg(feature = "nova")]
        "resonate" => Some(OpCode::Resonate),
        "transcribe" => Some(OpCode::Transcribe), // Core
        #[cfg(feature = "nova")]
        "methylate" => Some(OpCode::Methylate),
        #[cfg(feature = "nova")]
        "demethylate" => Some(OpCode::Demethylate),
        #[cfg(feature = "nova")]
        "mitosis" => Some(OpCode::Mitosis),
        #[cfg(feature = "nova")]
        "apoptosis" => Some(OpCode::Apoptosis),
        "g-read" => Some(OpCode::GRead),
        "g-write" => Some(OpCode::GWrite),
        "radiate" => Some(OpCode::Radiate),
        "siphon" => Some(OpCode::Siphon),
        "virus" => Some(OpCode::Virus),
        #[cfg(feature = "nova")]
        "warp" => Some(OpCode::TimeWarp),
        #[cfg(feature = "nova")]
        "shape" => Some(OpCode::Shape),
        #[cfg(feature = "nova")]
        "void" => Some(OpCode::Void),
        #[cfg(feature = "nova")]
        "rift" => Some(OpCode::Rift),
        #[cfg(feature = "nova")]
        "seal" => Some(OpCode::Seal),
        #[cfg(feature = "nova")]
        "scavenge" => Some(OpCode::Scavenge),
        #[cfg(feature = "nova")]
        "digest" => Some(OpCode::Digest),
        #[cfg(feature = "nova")]
        "compile" => Some(OpCode::Compile),
        #[cfg(feature = "nova")]
        "decompile" => Some(OpCode::Decompile),
        #[cfg(feature = "nova")]
        "lisp-eval" => Some(OpCode::LispEval),
        #[cfg(feature = "nova")]
        "babel-live" => Some(OpCode::BabelLive),
        #[cfg(feature = "nova")]
        "bio-hack" => Some(OpCode::BioHack),
        _ => OpCode::from_str(s).ok().or_else(|| {
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
    // Only return true if the VM expects arguments in the Gene struct.
    // Most ops pop arguments from the stack.
    match op {
        OpCode::Push | OpCode::Jump | OpCode::Brz => true,
        #[cfg(feature = "nova")]
        OpCode::Call => true,
        _ => false,
    }
}

fn compile_as_data(expr: &SExpr, depth: usize) -> Result<Nucleotide> {
    if depth > MAX_LISP_DEPTH {
        return Err(anyhow!("Recursion limit exceeded"));
    }
    match expr {
        SExpr::Atom(s) => {
            if let Ok(n) = s.parse::<i64>() {
                Ok(Nucleotide::Number(n))
            } else if s.starts_with('"') && s.ends_with('"') {
                Ok(Nucleotide::String(s[1..s.len() - 1].to_string()))
            } else {
                Ok(Nucleotide::Identifier(s.clone()))
            }
        }
        SExpr::List(items) => {
            let mut nucleos = Vec::new();
            for item in items {
                nucleos.push(compile_as_data(item, depth + 1)?);
            }
            Ok(Nucleotide::Junction(JunctionType::Any, nucleos))
        }
    }
}

fn compile_expr(expr: &SExpr, depth: usize) -> Result<Vec<Gene>> {
    if depth > MAX_LISP_DEPTH {
        return Err(anyhow!("Recursion limit exceeded"));
    }
    match expr {
        SExpr::Atom(s) => {
            if let Ok(n) = s.parse::<i64>() {
                Ok(vec![Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(n)],
                }])
            } else if s.starts_with('"') && s.ends_with('"') {
                let content = &s[1..s.len() - 1];
                Ok(vec![Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::String(content.to_string())],
                }])
            } else {
                if let Some(op) = map_op(s) {
                    Ok(vec![Gene { op, args: vec![] }])
                } else {
                    Ok(vec![Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::String(s.clone())],
                    }])
                }
            }
        }
        SExpr::List(items) => {
            if items.is_empty() {
                return Ok(vec![]);
            }

            if let SExpr::Atom(head) = &items[0] {
                match head.as_str() {
                    #[cfg(feature = "nova")]
                    "quote" => {
                        if items.len() != 2 {
                            return Err(anyhow!("quote requires exactly one argument"));
                        }
                        let data = compile_as_data(&items[1], depth + 1)?;
                        return Ok(vec![Gene {
                            op: OpCode::Push,
                            args: vec![data],
                        }]);
                    }
                    #[cfg(feature = "oracle")]
                    "rule" => {
                        if items.len() < 2 {
                            return Err(anyhow!("rule requires at least a head"));
                        }
                        let head = compile_as_data(&items[1], depth + 1)?;

                        let mut body_terms = Vec::new();
                        for item in items.iter().skip(2) {
                            body_terms.push(compile_as_data(item, depth + 1)?);
                        }
                        let body = Nucleotide::Junction(JunctionType::All, body_terms);

                        // Push head, then body (Rule op expects [..., head, body])
                        return Ok(vec![
                            Gene {
                                op: OpCode::Push,
                                args: vec![head],
                            },
                            Gene {
                                op: OpCode::Push,
                                args: vec![body],
                            },
                            Gene {
                                op: OpCode::Rule,
                                args: vec![],
                            },
                        ]);
                    }
                    #[cfg(feature = "oracle")]
                    "assert" => {
                        if items.len() != 2 {
                            return Err(anyhow!("assert requires exactly one argument"));
                        }
                        let fact = compile_as_data(&items[1], depth + 1)?;
                        return Ok(vec![
                            Gene {
                                op: OpCode::Push,
                                args: vec![fact],
                            },
                            Gene {
                                op: OpCode::Assert,
                                args: vec![],
                            },
                        ]);
                    }
                    #[cfg(feature = "oracle")]
                    "retract" => {
                        if items.len() != 2 {
                            return Err(anyhow!("retract requires exactly one argument"));
                        }
                        let fact = compile_as_data(&items[1], depth + 1)?;
                        return Ok(vec![
                            Gene {
                                op: OpCode::Push,
                                args: vec![fact],
                            },
                            Gene {
                                op: OpCode::Retract,
                                args: vec![],
                            },
                        ]);
                    }
                    #[cfg(feature = "oracle")]
                    "query" => {
                        if items.len() < 2 {
                            return Err(anyhow!("query requires at least one goal"));
                        }
                        // (query goal1 goal2...) -> implicit AND (All)
                        let goal = if items.len() == 2 {
                            compile_as_data(&items[1], depth + 1)?
                        } else {
                            let mut goals = Vec::new();
                            for item in items.iter().skip(1) {
                                goals.push(compile_as_data(item, depth + 1)?);
                            }
                            Nucleotide::Junction(JunctionType::All, goals)
                        };
                        return Ok(vec![
                            Gene {
                                op: OpCode::Push,
                                args: vec![goal],
                            },
                            Gene {
                                op: OpCode::Query,
                                args: vec![],
                            },
                        ]);
                    }
                    #[cfg(feature = "nova")]
                    "seq" => {
                        let mut genes = Vec::new();
                        let count = items.len() - 1;
                        if count == 0 {
                            return Err(anyhow!("seq requires at least one argument"));
                        }
                        for item in items.iter().skip(1) {
                            genes.extend(compile_expr(item, depth + 1)?);
                        }
                        genes.push(Gene {
                            op: OpCode::Push,
                            args: vec![Nucleotide::Number(count as i64)],
                        });
                        genes.push(Gene {
                            op: OpCode::ParserSeqN,
                            args: vec![],
                        });
                        return Ok(genes);
                    }
                    #[cfg(feature = "nova")]
                    "alt" => {
                        let mut genes = Vec::new();
                        let count = items.len() - 1;
                        if count == 0 {
                            return Err(anyhow!("alt requires at least one argument"));
                        }
                        for item in items.iter().skip(1) {
                            genes.extend(compile_expr(item, depth + 1)?);
                        }
                        genes.push(Gene {
                            op: OpCode::Push,
                            args: vec![Nucleotide::Number(count as i64)],
                        });
                        genes.push(Gene {
                            op: OpCode::ParserAltN,
                            args: vec![],
                        });
                        return Ok(genes);
                    }
                    #[cfg(feature = "nova")]
                    "match" => {
                        if items.len() != 2 {
                            return Err(anyhow!("match requires exactly one argument"));
                        }
                        let mut genes = compile_expr(&items[1], depth + 1)?;
                        genes.push(Gene {
                            op: OpCode::ParserMatch,
                            args: vec![],
                        });
                        return Ok(genes);
                    }
                    #[cfg(feature = "nova")]
                    "regex" => {
                        if items.len() != 2 {
                            return Err(anyhow!("regex requires exactly one argument"));
                        }
                        let mut genes = compile_expr(&items[1], depth + 1)?;
                        genes.push(Gene {
                            op: OpCode::ParserRegex,
                            args: vec![],
                        });
                        return Ok(genes);
                    }
                    #[cfg(feature = "nova")]
                    "many" => {
                        if items.len() != 2 {
                            return Err(anyhow!("many requires exactly one argument"));
                        }
                        let mut genes = compile_expr(&items[1], depth + 1)?;
                        genes.push(Gene {
                            op: OpCode::ParserMany,
                            args: vec![],
                        });
                        return Ok(genes);
                    }
                    #[cfg(feature = "nova")]
                    "opt" => {
                        if items.len() != 2 {
                            return Err(anyhow!("opt requires exactly one argument"));
                        }
                        let mut genes = compile_expr(&items[1], depth + 1)?;
                        genes.push(Gene {
                            op: OpCode::ParserOpt,
                            args: vec![],
                        });
                        return Ok(genes);
                    }
                    #[cfg(feature = "nova")]
                    "parse" => {
                        if items.len() != 3 {
                            return Err(anyhow!("parse requires (parse grammar input)"));
                        }
                        let mut genes = Vec::new();
                        genes.extend(compile_expr(&items[1], depth + 1)?);
                        genes.extend(compile_expr(&items[2], depth + 1)?);
                        genes.push(Gene {
                            op: OpCode::Parse,
                            args: vec![],
                        });
                        return Ok(genes);
                    }
                    #[cfg(feature = "nova")]
                    "generate" => {
                        if items.len() != 2 {
                            return Err(anyhow!("generate requires exactly one argument"));
                        }
                        let mut genes = compile_expr(&items[1], depth + 1)?;
                        genes.push(Gene {
                            op: OpCode::Generate,
                            args: vec![],
                        });
                        return Ok(genes);
                    }
                    _ => {}
                }

                if let Some(op) = map_op(head) {
                    if is_immediate(&op) {
                        let mut args = Vec::new();
                        for item in items.iter().skip(1) {
                            match item {
                                SExpr::Atom(s) => {
                                    if let Ok(n) = s.parse::<i64>() {
                                        args.push(Nucleotide::Number(n));
                                    } else if s.starts_with('"') {
                                        args.push(Nucleotide::String(
                                            s[1..s.len() - 1].to_string(),
                                        ));
                                    } else {
                                        args.push(Nucleotide::Identifier(s.clone()));
                                    }
                                }
                                SExpr::List(_) => {
                                    return Err(anyhow!(
                                        "Nested expression in immediate op args not supported"
                                    ))
                                }
                            }
                        }
                        return Ok(vec![Gene { op, args }]);
                    } else {
                        // Stack Op: (op arg1 arg2) -> arg1 arg2 op
                        let mut genes = Vec::new();
                        for arg in items.iter().skip(1) {
                            genes.extend(compile_expr(arg, depth + 1)?);
                        }
                        genes.push(Gene { op, args: vec![] });
                        return Ok(genes);
                    }
                }
            }

            let mut genes = Vec::new();
            for item in items {
                genes.extend(compile_expr(item, depth + 1)?);
            }
            Ok(genes)
        }
    }
}

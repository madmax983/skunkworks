#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};
use crate::ast::{JunctionType, Nucleotide};
use crate::opcode::OpCode;
use rand::Rng;

/// Executes Babel-related OpCodes.
pub fn exec_babel_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Generate => {
            if let Some(grammar) = vm.stack.pop() {
                let generated = generate_string(&grammar);
                vm.stack.push(Value::Str(generated));
            } else {
                vm.output
                    .push("Error: Stack underflow for Generate".to_string());
            }
        }
        OpCode::Scribe => {
            if let Some(val) = vm.stack.pop() {
                let s = match val {
                    Value::Str(s) => s,
                    _ => format!("{}", val),
                };
                vm.tablet.push(s.clone());
                vm.output.push(format!("SCRIBE: {}", s));
            } else {
                vm.output
                    .push("Error: Stack underflow for Scribe".to_string());
            }
        }
        OpCode::Grammar => {
            // Stack: [ ..., type_str, ...args ]
            if let Some(type_val) = vm.stack.pop() {
                if let Value::Str(type_str) = type_val {
                    let mut args = vec![Value::Str(type_str.clone())];
                    match type_str.as_str() {
                        "Match" | "Regex" => {
                            if let Some(pattern) = vm.stack.pop() {
                                args.push(pattern);
                                vm.stack.push(Value::Junction(JunctionType::Any, args));
                            } else {
                                vm.output.push(format!(
                                    "Error: Stack underflow for Grammar({})",
                                    type_str
                                ));
                            }
                        }
                        "Seq" | "Alt" => {
                            if vm.stack.len() >= 2 {
                                let p2 = vm.stack.pop().unwrap();
                                let p1 = vm.stack.pop().unwrap();
                                args.push(p1);
                                args.push(p2);
                                vm.stack.push(Value::Junction(JunctionType::Any, args));
                            } else {
                                vm.output.push(format!(
                                    "Error: Stack underflow for Grammar({})",
                                    type_str
                                ));
                            }
                        }
                        "Many" | "Opt" => {
                            if let Some(p) = vm.stack.pop() {
                                args.push(p);
                                vm.stack.push(Value::Junction(JunctionType::Any, args));
                            } else {
                                vm.output.push(format!(
                                    "Error: Stack underflow for Grammar({})",
                                    type_str
                                ));
                            }
                        }
                        "Mutate" => {
                            // Stack: [ ..., rate, grammar ] -> [ ..., mutated ]
                            // We already popped "Mutate" as type_str.
                            // The arguments popped earlier (args) was just [type_str].
                            // We need to pop real args from stack.
                            // Actually, exec_babel_op logic for Grammar op is:
                            // Pop type_str. Then switch on it to pop args.
                            if vm.stack.len() >= 2 {
                                let rate_val = vm.stack.pop().unwrap();
                                let grammar_val = vm.stack.pop().unwrap();

                                let rate = if let Value::Int(r) = rate_val {
                                    (r as f64) / 100.0
                                } else {
                                    0.1
                                };

                                let mutated = mutate_grammar(&grammar_val, rate);
                                vm.stack.push(mutated);
                            } else {
                                vm.output
                                    .push("Error: Stack underflow for Grammar(Mutate)".to_string());
                            }
                        }
                        _ => {
                            vm.output
                                .push(format!("Error: Unknown Grammar type '{}'", type_str));
                        }
                    }
                } else {
                    vm.output
                        .push("Error: Grammar type must be a string".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for Grammar".to_string());
            }
        }
        OpCode::Parse => {
            // Stack: [ ..., parser, input ]
            if vm.stack.len() >= 2 {
                let input_val = vm.stack.pop().unwrap();
                let parser_val = vm.stack.pop().unwrap();

                if let Value::Str(input_str) = input_val {
                    match run_parser(&parser_val, &input_str) {
                        Ok((ast, consumed)) => {
                            if consumed == input_str.len() {
                                vm.stack.push(ast);
                                vm.output.push("PARSE: Success".to_string());
                            } else {
                                vm.output
                                    .push(format!("PARSE: Partial match ({} chars)", consumed));
                                vm.stack.push(Value::Int(0)); // Failure indicator? Or partial AST? For now, failure.
                            }
                        }
                        Err(_) => {
                            vm.output.push("PARSE: Failed".to_string());
                            vm.stack.push(Value::Int(0));
                        }
                    }
                } else {
                    vm.output
                        .push("Error: Parse input must be a string".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for Parse".to_string());
            }
        }
        OpCode::ParserMatch => {
            if let Some(pattern) = vm.stack.pop() {
                vm.stack.push(Value::Junction(
                    JunctionType::Any,
                    vec![Value::Str("Match".to_string()), pattern],
                ));
            } else {
                vm.output
                    .push("Error: Stack underflow for ParserMatch".to_string());
            }
        }
        OpCode::ParserRegex => {
            if let Some(pattern) = vm.stack.pop() {
                vm.stack.push(Value::Junction(
                    JunctionType::Any,
                    vec![Value::Str("Regex".to_string()), pattern],
                ));
            } else {
                vm.output
                    .push("Error: Stack underflow for ParserRegex".to_string());
            }
        }
        OpCode::ParserSeq => {
            if vm.stack.len() >= 2 {
                let p2 = vm.stack.pop().unwrap();
                let p1 = vm.stack.pop().unwrap();
                vm.stack.push(Value::Junction(
                    JunctionType::Any,
                    vec![Value::Str("Seq".to_string()), p1, p2],
                ));
            } else {
                vm.output
                    .push("Error: Stack underflow for ParserSeq".to_string());
            }
        }
        OpCode::ParserAlt => {
            if vm.stack.len() >= 2 {
                let p2 = vm.stack.pop().unwrap();
                let p1 = vm.stack.pop().unwrap();
                vm.stack.push(Value::Junction(
                    JunctionType::Any,
                    vec![Value::Str("Alt".to_string()), p1, p2],
                ));
            } else {
                vm.output
                    .push("Error: Stack underflow for ParserAlt".to_string());
            }
        }
        OpCode::ParserMany => {
            if let Some(p) = vm.stack.pop() {
                vm.stack.push(Value::Junction(
                    JunctionType::Any,
                    vec![Value::Str("Many".to_string()), p],
                ));
            } else {
                vm.output
                    .push("Error: Stack underflow for ParserMany".to_string());
            }
        }
        OpCode::ParserOpt => {
            if let Some(p) = vm.stack.pop() {
                vm.stack.push(Value::Junction(
                    JunctionType::Any,
                    vec![Value::Str("Opt".to_string()), p],
                ));
            } else {
                vm.output
                    .push("Error: Stack underflow for ParserOpt".to_string());
            }
        }
        _ => {}
    }
    None
}

/// Runs a parser on an input string.
/// Returns Ok((AST, consumed_count)) or Err.
pub fn run_parser(parser: &Value, input: &str) -> Result<(Value, usize), ()> {
    if let Value::Junction(JunctionType::Any, args) = parser {
        if args.is_empty() {
            return Err(());
        }
        if let Value::Str(type_str) = &args[0] {
            match type_str.as_str() {
                "Match" => {
                    if args.len() < 2 {
                        return Err(());
                    }
                    if let Value::Str(pattern) = &args[1] {
                        if input.starts_with(pattern) {
                            return Ok((Value::Str(pattern.clone()), pattern.len()));
                        }
                    }
                    return Err(());
                }
                "Regex" => {
                    if args.len() < 2 {
                        return Err(());
                    }
                    if let Value::Str(pattern) = &args[1] {
                        // Compile regex. Note: This is inefficient to do every time.
                        // In a real VM we'd cache this or pre-compile.
                        // We prepend ^ to anchor to start of string for parser behavior
                        let anchored = format!("^{}", pattern);
                        if let Ok(re) = regex::Regex::new(&anchored) {
                            if let Some(mat) = re.find(input) {
                                let match_str = mat.as_str().to_string();
                                let len = match_str.len();
                                return Ok((Value::Str(match_str), len));
                            }
                        }
                    }
                    return Err(());
                }
                "Seq" => {
                    if args.len() < 3 {
                        return Err(());
                    }
                    let p1 = &args[1];
                    let p2 = &args[2];

                    let (res1, consumed1) = run_parser(p1, input)?;
                    let (res2, consumed2) = run_parser(p2, &input[consumed1..])?;

                    Ok((
                        Value::Junction(JunctionType::All, vec![res1, res2]),
                        consumed1 + consumed2,
                    ))
                }
                "Alt" => {
                    if args.len() < 3 {
                        return Err(());
                    }
                    let p1 = &args[1];
                    let p2 = &args[2];

                    if let Ok(res) = run_parser(p1, input) {
                        return Ok(res);
                    }
                    if let Ok(res) = run_parser(p2, input) {
                        return Ok(res);
                    }
                    Err(())
                }
                "Many" => {
                    if args.len() < 2 {
                        return Err(());
                    }
                    let p = &args[1];
                    let mut results = Vec::new();
                    let mut total_consumed = 0;

                    while let Ok((res, consumed)) = run_parser(p, &input[total_consumed..]) {
                        if consumed == 0 {
                            break;
                        } // Prevent infinite loops on empty matches
                        results.push(res);
                        total_consumed += consumed;
                    }

                    Ok((Value::Junction(JunctionType::All, results), total_consumed))
                }
                "Opt" => {
                    if args.len() < 2 {
                        return Err(());
                    }
                    let p = &args[1];

                    if let Ok(res) = run_parser(p, input) {
                        Ok(res)
                    } else {
                        Ok((Value::Junction(JunctionType::All, Vec::new()), 0))
                    }
                }
                _ => Err(()),
            }
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

/// Generates a string from a Grammar.
pub fn generate_string(parser: &Value) -> String {
    generate_string_depth(parser, 0)
}

fn generate_string_depth(parser: &Value, depth: usize) -> String {
    if depth > 50 {
        return "...".to_string();
    }
    if let Value::Junction(JunctionType::Any, args) = parser {
        if args.is_empty() {
            return String::new();
        }
        if let Value::Str(type_str) = &args[0] {
            let mut rng = rand::thread_rng();
            match type_str.as_str() {
                "Match" => {
                    if args.len() >= 2 {
                        if let Value::Str(pattern) = &args[1] {
                            return pattern.clone();
                        }
                    }
                }
                "Regex" => {
                    if args.len() >= 2 {
                        if let Value::Str(pattern) = &args[1] {
                            // Simple mock generation
                            return format!("~{}~", pattern);
                        }
                    }
                }
                "Seq" => {
                    let mut res = String::new();
                    for child in args.iter().skip(1) {
                        res.push_str(&generate_string_depth(child, depth + 1));
                    }
                    return res;
                }
                "Alt" => {
                    if args.len() > 1 {
                        let idx = rng.gen_range(1..args.len());
                        return generate_string_depth(&args[idx], depth + 1);
                    }
                }
                "Many" => {
                    if args.len() >= 2 {
                        let count = rng.gen_range(0..4); // Generate 0-3 times
                        let mut res = String::new();
                        for _ in 0..count {
                            res.push_str(&generate_string_depth(&args[1], depth + 1));
                        }
                        return res;
                    }
                }
                "Opt" => {
                    if args.len() >= 2 {
                        if rng.gen_bool(0.5) {
                            return generate_string_depth(&args[1], depth + 1);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    String::new()
}

pub fn mutate_grammar(grammar: &Value, rate: f64) -> Value {
    let mut rng = rand::thread_rng();
    if !rng.gen_bool(rate.clamp(0.0, 1.0)) {
        return grammar.clone();
    }

    if let Value::Junction(JunctionType::Any, args) = grammar {
        if args.is_empty() {
            return grammar.clone();
        }
        if let Value::Str(type_str) = &args[0] {
            // Chance to change the structure completely (Drift)
            if rng.gen_bool(0.05) {
                // Return a random simple grammar
                let simple_types = ["Match", "Regex"];
                let t = simple_types[rng.gen_range(0..simple_types.len())];
                let pattern = match t {
                    "Match" => "glitch",
                    "Regex" => "[a-z]{3}",
                    _ => "void",
                };
                return Value::Junction(
                    JunctionType::Any,
                    vec![Value::Str(t.to_string()), Value::Str(pattern.to_string())],
                );
            }

            match type_str.as_str() {
                "Match" => {
                    // Mutate literal
                    if args.len() >= 2 {
                        if let Value::Str(s) = &args[1] {
                            let mut chars: Vec<char> = s.chars().collect();
                            if !chars.is_empty() {
                                let idx = rng.gen_range(0..chars.len());
                                let mutation_type = rng.gen_range(0..3);
                                match mutation_type {
                                    0 => {
                                        // Change char
                                        chars[idx] = rng.gen_range(b'a'..=b'z') as char;
                                    }
                                    1 => {
                                        // Delete char
                                        if chars.len() > 1 {
                                            chars.remove(idx);
                                        }
                                    }
                                    2 => {
                                        // Insert char
                                        chars.insert(idx, rng.gen_range(b'a'..=b'z') as char);
                                    }
                                    _ => {}
                                }
                            } else {
                                chars.push('a');
                            }
                            return Value::Junction(
                                JunctionType::Any,
                                vec![
                                    Value::Str("Match".to_string()),
                                    Value::Str(chars.into_iter().collect()),
                                ],
                            );
                        }
                    }
                }
                "Seq" => {
                    // Swap or mutate children
                    let mut new_args = args.clone();
                    // Mutate children
                    for i in 1..new_args.len() {
                        new_args[i] = mutate_grammar(&new_args[i], rate);
                    }
                    // Swap
                    if new_args.len() > 2 && rng.gen_bool(0.3) {
                        let idx1 = rng.gen_range(1..new_args.len());
                        let idx2 = rng.gen_range(1..new_args.len());
                        new_args.swap(idx1, idx2);
                    }
                    return Value::Junction(JunctionType::Any, new_args);
                }
                "Alt" => {
                    // Similar to Seq
                    let mut new_args = args.clone();
                    for i in 1..new_args.len() {
                        new_args[i] = mutate_grammar(&new_args[i], rate);
                    }
                    // Chance to become Seq?
                    if rng.gen_bool(0.1) {
                        new_args[0] = Value::Str("Seq".to_string());
                    }
                    return Value::Junction(JunctionType::Any, new_args);
                }
                "Many" | "Opt" => {
                    let mut new_args = args.clone();
                    if new_args.len() >= 2 {
                        new_args[1] = mutate_grammar(&new_args[1], rate);
                    }
                    // Chance to unwrap?
                    if rng.gen_bool(0.1) && new_args.len() >= 2 {
                        return new_args[1].clone();
                    }
                    return Value::Junction(JunctionType::Any, new_args);
                }
                _ => {}
            }
        }
    }
    grammar.clone()
}

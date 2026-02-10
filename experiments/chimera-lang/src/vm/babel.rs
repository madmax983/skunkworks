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
        OpCode::GrammarMutate => {
            if let Some(g) = vm.stack.pop() {
                let mutated = mutate_grammar(&g);
                vm.stack.push(mutated);
                vm.output.push("GRAMMAR MUTATED".to_string());
            } else {
                vm.output
                    .push("Error: Stack underflow for GrammarMutate".to_string());
            }
        }
        OpCode::GrammarBreed => {
            if vm.stack.len() >= 2 {
                let g2 = vm.stack.pop().unwrap();
                let g1 = vm.stack.pop().unwrap();
                let child = breed_grammar(&g1, &g2);
                vm.stack.push(child);
                vm.output.push("GRAMMAR BRED".to_string());
            } else {
                vm.output
                    .push("Error: Stack underflow for GrammarBreed".to_string());
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

fn mutate_grammar(val: &Value) -> Value {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.1) {
        // 10% chance to completely replace with a random primitive
        return random_primitive_grammar();
    }

    match val {
        Value::Junction(t, args) => {
            // Deep copy args to mutate
            let mut new_args = args.clone();
            if !new_args.is_empty() {
                if let Value::Str(type_str) = &new_args[0] {
                    match type_str.as_str() {
                        "Match" => {
                            if new_args.len() > 1 {
                                if let Value::Str(s) = &mut new_args[1] {
                                    // Mutate string
                                    if !s.is_empty() && rng.gen_bool(0.5) {
                                        // For simplicity, append a random char
                                        let c = (rng.gen_range(0..26) + b'a') as char;
                                        s.push(c);
                                    } else if !s.is_empty() {
                                        // Or remove
                                        s.pop();
                                    }
                                }
                            }
                        }
                        "Seq" | "Alt" => {
                            // Mutate children
                            for i in 1..new_args.len() {
                                if rng.gen_bool(0.3) {
                                    new_args[i] = mutate_grammar(&new_args[i]);
                                }
                            }
                            // Chance to swap
                            if new_args.len() >= 3 && rng.gen_bool(0.2) {
                                new_args.swap(1, 2);
                            }
                        }
                        "Many" | "Opt" => {
                            if new_args.len() > 1 {
                                new_args[1] = mutate_grammar(&new_args[1]);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Value::Junction(*t, new_args)
        }
        _ => val.clone(),
    }
}

fn random_primitive_grammar() -> Value {
    let mut rng = rand::thread_rng();
    let c = ((rng.gen_range(0..26) + b'a') as char).to_string();
    Value::Junction(
        JunctionType::Any,
        vec![Value::Str("Match".to_string()), Value::Str(c)],
    )
}

fn breed_grammar(p1: &Value, p2: &Value) -> Value {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        return p1.clone(); // Fallback
    }

    // Attempt structured crossover
    if let (Value::Junction(t1, args1), Value::Junction(_t2, args2)) = (p1, p2) {
        if !args1.is_empty() && !args2.is_empty() {
            let mut new_args = args1.clone();
            // Replace last arg of p1 with last arg of p2 (Subtree crossover)
            if new_args.len() > 1 && args2.len() > 1 {
                let idx1 = new_args.len() - 1;
                let idx2 = args2.len() - 1;
                new_args[idx1] = args2[idx2].clone();
            }
            return Value::Junction(*t1, new_args);
        }
    }

    p2.clone()
}

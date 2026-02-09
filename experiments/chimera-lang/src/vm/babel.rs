#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};
use crate::ast::{JunctionType, Nucleotide};
use crate::opcode::OpCode;

/// Executes Babel-related OpCodes.
pub fn exec_babel_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Learn => {
            // Stack: [ ..., name_str, parser_junction ]
            if vm.stack.len() >= 2 {
                let parser_val = vm.stack.pop().unwrap();
                let name_val = vm.stack.pop().unwrap();

                if let Value::Str(name) = name_val {
                    // Validate parser structure lightly?
                    // For now, assume any Value can be stored, validation happens on Antibody usage.
                    vm.antibodies.insert(name.clone(), parser_val);
                    vm.output.push(format!("LEARN: Stored antibody '{}'", name));
                } else {
                    vm.output.push("Error: Antibody name must be a string".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for learn".to_string());
            }
        }
        OpCode::Antibody => {
            // Stack: [ ..., name_str, input_str ]
            if vm.stack.len() >= 2 {
                let input_val = vm.stack.pop().unwrap();
                let name_val = vm.stack.pop().unwrap();

                if let (Value::Str(name), Value::Str(input)) = (name_val, input_val) {
                    if let Some(parser) = vm.antibodies.get(&name) {
                        let mut trace = Vec::new();
                        match run_parser(parser, &input, 0, &mut trace) {
                            Ok((ast, consumed)) => {
                                // Full match required? Usually yes for antibodies.
                                if consumed == input.len() {
                                    vm.stack.push(ast);
                                    vm.output.push(format!("ANTIBODY '{}': Success", name));
                                    // Optional: Push trace for visualization?
                                    // vm.stack.push(Value::Str(trace.join("\n")));
                                } else {
                                    vm.stack.push(Value::Int(0)); // Fail
                                    vm.output.push(format!("ANTIBODY '{}': Partial match ({}/{})", name, consumed, input.len()));
                                }
                            }
                            Err(_) => {
                                vm.stack.push(Value::Int(0)); // Fail
                                vm.output.push(format!("ANTIBODY '{}': Failed", name));
                            }
                        }
                    } else {
                        vm.output.push(format!("ANTIBODY: Unknown antibody '{}'", name));
                        vm.stack.push(Value::Int(0));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for antibody".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for antibody".to_string());
            }
        }
        OpCode::Grammar => {
            // Stack: [ ..., type_str, ...args ]
            if let Some(type_val) = vm.stack.pop() {
                if let Value::Str(type_str) = type_val {
                    let mut args = vec![Value::Str(type_str.clone())];
                    match type_str.as_str() {
                        "Match" => {
                            if let Some(pattern) = vm.stack.pop() {
                                args.push(pattern);
                                vm.stack.push(Value::Junction(JunctionType::Any, args));
                            } else {
                                vm.output
                                    .push("Error: Stack underflow for Grammar(Match)".to_string());
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
                    let mut trace = Vec::new();
                    match run_parser(&parser_val, &input_str, 0, &mut trace) {
                        Ok((ast, consumed)) => {
                            if consumed == input_str.len() {
                                vm.stack.push(ast);
                                vm.output.push("PARSE: Success".to_string());
                            } else {
                                vm.output
                                    .push(format!("PARSE: Partial match ({} chars)", consumed));
                                vm.stack.push(Value::Int(0)); // Failure indicator
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
/// Trace records steps.
pub fn run_parser(parser: &Value, input: &str, depth: usize, trace: &mut Vec<String>) -> Result<(Value, usize), ()> {
    if depth > 100 {
        trace.push("Recursion limit exceeded".to_string());
        return Err(());
    }

    if let Value::Junction(JunctionType::Any, args) = parser {
        if args.is_empty() {
            return Err(());
        }
        if let Value::Str(type_str) = &args[0] {
            let indent = "  ".repeat(depth);
            trace.push(format!("{}Testing {} on '{}'", indent, type_str, input));

            match type_str.as_str() {
                "Match" => {
                    if args.len() < 2 {
                        return Err(());
                    }
                    if let Value::Str(pattern) = &args[1] {
                        if input.starts_with(pattern) {
                            trace.push(format!("{}  -> Match success: '{}'", indent, pattern));
                            return Ok((Value::Str(pattern.clone()), pattern.len()));
                        }
                    }
                    trace.push(format!("{}  -> Match failed", indent));
                    return Err(());
                }
                "Seq" => {
                    if args.len() < 3 {
                        return Err(());
                    }
                    let p1 = &args[1];
                    let p2 = &args[2];

                    let (res1, consumed1) = run_parser(p1, input, depth + 1, trace)?;
                    let (res2, consumed2) = run_parser(p2, &input[consumed1..], depth + 1, trace)?;

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

                    if let Ok(res) = run_parser(p1, input, depth + 1, trace) {
                        return Ok(res);
                    }
                    trace.push(format!("{}  -> Alt 1 failed, trying 2", indent));
                    if let Ok(res) = run_parser(p2, input, depth + 1, trace) {
                        return Ok(res);
                    }
                    trace.push(format!("{}  -> Alt 2 failed", indent));
                    Err(())
                }
                "Many" => {
                    if args.len() < 2 {
                        return Err(());
                    }
                    let p = &args[1];
                    let mut results = Vec::new();
                    let mut total_consumed = 0;

                    while let Ok((res, consumed)) = run_parser(p, &input[total_consumed..], depth + 1, trace) {
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

                    if let Ok(res) = run_parser(p, input, depth + 1, trace) {
                        Ok(res)
                    } else {
                        trace.push(format!("{}  -> Opt failed, returning empty", indent));
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

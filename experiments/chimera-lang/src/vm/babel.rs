#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};
use crate::ast::{Gene, JunctionType, Nucleotide, Strand};
use crate::opcode::OpCode;
use rand::Rng;

/// Executes Babel-related OpCodes.
pub fn exec_babel_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::BabelCompile => {
            // [ cst, handler_strand ] -> [ new_strand_idx ]
            if vm.stack.len() >= 2 {
                let handler_val = vm.stack.pop().unwrap();
                let cst_val = vm.stack.pop().unwrap();

                if let Value::Int(handler_idx) = handler_val {
                    if handler_idx >= 0 {
                        if let Some(new_idx) = compile_cst(vm, cst_val, handler_idx as usize) {
                            vm.stack.push(Value::Int(new_idx as i64));
                        } else {
                            vm.output
                                .push("Error: Compilation failed (Recursion limit)".to_string());
                            vm.stack.push(Value::Int(-1));
                        }
                    } else {
                        vm.output
                            .push("Error: Invalid handler index for BabelCompile".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Handler must be an Int (strand index)".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for BabelCompile".to_string());
            }
        }
        OpCode::Generate => {
            if let Some(grammar) = vm.stack.pop() {
                let generated = generate_string(&grammar);
                vm.stack.push(Value::Str(generated));
            } else {
                vm.output
                    .push("Error: Stack underflow for Generate".to_string());
            }
        }
        OpCode::Tongue => {
            // [ grammar, input ] -> [ corrupted ]
            if vm.stack.len() >= 2 {
                let input_val = vm.stack.pop().unwrap();
                let grammar_val = vm.stack.pop().unwrap();

                if let Value::Str(input_str) = input_val {
                    // 1. Parse
                    match run_parser(&grammar_val, &input_str) {
                        Ok((cst, consumed)) => {
                            if consumed == input_str.len() {
                                // 2. Mutate CST
                                let mutated_cst = mutate_cst(&cst, 0.2); // 20% base corruption rate
                                                                         // 3. Flatten
                                let output_str = flatten_cst(&mutated_cst);
                                vm.stack.push(Value::Str(output_str));
                                vm.output.push("TONGUE: Reality corrupted.".to_string());
                            } else {
                                vm.output.push(format!(
                                    "TONGUE: Partial match ({} chars), cannot corrupt.",
                                    consumed
                                ));
                                vm.stack.push(Value::Str(input_str));
                            }
                        }
                        Err(_) => {
                            vm.output.push("TONGUE: Parse failed.".to_string());
                            vm.stack.push(Value::Str(input_str));
                        }
                    }
                } else {
                    vm.output
                        .push("Error: Tongue input must be string".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for Tongue".to_string());
            }
        }
        OpCode::Scribe => {
            if let Some(val) = vm.stack.pop() {
                let s = match val {
                    Value::Str(s) => s,
                    Value::Symbol(id) => format!("§{:x}", id),
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
                if p1.depth() > 500 || p2.depth() > 500 {
                    vm.output.push("Error: Parser depth limit exceeded".to_string());
                    vm.stack.push(p1); // Restore stack roughly?
                    vm.stack.push(p2);
                } else {
                    vm.stack.push(Value::Junction(
                        JunctionType::Any,
                        vec![Value::Str("Seq".to_string()), p1, p2],
                    ));
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for ParserSeq".to_string());
            }
        }
        OpCode::ParserSeqN => {
            if let Some(Value::Int(count)) = vm.stack.pop() {
                let count = count as usize;
                if vm.stack.len() >= count {
                    let mut args = vec![Value::Str("Seq".to_string())];
                    let mut items = Vec::new();
                    for _ in 0..count {
                        items.push(vm.stack.pop().unwrap());
                    }
                    items.reverse();

                    let max_depth = items.iter().map(|v| v.depth()).max().unwrap_or(0);
                    if max_depth > 500 {
                        vm.output.push("Error: Parser depth limit exceeded".to_string());
                        // Push back?
                        for item in items {
                            vm.stack.push(item);
                        }
                    } else {
                        args.extend(items);
                        vm.stack.push(Value::Junction(JunctionType::Any, args));
                    }
                } else {
                    vm.output
                        .push("Error: Stack underflow for ParserSeqN".to_string());
                }
            } else {
                vm.output
                    .push("Error: ParserSeqN requires count".to_string());
            }
        }
        OpCode::ParserAlt => {
            if vm.stack.len() >= 2 {
                let p2 = vm.stack.pop().unwrap();
                let p1 = vm.stack.pop().unwrap();
                if p1.depth() > 500 || p2.depth() > 500 {
                    vm.output.push("Error: Parser depth limit exceeded".to_string());
                    vm.stack.push(p1);
                    vm.stack.push(p2);
                } else {
                    vm.stack.push(Value::Junction(
                        JunctionType::Any,
                        vec![Value::Str("Alt".to_string()), p1, p2],
                    ));
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for ParserAlt".to_string());
            }
        }
        OpCode::ParserAltN => {
            if let Some(Value::Int(count)) = vm.stack.pop() {
                let count = count as usize;
                if vm.stack.len() >= count {
                    let mut args = vec![Value::Str("Alt".to_string())];
                    let mut items = Vec::new();
                    for _ in 0..count {
                        items.push(vm.stack.pop().unwrap());
                    }
                    items.reverse();

                    let max_depth = items.iter().map(|v| v.depth()).max().unwrap_or(0);
                    if max_depth > 500 {
                        vm.output.push("Error: Parser depth limit exceeded".to_string());
                        for item in items {
                            vm.stack.push(item);
                        }
                    } else {
                        args.extend(items);
                        vm.stack.push(Value::Junction(JunctionType::Any, args));
                    }
                } else {
                    vm.output
                        .push("Error: Stack underflow for ParserAltN".to_string());
                }
            } else {
                vm.output
                    .push("Error: ParserAltN requires count".to_string());
            }
        }
        OpCode::ParserMany => {
            if let Some(p) = vm.stack.pop() {
                if p.depth() > 500 {
                    vm.output.push("Error: Parser depth limit exceeded".to_string());
                    vm.stack.push(p);
                } else {
                    vm.stack.push(Value::Junction(
                        JunctionType::Any,
                        vec![Value::Str("Many".to_string()), p],
                    ));
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for ParserMany".to_string());
            }
        }
        OpCode::ParserOpt => {
            if let Some(p) = vm.stack.pop() {
                if p.depth() > 500 {
                    vm.output.push("Error: Parser depth limit exceeded".to_string());
                    vm.stack.push(p);
                } else {
                    vm.stack.push(Value::Junction(
                        JunctionType::Any,
                        vec![Value::Str("Opt".to_string()), p],
                    ));
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for ParserOpt".to_string());
            }
        }
        OpCode::GridGrammar => {
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        let grammar = read_grammar_from_grid(vm, ny, nx);
                        vm.stack.push(grammar);
                        vm.output
                            .push(format!("GRID_GRAMMAR: Read from {},{}", nx, ny));
                    }
                } else {
                    vm.output
                        .push("Error: GridGrammar coords must be Int".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for GridGrammar".to_string());
            }
        }
        OpCode::Glossolalia => {
            // Stack: [ ..., amount ] -> [ ... ]
            let amount = if let Some(Value::Int(n)) = vm.stack.pop() {
                n.max(1) as usize
            } else {
                5
            };
            let (y, x) = vm.context_loc;
            generate_random_grid_grammar(vm, y, x, amount);
            vm.output.push("GLOSSOLALIA: The grid speaks!".to_string());
        }
        OpCode::BabelLive => {
            if vm.stack.len() >= 3 {
                let input_val = vm.stack.pop().unwrap();
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();

                if let (Value::Int(y), Value::Int(x), Value::Str(input)) = (y_val, x_val, input_val)
                {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        let success =
                            crate::vm::nova_babel_live::exec_live_parse(vm, ny, nx, input);
                        vm.stack.push(Value::Int(if success { 1 } else { 0 }));
                        let status = if success { "Success" } else { "Failure" };
                        vm.output
                            .push(format!("BABEL_LIVE: Parse {} at {},{}", status, nx, ny));
                    } else {
                        vm.output
                            .push("Error: Coordinates out of bounds for BabelLive".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for BabelLive".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for BabelLive".to_string());
            }
        }
        OpCode::SelfRewrite => {
            if let Some(grammar) = vm.stack.pop() {
                vm.active_grammar = grammar;
                vm.output
                    .push("SELF_REWRITE: Active Grammar Updated".to_string());
            } else {
                vm.output
                    .push("Error: Stack underflow for SelfRewrite".to_string());
            }
        }
        OpCode::Perceive => {
            if let Some(Value::Int(len)) = vm.stack.pop() {
                if len > 0 {
                    let max_len = (len as usize).min(crate::vm::GRID_SIZE * crate::vm::GRID_SIZE);
                    let (y, x) = vm.context_loc;
                    let mut input = String::new();

                    for k in 0..max_len {
                        if let Some((ny, nx)) = vm.normalize_coords(y as i64, x as i64 + k as i64) {
                            let val = &vm.grid[ny][nx];
                            match val {
                                Value::Str(s) => input.push_str(s),
                                Value::Int(n) => input.push_str(&n.to_string()),
                                _ => input.push(' '),
                            }
                        }
                    }

                    // Clone active grammar to avoid borrow issues with vm
                    let grammar = vm.active_grammar.clone();
                    match run_parser(&grammar, &input) {
                        Ok((cst, consumed)) => {
                            if consumed > 0 {
                                let handler_idx = vm.ip.0;
                                if let Some(new_idx) = compile_cst(vm, cst, handler_idx) {
                                    if vm.call_stack.len() < crate::vm::MAX_CALL_STACK_DEPTH {
                                        vm.call_stack.push((vm.ip.0, vm.ip.1 + 1));
                                        vm.stack.push(Value::Int(1)); // Success
                                        vm.output.push(format!(
                                            "PERCEIVE: Parsed '{}' -> Strand {}",
                                            input[..consumed].to_string(),
                                            new_idx
                                        ));
                                        return Some((new_idx, 0));
                                    } else {
                                        vm.output.push("PERCEIVE: Call stack full".to_string());
                                        vm.stack.push(Value::Int(0));
                                    }
                                } else {
                                    vm.output.push("PERCEIVE: Compilation failed".to_string());
                                    vm.stack.push(Value::Int(0));
                                }
                            } else {
                                vm.stack.push(Value::Int(0)); // Fail
                            }
                        }
                        Err(_) => {
                            vm.stack.push(Value::Int(0)); // Fail
                        }
                    }
                } else {
                    vm.output
                        .push("Error: Perceive length must be > 0".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for Perceive".to_string());
            }
        }
        OpCode::Ouroboros => {
            return exec_ouroboros(vm);
        }
        _ => {}
    }
    None
}

/// The Ouroboros Protocol: Self-consumption and rebirth.
///
/// **OpCode:** `Ouroboros`
/// **Stack:** `[ ..., grammar_junction ] -> [ ... ]`
/// **Effect:** Decompiles self, parses with grammar, mutates, recompiles, replaces self.
pub fn exec_ouroboros(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(grammar) = vm.stack.pop() {
        let current_strand_idx = vm.ip.0;

        if current_strand_idx >= vm.dna.helix.strands.len() {
            vm.output
                .push("OUROBOROS: Invalid strand index".to_string());
            return None;
        }

        // 1. Decompile Self
        let strand = &vm.dna.helix.strands[current_strand_idx];
        let source = crate::vm::nova_genetics::strand_to_string(strand);

        // 2. Parse with Grammar
        if let Ok((cst, _consumed)) = run_parser(&grammar, &source) {
            // 3. Mutate CST
            // Base mutation rate + Glitch Level
            let rate = 0.1 + vm.glitch_level as f64;
            let mutated_cst = mutate_cst(&cst, rate);

            // 4. Compile to New Strand
            // We use current strand as handler for any recursive structures
            if let Some(new_strand_idx) = compile_cst(vm, mutated_cst, current_strand_idx) {
                // 5. Hot-Swap (Rebirth)
                vm.output.push(format!(
                    "OUROBOROS: Strand {} rebirthed as {}",
                    current_strand_idx, new_strand_idx
                ));
                return Some((new_strand_idx, 0));
            } else {
                vm.output
                    .push("OUROBOROS: Compilation failed".to_string());
            }
        } else {
            vm.output
                .push("OUROBOROS: Failed to parse self.".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for Ouroboros".to_string());
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
                    // Variadic Seq
                    let mut total_consumed = 0;
                    let mut results = Vec::new();
                    // Args[1..] are sub-parsers
                    for parser in args.iter().skip(1) {
                        let current_input = &input[total_consumed..];
                        match run_parser(parser, current_input) {
                            Ok((res, consumed)) => {
                                results.push(res);
                                total_consumed += consumed;
                            }
                            Err(_) => return Err(()),
                        }
                    }
                    Ok((Value::Junction(JunctionType::All, results), total_consumed))
                }
                "Alt" => {
                    // Variadic Alt
                    for parser in args.iter().skip(1) {
                        if let Ok(res) = run_parser(parser, input) {
                            return Ok(res);
                        }
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
                            // Prevent infinite loops on empty matches
                            break;
                        }
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
                        // Optional returns empty junction on fail? Or just empty consumed?
                        // Return empty match AST or None?
                        // Let's return empty junction.
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
                    if args.len() >= 2 {
                        if let Value::Str(s) = &args[1] {
                            let mut chars: Vec<char> = s.chars().collect();
                            if !chars.is_empty() {
                                let idx = rng.gen_range(0..chars.len());
                                let mutation_type = rng.gen_range(0..3);
                                match mutation_type {
                                    0 => {
                                        chars[idx] = rng.gen_range(b'a'..=b'z') as char;
                                    }
                                    1 => {
                                        if chars.len() > 1 {
                                            chars.remove(idx);
                                        }
                                    }
                                    2 => {
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
                    let mut new_args = args.clone();
                    for i in 1..new_args.len() {
                        new_args[i] = mutate_grammar(&new_args[i], rate);
                    }
                    if new_args.len() > 2 && rng.gen_bool(0.3) {
                        let idx1 = rng.gen_range(1..new_args.len());
                        let idx2 = rng.gen_range(1..new_args.len());
                        new_args.swap(idx1, idx2);
                    }
                    return Value::Junction(JunctionType::Any, new_args);
                }
                "Alt" => {
                    let mut new_args = args.clone();
                    for i in 1..new_args.len() {
                        new_args[i] = mutate_grammar(&new_args[i], rate);
                    }
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
                    return Value::Junction(JunctionType::Any, new_args);
                }
                _ => {}
            }
        }
    }
    grammar.clone()
}

pub fn mutate_cst(cst: &Value, rate: f64) -> Value {
    let mut rng = rand::thread_rng();
    if !rng.gen_bool(rate) {
        return cst.clone();
    }

    match cst {
        Value::Str(s) => {
            let mut chars: Vec<char> = s.chars().collect();
            if !chars.is_empty() {
                let idx = rng.gen_range(0..chars.len());
                chars[idx] = match rng.gen_range(0..3) {
                    0 => (chars[idx] as u8 ^ 32) as char,
                    1 => rng.gen_range(33..126) as u8 as char,
                    _ => '?',
                };
            }
            Value::Str(chars.into_iter().collect())
        }
        Value::Junction(JunctionType::All, children) => {
            let mut new_children = children.clone();
            if new_children.len() > 1 && rng.gen_bool(0.3) {
                let i1 = rng.gen_range(0..new_children.len());
                let i2 = rng.gen_range(0..new_children.len());
                new_children.swap(i1, i2);
            }
            for child in &mut new_children {
                *child = mutate_cst(child, rate);
            }
            Value::Junction(JunctionType::All, new_children)
        }
        _ => cst.clone(),
    }
}

pub fn flatten_cst(cst: &Value) -> String {
    match cst {
        Value::Str(s) => s.clone(),
        Value::Junction(JunctionType::All, children) => {
            let mut s = String::new();
            for child in children {
                s.push_str(&flatten_cst(child));
            }
            s
        }
        _ => String::new(),
    }
}

pub fn compile_cst(vm: &mut ChimeraVM, cst: Value, handler_idx: usize) -> Option<usize> {
    let mut genes = Vec::new();
    if compile_cst_recursive(&cst, &mut genes, handler_idx, 0).is_err() {
        return None;
    }

    vm.dna.helix.strands.push(Strand { genes });
    Some(vm.dna.helix.strands.len() - 1)
}

fn compile_cst_recursive(val: &Value, genes: &mut Vec<Gene>, handler_idx: usize, depth: usize) -> Result<(), ()> {
    if depth > 500 {
        return Err(());
    }
    match val {
        Value::Int(n) => {
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(*n)],
            });
        }
        Value::Str(s) => {
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(s.clone())],
            });
        }
        Value::Junction(t, children) => {
            for child in children {
                compile_cst_recursive(child, genes, handler_idx, depth + 1)?;
            }
            // Push Type
            let t_str = match t {
                JunctionType::Any => "Any",
                JunctionType::All => "All",
                JunctionType::Dish => "Dish",
            };
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(t_str.to_string())],
            });
            // Push Count
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(children.len() as i64)],
            });
            // Call Handler
            genes.push(Gene {
                op: OpCode::Call,
                args: vec![Nucleotide::Number(handler_idx as i64)],
            });
        }
        Value::Superposition(states) => {
            // Treat like a junction but with specific tag?
            // "Superposition"
            for (v, p) in states {
                compile_cst_recursive(v, genes, handler_idx, depth + 1)?;
                genes.push(Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number((p * 1000.0) as i64)], // Prob as int 0-1000
                });
            }
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Superposition".to_string())],
            });
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(states.len() as i64)],
            });
            genes.push(Gene {
                op: OpCode::Call,
                args: vec![Nucleotide::Number(handler_idx as i64)],
            });
        }
        Value::Symbol(id) => {
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(format!("§{:x}", id))],
            });
        }
    }
    Ok(())
}

pub fn read_grammar_from_grid(vm: &ChimeraVM, y: usize, x: usize) -> Value {
    let mut scanner = GridScanner {
        vm,
        visited: std::collections::HashSet::new(),
    };
    scanner.scan(y, x).unwrap_or(Value::Junction(
        JunctionType::Any,
        vec![Value::Str("Match".to_string()), Value::Str("".to_string())],
    ))
}

struct GridScanner<'a> {
    vm: &'a ChimeraVM,
    visited: std::collections::HashSet<(usize, usize)>,
}

impl<'a> GridScanner<'a> {
    fn scan(&mut self, y: usize, x: usize) -> Option<Value> {
        if self.visited.contains(&(y, x)) {
            return None;
        }
        self.visited.insert((y, x));

        // Use peek to safely check boundaries
        if let Some(val) = self.peek(y, x) {
            match val {
                Value::Str(s) => {
                    let c = s.chars().next().unwrap_or('\0');
                    match c {
                        '"' => self.scan_string(y, x),
                        '[' => self.scan_regex(y, x),
                        '?' => self.scan_modifier(y, x, "Opt"),
                        '*' => self.scan_modifier(y, x, "Many"),
                        '+' => self.scan_modifier(y, x, "OneOrMore"),
                        '|' | '-' | '.' | '>' | 'v' => self.scan_connector(y, x),
                        _ => None,
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn peek(&self, y: usize, x: usize) -> Option<&Value> {
        if y < crate::vm::GRID_SIZE && x < crate::vm::GRID_SIZE {
            Some(&self.vm.grid[y][x])
        } else {
            None
        }
    }

    fn scan_string(&mut self, y: usize, x: usize) -> Option<Value> {
        // Read until closing quote
        let mut content = String::new();
        let mut curr_x = x + 1;

        loop {
            if let Some(val) = self.peek(y, curr_x) {
                match val {
                    Value::Str(s) => {
                        if s == "\"" {
                            break;
                        }
                        content.push_str(s);
                        self.visited.insert((y, curr_x)); // Mark string content as visited
                        curr_x += 1;
                    }
                    _ => break, // Unexpected end
                }
            } else {
                break;
            }
            if curr_x >= crate::vm::GRID_SIZE {
                break;
            }
        }
        self.visited.insert((y, curr_x)); // Mark closing quote

        let node = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("Match".to_string()), Value::Str(content)],
        );

        // Continue scanning neighbors of the closing quote
        self.attach_neighbors(y, curr_x, node)
    }

    fn scan_regex(&mut self, y: usize, x: usize) -> Option<Value> {
        // Read until closing bracket
        let mut content = String::new();
        let mut curr_x = x + 1;

        loop {
            if let Some(val) = self.peek(y, curr_x) {
                match val {
                    Value::Str(s) => {
                        if s == "]" {
                            break;
                        }
                        content.push_str(s);
                        self.visited.insert((y, curr_x));
                        curr_x += 1;
                    }
                    _ => break,
                }
            } else {
                break;
            }
            if curr_x >= crate::vm::GRID_SIZE {
                break;
            }
        }
        self.visited.insert((y, curr_x));

        let node = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("Regex".to_string()), Value::Str(content)],
        );

        self.attach_neighbors(y, curr_x, node)
    }

    fn scan_modifier(&mut self, y: usize, x: usize, type_str: &str) -> Option<Value> {
        // Modifiers connect to ONE child usually? Or behave like connectors?
        // Let's say * connects to the thing it modifies.
        // E.g.  A - * - B  (Many(A)? No, usually * follows A)
        // Or  * - A  (Many A)
        // Let's assume PREFIX notation for visual simplicity in tracing?
        // * -> A  means Many(A).

        let children = self.find_children(y, x);
        if children.is_empty() {
            return None;
        }

        // Wrap first child (or all in Alt?)
        // Let's wrap the sequence of children.
        // If multiple children, it's Many(Alt(Children)).

        let child_node = if children.len() == 1 {
            children[0].clone()
        } else {
            let mut args = vec![Value::Str("Alt".to_string())];
            args.extend(children);
            Value::Junction(JunctionType::Any, args)
        };

        Some(Value::Junction(
            JunctionType::Any,
            vec![Value::Str(type_str.to_string()), child_node],
        ))
    }

    fn scan_connector(&mut self, y: usize, x: usize) -> Option<Value> {
        let children = self.find_children(y, x);

        if children.is_empty() {
            None
        } else if children.len() == 1 {
            Some(children[0].clone())
        } else {
            // Branching -> Alt
            let mut args = vec![Value::Str("Alt".to_string())];
            args.extend(children);
            Some(Value::Junction(JunctionType::Any, args))
        }
    }

    fn find_children(&mut self, y: usize, x: usize) -> Vec<Value> {
        let neighbors = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut children = Vec::new();

        for (dy, dx) in neighbors {
            let ny = y as i64 + dy;
            let nx = x as i64 + dx;

            if let Some((valid_y, valid_x)) = self.vm.normalize_coords(ny, nx) {
                // To avoid infinite back-and-forth on a line A-B, checked visited.
                // But scan() checks visited.
                if let Some(child) = self.scan(valid_y, valid_x) {
                    children.push(child);
                }
            }
        }
        children
    }

    fn attach_neighbors(&mut self, y: usize, x: usize, node: Value) -> Option<Value> {
        // Find what comes AFTER this node.
        let next_nodes = self.find_children(y, x);

        if next_nodes.is_empty() {
            Some(node)
        } else {
            // Sequence: Node -> Next
            // If Next is Alt, then Node -> Alt(...)
            let next_val = if next_nodes.len() == 1 {
                next_nodes[0].clone()
            } else {
                let mut args = vec![Value::Str("Alt".to_string())];
                args.extend(next_nodes);
                Value::Junction(JunctionType::Any, args)
            };

            Some(Value::Junction(
                JunctionType::Any,
                vec![Value::Str("Seq".to_string()), node, next_val],
            ))
        }
    }
}

pub fn generate_random_grid_grammar(vm: &mut ChimeraVM, y: usize, x: usize, amount: usize) {
    let mut rng = rand::thread_rng();
    grow_grammar(vm, y as i64, x as i64, amount, &mut rng);
}

fn grow_grammar(vm: &mut ChimeraVM, y: i64, x: i64, energy: usize, rng: &mut impl rand::Rng) {
    if energy == 0 {
        return;
    }

    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
        if vm.grid[ny][nx] != Value::Int(0) {
            return;
        }

        let choice = rng.gen_range(0..10);
        match choice {
            0..=3 => {
                // String
                let len = rng.gen_range(2..5);
                let s: String = (0..len)
                    .map(|_| rng.gen_range(b'a'..=b'z') as char)
                    .collect();

                // Write "s"
                if let Some((qy, qx)) = vm.normalize_coords(y, x) {
                    vm.grid[qy][qx] = Value::Str("\"".to_string());
                }
                for (i, c) in s.chars().enumerate() {
                    if let Some((cy, cx)) = vm.normalize_coords(y, x + 1 + i as i64) {
                        vm.grid[cy][cx] = Value::Str(c.to_string());
                    }
                }
                if let Some((eqy, eqx)) = vm.normalize_coords(y, x + 1 + len as i64) {
                    vm.grid[eqy][eqx] = Value::Str("\"".to_string());
                }

                // Connector
                if let Some((cy, cx)) = vm.normalize_coords(y, x + 2 + len as i64) {
                    vm.grid[cy][cx] = Value::Str("-".to_string());
                    grow_grammar(vm, y, x + 3 + len as i64, energy - 1, rng);
                }
            }
            4..=5 => {
                // Regex
                let patterns = ["[0-9]", "[a-z]", "\\w+"];
                let p = patterns[rng.gen_range(0..patterns.len())];

                // Write [p]
                if let Some((by, bx)) = vm.normalize_coords(y, x) {
                    vm.grid[by][bx] = Value::Str("[".to_string());
                }
                for (i, c) in p.chars().enumerate() {
                    if let Some((cy, cx)) = vm.normalize_coords(y, x + 1 + i as i64) {
                        vm.grid[cy][cx] = Value::Str(c.to_string());
                    }
                }
                if let Some((eby, ebx)) = vm.normalize_coords(y, x + 1 + p.len() as i64) {
                    vm.grid[eby][ebx] = Value::Str("]".to_string());
                }

                if let Some((cy, cx)) = vm.normalize_coords(y, x + 2 + p.len() as i64) {
                    vm.grid[cy][cx] = Value::Str("-".to_string());
                    grow_grammar(vm, y, x + 3 + p.len() as i64, energy - 1, rng);
                }
            }
            6..=7 => {
                // Modifier
                let mods = ["*", "?", "+"];
                let m = mods[rng.gen_range(0..mods.len())];
                vm.grid[ny][nx] = Value::Str(m.to_string());
                // Grow next
                if let Some((cy, cx)) = vm.normalize_coords(y, x + 1) {
                    vm.grid[cy][cx] = Value::Str("-".to_string());
                    grow_grammar(vm, y, x + 2, energy, rng); // Energy doesn't decrease for mod?
                }
            }
            8..=9 => {
                // Branch (.)
                vm.grid[ny][nx] = Value::Str(".".to_string());

                // Grow Up
                if let Some((uy, ux)) = vm.normalize_coords(y - 1, x) {
                    vm.grid[uy][ux] = Value::Str("|".to_string());
                    grow_grammar(vm, y - 2, x, energy / 2, rng);
                }

                // Grow Down
                if let Some((dy, dx)) = vm.normalize_coords(y + 1, x) {
                    vm.grid[dy][dx] = Value::Str("|".to_string());
                    grow_grammar(vm, y + 2, x, energy / 2, rng);
                }

                // Grow Right
                if let Some((ry, rx)) = vm.normalize_coords(y, x + 1) {
                    vm.grid[ry][rx] = Value::Str("-".to_string());
                    grow_grammar(vm, y, x + 2, energy / 2, rng);
                }
            }
            _ => {}
        }
    }
}

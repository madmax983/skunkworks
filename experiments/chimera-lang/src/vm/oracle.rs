#![cfg(feature = "oracle")]
use super::{ChimeraVM, Value};
use crate::ast::{JunctionType, Nucleotide};
use crate::opcode::OpCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Omen {
    pub condition: Value,
    pub effect: Value,
}

type Subst = HashMap<String, Value>;

pub fn exec_oracle_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::Assert => {
            if let Some(fact) = vm.stack.pop() {
                if !vm.knowledge_base.contains(&fact) {
                    vm.knowledge_base.push(fact.clone());
                    vm.output.push(format!("ASSERT: Added {}", fact));
                } else {
                    vm.output
                        .push(format!("ASSERT: Fact already exists {}", fact));
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for assert".to_string());
            }
        }
        OpCode::Rule => {
            // Stack: [ ..., body_junction, head ]
            // Rule format in KB: Junction(Any, ["rule", Head, Body...])
            if vm.stack.len() >= 2 {
                let body_val = vm.stack.pop().unwrap();
                let head_val = vm.stack.pop().unwrap();

                let mut rule_args = vec![Value::Str("rule".to_string()), head_val];

                match body_val {
                    Value::Junction(JunctionType::All, body_terms) => {
                        rule_args.extend(body_terms);
                    }
                    _ => {
                        rule_args.push(body_val);
                    }
                }

                let rule = Value::Junction(JunctionType::Any, rule_args);
                vm.knowledge_base.push(rule.clone());
                vm.output.push(format!("RULE: Added {}", rule));
            } else {
                vm.output
                    .push("Error: Stack underflow for rule".to_string());
            }
        }
        OpCode::Retract => {
            if let Some(fact) = vm.stack.pop() {
                if let Some(pos) = vm.knowledge_base.iter().position(|x| *x == fact) {
                    vm.knowledge_base.remove(pos);
                    vm.output.push(format!("RETRACT: Removed {}", fact));
                } else {
                    vm.output.push(format!("RETRACT: Fact not found {}", fact));
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for retract".to_string());
            }
        }
        OpCode::Query => {
            if let Some(query) = vm.stack.pop() {
                let goals = match query {
                    Value::Junction(JunctionType::All, ref args) => args.clone(),
                    _ => vec![query.clone()],
                };

                vm.output.push(format!("QUERY: Solving {:?}", goals));

                let mut solutions = Vec::new();
                solve(
                    &goals,
                    HashMap::new(),
                    &vm.knowledge_base,
                    vm,
                    &mut solutions,
                    0,
                );

                if solutions.is_empty() {
                    vm.stack.push(Value::Int(0));
                    vm.output.push("QUERY: Failed".to_string());
                } else {
                    vm.stack.push(Value::Int(1));
                    vm.output
                        .push(format!("QUERY: Success ({} solutions)", solutions.len()));

                    // Push solutions to stack as Junction of Junctions (Bindings)
                    // Each binding: Junction(All, ["VarName", Value])
                    // Actually, let's just log them for now as per previous implementation,
                    // or push the FIRST solution's bindings to stack if requested?
                    // "Prolog" usually iterates.
                    // For now, logging is fine, but let's push the bindings map of the first solution
                    // so code can use it.

                    if let Some(first_sol) = solutions.first() {
                        let mut binding_list = Vec::new();
                        for (k, v) in first_sol {
                            let pair = Value::Junction(
                                JunctionType::All,
                                vec![Value::Str(k.clone()), v.clone()],
                            );
                            binding_list.push(pair);
                        }
                        vm.stack
                            .push(Value::Junction(JunctionType::All, binding_list));
                    }
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for query".to_string());
            }
        }
        OpCode::Augury => {
            if vm.stack.len() >= 2 {
                let effect = vm.stack.pop().unwrap();
                let condition = vm.stack.pop().unwrap();
                vm.omens.push(Omen {
                    condition: condition.clone(),
                    effect: effect.clone(),
                });
                vm.output
                    .push(format!("AUGURY: Registered omen for {}", condition));
            } else {
                vm.output
                    .push("Error: Stack underflow for augury".to_string());
            }
        }
        OpCode::Divinate => {
            let mut triggered_count = 0;
            let omens_snapshot = vm.omens.clone();

            for omen in omens_snapshot {
                let mut solutions = Vec::new();
                solve(
                    &[omen.condition.clone()],
                    HashMap::new(),
                    &vm.knowledge_base,
                    vm,
                    &mut solutions,
                    0,
                );

                if !solutions.is_empty() {
                    triggered_count += 1;
                    vm.output
                        .push(format!("DIVINATE: Omen fulfilled! {}", omen.condition));

                    match omen.effect {
                        Value::Int(n) => {
                            if n >= 0 {
                                vm.ip = (n as usize, 0);
                            }
                        }
                        Value::Str(ref s) => {
                            if let Ok(op) = s.parse::<OpCode>() {
                                let _ = vm.execute_gene_inner(op, &[]);
                            }
                        }
                        _ => {}
                    }
                }
            }
            vm.stack.push(Value::Int(triggered_count));
        }
        _ => {}
    }
}

fn is_var(v: &Value) -> Option<String> {
    if let Value::Str(s) = v {
        if s.starts_with('?') {
            return Some(s.clone());
        }
    }
    None
}

fn resolve(term: &Value, subst: &Subst) -> Value {
    match term {
        Value::Str(s) if s.starts_with('?') => {
            if let Some(val) = subst.get(s) {
                resolve(val, subst)
            } else {
                term.clone()
            }
        }
        Value::Junction(t, args) => {
            let resolved_args = args.iter().map(|a| resolve(a, subst)).collect();
            Value::Junction(*t, resolved_args)
        }
        _ => term.clone(),
    }
}

fn unify(t1: &Value, t2: &Value, subst: &Subst) -> Option<Subst> {
    let t1 = resolve(t1, subst);
    let t2 = resolve(t2, subst);

    if t1 == t2 {
        return Some(subst.clone());
    }

    if let Some(var_name) = is_var(&t1) {
        return bind(&var_name, &t2, subst);
    }
    if let Some(var_name) = is_var(&t2) {
        return bind(&var_name, &t1, subst);
    }

    match (t1, t2) {
        (Value::Junction(type1, args1), Value::Junction(type2, args2)) => {
            if type1 != type2 || args1.len() != args2.len() {
                return None;
            }
            let mut current_subst = subst.clone();
            for (a1, a2) in args1.iter().zip(args2.iter()) {
                if let Some(new_subst) = unify(a1, a2, &current_subst) {
                    current_subst = new_subst;
                } else {
                    return None;
                }
            }
            Some(current_subst)
        }
        _ => None,
    }
}

fn bind(var: &str, val: &Value, subst: &Subst) -> Option<Subst> {
    if let Some(v_name) = is_var(val) {
        if v_name == var {
            return Some(subst.clone());
        }
    }
    // Occurs check skipped for simplicity
    let mut new_subst = subst.clone();
    new_subst.insert(var.to_string(), val.clone());
    Some(new_subst)
}

pub fn solve(
    goals: &[Value],
    subst: Subst,
    kb: &[Value],
    vm: &ChimeraVM,
    solutions: &mut Vec<Subst>,
    depth: usize,
) {
    if depth > 50 {
        return;
    } // Prevent infinite recursion

    if goals.is_empty() {
        solutions.push(subst);
        return;
    }

    let goal = &goals[0];
    let remaining_goals = &goals[1..];

    let resolved_goal = resolve(goal, &subst);

    // Dynamic Predicates Check
    if check_dynamic_predicates(
        &resolved_goal,
        remaining_goals,
        &subst,
        kb,
        vm,
        solutions,
        depth,
    ) {
        return;
    }

    for fact in kb {
        let (head, body) = parse_kb_entry(fact);

        // Rename variables in rule to be fresh to avoid collision with goal vars
        // Simple strategy: Append suffix based on depth
        // This is crucial for recursion (e.g. ancestor(A, B) :- parent(A, X), ancestor(X, B)).
        let fresh_head = rename_vars(&head, depth);
        let fresh_body: Vec<Value> = body.iter().map(|t| rename_vars(t, depth)).collect();

        if let Some(new_subst) = unify(&resolved_goal, &fresh_head, &subst) {
            let mut new_goals = fresh_body.clone();
            new_goals.extend_from_slice(remaining_goals);
            solve(&new_goals, new_subst, kb, vm, solutions, depth + 1);
        }
    }
}

fn check_dynamic_predicates(
    goal: &Value,
    remaining_goals: &[Value],
    subst: &Subst,
    kb: &[Value],
    vm: &ChimeraVM,
    solutions: &mut Vec<Subst>,
    depth: usize,
) -> bool {
    if let Value::Junction(JunctionType::Any, args) = goal {
        // We use Junction(Any, [Name, Args...]) as predicate format generally?
        // But tests use Junction(Any, [Pred, Arg1...]).
        if args.is_empty() {
            return false;
        }
        if let Value::Str(pred_name) = &args[0] {
            match pred_name.as_str() {
                "cell" => {
                    // cell(X, Y, Val)
                    if args.len() == 4 {
                        let arg_x = &args[1];
                        let arg_y = &args[2];
                        let arg_val = &args[3];

                        // Iterate over grid (0..16, 0..16)
                        for y in 0..crate::vm::GRID_SIZE {
                            for x in 0..crate::vm::GRID_SIZE {
                                let fact_x = Value::Int(x as i64);
                                let fact_y = Value::Int(y as i64);
                                let fact_val = vm.grid[y][x].clone();

                                // Try to unify X
                                if let Some(subst_x) = unify(arg_x, &fact_x, subst) {
                                    // Try to unify Y
                                    if let Some(subst_y) = unify(arg_y, &fact_y, &subst_x) {
                                        // Try to unify Val
                                        if let Some(final_subst) =
                                            unify(arg_val, &fact_val, &subst_y)
                                        {
                                            solve(
                                                remaining_goals,
                                                final_subst,
                                                kb,
                                                vm,
                                                solutions,
                                                depth + 1,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        return true; // Handled
                    }
                }
                "energy" => {
                    // energy(E)
                    if args.len() == 2 {
                        let arg_e = &args[1];
                        let fact_e = Value::Int(vm.energy);
                        if let Some(new_subst) = unify(arg_e, &fact_e, subst) {
                            solve(remaining_goals, new_subst, kb, vm, solutions, depth + 1);
                        }
                        return true;
                    }
                }
                "organelle" => {
                    // organelle(Idx, Type, X, Y)
                    // organelle(Name, Type, X, Y) maybe better if Name is unique? But idx is safer.
                    if args.len() == 5 {
                        #[cfg(feature = "nova")]
                        for (i, org) in vm.organelles.iter().enumerate() {
                            let fact_idx = Value::Int(i as i64);
                            let fact_type = Value::Str(format!("{:?}", org.kind));
                            let fact_x = Value::Int(org.context_loc.1 as i64);
                            let fact_y = Value::Int(org.context_loc.0 as i64);

                            let mut current_subst = subst.clone();
                            if let Some(s1) = unify(&args[1], &fact_idx, &current_subst) {
                                current_subst = s1;
                                if let Some(s2) = unify(&args[2], &fact_type, &current_subst) {
                                    current_subst = s2;
                                    if let Some(s3) = unify(&args[3], &fact_x, &current_subst) {
                                        current_subst = s3;
                                        if let Some(s4) = unify(&args[4], &fact_y, &current_subst) {
                                            solve(
                                                remaining_goals,
                                                s4,
                                                kb,
                                                vm,
                                                solutions,
                                                depth + 1,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        return true;
                    }
                }
                "past_cell" => {
                    // past_cell(Ticks, X, Y, Val)
                    #[cfg(feature = "nova")]
                    if args.len() == 5 {
                        let arg_t = &args[1];
                        let arg_x = &args[2];
                        let arg_y = &args[3];
                        let arg_val = &args[4];

                        let r_t = resolve(arg_t, subst);
                        let r_x = resolve(arg_x, subst);
                        let r_y = resolve(arg_y, subst);

                        let history_len = vm.grid_history.len();
                        let t_range = if let Value::Int(t) = r_t {
                            if t >= 0 && (t as usize) < history_len {
                                (t as usize)..(t as usize + 1)
                            } else {
                                0..0
                            }
                        } else {
                            0..history_len
                        };

                        let x_range = if let Value::Int(x) = r_x {
                            if x >= 0 && (x as usize) < crate::vm::GRID_SIZE {
                                (x as usize)..(x as usize + 1)
                            } else {
                                0..0
                            }
                        } else {
                            0..crate::vm::GRID_SIZE
                        };

                        let y_range = if let Value::Int(y) = r_y {
                            if y >= 0 && (y as usize) < crate::vm::GRID_SIZE {
                                (y as usize)..(y as usize + 1)
                            } else {
                                0..0
                            }
                        } else {
                            0..crate::vm::GRID_SIZE
                        };

                        for t in t_range {
                            let idx = history_len.saturating_sub(1).saturating_sub(t);
                            let grid_snapshot = &vm.grid_history[idx];
                            let fact_t = Value::Int(t as i64);

                            for y in y_range.clone() {
                                for x in x_range.clone() {
                                    let fact_x = Value::Int(x as i64);
                                    let fact_y = Value::Int(y as i64);
                                    let fact_val = grid_snapshot[y][x].clone();

                                    let mut current_subst = subst.clone();
                                    if let Some(s1) = unify(arg_t, &fact_t, &current_subst) {
                                        current_subst = s1;
                                        if let Some(s2) = unify(arg_x, &fact_x, &current_subst) {
                                            current_subst = s2;
                                            if let Some(s3) = unify(arg_y, &fact_y, &current_subst)
                                            {
                                                current_subst = s3;
                                                if let Some(s4) =
                                                    unify(arg_val, &fact_val, &current_subst)
                                                {
                                                    solve(
                                                        remaining_goals,
                                                        s4,
                                                        kb,
                                                        vm,
                                                        solutions,
                                                        depth + 1,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        return true;
                    }
                }
                "neighbor" => {
                    // neighbor(X, Y, Dir, NX, NY)
                    if args.len() == 6 {
                        let arg_x = &args[1];
                        let arg_y = &args[2];
                        let arg_dir = &args[3];
                        let arg_nx = &args[4];
                        let arg_ny = &args[5];

                        let r_x = resolve(arg_x, subst);
                        let r_y = resolve(arg_y, subst);

                        let x_range = if let Value::Int(x) = r_x {
                            if x >= 0 && (x as usize) < crate::vm::GRID_SIZE {
                                (x as usize)..(x as usize + 1)
                            } else {
                                0..0
                            }
                        } else {
                            0..crate::vm::GRID_SIZE
                        };

                        let y_range = if let Value::Int(y) = r_y {
                            if y >= 0 && (y as usize) < crate::vm::GRID_SIZE {
                                (y as usize)..(y as usize + 1)
                            } else {
                                0..0
                            }
                        } else {
                            0..crate::vm::GRID_SIZE
                        };

                        for y in y_range {
                            for x in x_range.clone() {
                                // 0=N, 1=E, 2=S, 3=W
                                let dirs = [(-1, 0, 0), (0, 1, 1), (1, 0, 2), (0, -1, 3)];
                                for (dy, dx, d_code) in dirs {
                                    #[allow(unused_assignments)]
                                    let mut neighbor_opt = None;

                                    #[cfg(any(feature = "nova", feature = "silicon"))]
                                    {
                                        neighbor_opt =
                                            vm.normalize_coords(y as i64 + dy, x as i64 + dx);
                                    }
                                    #[cfg(not(any(feature = "nova", feature = "silicon")))]
                                    {
                                        let ny = y as i64 + dy;
                                        let nx = x as i64 + dx;
                                        if ny >= 0
                                            && ny < crate::vm::GRID_SIZE as i64
                                            && nx >= 0
                                            && nx < crate::vm::GRID_SIZE as i64
                                        {
                                            neighbor_opt = Some((ny as usize, nx as usize));
                                        }
                                    }

                                    if let Some((ny, nx)) = neighbor_opt {
                                        let fact_x = Value::Int(x as i64);
                                        let fact_y = Value::Int(y as i64);
                                        let fact_dir = Value::Int(d_code);
                                        let fact_nx = Value::Int(nx as i64);
                                        let fact_ny = Value::Int(ny as i64);

                                        let mut current_subst = subst.clone();
                                        if let Some(s1) = unify(arg_x, &fact_x, &current_subst) {
                                            current_subst = s1;
                                            if let Some(s2) = unify(arg_y, &fact_y, &current_subst)
                                            {
                                                current_subst = s2;
                                                if let Some(s3) =
                                                    unify(arg_dir, &fact_dir, &current_subst)
                                                {
                                                    current_subst = s3;
                                                    if let Some(s4) =
                                                        unify(arg_nx, &fact_nx, &current_subst)
                                                    {
                                                        current_subst = s4;
                                                        if let Some(s5) =
                                                            unify(arg_ny, &fact_ny, &current_subst)
                                                        {
                                                            solve(
                                                                remaining_goals,
                                                                s5,
                                                                kb,
                                                                vm,
                                                                solutions,
                                                                depth + 1,
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        return true;
                    }
                }
                "opcode" => {
                    // opcode(Name, OpVal)
                    if args.len() == 3 {
                        let arg_name = &args[1];
                        let arg_val = &args[2];

                        for op in OpCode::iter() {
                            let op_name = op.to_string();
                            let fact_name = Value::Str(op_name.clone());
                            let fact_val = Value::Str(op_name); // For now, Val is same as Name

                            let mut current_subst = subst.clone();
                            if let Some(s1) = unify(arg_name, &fact_name, &current_subst) {
                                if let Some(s2) = unify(arg_val, &fact_val, &s1) {
                                    solve(remaining_goals, s2, kb, vm, solutions, depth + 1);
                                }
                            }
                        }
                        return true;
                    }
                }
                _ => {}
            }
        }
    }
    false
}

fn parse_kb_entry(entry: &Value) -> (Value, Vec<Value>) {
    // Check if it's a rule: any("rule", Head, Body...)
    if let Value::Junction(JunctionType::Any, ref args) = entry {
        if !args.is_empty() {
            if let Value::Str(ref s) = args[0] {
                if s == "rule" && args.len() >= 2 {
                    return (args[1].clone(), args[2..].to_vec());
                }
            }
        }
    }
    (entry.clone(), Vec::new())
}

fn rename_vars(val: &Value, suffix: usize) -> Value {
    match val {
        Value::Str(s) if s.starts_with('?') => Value::Str(format!("{}_{}", s, suffix)),
        Value::Junction(t, args) => {
            Value::Junction(*t, args.iter().map(|a| rename_vars(a, suffix)).collect())
        }
        _ => val.clone(),
    }
}

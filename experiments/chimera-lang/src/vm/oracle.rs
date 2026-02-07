#![cfg(feature = "oracle")]
use super::{ChimeraVM, Value};
use crate::ast::{JunctionType, Nucleotide};
use crate::opcode::OpCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
                vm.output.push("Error: Stack underflow for rule".to_string());
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
                solve(&goals, HashMap::new(), &vm.knowledge_base, &mut solutions, 0);

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
                            let pair = Value::Junction(JunctionType::All, vec![Value::Str(k.clone()), v.clone()]);
                            binding_list.push(pair);
                        }
                        vm.stack.push(Value::Junction(JunctionType::All, binding_list));
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

fn solve(goals: &[Value], subst: Subst, kb: &[Value], solutions: &mut Vec<Subst>, depth: usize) {
    if depth > 50 { return; } // Prevent infinite recursion

    if goals.is_empty() {
        solutions.push(subst);
        return;
    }

    let goal = &goals[0];
    let remaining_goals = &goals[1..];

    let resolved_goal = resolve(goal, &subst);

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
            solve(&new_goals, new_subst, kb, solutions, depth + 1);
        }
    }
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
        Value::Str(s) if s.starts_with('?') => {
            Value::Str(format!("{}_{}", s, suffix))
        }
        Value::Junction(t, args) => {
            Value::Junction(*t, args.iter().map(|a| rename_vars(a, suffix)).collect())
        }
        _ => val.clone(),
    }
}

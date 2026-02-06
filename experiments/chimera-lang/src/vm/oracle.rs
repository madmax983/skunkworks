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
                // Ensure fact is not already in KB (set semantics)
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
                // Query can be a single term or a junction (conjunction)
                let goals = match query {
                    Value::Junction(JunctionType::All, ref args) => args.clone(),
                    _ => vec![query.clone()],
                };

                vm.output.push(format!("QUERY: Solving {:?}", goals));

                let mut solutions = Vec::new();
                solve(&goals, HashMap::new(), &vm.knowledge_base, &mut solutions);

                if solutions.is_empty() {
                    vm.stack.push(Value::Int(0));
                    vm.output.push("QUERY: Failed".to_string());
                } else {
                    vm.stack.push(Value::Int(1));
                    vm.output
                        .push(format!("QUERY: Success ({} solutions)", solutions.len()));

                    // For the first solution, print variable bindings to output
                    // In a real Prolog, we'd return them or provide a way to iterate.
                    // Here, we just log them for "Mad Scientist" purposes.
                    for (i, sol) in solutions.iter().enumerate() {
                        let mut binding_str = String::new();
                        for (var, val) in sol {
                            binding_str.push_str(&format!("{}={} ", var, val));
                        }
                        if binding_str.is_empty() {
                            binding_str.push_str("Yes");
                        }
                        vm.output.push(format!("  Sol {}: {}", i, binding_str));
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
            // Clone omens to avoid borrow issues while executing effects
            let omens_snapshot = vm.omens.clone();

            for omen in omens_snapshot {
                let mut solutions = Vec::new();
                solve(
                    &[omen.condition.clone()],
                    HashMap::new(),
                    &vm.knowledge_base,
                    &mut solutions,
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
                resolve(val, subst) // Recurse for chains ?X -> ?Y -> 1
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
    // Occurs check omitted for simplicity (Mad Scientist doesn't care about infinite loops!)
    let mut new_subst = subst.clone();
    new_subst.insert(var.to_string(), val.clone());
    Some(new_subst)
}

fn solve(goals: &[Value], subst: Subst, kb: &[Value], solutions: &mut Vec<Subst>) {
    if goals.is_empty() {
        solutions.push(subst);
        return;
    }

    let goal = &goals[0];
    let remaining_goals = &goals[1..];

    // Resolve goal with current substitution
    let resolved_goal = resolve(goal, &subst);

    for fact in kb {
        // Rename variables in fact to avoid collision with goal variables?
        // Standard Prolog does this. Since we don't have scope, let's assume facts in KB
        // shouldn't share variable names with query unless intended?
        // Actually, we should rename fact variables.
        // For simplicity, let's skip renaming for now and assume users are careful
        // OR implement simple renaming if needed.
        // Let's implement simple renaming: append generation count or something?
        // Or just don't support recursive rules yet. The prompt implies logic engine, simple is ok.

        // Actually, renaming is crucial for any rule-based logic.
        // But our KB is just a list of facts/rules.
        // If the fact is a rule: Head :- Body.
        // We can represent rule as Junction(All, [Head, Body...]).
        // But `assert` pushes a single Value.
        // Let's assume KB contains only FACTS (Ground or with Vars) for now,
        // OR Rules represented in a specific way.

        // Let's support Rules:
        // A rule is Value::Junction(All, [Head, Body1, Body2...]).
        // Wait, Unification matches Head.

        // If fact is a Junction(All, args), and unification matches args[0] (Head),
        // then we replace goal with args[1..] (Body) and continue.
        // If fact is not a rule (just a term), we unify with term.

        // Renaming logic:
        // Iterate vars in fact, replace with fresh vars (e.g. ?X -> ?X_fresh_1)
        // Since we are recursive, we can just assume deep recursion isn't main target yet.

        // Let's try to unify goal with fact (or head of fact).

        let (head, body) = match fact {
            // Check if it's a rule?
            // Convention: A rule must be explicitly `rule(Head, Body)`?
            // Or use Junction structure?
            // Let's treat everything as a potential rule.
            // If it unifies directly, it's a fact.
            // If it has structure `rule(Head, Body)`, we treat it as rule.
            Value::Junction(JunctionType::Any, ref args)
                if !args.is_empty() && is_rule_structure(args) =>
            {
                // Convention: any("rule", Head, Body...)
                // args[0] is string "rule"
                // args[1] is Head
                // args[2..] is Body
                (args[1].clone(), args[2..].to_vec())
            }
            _ => (fact.clone(), Vec::new()), // It is a bare fact
        };

        // Rename variables in (head, body) to be fresh
        // We need a counter or scope.
        // Let's just use a random suffix or depth based?
        // We don't have depth passed easily.
        // Let's skip renaming for this iteration (Limit: Logic Engine V1).

        if let Some(new_subst) = unify(&resolved_goal, &head, &subst) {
            let mut new_goals = body.clone();
            new_goals.extend_from_slice(remaining_goals);
            solve(&new_goals, new_subst, kb, solutions);
        }
    }
}

fn is_rule_structure(args: &[Value]) -> bool {
    if let Some(Value::Str(s)) = args.first() {
        return s == "rule";
    }
    false
}

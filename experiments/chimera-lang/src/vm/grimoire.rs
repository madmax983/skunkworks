#![cfg(feature = "nova")]
use crate::ast::JunctionType;
use crate::vm::Value;

pub fn load_standard_library() -> Vec<Value> {
    let mut kb = Vec::new();

    // Helper to create simple fact: predicate(A, B, C...)
    let fact = |name: &str, args: Vec<&str>| -> Value {
        let mut vals = vec![Value::Str(name.to_string())];
        for arg in args {
            vals.push(Value::Str(arg.to_string()));
        }
        Value::Junction(JunctionType::Any, vals)
    };

    // Alchemy Recipes
    // recipe(Ing1, Ing2, Result)
    kb.push(fact("recipe", vec!["Fire", "Water", "Steam"]));
    kb.push(fact("recipe", vec!["Earth", "Fire", "Lava"]));
    kb.push(fact("recipe", vec!["Air", "Water", "Cloud"]));
    kb.push(fact("recipe", vec!["Life", "Death", "Spirit"]));
    kb.push(fact("recipe", vec!["Energy", "Lead", "Gold"]));

    // Rule Helpers
    let var = |s: &str| Value::Str(s.to_string());
    let pred = |name: &str, args: Vec<Value>| -> Value {
        let mut v = vec![Value::Str(name.to_string())];
        v.extend(args);
        Value::Junction(JunctionType::Any, v)
    };

    // Rule: reaction(A, B, C) :- recipe(A, B, C).
    let rule_direct = Value::Junction(
        JunctionType::Any,
        vec![
            Value::Str("rule".to_string()),
            pred("reaction", vec![var("?A"), var("?B"), var("?C")]),
            pred("recipe", vec![var("?A"), var("?B"), var("?C")]),
        ],
    );
    kb.push(rule_direct);

    // Rule: reaction(A, B, C) :- recipe(B, A, C). (Commutative)
    let rule_reverse = Value::Junction(
        JunctionType::Any,
        vec![
            Value::Str("rule".to_string()),
            pred("reaction", vec![var("?A"), var("?B"), var("?C")]),
            pred("recipe", vec![var("?B"), var("?A"), var("?C")]),
        ],
    );
    kb.push(rule_reverse);

    // Fun: What happens if we mix Life and Void?
    // recipe("Life", "Void", "Zombie")
    kb.push(fact("recipe", vec!["Life", "Void", "Zombie"]));

    kb
}

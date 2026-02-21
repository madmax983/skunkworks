use chimera_lang::vm::Value;
use chimera_lang::ast::JunctionType;

#[test]
fn test_deep_display_crash() {
    let mut v = Value::Int(0);
    // Build a 20,000 deep nested structure
    for _ in 0..20000 {
        v = Value::Junction(JunctionType::Any, vec![v]);
    }

    // This should panic/crash if Display is unbounded
    // We catch unwind to detect it (though stack overflow often aborts process)
    let _ = format!("{}", v);
}

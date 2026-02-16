#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::nova_babel_live::exec_live_parse;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_babel_live_string_match() {
        let mut vm = make_vm();
        // Setup grid: "foo"!
        vm.grid[0][0] = Value::Str("\"".to_string());
        vm.grid[0][1] = Value::Str("f".to_string());
        vm.grid[0][2] = Value::Str("o".to_string());
        vm.grid[0][3] = Value::Str("o".to_string());
        vm.grid[0][4] = Value::Str("\"".to_string());
        vm.grid[0][5] = Value::Str("!".to_string());

        let result = exec_live_parse(&mut vm, 0, 0, "foo".to_string());
        assert!(result, "Expected parse to succeed for 'foo'");

        let trace_len = vm.babel_live_trace.len();
        assert!(trace_len > 0, "Trace should not be empty");
    }

    #[test]
    fn test_babel_live_string_mismatch() {
        let mut vm = make_vm();
        // Setup grid: "foo"!
        vm.grid[0][0] = Value::Str("\"".to_string());
        vm.grid[0][1] = Value::Str("f".to_string());
        vm.grid[0][2] = Value::Str("o".to_string());
        vm.grid[0][3] = Value::Str("o".to_string());
        vm.grid[0][4] = Value::Str("\"".to_string());
        vm.grid[0][5] = Value::Str("!".to_string());

        let result = exec_live_parse(&mut vm, 0, 0, "bar".to_string());
        assert!(!result, "Expected parse to fail for 'bar'");
    }

    #[test]
    fn test_babel_live_regex() {
        let mut vm = make_vm();
        // Setup grid: [a-z]+!
        // Correct test: [a-z+]
        vm.grid[1][0] = Value::Str("[".to_string());
        vm.grid[1][1] = Value::Str("a".to_string());
        vm.grid[1][2] = Value::Str("-".to_string());
        vm.grid[1][3] = Value::Str("z".to_string());
        vm.grid[1][4] = Value::Str("]".to_string());

        vm.grid[2][0] = Value::Str("[".to_string());
        vm.grid[2][1] = Value::Str("a".to_string());
        vm.grid[2][2] = Value::Str("-".to_string());
        vm.grid[2][3] = Value::Str("z".to_string());
        vm.grid[2][4] = Value::Str("+".to_string());
        vm.grid[2][5] = Value::Str("]".to_string());
        vm.grid[2][6] = Value::Str("!".to_string());

        let result = exec_live_parse(&mut vm, 2, 0, "hello".to_string());
        assert!(result, "Expected regex parse to succeed for 'hello'");
    }

    #[test]
    fn test_babel_live_action() {
        let mut vm = make_vm();
        // Setup grid: { 1 1 add }!
        vm.grid[0][0] = Value::Str("{".to_string());
        vm.grid[0][1] = Value::Str("1".to_string());
        vm.grid[0][2] = Value::Str(" ".to_string());
        vm.grid[0][3] = Value::Str("1".to_string());
        vm.grid[0][4] = Value::Str(" ".to_string());
        vm.grid[0][5] = Value::Str("a".to_string());
        vm.grid[0][6] = Value::Str("d".to_string());
        vm.grid[0][7] = Value::Str("d".to_string());
        vm.grid[0][8] = Value::Str("}".to_string());
        vm.grid[0][9] = Value::Str("!".to_string());

        // Empty input matches empty string requirement, actions don't consume input
        let result = exec_live_parse(&mut vm, 0, 0, "".to_string());
        assert!(result, "Expected action parse to succeed");

        assert_eq!(vm.stack.len(), 1, "Stack should have result");
        if let Value::Int(n) = vm.stack[0] {
            assert_eq!(n, 2, "Expected 1+1=2");
        } else {
            panic!("Expected Int(2) on stack");
        }
    }
}

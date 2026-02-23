use chimera_lang::vm::prologue::logos::LogosEngine;
use chimera_lang::vm::Value;

#[test]
fn test_logos_regex_parsing() {
    let mut engine = LogosEngine::new();

    // Define a rule using regex
    // Rule "digit": /\d+/
    engine.define_rule("digit", r"/\d+/");

    // Parse "123"
    let result = engine.parse_input("digit", "123");
    assert!(result.is_ok());
    if let Ok(val) = result {
        assert_eq!(val, Value::Str("123".to_string()));
    }

    // Parse "abc" -> Should fail
    let fail = engine.parse_input("digit", "abc");
    assert!(fail.is_err());
}

#[test]
fn test_logos_choice_parsing() {
    let mut engine = LogosEngine::new();

    // Rule "choice": "A" | "B"
    engine.define_rule("choice", "\"A\" | \"B\"");

    // Parse "A"
    let result_a = engine.parse_input("choice", "A");
    assert!(result_a.is_ok());
    if let Ok(val) = result_a {
        assert_eq!(val, Value::Str("A".to_string()));
    }

    // Parse "B"
    let result_b = engine.parse_input("choice", "B");
    assert!(result_b.is_ok());
    if let Ok(val) = result_b {
        assert_eq!(val, Value::Str("B".to_string()));
    }

    // Parse "C" -> Fail
    let result_c = engine.parse_input("choice", "C");
    assert!(result_c.is_err());
}

#[test]
fn test_logos_weighted_choice_parsing() {
    let mut engine = LogosEngine::new();

    // Rule "wchoice": 10:"A" | 1:"B"
    // Parsing should ignore weights and just try both
    engine.define_rule("wchoice", "10:\"A\" | 1:\"B\"");

    // Parse "A"
    let result_a = engine.parse_input("wchoice", "A");
    assert!(result_a.is_ok());

    // Parse "B"
    let result_b = engine.parse_input("wchoice", "B");
    assert!(result_b.is_ok());
}

#[test]
fn test_logos_sequence_regex() {
    let mut engine = LogosEngine::new();

    // Rule "seq": /\d+/ "end"
    engine.define_rule("seq", r"/\d+/ end"); // "end" is a reference if no quotes, but we parse single token reference.
                                             // Wait, parse_single_token treats non-quoted as Reference. "end" must be defined.
                                             // Or we quote it: "/\d+/ \"end\""
    engine.define_rule("end_rule", "\"end\"");
    engine.define_rule("seq", r"/\d+/ end_rule");

    // Parse "42 end"
    let result = engine.parse_input("seq", "42 end");
    assert!(result.is_ok());

    // Check junction result
    if let Ok(Value::Junction(_, vals)) = result {
        assert_eq!(vals.len(), 2);
        assert_eq!(vals[0], Value::Str("42".to_string()));
        assert_eq!(vals[1], Value::Str("end".to_string()));
    } else {
        panic!("Expected Junction");
    }
}

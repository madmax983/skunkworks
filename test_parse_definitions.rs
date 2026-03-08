use std::collections::HashMap;

fn main() {
    let mut custom_runes: HashMap<String, usize> = HashMap::new();
    let content1 = r#"
        strand alpha {
            "Alpha Triggered" print
        }
    "#;
    let content2 = r#"
        strand beta {
            "Beta Triggered" print
        }
    "#;

    // This simulates the behavior inside `prologue_compiler::compile`:

    // Process Rune 'A'
    let wrapped_content1 = format!("strand rune_{} {{ {} }}", custom_runes.len(), content1);
    println!("wrapped_content1:\n{}", wrapped_content1);
    custom_runes.insert("A".to_string(), 0);

    // Process Rune 'B'
    let wrapped_content2 = format!("strand rune_{} {{ {} }}", custom_runes.len(), content2);
    println!("wrapped_content2:\n{}", wrapped_content2);
    custom_runes.insert("B".to_string(), 1);
}

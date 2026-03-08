fn main() {
    let content = r#"
        strand alpha {
            "Alpha Triggered" print
        }
    "#;
    let wrapped_content = format!("strand rune_{} {{ {} }}", 0, content);
    println!("{}", wrapped_content);
}

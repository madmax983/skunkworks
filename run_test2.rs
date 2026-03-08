fn main() {
    let content = r#"
        strand alpha {
            "Alpha Triggered" print
        }
    "#;
    let trimmed = content.trim_matches(|c| c == '\n' || c == '\r' || c == '{' || c == '}');
    println!("{}", trimmed);
}

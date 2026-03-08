fn main() {
    let src = r#"
        strand rune_0 {
            strand beta {
                "Beta Triggered" print
            }
        }
    "#;
    println!("We are trying to compile:\n{}", src);
}

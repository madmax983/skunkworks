use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub name: String,
    pub inputs: Vec<String>,
    pub output: String,
    pub file_path: String,
}

pub fn harvest_functions(root: &str) -> Vec<FunctionSignature> {
    let mut signatures = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().is_some_and(|ext| ext == "rs") {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                signatures.extend(parse_file_content(&content, entry.path()));
            }
        }
    }
    signatures
}

fn parse_file_content(content: &str, path: &Path) -> Vec<FunctionSignature> {
    let mut sigs = Vec::new();
    let re = Regex::new(r"fn\s+(\w+)(?:<[^>]+>)?\s*\(([^)]*)\)\s*(?:->\s*([^{]+))?\s*\{").unwrap();
    let path_str = path.to_string_lossy().to_string();

    for cap in re.captures_iter(content) {
        let name = cap[1].to_string();
        let args_str = &cap[2];
        let ret_str = cap.get(3).map(|m| m.as_str()).unwrap_or("()");

        let inputs = parse_args(args_str);
        let output = clean_type(ret_str);

        sigs.push(FunctionSignature {
            name,
            inputs,
            output,
            file_path: path_str.clone(),
        });
    }
    sigs
}

fn parse_args(args: &str) -> Vec<String> {
    if args.trim().is_empty() {
        return Vec::new();
    }
    args.split(',')
        .filter_map(|arg| {
            // arg is like "x: i32" or "&self"
            let parts: Vec<&str> = arg.split(':').collect();
            if parts.len() == 2 {
                Some(clean_type(parts[1]))
            } else {
                // Ignore self, &self, etc for food?
                // Or make them "Self" food?
                // Let's ignore self for now as it makes the graph too connected to itself
                None
            }
        })
        .collect()
}

fn clean_type(type_str: &str) -> String {
    let mut t = type_str.trim().to_string();

    // Remove ref modifiers
    t = t.replace("&mut ", "").replace("&", "").replace("mut ", "");

    // Simple recursion for generics like Vec<String> -> String
    // logic: keep stripping outer wrappers until we hit a base word
    // This is a heuristic for the simulation
    let wrappers = [
        "Vec", "Option", "Result", "Box", "Rc", "Arc", "RefCell", "Mutex",
    ];

    let mut changed = true;
    while changed {
        changed = false;
        for w in wrappers {
            if t.starts_with(&format!("{}<", w)) && t.ends_with('>') {
                // Extract inner
                // Vec<String> -> String
                // Result<T, E> -> T (just take first for simplicity)
                if let Some(start) = t.find('<') {
                    if let Some(end) = t.rfind('>') {
                        let inner = &t[start + 1..end];
                        // If there is a comma (Result<T, E>), take first part
                        let inner_first = inner.split(',').next().unwrap_or(inner);
                        t = inner_first.trim().to_string();
                        changed = true;
                    }
                }
            }
        }
    }

    // Remove lifetimes 'a
    // Remove whitespace
    t.replace(' ', "")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_parsing() {
        let code = r#"
            fn simple(x: i32, y: f64) -> bool {
            fn no_args() {
            fn with_generics<T>(input: Vec<T>) -> Option<T> {
            fn complex(a: &str, b: &mut String) -> Result<Vec<u8>, Error> {
        "#;

        let path = Path::new("dummy.rs");
        let sigs = parse_file_content(code, path);

        assert_eq!(sigs.len(), 4);

        // simple
        assert_eq!(sigs[0].name, "simple");
        assert_eq!(sigs[0].inputs, vec!["i32", "f64"]);
        assert_eq!(sigs[0].output, "bool");

        // no_args
        assert_eq!(sigs[1].name, "no_args");
        assert!(sigs[1].inputs.is_empty());
        assert_eq!(sigs[1].output, ""); // clean_type("()") -> "" or "()" depending on impl.
                                        // My impl removes non-alphanumeric so "()" becomes ""
                                        // Let's fix that or accept it.
                                        // Empty string usually means Void/Unit in this sim.

        // with_generics
        assert_eq!(sigs[2].name, "with_generics");
        assert_eq!(sigs[2].inputs, vec!["T"]);
        assert_eq!(sigs[2].output, "T");

        // complex
        assert_eq!(sigs[3].name, "complex");
        assert_eq!(sigs[3].inputs, vec!["str", "String"]);
        assert_eq!(sigs[3].output, "u8");
    }

    #[test]
    fn test_clean_type() {
        assert_eq!(clean_type("String"), "String");
        assert_eq!(clean_type("&str"), "str");
        assert_eq!(clean_type("Vec<i32>"), "i32");
        assert_eq!(clean_type("Result<Option<String>, Error>"), "String");
        assert_eq!(clean_type("()"), "");
    }
}

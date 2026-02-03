use crate::phonology::{EvolutionTrace, Evolver};
use std::collections::{HashMap, HashSet};
use syn::visit_mut::VisitMut;
use syn::{File, Ident};

pub struct CodeEvolver {
    evolver: Evolver,
    cache: HashMap<String, String>,
    pub traces: HashMap<String, EvolutionTrace>,
    reserved: HashSet<String>,
}

impl CodeEvolver {
    pub fn new(evolver: Evolver) -> Self {
        let reserved = vec![
            // Keywords
            "as",
            "break",
            "const",
            "continue",
            "crate",
            "else",
            "enum",
            "extern",
            "false",
            "fn",
            "for",
            "if",
            "impl",
            "in",
            "let",
            "loop",
            "match",
            "mod",
            "move",
            "mut",
            "pub",
            "ref",
            "return",
            "self",
            "Self",
            "static",
            "struct",
            "super",
            "trait",
            "true",
            "type",
            "unsafe",
            "use",
            "where",
            "while",
            "async",
            "await",
            "dyn",
            // Types & Common std
            "Option",
            "Result",
            "String",
            "Vec",
            "Ok",
            "Err",
            "Some",
            "None",
            "main",
            "println",
            "format",
            "vec",
            "Box",
            "u8",
            "u16",
            "u32",
            "u64",
            "u128",
            "i8",
            "i16",
            "i32",
            "i64",
            "i128",
            "f32",
            "f64",
            "bool",
            "char",
            "usize",
            "isize",
            "str",
            "std",
            "core",
            "alloc",
            "io",
            "fmt",
            "Debug",
            "Display",
            "Clone",
            "Copy",
            "PartialEq",
            "Eq",
            "derive",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        Self {
            evolver,
            cache: HashMap::new(),
            traces: HashMap::new(),
            reserved,
        }
    }

    fn evolve_ident(&mut self, i: &Ident) -> Ident {
        let s = i.to_string();

        // Skip reserved words
        if self.reserved.contains(&s) {
            return i.clone();
        }

        // Check if already evolved
        if let Some(evolved) = self.cache.get(&s) {
            return Ident::new(evolved, i.span());
        }

        // Evolve
        let (evolved_str, trace) = self.evolver.evolve_with_trace(&s);

        // Fix identifier validity:
        // 1. Cannot be empty
        // 2. Cannot start with a number.
        // 3. Must not be a keyword

        let mut valid_evolved = evolved_str.clone();
        if valid_evolved.is_empty() {
            valid_evolved = format!("_{}", s); // Fallback
        } else if valid_evolved
            .chars()
            .next()
            .map_or(false, |c| c.is_digit(10))
        {
            valid_evolved = format!("_{}", valid_evolved);
        }

        // If the evolution accidentally creates a keyword, append '_'
        if self.reserved.contains(&valid_evolved) {
            valid_evolved.push('_');
        }

        self.traces.insert(valid_evolved.clone(), trace);
        self.cache.insert(s, valid_evolved.clone());
        Ident::new(&valid_evolved, i.span())
    }
}

impl VisitMut for CodeEvolver {
    fn visit_ident_mut(&mut self, i: &mut Ident) {
        *i = self.evolve_ident(i);
    }
}

pub fn evolve_code(
    code: &str,
    evolver: Evolver,
) -> anyhow::Result<(String, HashMap<String, EvolutionTrace>)> {
    let mut file: File = syn::parse_str(code)?;
    let mut code_evolver = CodeEvolver::new(evolver);
    code_evolver.visit_file_mut(&mut file);
    Ok((prettyplease::unparse(&file), code_evolver.traces))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phonology::SoundLaw;

    #[test]
    fn test_evolve_fn_name() {
        let code = "fn pater() {}";
        let mut evolver = Evolver::new();
        evolver.add_law(SoundLaw::GrimmsLaw);

        let (evolved, _) = evolve_code(code, evolver).unwrap();
        assert!(evolved.contains("fn father()"));
    }

    #[test]
    fn test_evolve_var_name() {
        let code = "fn main() { let pater = 10; }";
        let mut evolver = Evolver::new();
        evolver.add_law(SoundLaw::GrimmsLaw);

        let (evolved, _) = evolve_code(code, evolver).unwrap();
        assert!(evolved.contains("let father = 10;"));
    }

    #[test]
    fn test_skips_keywords() {
        let code = "fn main() { let mut x = 0; }";
        let mut evolver = Evolver::new();
        evolver.add_law(SoundLaw::GrimmsLaw); // x is not affected really, but let's see.

        let (evolved, _) = evolve_code(code, evolver).unwrap();
        // main should be preserved
        assert!(evolved.contains("fn main()"));
        // let, mut should be preserved
        assert!(evolved.contains("let mut"));
    }

    #[test]
    fn test_returns_trace() {
        let code = "fn pater() {}";
        let mut evolver = Evolver::new();
        evolver.add_law(SoundLaw::GrimmsLaw);

        let (_, traces) = evolve_code(code, evolver).unwrap();
        assert!(traces.contains_key("father"));
        let trace = &traces["father"];
        assert_eq!(trace.steps.len(), 1);
        assert_eq!(trace.steps[0].1, "father");
    }
}

pub mod phonology;
pub mod obfuscator;
pub mod tui;

#[cfg(test)]
mod tests {
    use super::phonology::*;
    use super::obfuscator::*;
    use syn::{visit::Visit, visit_mut::VisitMut};
    use std::collections::HashSet;

    #[test]
    fn test_grimms_law() {
        let engine = PhonologyEngine::grimms_law();
        // pater -> fater -> father
        let res = engine.evolve("pater");
        assert_eq!(res, "father");
    }

    #[test]
    fn test_obfuscation() {
        let code = r#"
            fn calculate_velocity(dist: f32, time: f32) -> f32 {
                let result = dist / time;
                result
            }
        "#;
        let ast = syn::parse_file(code).unwrap();

        let mut finder = DefinitionFinder::new();
        finder.visit_file(&ast);

        assert!(finder.identifiers.contains("calculate_velocity"));

        let engine = PhonologyEngine::grimms_law();
        let mut obfuscator = Obfuscator::new(&finder.identifiers, &engine);

        let mut new_ast = ast.clone();
        obfuscator.visit_file_mut(&mut new_ast);

        let new_code = quote::quote!(#new_ast).to_string();
        println!("New code: {}", new_code);

        assert!(new_code.contains("thime"));
        assert!(new_code.contains("tist"));
    }
}

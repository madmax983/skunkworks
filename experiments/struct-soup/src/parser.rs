use anyhow::Result;
use std::fs;
use std::path::Path;
use syn::visit::Visit;
use syn::{GenericArgument, ItemEnum, ItemStruct, PathArguments, Type};
use walkdir::WalkDir;

#[derive(Debug, Default)]
pub struct Scanner {
    pub structs: Vec<StructInfo>,
}

#[derive(Debug, Clone)]
pub struct StructInfo {
    pub name: String,
    pub fields: Vec<String>, // List of types this struct depends on
}

impl<'ast> Visit<'ast> for Scanner {
    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        let name = i.ident.to_string();
        let mut fields = Vec::new();

        for field in &i.fields {
            if let Some(type_name) = extract_type_name(&field.ty) {
                fields.push(type_name);
            }
        }

        self.structs.push(StructInfo { name, fields });

        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        let name = i.ident.to_string();
        let mut fields = Vec::new();

        for variant in &i.variants {
            for field in &variant.fields {
                if let Some(type_name) = extract_type_name(&field.ty) {
                    fields.push(type_name);
                }
            }
        }
        self.structs.push(StructInfo { name, fields });
    }
}

fn extract_type_name(ty: &Type) -> Option<String> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            // If it has generics, try to dive in
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                // Start with the outer type (e.g., Vec, Option)
                // Actually, for visualization, "Vec" is boring. We want what's INSIDE.
                // But if we have `MyWrapper<T>`, we might want `MyWrapper`.
                // Heuristic: If it's a standard collection, dive in.

                let ident = segment.ident.to_string();
                if [
                    "Vec", "Option", "Result", "Box", "Arc", "Rc", "Mutex", "RwLock", "Cell",
                    "RefCell",
                ]
                .contains(&ident.as_str())
                {
                    if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                        return extract_type_name(inner_type);
                    }
                }
            }

            return Some(segment.ident.to_string());
        }
    }
    None
}

pub fn scan_workspace(root: impl AsRef<Path>) -> Result<Vec<StructInfo>> {
    let mut scanner = Scanner::default();

    for entry in WalkDir::new(root) {
        let entry = entry?;
        if entry.file_type().is_file() && entry.path().extension().is_some_and(|e| e == "rs") {
            // println!("Scanning {:?}", entry.path());
            let content = match fs::read_to_string(entry.path()) {
                Ok(c) => c,
                Err(_) => continue,
            };

            // Parse file. If it fails (syntax error), just skip it.
            if let Ok(file) = syn::parse_file(&content) {
                scanner.visit_file(&file);
            }
        }
    }

    Ok(scanner.structs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_type_name() {
        // We can't easily construct syn::Type types manually without parsing
        // So let's parse a small string
        let code = r#"
            struct Foo {
                a: Vec<Bar>,
                b: Option<Baz>,
                c: i32,
                d: Box<qux::Quux>,
            }
        "#;

        let file = syn::parse_file(code).unwrap();
        let mut scanner = Scanner::default();
        scanner.visit_file(&file);

        let info = &scanner.structs[0];
        assert_eq!(info.name, "Foo");

        // a: Vec<Bar> -> Bar
        assert!(info.fields.contains(&"Bar".to_string()));
        // b: Option<Baz> -> Baz
        assert!(info.fields.contains(&"Baz".to_string()));
        // c: i32 -> i32
        assert!(info.fields.contains(&"i32".to_string()));
        // d: Box<qux::Quux> -> Quux (last segment)
        assert!(info.fields.contains(&"Quux".to_string()));
    }
}

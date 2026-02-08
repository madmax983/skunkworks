use anyhow::{Context, Result};
use proc_macro2::LineColumn;
use std::path::Path;
use syn::{visit::Visit, ItemFn, ItemImpl, ItemStruct};

#[derive(Debug, Clone)]
pub enum EntityKind {
    Function,
    Struct,
    Impl,
    #[allow(dead_code)]
    Other,
}

#[derive(Debug, Clone)]
pub struct CodeEntity {
    pub kind: EntityKind,
    pub name: String,
    pub start: LineColumn,
    pub end: LineColumn,
}

struct EntityVisitor {
    entities: Vec<CodeEntity>,
}

impl EntityVisitor {
    fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for EntityVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let name = node.sig.ident.to_string();
        // We want the whole function body span actually, or at least the brace span?
        // syn::spanned::Spanned is needed but Span gives us start/end.
        // Wait, node.span() covers the whole item.
        let full_span = syn::spanned::Spanned::span(node);

        self.entities.push(CodeEntity {
            kind: EntityKind::Function,
            name,
            start: full_span.start(),
            end: full_span.end(),
        });

        // Continue visiting children (nested functions?)
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        let name = node.ident.to_string();
        let full_span = syn::spanned::Spanned::span(node);

        self.entities.push(CodeEntity {
            kind: EntityKind::Struct,
            name,
            start: full_span.start(),
            end: full_span.end(),
        });

        syn::visit::visit_item_struct(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        let name = if let Some((_, trait_path, _)) = &node.trait_ {
             // Trait impl
             if let Some(last) = trait_path.segments.last() {
                 last.ident.to_string()
             } else {
                 "Impl".to_string()
             }
        } else if let syn::Type::Path(type_path) = &*node.self_ty {
            // Inherent impl
            if let Some(last) = type_path.path.segments.last() {
                last.ident.to_string()
            } else {
                "Impl".to_string()
            }
        } else {
            "Impl".to_string()
        };

        let full_span = syn::spanned::Spanned::span(node);

        self.entities.push(CodeEntity {
            kind: EntityKind::Impl,
            name: format!("impl {}", name),
            start: full_span.start(),
            end: full_span.end(),
        });

        syn::visit::visit_item_impl(self, node);
    }
}

pub fn parse_file(path: &Path) -> Result<Vec<CodeEntity>> {
    let content = std::fs::read_to_string(path).context("Failed to read file")?;
    let file = syn::parse_file(&content).context("Failed to parse file")?;

    let mut visitor = EntityVisitor::new();
    visitor.visit_file(&file);

    Ok(visitor.entities)
}

// Also return the raw lines for display
pub fn read_lines(path: &Path) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(path)?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let code = r#"
            fn hello() {
                println!("Hello");
            }
        "#;
        let file = syn::parse_file(code).unwrap();
        let mut visitor = EntityVisitor::new();
        visitor.visit_file(&file);

        assert_eq!(visitor.entities.len(), 1);
        let entity = &visitor.entities[0];
        assert_eq!(entity.name, "hello");
        assert!(matches!(entity.kind, EntityKind::Function));
    }

    #[test]
    fn test_parse_struct() {
        let code = r#"
            struct MyStruct {
                field: i32,
            }
        "#;
        let file = syn::parse_file(code).unwrap();
        let mut visitor = EntityVisitor::new();
        visitor.visit_file(&file);

        assert_eq!(visitor.entities.len(), 1);
        let entity = &visitor.entities[0];
        assert_eq!(entity.name, "MyStruct");
        assert!(matches!(entity.kind, EntityKind::Struct));
    }
}

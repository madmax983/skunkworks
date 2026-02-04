use anyhow::Result;
use std::fs;
use std::path::Path;
use syn::{visit, visit::Visit, Block, ItemEnum, ItemFn, ItemImpl, ItemMacro, ItemMod, ItemStruct};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    File,
    Mod,
    Fn,
    Struct,
    Enum,
    Impl,
    Block,
    Macro,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct AstNode {
    pub kind: NodeKind,
    pub name: String,
    pub children: Vec<AstNode>,
}

impl AstNode {
    pub fn new(kind: NodeKind, name: String) -> Self {
        Self {
            kind,
            name,
            children: Vec::new(),
        }
    }
}

struct AstBuilder {
    stack: Vec<AstNode>,
}

impl AstBuilder {
    fn new(root_name: String) -> Self {
        Self {
            stack: vec![AstNode::new(NodeKind::File, root_name)],
        }
    }

    fn current(&mut self) -> &mut AstNode {
        self.stack.last_mut().expect("Stack should never be empty")
    }

    fn push_node(&mut self, kind: NodeKind, name: String) {
        self.stack.push(AstNode::new(kind, name));
    }

    fn pop_node(&mut self) {
        if self.stack.len() > 1 {
            let node = self.stack.pop().unwrap();
            self.current().children.push(node);
        }
    }

    fn finish(mut self) -> AstNode {
        self.stack.pop().unwrap()
    }
}

impl<'ast> Visit<'ast> for AstBuilder {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        self.push_node(NodeKind::Fn, i.sig.ident.to_string());
        visit::visit_item_fn(self, i);
        self.pop_node();
    }

    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        self.push_node(NodeKind::Struct, i.ident.to_string());
        visit::visit_item_struct(self, i);
        self.pop_node();
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        self.push_node(NodeKind::Enum, i.ident.to_string());
        visit::visit_item_enum(self, i);
        self.pop_node();
    }

    fn visit_item_mod(&mut self, i: &'ast ItemMod) {
        self.push_node(NodeKind::Mod, i.ident.to_string());
        visit::visit_item_mod(self, i);
        self.pop_node();
    }

    fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
        // Impl blocks don't always have a clear name, simplify for now
        let name = "impl".to_string();
        self.push_node(NodeKind::Impl, name);
        visit::visit_item_impl(self, i);
        self.pop_node();
    }

    fn visit_block(&mut self, i: &'ast Block) {
        // Only show blocks if they have content?
        // Or maybe just show them to visualize code depth/complexity
        self.push_node(NodeKind::Block, "{}".to_string());
        visit::visit_block(self, i);
        self.pop_node();
    }

    fn visit_item_macro(&mut self, i: &'ast ItemMacro) {
        if let Some(ident) = &i.ident {
            self.push_node(NodeKind::Macro, ident.to_string());
        } else {
            self.push_node(NodeKind::Macro, "!".to_string());
        }
        visit::visit_item_macro(self, i);
        self.pop_node();
    }
}

pub fn parse_str(code: &str) -> Result<AstNode> {
    let syntax = syn::parse_file(code)?;
    let mut builder = AstBuilder::new("root".to_string());
    builder.visit_file(&syntax);
    Ok(builder.finish())
}

pub fn parse_file(path: impl AsRef<Path>) -> Result<AstNode> {
    let content = fs::read_to_string(path.as_ref())?;
    let file_name = path
        .as_ref()
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let syntax = syn::parse_file(&content)?;
    let mut builder = AstBuilder::new(file_name);
    builder.visit_file(&syntax);
    Ok(builder.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_fn() {
        let code = r#"
            fn main() {
                println!("Hello");
            }
        "#;
        let root = parse_str(code).unwrap();
        assert_eq!(root.kind, NodeKind::File);
        assert_eq!(root.children.len(), 1);

        let func = &root.children[0];
        assert_eq!(func.kind, NodeKind::Fn);
        assert_eq!(func.name, "main");

        // Function has a block
        assert_eq!(func.children.len(), 1);
        assert_eq!(func.children[0].kind, NodeKind::Block);
    }

    #[test]
    fn test_parse_nested() {
        let code = r#"
            mod my_mod {
                struct MyStruct {}
                fn my_func() {}
            }
        "#;
        let root = parse_str(code).unwrap();
        assert_eq!(root.children.len(), 1); // mod

        let module = &root.children[0];
        assert_eq!(module.kind, NodeKind::Mod);
        assert_eq!(module.children.len(), 2); // struct, fn
    }
}

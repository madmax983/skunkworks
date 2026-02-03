use anyhow::Result;
use syn::{visit::Visit, ItemFn, spanned::Spanned};
use walkdir::WalkDir;
use std::path::Path;
use std::fs;

#[derive(Debug, Clone)]
pub struct Fighter {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub signature: String,
    pub file_path: String,
}

struct ComplexityVisitor {
    score: i32,
}

impl<'ast> Visit<'ast> for ComplexityVisitor {
    fn visit_expr_if(&mut self, i: &'ast syn::ExprIf) {
        self.score += 2;
        syn::visit::visit_expr_if(self, i);
    }
    fn visit_expr_match(&mut self, i: &'ast syn::ExprMatch) {
        self.score += 2;
        self.score += i.arms.len() as i32;
        syn::visit::visit_expr_match(self, i);
    }
    fn visit_expr_loop(&mut self, i: &'ast syn::ExprLoop) {
        self.score += 3;
        syn::visit::visit_expr_loop(self, i);
    }
    fn visit_expr_for_loop(&mut self, i: &'ast syn::ExprForLoop) {
        self.score += 3;
        syn::visit::visit_expr_for_loop(self, i);
    }
    fn visit_expr_while(&mut self, i: &'ast syn::ExprWhile) {
        self.score += 3;
        syn::visit::visit_expr_while(self, i);
    }
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        self.score += 1;
        syn::visit::visit_expr_call(self, i);
    }
}

struct FighterVisitor {
    fighters: Vec<Fighter>,
    current_file_path: String,
}

impl<'ast> Visit<'ast> for FighterVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let name = node.sig.ident.to_string();

        let start = node.span().start().line;
        let end = node.span().end().line;
        let lines = (end.saturating_sub(start)).max(1) as i32;

        // Stats Calculation
        let max_hp = lines * 5 + 50;

        let args = node.sig.inputs.len() as i32;
        let defense = args * 2;

        let mut complexity_visitor = ComplexityVisitor { score: 1 };
        complexity_visitor.visit_block(&node.block);
        let attack = complexity_visitor.score * 2;

        let speed = (40 - name.len() as i32).clamp(1, 30);

        let signature = format!("fn {}(...)", name);

        self.fighters.push(Fighter {
            name,
            hp: max_hp,
            max_hp,
            attack,
            defense,
            speed,
            signature,
            file_path: self.current_file_path.clone(),
        });

        syn::visit::visit_item_fn(self, node);
    }
}

pub fn scan_files(path: &Path) -> Result<Vec<Fighter>> {
    let mut fighters = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().map_or(false, |ext| ext == "rs") {
            // Skip target directory
            if entry.path().components().any(|c| c.as_os_str() == "target") {
                continue;
            }

            let content = match fs::read_to_string(entry.path()) {
                Ok(c) => c,
                Err(_) => continue,
            };

            // Use parse_file to get AST.
            // Note: If the file is incomplete or invalid, we skip it.
            if let Ok(ast) = syn::parse_file(&content) {
                let mut visitor = FighterVisitor {
                    fighters: Vec::new(),
                    current_file_path: entry.path().to_string_lossy().to_string(),
                };
                visitor.visit_file(&ast);
                fighters.extend(visitor.fighters);
            }
        }
    }

    Ok(fighters)
}

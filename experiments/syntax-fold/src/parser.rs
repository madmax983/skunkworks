use syn::visit::{self, Visit};
use syn::{Block, ItemFn, ItemImpl};
use syn::spanned::Spanned;

#[derive(Debug, Clone)]
pub struct Line {
    pub content: String,
    pub indent: usize,
    pub original_index: usize,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub start_line: usize, // 0-based index
    pub end_line: usize,
    pub children: Vec<Scope>,
    pub folded: bool,
}

struct ScopeVisitor {
    scopes: Vec<Scope>,
}

impl ScopeVisitor {
    fn new() -> Self {
        Self { scopes: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for ScopeVisitor {
    fn visit_block(&mut self, i: &'ast Block) {
        let span = i.span();
        let start = span.start().line - 1;
        let end = span.end().line - 1;

        if end > start {
            self.scopes.push(Scope {
                start_line: start,
                end_line: end,
                children: Vec::new(),
                folded: false,
            });
        }
        visit::visit_block(self, i);
    }

    fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
        let span = i.span();
        let start = span.start().line - 1;
        let end = span.end().line - 1;

        if end > start {
            self.scopes.push(Scope {
                start_line: start,
                end_line: end,
                children: Vec::new(),
                folded: false,
            });
        }
        visit::visit_item_impl(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        // We handle the function body block in visit_block.
        // But if we want to fold the whole function including signature?
        // Usually code folding leaves the signature visible.
        // So visiting the block is enough for the body.
        visit::visit_item_fn(self, i);
    }
}

pub fn parse_code(code: &str) -> (Vec<Line>, Scope) {
    let lines: Vec<Line> = code
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let indent = line.chars().take_while(|c| c.is_whitespace()).count();
            Line {
                content: line.to_string(),
                indent,
                original_index: i,
            }
        })
        .collect();

    // Parse with syn
    // Note: syn::parse_file expects valid Rust code.
    let syntax = syn::parse_file(code).expect("Failed to parse code");

    let mut visitor = ScopeVisitor::new();
    visitor.visit_file(&syntax);

    // Now we have a flat list of scopes. We need to build a hierarchy.
    // Sort by start_line
    visitor.scopes.sort_by_key(|s| s.start_line);

    // Build tree
    let root = Scope {
        start_line: 0,
        end_line: lines.len().saturating_sub(1),
        children: build_hierarchy(&mut visitor.scopes, 0, lines.len()),
        folded: false,
    };

    (lines, root)
}

fn build_hierarchy(scopes: &mut Vec<Scope>, _parent_start: usize, parent_end: usize) -> Vec<Scope> {
    let mut children = Vec::new();

    // Iterate through available scopes
    // We use a while loop because we might consume scopes recursively
    while !scopes.is_empty() {
        // Peek at the next scope
        let next_start = scopes[0].start_line;
        let next_end = scopes[0].end_line;

        // If the next scope starts after the parent ends, it's not a child (and we are done with this parent)
        // Or if it's completely outside.
        // Since we sorted by start_line, if next_start >= parent_end, it's not a child.
        if next_start >= parent_end {
            break;
        }

        // If the next scope is strictly contained within parent (start >= parent_start is guaranteed by sort)
        if next_end <= parent_end {
             let mut scope = scopes.remove(0);
             scope.children = build_hierarchy(scopes, scope.start_line, scope.end_line);
             children.push(scope);
        } else {
            // Overlapping or malformed scope? Should not happen with valid AST.
            // But if it does, skip it to avoid infinite loop
             scopes.remove(0);
        }
    }

    children
}

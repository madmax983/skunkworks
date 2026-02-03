use anyhow::Result;
use rand::Rng;
use std::fs;
use std::path::Path;
use syn::{spanned::Spanned, visit::Visit, ItemFn};
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockType {
    Solid,  // Normal ground (let, fn calls)
    Hazard, // Spikes (unsafe, panic)
    Gap,    // Empty lines
    Bouncy, // Loops?
}

#[derive(Debug, Clone)]
pub struct LevelSegment {
    pub width: usize,
    pub block_type: BlockType,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct BossStats {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
}

#[derive(Debug, Clone)]
pub struct LevelProfile {
    pub segments: Vec<LevelSegment>,
    pub boss: BossStats,
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
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        // panics are hazardous
        if i.path.is_ident("panic") || i.path.is_ident("todo") {
            self.score += 5;
        }
        syn::visit::visit_macro(self, i);
    }
}

pub fn generate_level(path: &Path) -> Result<Option<LevelProfile>> {
    // 1. Collect all functions in the path

    // If path is a file, just use it. If dir, walk it.
    let walker = if path.is_file() {
        WalkDir::new(path.parent().unwrap()).max_depth(1) // Hacky way to walk just one file? No.
    } else {
        WalkDir::new(path)
    };

    let entries: Vec<_> = walker.into_iter().filter_map(|e| e.ok()).collect();
    // Use a fixed seed for file selection? No, random is fine.
    // But we need to find *one* function to be the level.

    // Let's filter for .rs files
    let rs_files: Vec<_> = entries
        .iter()
        .filter(|e| {
            e.path().extension().map_or(false, |ext| ext == "rs")
                && !e.path().to_string_lossy().contains("target")
        })
        .collect();

    if rs_files.is_empty() {
        return Ok(None);
    }

    let mut rng = rand::thread_rng();
    // Try up to 10 files to find a valid function
    for _ in 0..10 {
        let file_entry = rs_files[rng.gen_range(0..rs_files.len())];
        let content = fs::read_to_string(file_entry.path())?;

        if let Ok(ast) = syn::parse_file(&content) {
            struct FnCollector<'a> {
                funcs: Vec<&'a ItemFn>,
            }
            impl<'a> Visit<'a> for FnCollector<'a> {
                fn visit_item_fn(&mut self, i: &'a ItemFn) {
                    self.funcs.push(i);
                    syn::visit::visit_item_fn(self, i);
                }
            }

            let mut collector = FnCollector { funcs: Vec::new() };
            collector.visit_file(&ast);

            if !collector.funcs.is_empty() {
                let func = collector.funcs[rng.gen_range(0..collector.funcs.len())];

                // Analyze this function
                let start = func.span().start().line - 1; // 0-indexed
                let end = func.span().end().line;

                let file_lines: Vec<&str> = content.lines().collect();
                if start >= file_lines.len() {
                    continue;
                }

                // Extract lines for level generation
                let body_lines = &file_lines[start..end.min(file_lines.len())];

                // Calculate Stats
                let lines_count = body_lines.len() as i32;
                let max_hp = lines_count * 5 + 50;
                let args = func.sig.inputs.len() as i32;
                let defense = args * 2;

                let mut comp_visitor = ComplexityVisitor { score: 1 };
                comp_visitor.visit_item_fn(func);
                let attack = comp_visitor.score * 2;

                let name = func.sig.ident.to_string();
                let speed = (40 - name.len() as i32).clamp(1, 30);

                let boss = BossStats {
                    name,
                    hp: max_hp,
                    max_hp,
                    attack,
                    defense,
                    speed,
                };

                // Generate Segments from lines
                let mut segments = Vec::new();
                for line in body_lines {
                    let trim = line.trim();
                    if trim.is_empty() {
                        segments.push(LevelSegment {
                            width: 5, // Small gap
                            block_type: BlockType::Gap,
                            code: "".to_string(),
                        });
                        continue;
                    }

                    let width = (line.len() / 2).clamp(4, 30); // Width proportional to line length

                    let block_type = if trim.starts_with("unsafe") || trim.contains("panic!") {
                        BlockType::Hazard
                    } else if trim.starts_with("if") || trim.starts_with("match") {
                        BlockType::Solid // Normal
                    } else if trim.starts_with("loop")
                        || trim.starts_with("for")
                        || trim.starts_with("while")
                    {
                        BlockType::Bouncy
                    } else {
                        BlockType::Solid
                    };

                    segments.push(LevelSegment {
                        width,
                        block_type,
                        code: line.to_string(),
                    });
                }

                // Ensure there's a start and end platform
                if segments.is_empty() {
                    segments.push(LevelSegment {
                        width: 20,
                        block_type: BlockType::Solid,
                        code: "// Empty function".to_string(),
                    });
                }

                return Ok(Some(LevelProfile { segments, boss }));
            }
        }
    }

    Ok(None)
}

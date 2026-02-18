use std::collections::HashMap;
use syn::visit::{self, Visit};
use syn::{Ident};
use proc_macro2::{Span};

pub struct IdentifierCollector {
    pub locations: HashMap<String, Vec<Span>>,
}

impl IdentifierCollector {
    pub fn new() -> Self {
        Self { locations: HashMap::new() }
    }
}

impl<'ast> Visit<'ast> for IdentifierCollector {
    fn visit_ident(&mut self, i: &'ast Ident) {
        let name = i.to_string();
        self.locations.entry(name).or_default().push(i.span());
        visit::visit_ident(self, i);
    }
}

pub fn collect_identifiers(source: &str) -> anyhow::Result<HashMap<String, Vec<Span>>> {
    let syntax = syn::parse_file(source)?;
    let mut collector = IdentifierCollector::new();
    collector.visit_file(&syntax);
    Ok(collector.locations)
}

// Returns plain string
#[allow(dead_code)]
pub fn rewrite_source(source: &str, mapping: &HashMap<String, String>, locations: &HashMap<String, Vec<Span>>) -> String {
    rewrite_source_segments(source, mapping, locations)
        .into_iter()
        .map(|line| line.into_iter().map(|(text, _)| text).collect::<String>())
        .collect::<Vec<String>>()
        .join("") // Join with nothing? No, split_inclusive keeps newlines.
}

// Returns segments: (text, is_identifier)
pub fn rewrite_source_segments(source: &str, mapping: &HashMap<String, String>, locations: &HashMap<String, Vec<Span>>) -> Vec<Vec<(String, bool)>> {
    let mut replacements: Vec<(Span, String)> = Vec::new();

    for (original, new_name) in mapping {
        if let Some(spans) = locations.get(original) {
            for span in spans {
                replacements.push((*span, new_name.clone()));
            }
        }
    }

    let mut replacements_by_line: HashMap<usize, Vec<(usize, usize, String)>> = HashMap::new();

    for (span, new_text) in replacements {
        let start = span.start();
        let end = span.end();
        if start.line == end.line {
            let line_idx = start.line - 1;
            replacements_by_line.entry(line_idx).or_default().push((start.column, end.column, new_text));
        }
    }

    let original_lines: Vec<&str> = source.split_inclusive('\n').collect();
    let mut result_lines = Vec::new();

    for (i, line) in original_lines.iter().enumerate() {
        let mut segments = Vec::new();
        if let Some(line_repls) = replacements_by_line.get_mut(&i) {
            // Sort by column ascending this time for easier segmentation
            line_repls.sort_by(|a, b| a.0.cmp(&b.0));

            let mut last_idx = 0;
            for (start_col, end_col, new_text) in line_repls {
                // Check if start_col is valid
                if *start_col < line.len() && *end_col <= line.len() && *start_col >= last_idx {
                     // Text before identifier
                    if *start_col > last_idx {
                        segments.push((line[last_idx..*start_col].to_string(), false));
                    }
                    // Identifier
                    segments.push((new_text.clone(), true));
                    last_idx = *end_col;
                }
            }
            // Remaining text
            if last_idx < line.len() {
                segments.push((line[last_idx..].to_string(), false));
            }
        } else {
            segments.push((line.to_string(), false));
        }
        result_lines.push(segments);
    }

    result_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrite_simple() {
        let source = "fn main() { let x = 5; }";
        let mut mapping = HashMap::new();
        mapping.insert("main".to_string(), "mein".to_string());
        mapping.insert("x".to_string(), "y".to_string());

        let locations = collect_identifiers(source).unwrap();
        let rewritten = rewrite_source(source, &mapping, &locations);

        assert_eq!(rewritten, "fn mein() { let y = 5; }");
    }
}

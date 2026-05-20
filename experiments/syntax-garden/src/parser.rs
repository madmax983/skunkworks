use anyhow::Result;
use std::fs;
use std::io::Read;
use syn::Item;
use walkdir::WalkDir;

#[derive(Default)]
pub struct GardenParser {}

impl GardenParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse_directory(&mut self, path: &str) -> Result<String> {
        let mut genome = String::from("F"); // Initial stem

        let mut entries = Vec::new();
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().is_some_and(|ext| ext == "rs") {
                entries.push(entry.path().to_owned());
            }
        }

        // Sort for determinism
        entries.sort();

        for file_path in entries {
            let file = fs::File::open(&file_path)?;
            let mut content = String::new();
            let limit = 1024 * 1024; // 1MB limit
            let bytes_read = file.take(limit + 1).read_to_string(&mut content)?;

            if bytes_read as u64 <= limit {
                if let Ok(ast) = syn::parse_file(&content) {
                    genome.push('['); // Branch for each file
                    let file_dna = self.analyze_file(&ast);
                    genome.push_str(&file_dna);
                    genome.push(']');
                    genome.push('F'); // Grow main stem between files
                }
            } else {
                eprintln!("Skipping file {:?} (exceeds 1MB limit)", file_path);
            }
        }

        Ok(genome)
    }

    fn analyze_file(&self, file: &syn::File) -> String {
        let mut dna = String::new();
        dna.push('F'); // File stem

        for item in &file.items {
            match item {
                Item::Struct(_) => {
                    // Structs branch right
                    dna.push_str("[+F]");
                }
                Item::Enum(_) => {
                    // Enums branch left
                    dna.push_str("[-F]");
                }
                Item::Fn(_) => {
                    // Functions are leaves/flowers
                    dna.push_str("F[L]");
                }
                Item::Impl(_) => {
                    // Impls extend the branch
                    dna.push('F');
                }
                Item::Mod(m) => {
                    // Modules branch and recurse if inline
                    dna.push('[');
                    if let Some((_, items)) = &m.content {
                        for sub_item in items {
                            // Simplified recursion for inline mods
                            match sub_item {
                                Item::Fn(_) => dna.push_str("F[L]"),
                                Item::Struct(_) => dna.push_str("[+F]"),
                                _ => dna.push('F'),
                            }
                        }
                    }
                    dna.push(']');
                }
                _ => {
                    // Other items just add length
                    dna.push('f'); // f = move without drawing (gap?) or just F
                }
            }
        }
        dna
    }
}

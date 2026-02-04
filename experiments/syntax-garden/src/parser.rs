use anyhow::Result;
use std::fs;
use walkdir::WalkDir;
use syn::Item;

pub struct GardenParser {
    pub axiom: String,
}

impl GardenParser {
    pub fn new() -> Self {
        Self {
            axiom: String::new(),
        }
    }

    pub fn parse_directory(&mut self, path: &str) -> Result<String> {
        let mut genome = String::from("F"); // Initial stem

        let mut entries = Vec::new();
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().map_or(false, |ext| ext == "rs") {
                entries.push(entry.path().to_owned());
            }
        }

        // Sort for determinism
        entries.sort();

        for file_path in entries {
            let content = fs::read_to_string(&file_path)?;
            if let Ok(ast) = syn::parse_file(&content) {
                genome.push_str("["); // Branch for each file
                let file_dna = self.analyze_file(&ast);
                genome.push_str(&file_dna);
                genome.push_str("]");
                genome.push_str("F"); // Grow main stem between files
            }
        }

        Ok(genome)
    }

    fn analyze_file(&self, file: &syn::File) -> String {
        let mut dna = String::new();
        dna.push_str("F"); // File stem

        for item in &file.items {
            match item {
                Item::Struct(_) => {
                    // Structs branch right
                    dna.push_str("[+F]");
                },
                Item::Enum(_) => {
                    // Enums branch left
                    dna.push_str("[-F]");
                },
                Item::Fn(_) => {
                    // Functions are leaves/flowers
                    dna.push_str("F[L]");
                },
                Item::Impl(_) => {
                    // Impls extend the branch
                    dna.push_str("F");
                },
                Item::Mod(m) => {
                     // Modules branch and recurse if inline
                     dna.push_str("[");
                     if let Some((_, items)) = &m.content {
                         for sub_item in items {
                             // Simplified recursion for inline mods
                             match sub_item {
                                 Item::Fn(_) => dna.push_str("F[L]"),
                                 Item::Struct(_) => dna.push_str("[+F]"),
                                 _ => dna.push_str("F"),
                             }
                         }
                     }
                     dna.push_str("]");
                },
                _ => {
                    // Other items just add length
                    dna.push_str("f"); // f = move without drawing (gap?) or just F
                }
            }
        }
        dna
    }
}

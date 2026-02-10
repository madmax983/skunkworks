use std::collections::HashMap;
use syn::visit_mut::VisitMut;
use syn::Ident;
use crate::phonology::PhoneticEngine;

pub struct Obfuscator {
    pub engine: PhoneticEngine,
    pub cache: HashMap<String, String>,
}

impl Obfuscator {
    pub fn new(engine: PhoneticEngine) -> Self {
        Self {
            engine: engine,
            cache: HashMap::new(),
        }
    }
}

impl VisitMut for Obfuscator {
    fn visit_ident_mut(&mut self, i: &mut Ident) {
        let original = i.to_string();

        // Keywords like `self` are sometimes parsed as identifiers in paths.
        if original == "self" || original == "Self" || original == "super" || original == "crate" {
            return;
        }

        if let Some(evolved) = self.cache.get(&original) {
            *i = Ident::new(evolved, i.span());
        } else {
            let evolved = self.engine.apply(&original);

            // Check if valid ident
            if !evolved.is_empty() && syn::parse_str::<Ident>(&evolved).is_ok() {
                self.cache.insert(original, evolved.clone());
                *i = Ident::new(&evolved, i.span());
            } else {
                // Try prefixing with '_'
                let fixed = format!("_{}", evolved);
                if syn::parse_str::<Ident>(&fixed).is_ok() {
                     self.cache.insert(original.clone(), fixed.clone());
                     *i = Ident::new(&fixed, i.span());
                }
            }
        }
    }
}

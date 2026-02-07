use std::collections::{HashSet, HashMap};
use syn::visit::{self, Visit};
use syn::visit_mut::{self, VisitMut};
use syn::{Ident, ItemFn, ItemStruct, ItemEnum, ItemMod, ItemTrait, ItemConst, ItemStatic, Pat, Local, FnArg, ImplItemFn};
use crate::phonology::PhonologyEngine;

pub struct DefinitionFinder {
    pub identifiers: HashSet<String>,
}

impl DefinitionFinder {
    pub fn new() -> Self {
        Self { identifiers: HashSet::new() }
    }
}

impl<'ast> Visit<'ast> for DefinitionFinder {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        self.identifiers.insert(i.sig.ident.to_string());
        visit::visit_item_fn(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        self.identifiers.insert(i.ident.to_string());
        // Visit fields
        visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        self.identifiers.insert(i.ident.to_string());
        visit::visit_item_enum(self, i);
    }

    fn visit_item_mod(&mut self, i: &'ast ItemMod) {
        self.identifiers.insert(i.ident.to_string());
        visit::visit_item_mod(self, i);
    }

    fn visit_item_trait(&mut self, i: &'ast ItemTrait) {
        self.identifiers.insert(i.ident.to_string());
        visit::visit_item_trait(self, i);
    }

    fn visit_item_const(&mut self, i: &'ast ItemConst) {
        self.identifiers.insert(i.ident.to_string());
        visit::visit_item_const(self, i);
    }

    fn visit_item_static(&mut self, i: &'ast ItemStatic) {
        self.identifiers.insert(i.ident.to_string());
        visit::visit_item_static(self, i);
    }

    fn visit_local(&mut self, i: &'ast Local) {
        // let x = ...
        self.visit_pat(&i.pat);
        // Don't visit init yet, or do?
        // We just want to find definitions.
        // If I say `let x = y;`, x is def, y is usage.
        // `visit_local` visits both pat and init.
        // I want to extract from pat.
        // `visit_pat` will extract identifiers from pat.
        // But `visit::visit_local` calls `visit_pat` AND `visit_expr`.
        // I need to be careful.
        // `visit_pat` is generic. It visits pattern.
        // In `let x = 5`, pat is `x`.
        // In `fn foo(x: i32)`, arg pat is `x`.
        visit::visit_local(self, i);
    }

    fn visit_pat(&mut self, i: &'ast Pat) {
        match i {
            Pat::Ident(pat_ident) => {
                self.identifiers.insert(pat_ident.ident.to_string());
            },
            _ => visit::visit_pat(self, i),
        }
    }

    fn visit_impl_item_fn(&mut self, i: &'ast ImplItemFn) {
        self.identifiers.insert(i.sig.ident.to_string());
        visit::visit_impl_item_fn(self, i);
    }
}

pub struct Obfuscator {
    pub mapping: HashMap<String, String>,
}

impl Obfuscator {
    pub fn new(identifiers: &HashSet<String>, engine: &PhonologyEngine) -> Self {
        let mut mapping = HashMap::new();
        for id in identifiers {
            let mut new_id = engine.evolve(id);
            // Ensure it's a valid identifier?
            // If sound change produces "3d", it's invalid.
            // If it produces keyword "fn", it's invalid.
            // For now, trust the engine or user.
            // Maybe prefix if starts with digit?
            if new_id.chars().next().map(|c| c.is_digit(10)).unwrap_or(false) {
                new_id = format!("_{}", new_id);
            }
            mapping.insert(id.clone(), new_id);
        }
        Self { mapping }
    }
}

impl VisitMut for Obfuscator {
    fn visit_ident_mut(&mut self, i: &mut Ident) {
        let s = i.to_string();
        if let Some(new_s) = self.mapping.get(&s) {
            if s != *new_s {
                 *i = Ident::new(new_s, i.span());
            }
        }
    }
}

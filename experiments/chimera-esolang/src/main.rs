use anyhow::Result;
use chimera_esolang::compiler::compile;
use chimera_esolang::parse;
use chimera_lang::vm::ChimeraVM;
use std::fs;

mod tui;

fn main() -> Result<()> {
    // Read source from arg or default
    let source = fs::read_to_string("examples/hello.ges")
        .unwrap_or_else(|_| "strand main { \"Hello Genesis\" print }".to_string());

    let program = parse(&source)?;
    let dna = compile(&program)?;
    let vm = ChimeraVM::new(dna);

    tui::run_tui(vm)
}

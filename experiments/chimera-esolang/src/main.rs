use anyhow::Result;
use chimera_esolang::compile;
use chimera_esolang::parse;
use chimera_lang::vm::ChimeraVM;
use std::fs;

mod tui;

fn main() -> Result<()> {
    // Read source from arg or default
    let args: Vec<String> = std::env::args().collect();
    let is_headless = args.iter().any(|arg| arg == "--headless");

    let source = fs::read_to_string("examples/hello.ges")
        .unwrap_or_else(|_| "strand main { \"Hello Genesis\" print }".to_string());

    let program = parse(&source)?;
    let dna = compile(&program)?;
    let mut vm = ChimeraVM::new(dna);

    if is_headless {
        println!("Running chimera-esolang in headless mode...");
        for _ in 0..10 {
            vm.step();
        }
        return Ok(());
    }

    tui::run_tui(vm)
}

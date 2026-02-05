use anyhow::Result;
use clap::Parser as ClapParser;
use pest::Parser;
use std::fs;

use chimera_lang::{ast::Dna, compiler, tui::run_tui, vm::ChimeraVM, ChimeraParser, Rule};
use std::path::Path;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,

    /// Run in headless mode (no TUI)
    #[arg(long)]
    headless: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let unparsed_file = fs::read_to_string(&cli.input)?;

    // Set a panic hook to restore the terminal if we panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen);
        let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::Show);
        original_hook(panic_info);
    }));

    let path = Path::new(&cli.input);
    let extension = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");

    let dna = if extension == "chs" {
        compiler::compile(&unparsed_file)?
    } else {
        let dna_pair = ChimeraParser::parse(Rule::dna, &unparsed_file)?
            .next()
            .ok_or_else(|| anyhow::anyhow!("No DNA found"))?;
        Dna::from_pair(dna_pair)
    };

    let mut vm = ChimeraVM::new(dna);

    if cli.headless {
        while !vm.halted {
            vm.step();
        }

        println!("Execution complete.");
        println!("Final Stack: {:?}", vm.stack);
        println!("Output Log:");
        for line in vm.output {
            println!("  {}", line);
        }
    } else {
        run_tui(vm)?;
    }

    Ok(())
}

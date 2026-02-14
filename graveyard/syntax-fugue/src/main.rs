use anyhow::{Context, Result};
use std::env;
use std::fs;
use syn::visit::Visit;
use syntax_fugue::audio::Synthesizer;
use syntax_fugue::parser::SyntaxListener;
use syntax_fugue::tui::run_tui;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let default_path = "experiments/syntax-fugue/src/parser.rs"; // Play itself by default
    let path = if args.len() > 1 {
        &args[1]
    } else {
        default_path
    };

    println!("🎵 Parsing {}...", path);
    let code =
        fs::read_to_string(path).with_context(|| format!("Failed to read file: {}", path))?;

    let ast = syn::parse_file(&code).with_context(|| "Failed to parse Rust file")?;

    let mut listener = SyntaxListener::new(12345); // Seed
    listener.visit_file(&ast);

    let events = listener.events;
    println!("Found {} musical events.", events.len());

    if events.is_empty() {
        println!("No events found. Exiting.");
        return Ok(());
    }

    println!("🎹 Initializing Synthesizer...");
    let synthesizer = Synthesizer::new()?;

    println!("🚀 Starting TUI...");
    run_tui(synthesizer, events)?;

    Ok(())
}

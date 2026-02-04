use pest::Parser;
use pest_derive::Parser;
use clap::Parser as ClapParser;
use std::fs;
use anyhow::Result;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ChimeraParser;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let unparsed_file = fs::read_to_string(&cli.input)?;

    let dna = ChimeraParser::parse(Rule::dna, &unparsed_file)?
        .next()
        .ok_or_else(|| anyhow::anyhow!("No DNA found"))?;

    println!("Successfully parsed Chimera DNA!");

    // Simple traversal logging
    for helix in dna.into_inner() {
        match helix.as_rule() {
            Rule::helix => {
                println!("Helix detected.");
                for strand in helix.into_inner() {
                     println!("  Strand: {}", strand.as_str());
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(())
}

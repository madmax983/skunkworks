use anyhow::Result;
use std::env;

mod player;
mod producer;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "record" => producer::run_producer(),
        "play" => player::run_player(),
        _ => {
            print_usage();
            Ok(())
        }
    }
}

fn print_usage() {
    println!("Usage: semantic-dvr <command>");
    println!("Commands:");
    println!("  record   Run the producer simulation and output JSON lines to stdout.");
    println!("  play     Read JSON lines from stdin and replay/visualize.");
    println!("Example:");
    println!("  cargo run --bin semantic-dvr -- record | cargo run --bin semantic-dvr -- play");
}

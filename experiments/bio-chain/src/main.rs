mod network;
mod types;
mod validator;

use network::Network;
use types::*;
use validator::Validator;

fn main() -> anyhow::Result<()> {
    println!("🧬 BioCoin: Living Blockchain");
    println!("================================\n");

    // Create network
    let mut network = Network::new();

    // Create genesis validators
    println!("🌱 Genesis Validators:");
    for i in 0..5 {
        let v = Validator::genesis(i, 1000);
        println!("  Validator {}: Energy={}", v.id, v.energy);
        network.add_validator(v);
    }

    network.status();

    // Demo: Propose a block with transactions
    println!("\n🔬 Starting Consensus Demo\n");
    println!("{}", "=".repeat(50));

    let transactions = vec![
        Transaction {
            from: [1; 20],
            to: [2; 20],
            amount: 100,
            nonce: 0,
            fee: 10,
        },
        Transaction {
            from: [3; 20],
            to: [4; 20],
            amount: 50,
            nonce: 0,
            fee: 5,
        },
    ];

    // Validator 0 proposes block
    network.propose_block(0, transactions);

    // Run consensus for a few ticks
    for _ in 0..5 {
        network.step();

        if !network.pending_blocks.is_empty() {
            // Still pending
        } else if !network.finalized_chain.is_empty() {
            println!("\n🎊 SUCCESS! Block finalized through biological consensus!");
            break;
        }
    }

    network.status();

    println!("\n{}", "=".repeat(50));
    println!("🧬 Biological consensus complete!");

    Ok(())
}

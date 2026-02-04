mod byzantine_test;
mod network;
mod types;
mod validator;

use byzantine_test::run_byzantine_test;
use network::Network;
use types::*;
use validator::Validator;

fn main() -> anyhow::Result<()> {
    println!("🧬 BioCoin: Living Blockchain");
    println!("================================\n");

    // Check if user wants Byzantine test
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "--byzantine" {
        return run_byzantine_test();
    }

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

    // Demo: Run evolution simulation
    println!("\n🔬 Starting Evolution Simulation\n");
    println!("{}", "=".repeat(60));

    println!("\n📖 Simulation Rules:");
    println!("  • Validators burn 5 energy/tick (metabolism)");
    println!("  • Successful votes earn fees → energy");
    println!("  • Energy > 1200 → Mitosis (reproduction, cost: 300)");
    println!("  • Energy = 0 → Apoptosis (death)");
    println!("  • Population evolves through natural selection\n");

    // Run for 40 ticks with periodic block proposals
    for i in 0..40 {
        // Propose a block every 3 ticks
        if i % 3 == 0 && !network.validators.is_empty() {
            let proposer_id = i % network.validators.len();
            let transactions = vec![
                Transaction {
                    from: [1; 20],
                    to: [2; 20],
                    amount: 100,
                    nonce: i as u64,
                    fee: 100, // Higher fees for faster evolution
                },
                Transaction {
                    from: [3; 20],
                    to: [4; 20],
                    amount: 50,
                    nonce: i as u64,
                    fee: 100,
                },
            ];
            network.propose_block(proposer_id, transactions);
        }

        network.step();

        // Show status every 5 ticks
        if (i + 1) % 5 == 0 {
            network.status();
        }

        // Stop if all validators died
        if network.validators.is_empty() {
            println!("\n💀 EXTINCTION! All validators died.");
            break;
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("🧬 Evolution Simulation Complete!\n");
    network.status();

    println!("\n📈 Final Results:");
    println!("  Total Births: {}", network.births);
    println!("  Total Deaths: {}", network.deaths);
    println!("  Population Change: {:+}", network.validators.len() as i64 - 5);
    println!("  Blocks Finalized: {}", network.finalized_chain.len());

    Ok(())
}

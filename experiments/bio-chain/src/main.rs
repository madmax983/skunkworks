//! # 🧬 BioCoin: Living Blockchain
//!
//! **A blockchain where validators are living organisms that evolve.**
//!
//! ## Concept
//!
//! Traditional blockchains use Proof-of-Work or Proof-of-Stake. BioCoin uses **Proof-of-Life**:
//!
//! - **Validators = Chimera organisms** with DNA encoding validation logic
//! - **Consensus = Chemical signaling** via hormone secretion
//! - **Security = Natural selection** - honest validators reproduce, malicious ones starve
//! - **Finality = Evolution** - dead validators can't revert history
//!
//! ## Features
//!
//! ### Biological Consensus
//! - Validators vote by secreting hormones on channels (block hashes)
//! - Consensus reached when hormone level exceeds threshold
//! - Asynchronous, weighted voting with natural timeouts (hormone decay)
//!
//! ### Natural Selection
//! - Successful validators earn fees → gain energy → reproduce (mitosis)
//! - Failed validators lose energy → starve → die (apoptosis)
//! - DNA mutations during reproduction create genetic variation (5% rate)
//! - Network evolves toward optimal validation strategies through genetic drift
//!
//! ### Byzantine Fault Tolerance via Evolution
//! - Malicious validators waste energy on invalid votes
//! - Honest validators earn fees and reproduce
//! - Population naturally trends toward honesty over time
//!
//! ## Running
//!
//! ### Evolution Demo (Text Output)
//! ```bash
//! cargo run --release
//! ```
//!
//! ### Byzantine Fault Tolerance Test
//! ```bash
//! cargo run --release -- --byzantine
//! ```
//!
//! Runs a test with 60% honest + 40% malicious validators.
//! Watch evolution eliminate the Byzantine actors!
//!
//! ### Live TUI Visualization
//! ```bash
//! cargo run --release -- --ui
//! ```
//!
//! Watch in real-time:
//! - Population evolution (honest vs malicious)
//! - Energy dynamics
//! - Validator reproduction and death
//! - Block finalization
//! - Natural selection in action
//!
//! **Controls:**
//! - `SPACE`: Pause/Resume
//! - `Q`: Quit
//!
//! ## Architecture
//!
//! See `ARCHITECTURE.md` for detailed design.
//!
//! ## Status
//!
//! **Working prototype with proven Byzantine fault tolerance.**
//!
//! Implemented features:
//! - [x] Architecture design (ARCHITECTURE.md)
//! - [x] Validator organisms (ChimeraVM with DNA)
//! - [x] Chemical voting (hormone-based consensus)
//! - [x] Natural selection (mitosis/apoptosis)
//! - [x] DNA mutations (genetic variation during reproduction)
//! - [x] Byzantine fault tolerance (proven via evolution)
//! - [x] TUI visualization (real-time population tracking)
//!
//! ## Research Goals
//!
//! Can biological consensus actually work? Can evolution create Byzantine fault tolerance?
//!
//! **Let's find out.** 🧬⛓️
//!
mod byzantine_test;
mod network;
#[cfg(all(test, feature = "nova"))]
mod tests_virus;
mod types;
mod ui;
mod validator;
#[cfg(feature = "nova")]
mod virus;

use byzantine_test::run_byzantine_test;
use network::Network;
use types::*;
use ui::{App, run_ui};
use validator::Validator;

fn main() -> anyhow::Result<()> {
    println!("🧬 BioCoin: Living Blockchain");
    println!("================================\n");

    // Check arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--byzantine" => return run_byzantine_test(),
            "--ui" => {
                // TUI mode with mixed population
                let mut network = Network::new();

                // 3 honest + 2 malicious
                for i in 0..3 {
                    network.add_validator(Validator::genesis(i, 1000));
                }
                for i in 3..5 {
                    network.add_validator(Validator::malicious(i, 1000));
                }

                let app = App::new(network);
                return run_ui(app).map_err(|e| anyhow::anyhow!(e));
            }
            _ => {}
        }
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
    println!(
        "  Population Change: {:+}",
        network.validators.len() as i64 - 5
    );
    println!("  Blocks Finalized: {}", network.finalized_chain.len());

    Ok(())
}

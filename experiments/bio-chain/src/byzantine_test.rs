use crate::network::Network;
use crate::types::*;
use crate::validator::Validator;

pub fn run_byzantine_test() -> anyhow::Result<()> {
    println!("\n🔴 BYZANTINE FAULT TOLERANCE TEST");
    println!("{}", "=".repeat(70));

    println!("\n📖 Test Scenario:");
    println!("  • Start with 3 honest validators + 2 malicious (40% Byzantine)");
    println!("  • Malicious validators vote for INVALID blocks");
    println!("  • Honest validators vote correctly");
    println!("  • Malicious validators earn NO rewards (wasted votes)");
    println!("  • Hypothesis: Malicious validators starve, honest ones thrive\n");

    let mut network = Network::new();

    // Create honest validators
    println!("🟢 Honest Validators:");
    for i in 0..3 {
        let v = Validator::genesis(i, 1000);
        println!("  Validator {}: Energy={}", v.id, v.energy);
        network.add_validator(v);
    }

    // Create malicious validators
    println!("\n🔴 Malicious Validators:");
    for i in 3..5 {
        let v = Validator::malicious(i, 1000);
        println!("  Validator {} (MALICIOUS): Energy={}", v.id, v.energy);
        network.add_validator(v);
    }

    network.status();

    println!("\n{}", "=".repeat(70));
    println!("🔬 Running Evolution...\n");

    // Run for 200 ticks with periodic block proposals
    for i in 0..200 {
        // Propose a block every 3 ticks
        if i % 3 == 0 && !network.validators.is_empty() {
            let honest_validators: Vec<_> = network
                .validators
                .iter()
                .filter(|v| !v.is_malicious)
                .collect();

            if !honest_validators.is_empty() {
                let proposer_idx = i % honest_validators.len();
                let proposer_id = honest_validators[proposer_idx].id;

                let transactions = vec![
                    Transaction {
                        from: [1; 20],
                        to: [2; 20],
                        amount: 100,
                        nonce: i as u64,
                        fee: 100,
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
        }

        network.step();

        // Show status every 15 ticks
        if (i + 1) % 15 == 0 {
            println!("\n{}", "-".repeat(70));
            network.status();
            show_population_breakdown(&network);
        }

        // Stop if all validators died
        if network.validators.is_empty() {
            println!("\n💀 EXTINCTION! All validators died.");
            break;
        }
    }

    println!("\n{}", "=".repeat(70));
    println!("🧬 Byzantine Test Complete!\n");

    network.status();
    show_population_breakdown(&network);

    // Analyze results
    println!("\n📊 Analysis:");
    println!("  Total Births: {}", network.births);
    println!("  Total Deaths: {}", network.deaths);
    println!("  Blocks Finalized: {}", network.finalized_chain.len());

    let honest_count = network.validators.iter().filter(|v| !v.is_malicious).count();
    let malicious_count = network.validators.iter().filter(|v| v.is_malicious).count();

    println!("\n🎯 Security Result:");
    if malicious_count == 0 && honest_count > 0 {
        println!("  ✅ SUCCESS! All malicious validators eliminated");
        println!("  ✅ Honest validators thrived and reproduced");
        println!("  ✅ Natural selection created Byzantine fault tolerance!");
    } else if malicious_count > honest_count {
        println!("  ❌ FAILURE! Malicious validators dominated");
    } else {
        println!("  ⚠️  MIXED RESULT: Both types survived");
        println!("     Honest: {} | Malicious: {}", honest_count, malicious_count);
    }

    Ok(())
}

fn show_population_breakdown(network: &Network) {
    let honest: Vec<_> = network.validators.iter().filter(|v| !v.is_malicious).collect();
    let malicious: Vec<_> = network.validators.iter().filter(|v| v.is_malicious).collect();

    println!("\n  👥 Population Breakdown:");
    println!("     🟢 Honest: {} validators", honest.len());
    if !honest.is_empty() {
        let avg_energy: i64 = honest.iter().map(|v| v.energy).sum::<i64>() / honest.len() as i64;
        println!("        Avg Energy: {}", avg_energy);
    }

    println!("     🔴 Malicious: {} validators", malicious.len());
    if !malicious.is_empty() {
        let avg_energy: i64 =
            malicious.iter().map(|v| v.energy).sum::<i64>() / malicious.len() as i64;
        println!("        Avg Energy: {}", avg_energy);
    }
}

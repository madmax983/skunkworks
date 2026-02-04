mod types;
mod validator;

use validator::Validator;

fn main() -> anyhow::Result<()> {
    println!("🧬 BioCoin: Living Blockchain");
    println!("================================\n");

    // Create genesis validators
    let mut validators = vec![];
    for i in 0..5 {
        let v = Validator::genesis(i, 1000);
        println!("Genesis Validator {}: Energy={}", v.id, v.energy);
        validators.push(v);
    }

    println!("\n✅ Network initialized with {} validators", validators.len());
    println!("🔬 Ready for biological consensus...\n");

    // TODO: Start simulation loop

    Ok(())
}

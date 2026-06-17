#[path = "../src/circuit.rs"]
mod circuit;
use circuit::CircuitGenerator;

// 👺 Havoc: Prove that `CircuitGenerator::generate` panics via out-of-bounds indexing and underflow
// when the public `generate` method is called after creating a 0x0 circuit.
#[test]
#[should_panic]
fn havoc_test_circuit_generator_panic() {
    // 🧨 The Trigger: Give it a 0x0 width and height
    let generator = CircuitGenerator::new(0, 0);

    // 💥 Detonate: `gen_range(20..self.width - 20)` will panic because start > end!
    let _ = generator.generate("havoc");
}

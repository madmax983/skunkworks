// Run with: cargo run --features nova --example story_demo
use git_rhythm::nova::NarrativeGenerator;

fn main() {
    println!("Starting Story Mode...");
    let gen = NarrativeGenerator::new();
    let story = gen.generate("Once upon a commit...");
    println!("{}", story);
}

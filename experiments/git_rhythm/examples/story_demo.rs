// Run with: cargo run --features nova --example story_demo
use git_rhythm::nova::NarrativeGenerator;

fn main() {
    println!("Starting Story Mode...");
    let generator = NarrativeGenerator::new();
    let story = generator.generate("Once upon a commit...");
    println!("{}", story);
}

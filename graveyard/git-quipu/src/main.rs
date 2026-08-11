use git_associates::GitModel;
use quipu::{Color, Cord, Quipu};

fn main() {
    let mut args = std::env::args();
    args.next();

    // Support headless bypass for CI tests
    if let Some(arg) = args.next() {
        if arg == "--headless" {
            println!("Running in headless mode. Bypassing execution.");
            return;
        }
    }

    println!("Initializing git-quipu...");

    // Open current repository
    let model = GitModel::open(".").expect("Failed to open git repository");
    let commits = model
        .history_with_diffs(10)
        .expect("Failed to read git history");

    let mut quipu = Quipu::new();

    for commit in commits.into_iter().rev() {
        if let Some(stats) = commit.stats {
            let mut cord = Cord::from(stats.insertions as u64);
            cord.color = Color::Blue;

            let mut deletions_cord = Cord::from(stats.deletions as u64);
            deletions_cord.color = Color::Red;

            cord.subsidiaries.push(deletions_cord);
            quipu.add_cord(cord);
        }
    }

    println!("Generated Git-Quipu Database:");
    println!("{}", quipu);
}

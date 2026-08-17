//! # arthropod-git
//! ## Lineage
//! - **Parent A**: `crates/arthropod` (Immediate Mode UI)
//! - **Parent B**: `crates/git-associates` (Chronological Git History)
//!
//! ## Usage
//! Run with `cargo run -p arthropod-git`.
//! To bypass graphical output for CI, run `cargo run -p arthropod-git -- --headless`.
//!
use arthropod::Button;
use git_associates::GitModel;
use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: "Arthropod Git".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn run_headless() {
    println!("Running in headless mode. Bypassing macroquad initialization.");
}

fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        run_headless();
        return;
    }

    macroquad::Window::from_config(conf(), async_main());
}

async fn async_main() {
    let mut commit_index: usize = 0;

    // Load git history
    let model = GitModel::open(".").expect("Failed to open git repo");
    let commits = model.history(100).unwrap_or_default();

    let btn_prev = Button::new("Previous Commit", 20.0, 20.0, 250.0, 40.0)
        .with_colors(BLUE, LIGHTGRAY, DARKGRAY);

    let btn_next =
        Button::new("Next Commit", 290.0, 20.0, 250.0, 40.0).with_colors(RED, LIGHTGRAY, DARKGRAY);

    loop {
        clear_background(BLACK);

        if btn_prev.draw() && commit_index > 0 {
            commit_index -= 1;
        }
        if btn_next.draw() && !commits.is_empty() && commit_index < commits.len() - 1 {
            commit_index += 1;
        }

        if !commits.is_empty() {
            let commit = &commits[commit_index];
            draw_text(
                format!("Commit: {}", commit.short_hash).as_str(),
                20.0,
                100.0,
                30.0,
                WHITE,
            );
            draw_text(
                format!("Author: {}", commit.author).as_str(),
                20.0,
                140.0,
                30.0,
                WHITE,
            );
            draw_text(
                format!("Message: {}", commit.message).as_str(),
                20.0,
                180.0,
                30.0,
                WHITE,
            );
            draw_text(
                format!("Date: {}", commit.timestamp).as_str(),
                20.0,
                220.0,
                30.0,
                WHITE,
            );
        } else {
            draw_text("No commits found.", 20.0, 100.0, 30.0, WHITE);
        }

        next_frame().await;
    }
}

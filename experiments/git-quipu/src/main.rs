//! 🧬 Splice: Cross git-associates × quipu
//!
//! **Concept**: Codebase Knotted Ledger.
//! **Novel trait**: The discrete chronological git commit history (`git-associates`) is permanently recorded onto structural Quipu cords (`quipu`). Insertions and deletions from each commit are encoded as discrete integers mapped to simple, long, and figure-eight knots along the continuous history cord.
//!
//! **Lineage Plan:**
//! - From `git-associates`: Git metadata parsing, extracting insertions and deletions for each commit.
//! - From `quipu`: The knotted cord data structures and integer encoding logic.
//! - Novel trait: Tying git history into physical structural data knots.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use git_associates::GitModel;
use quipu::{Cord, Knot, Quipu};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::{
    env,
    time::{Duration, Instant},
};
use tui_shared::Tui;

struct GitQuipuApp {
    quipu: Quipu,
    commit_messages: Vec<String>,
}

impl GitQuipuApp {
    fn new(history_limit: usize) -> anyhow::Result<Self> {
        let repo_path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let git_model = GitModel::open(repo_path)?;

        let commits = git_model.history_with_diffs(history_limit)?;

        let mut quipu = Quipu::new();
        let mut commit_messages = Vec::new();

        for commit in commits.into_iter().rev() {
            let insertions = commit.stats.as_ref().map_or(0, |s| s.insertions);
            let deletions = commit.stats.as_ref().map_or(0, |s| s.deletions);

            // We use insertions + deletions to represent the total "volume" of the commit
            let volume = (insertions + deletions) as u64;
            if volume > 0 {
                // Limit volume to prevent extreme cord sizes, max 999
                let limited_volume = volume.min(999);

                let cord = Cord::from(limited_volume);
                quipu.add_cord(cord);
                commit_messages.push(format!(
                    "{} - {}",
                    commit.short_hash,
                    commit.message.lines().next().unwrap_or("")
                ));
            }
        }

        Ok(Self {
            quipu,
            commit_messages,
        })
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Headless mode: exiting early to prevent CI timeouts.");
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let app = GitQuipuApp::new(20).unwrap_or_else(|_| GitQuipuApp {
        quipu: Quipu::new(),
        commit_messages: vec!["No Git history found or error parsing".to_string()],
    });

    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f: &mut Frame| {
            let mut text = vec![
                Line::from(Span::styled(
                    "Git Quipu: Knotted Codebase History",
                    Style::default().fg(Color::Yellow),
                )),
                Line::from(""),
            ];

            for (i, cord) in app.quipu.cords.iter().enumerate() {
                let msg = app.commit_messages.get(i).map(|s| s.as_str()).unwrap_or("");
                let mut cord_str = String::new();
                for cluster in cord.clusters.iter().rev() {
                    for knot in cluster {
                        match knot {
                            Knot::Simple => cord_str.push_str("●-"),
                            Knot::Long(t) => cord_str.push_str(&format!("(≡{})-", t)),
                            Knot::FigureEight => cord_str.push_str("∞-"),
                        }
                    }
                    cord_str.push_str("  "); // space between clusters
                }
                text.push(Line::from(vec![
                    Span::styled(
                        format!("{:>20} | ", msg),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(format!("--{}", cord_str), Style::default().fg(Color::Cyan)),
                    Span::styled(
                        format!(" (Val: {})", cord.value()),
                        Style::default().fg(Color::Gray),
                    ),
                ]));
            }

            let paragraph = Paragraph::new(text).block(
                Block::default()
                    .title(" Quipu Ledger ")
                    .borders(Borders::ALL),
            );

            f.render_widget(paragraph, f.area());
        })?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    Ok(())
}

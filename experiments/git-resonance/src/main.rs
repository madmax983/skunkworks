//! # Git Resonance 🔔
//!
//! **Parents:** `git-associates` × `resonance-audio`
//!
//! Lineage:
//! - Parent A (`git-associates`): Provides the repository's past —
//!   [`GitModel::history_with_diffs`] yields commits with hashes, parents,
//!   and churn statistics.
//! - Parent B (`resonance-audio`): Provides the physics of hearing —
//!   [`PhysicsGrid`], a 2D FDTD wave solver where a `pluck()` rings,
//!   propagates, reflects, and interferes.
//!
//! ## Phenotype
//! A repository, rung like a bell. Every commit becomes a struck note on the
//! wave grid: its position comes from its hash, its strength from its churn
//! (insertions + deletions), and merge commits strike twice. A listener at
//! the center of the grid records what the whole history sounds like
//! interfering with itself. Bursty eras ring loud and muddy; quiet eras
//! leave the water nearly flat. The sampled listener trace after the final
//! commit — the *chord fingerprint* — belongs to that history and no other:
//! change the history and the chord changes.
//!
//! ## Headless
//! `cargo run -p git-resonance -- --headless [repo_path] [limit]` plays a
//! repository's history (default: `.`, 120 commits) with no terminal and
//! prints the ledger: notes struck, listener peak, and the fingerprint.
//! If no repository can be opened, a small synthetic demo history is rung
//! instead, and the ledger says so — a bell in a vacuum still gets heard.

use git_associates::{Commit, GitModel};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use resonance_audio::{Material, PhysicsGrid};
use std::time::{Duration, Instant};
use tui_shared::{
    crossterm::event::{self, Event, KeyCode},
    ratatui::{
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{Block, Borders, Paragraph},
    },
    Tui,
};

pub const WIDTH: usize = 72;
pub const HEIGHT: usize = 36;
/// Ticks between successive commit strikes.
const NOTE_SPACING: u64 = 6;
/// Quiet ticks after the last strike while the bell decays and is sampled.
const DECAY_TICKS: u64 = 96;

/// One struck commit, reduced to what the grid needs.
#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub x: usize,
    pub y: usize,
    pub strength: f32,
    /// Merge commits strike a second, central note.
    pub double_strike: bool,
    pub label: String,
}

/// Map a commit to a note. Pure and deterministic — the same commit always
/// rings at the same place, which is what makes fingerprints comparable.
pub fn note_from_commit(c: &Commit) -> Note {
    let bytes = c.hash.as_bytes();
    let b0 = hex_val(bytes.first().copied().unwrap_or(b'0')) * 16
        + hex_val(bytes.get(1).copied().unwrap_or(b'0'));
    let b1 = hex_val(bytes.get(2).copied().unwrap_or(b'0')) * 16
        + hex_val(bytes.get(3).copied().unwrap_or(b'0'));
    let x = 2 + (b0 as usize * (WIDTH - 4)) / 255;
    let y = 2 + (b1 as usize * (HEIGHT - 4)) / 255;

    let churn = c
        .stats
        .as_ref()
        .map(|s| s.insertions + s.deletions)
        .unwrap_or(8);
    // sqrt curve: a 1-line fix whispers, a 10k-line refactor *rings*, but
    // nothing gets to shout the whole cave down.
    let strength = (0.35 + (churn as f32).sqrt() / 22.0).min(2.6);

    let subject = c.message.lines().next().unwrap_or("").trim().to_string();
    Note {
        x,
        y,
        strength,
        double_strike: c.parents.len() >= 2,
        label: format!("{} {}", c.short_hash, subject),
    }
}

fn hex_val(b: u8) -> usize {
    match b {
        b'0'..=b'9' => (b - b'0') as usize,
        b'a'..=b'f' => (b - b'a' + 10) as usize,
        b'A'..=b'F' => (b - b'A' + 10) as usize,
        _ => 0,
    }
}

/// A synthetic history for when there is no repository to ring — twelve
/// commits of a small, imaginary, increasingly dramatic project.
pub fn demo_notes() -> Vec<Note> {
    let mut rng = StdRng::seed_from_u64(0xBE11);
    (0..12)
        .map(|i| Note {
            x: rng.gen_range(3..WIDTH - 3),
            y: rng.gen_range(3..HEIGHT - 3),
            strength: 0.5 + (i as f32) * 0.15,
            double_strike: i == 7,
            label: format!("demo{i:02} a synthetic commit, struck for science"),
        })
        .collect()
}

/// Load notes from a real repository, oldest first (history is a story; you
/// ring it forward). Falls back to the demo history when the path will not
/// open, and says which happened.
pub fn load_notes(path: &str, limit: usize) -> (Vec<Note>, String) {
    let rung = (|| -> Result<Vec<Note>, Box<dyn std::error::Error>> {
        let model = GitModel::open(path)?;
        let mut commits = model.history_with_diffs(limit)?;
        commits.reverse(); // history() walks newest-first; bells ring forward
        Ok(commits.iter().map(note_from_commit).collect())
    })();
    match rung {
        Ok(notes) if !notes.is_empty() => (notes, format!("repository at {path}")),
        _ => (demo_notes(), "synthetic demo history (no repo opened)".to_string()),
    }
}

#[derive(Debug, Default, Clone)]
pub struct BellLedger {
    pub notes_struck: u64,
    pub listener_peak: f32,
    pub listener_energy: f64,
    pub fingerprint: Vec<f32>,
}

pub struct Bell {
    grid: PhysicsGrid,
    notes: Vec<Note>,
    next_note: usize,
    tick: u64,
    current_label: String,
    pub ledger: BellLedger,
}

impl Bell {
    pub fn new(notes: Vec<Note>) -> Self {
        let mut grid = PhysicsGrid::new(WIDTH, HEIGHT);
        for x in 0..WIDTH {
            grid.set_material(x, 0, Material::Wall);
            grid.set_material(x, HEIGHT - 1, Material::Wall);
        }
        for y in 0..HEIGHT {
            grid.set_material(0, y, Material::Wall);
            grid.set_material(WIDTH - 1, y, Material::Wall);
        }
        Self {
            grid,
            notes,
            next_note: 0,
            tick: 0,
            current_label: String::new(),
            ledger: BellLedger::default(),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.next_note >= self.notes.len()
            && self.tick > self.notes.len() as u64 * NOTE_SPACING + DECAY_TICKS
    }

    pub fn step(&mut self) {
        self.tick += 1;

        if self.next_note < self.notes.len()
            && self.tick >= self.next_note as u64 * NOTE_SPACING + 1
        {
            let note = self.notes[self.next_note].clone();
            self.grid.pluck(note.x, note.y, note.strength);
            if note.double_strike {
                self.grid.pluck(WIDTH / 2, HEIGHT / 2, note.strength * 0.7);
            }
            self.current_label = note.label.clone();
            self.ledger.notes_struck += 1;
            self.next_note += 1;
        }

        self.grid.step();
        self.grid.step();

        // The listener hears only its own cell. That is the whole ethic.
        let amp = self.grid.get(WIDTH / 2, HEIGHT / 2).abs();
        if amp > self.ledger.listener_peak {
            self.ledger.listener_peak = amp;
        }
        self.ledger.listener_energy += amp as f64;

        // Sample the decay into a 16-point chord fingerprint.
        let play_end = self.notes.len() as u64 * NOTE_SPACING;
        if self.tick > play_end && (self.tick - play_end) % 6 == 0 && self.ledger.fingerprint.len() < 16 {
            self.ledger.fingerprint.push(self.grid.get(WIDTH / 2, HEIGHT / 2));
        }
    }

    fn render_lines(&self) -> Vec<Line<'static>> {
        let shades = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
        let mut lines = Vec::with_capacity(HEIGHT);
        for y in 0..HEIGHT {
            let mut spans = Vec::with_capacity(WIDTH);
            for x in 0..WIDTH {
                if x == 0 || y == 0 || x == WIDTH - 1 || y == HEIGHT - 1 {
                    spans.push(Span::styled("█", Style::default().fg(Color::DarkGray)));
                    continue;
                }
                if x == WIDTH / 2 && y == HEIGHT / 2 {
                    spans.push(Span::styled(
                        "+",
                        Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                    ));
                    continue;
                }
                let amp = self.grid.get(x, y).abs();
                let idx = ((amp * 5.0) as usize).min(shades.len() - 1);
                spans.push(Span::styled(
                    shades[idx].to_string(),
                    Style::default().fg(Color::Cyan),
                ));
            }
            lines.push(Line::from(spans));
        }
        lines
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Headless FIRST, before any terminal is touched — the Reaper reads this file.
    if args.iter().any(|a| a == "--headless") {
        let pos = args.iter().position(|a| a == "--headless").unwrap();
        let path = args.get(pos + 1).cloned().unwrap_or_else(|| ".".to_string());
        let limit = args
            .get(pos + 2)
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(120);
        let (notes, source) = load_notes(&path, limit);
        let total = notes.len();
        let mut bell = Bell::new(notes);
        while !bell.is_finished() {
            bell.step();
        }
        println!("git-resonance headless: rung {source}");
        println!("  notes struck (commits): {total}");
        println!("  listener peak:           {:.3}", bell.ledger.listener_peak);
        println!("  listener energy:         {:.1}", bell.ledger.listener_energy);
        print!("  chord fingerprint:      ");
        for v in &bell.ledger.fingerprint {
            print!("{v:+.2} ");
        }
        println!();
        return Ok(());
    }

    let path = args.get(1).cloned().unwrap_or_else(|| ".".to_string());
    let (notes, source) = load_notes(&path, 120);
    let mut tui = Tui::init()?;
    let mut bell = Bell::new(notes);
    let tick_rate = Duration::from_millis(40);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| {
            let area = f.area();
            let mut lines = vec![Line::from(Span::styled(
                format!(
                    " 🔔 git-resonance — {} · struck {} · peak {:.2} ",
                    source, bell.ledger.notes_struck, bell.ledger.listener_peak
                ),
                Style::default().fg(Color::Cyan),
            ))];
            lines.extend(bell.render_lines());
            lines.push(Line::from(format!(" ♪ {} ", bell.current_label)));
            lines.push(Line::from(" [r] ring it again · [q] quit "));
            let p = Paragraph::new(lines)
                .block(Block::default().title("The Repository Bell").borders(Borders::ALL));
            f.render_widget(p, area);
        })?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => {
                        let (notes, _) = load_notes(&path, 120);
                        bell = Bell::new(notes);
                    }
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            if !bell.is_finished() {
                bell.step();
            }
            last_tick = Instant::now();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use git_associates::Commit;
    use chrono::{TimeZone, Utc};

    fn fake_commit(hash: &str, churn: usize, parents: usize) -> Commit {
        Commit {
            hash: hash.to_string(),
            short_hash: hash[..7].to_string(),
            author: "A. Listener".to_string(),
            message: "strike the bell".to_string(),
            timestamp: Utc.timestamp_opt(1_700_000_000, 0).unwrap(),
            parents: (0..parents).map(|i| format!("p{i}")).collect(),
            stats: Some(git_associates::CommitStats {
                insertions: churn,
                deletions: 0,
                files_changed: 1,
            }),
            files: vec![],
        }
    }

    #[test]
    fn note_mapping_is_deterministic_and_in_bounds() {
        let c = fake_commit("d3b07384deadbeef", 400, 1);
        let a = note_from_commit(&c);
        let b = note_from_commit(&c);
        assert_eq!(a, b);
        assert!(a.x >= 2 && a.x < WIDTH - 2 && a.y >= 2 && a.y < HEIGHT - 2);
        // A 400-line churn rings louder than a 1-line fix.
        let quiet = note_from_commit(&fake_commit("d3b07384deadbeef", 1, 1));
        assert!(a.strength > quiet.strength);
        // Merges strike twice.
        assert!(note_from_commit(&fake_commit("aaaa0000bbbb", 10, 2)).double_strike);
    }

    #[test]
    fn the_bell_actually_rings() {
        let mut bell = Bell::new(demo_notes());
        while !bell.is_finished() {
            bell.step();
        }
        assert_eq!(bell.ledger.notes_struck, 12);
        assert!(
            bell.ledger.listener_peak > 0.01,
            "the listener should hear the history arrive: {:?}",
            bell.ledger
        );
        assert_eq!(bell.ledger.fingerprint.len(), 16);
    }

    #[test]
    fn different_histories_different_chords() {
        let ring = |notes: Vec<Note>| {
            let mut bell = Bell::new(notes);
            while !bell.is_finished() {
                bell.step();
            }
            bell.ledger.fingerprint
        };
        let a = ring(demo_notes());
        let mut other = demo_notes();
        other[3].x = 8;
        other[3].y = 8;
        other[5].strength = 2.4;
        let b = ring(other);
        assert_ne!(a, b, "change the history and the chord must change");
    }
}
